use crate::server::{RpcParams, SubscriptionId, SubscriptionSink};
use jsonrpsee_types::{error::InvalidParams, traits::RpcMethod, v2::error::CALL_EXECUTION_FAILED_CODE};
use jsonrpsee_types::{
	error::{CallError, Error},
	v2::error::{JsonRpcErrorCode, JsonRpcErrorObject},
};
use jsonrpsee_utils::server::{send_error, send_response, Methods};
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::Serialize;
use std::sync::Arc;

#[derive(Default)]
pub struct RpcModule {
	methods: Methods,
}

impl RpcModule {
	/// Instantiate a new `RpcModule`.
	pub fn new() -> Self {
		RpcModule { methods: Methods::default() }
	}

	/// Add context for this module, turning it into an `RpcContextModule`.
	pub fn with_context<Context>(self, ctx: Context) -> RpcContextModule<Context> {
		RpcContextModule { ctx: Arc::new(ctx), module: self }
	}

	fn verify_method_name(&mut self, name: &str) -> Result<(), Error> {
		if self.methods.get(name).is_some() {
			return Err(Error::MethodAlreadyRegistered(name.into()));
		}

		Ok(())
	}

	/// Register a new RPC method, which responds with a given callback.
	pub fn register_method<F, R>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize,
		F: RpcMethod<R, CallError>,
	{
		self.verify_method_name(method_name)?;

		self.methods.insert(
			method_name,
			Box::new(move |id, params, tx, _| {
				match callback(params) {
					Ok(res) => send_response(id, tx, res),
					// TODO: this looks wonky...
					Err(CallError::InvalidParams(InvalidParams)) => send_error(id, tx, JsonRpcErrorCode::InvalidParams.into()),
					Err(CallError::Failed(e)) => {
						// TODO: do something smart(-er) with the error?
						log::error!("Call failed with: {}", e);
						send_error(id, tx, JsonRpcErrorCode::ServerError(CALL_EXECUTION_FAILED_CODE).into())
					}
				};

				Ok(())
			}),
		);

		Ok(())
	}

	/// Register a new RPC subscription, with subscribe and unsubscribe methods.
	pub fn register_subscription(
		&mut self,
		subscribe_method_name: &'static str,
		unsubscribe_method_name: &'static str,
	) -> Result<SubscriptionSink, Error> {
		if subscribe_method_name == unsubscribe_method_name {
			return Err(Error::SubscriptionNameConflict(subscribe_method_name.into()));
		}

		self.verify_method_name(subscribe_method_name)?;
		self.verify_method_name(unsubscribe_method_name)?;

		let subscribers = Arc::new(Mutex::new(FxHashMap::default()));

		{
			let subscribers = subscribers.clone();
			self.methods.insert(
				subscribe_method_name,
				Box::new(move |id, _, tx, conn| {
					let sub_id = {
						const JS_NUM_MASK: SubscriptionId = !0 >> 11;

						let sub_id = rand::random::<SubscriptionId>() & JS_NUM_MASK;

						subscribers.lock().insert((conn, sub_id), tx.clone());

						sub_id
					};

					send_response(id, tx, sub_id);

					Ok(())
				}),
			);
		}

		{
			let subscribers = subscribers.clone();
			self.methods.insert(
				unsubscribe_method_name,
				Box::new(move |id, params, tx, conn| {
					let sub_id = params.one().map_err(|e| anyhow::anyhow!("{:?}", e))?;

					subscribers.lock().remove(&(conn, sub_id));

					send_response(id, tx, "Unsubscribed");

					Ok(())
				}),
			);
		}

		Ok(SubscriptionSink { method: subscribe_method_name, subscribers })
	}

	pub(crate) fn into_methods(self) -> Methods {
		self.methods
	}

	pub(crate) fn merge(&mut self, other: RpcModule) -> Result<(), Error> {
		for name in other.methods.keys() {
			self.verify_method_name(name)?;
		}

		for (name, callback) in other.methods {
			self.methods.insert(name, callback);
		}

		Ok(())
	}
}

pub struct RpcContextModule<Context> {
	ctx: Arc<Context>,
	module: RpcModule,
}

impl<Context> RpcContextModule<Context> {
	/// Create a new module with a given shared `Context`.
	pub fn new(ctx: Context) -> Self {
		RpcContextModule { ctx: Arc::new(ctx), module: RpcModule::new() }
	}

	/// Register a new RPC method, which responds with a given callback.
	pub fn register_method<F, R>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		Context: Send + Sync + 'static,
		R: Serialize,
		F: Fn(RpcParams, &Context) -> Result<R, CallError> + Send + Sync + 'static,
	{
		self.module.verify_method_name(method_name)?;

		let ctx = self.ctx.clone();

		self.module.methods.insert(
			method_name,
			Box::new(move |id, params, tx, _| {
				match callback(params, &*ctx) {
					Ok(res) => send_response(id, tx, res),
					Err(CallError::InvalidParams(_)) => send_error(id, tx, JsonRpcErrorCode::InvalidParams.into()),
					Err(CallError::Failed(err)) => {
						let err = JsonRpcErrorObject {
							code: JsonRpcErrorCode::ServerError(CALL_EXECUTION_FAILED_CODE),
							message: &err.to_string(),
							data: None,
						};
						send_error(id, tx, err)
					}
				};

				Ok(())
			}),
		);
		Ok(())
	}

	/// Convert this `RpcContextModule` into a regular `RpcModule` that can be registered on the `Server`.
	pub fn into_module(self) -> RpcModule {
		self.module
	}
}
