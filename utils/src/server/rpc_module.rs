use crate::server::helpers::{send_error, send_response};
use futures_channel::mpsc;
use jsonrpsee_types::error::{CallError, Error};
use jsonrpsee_types::traits::RpcMethod;
use jsonrpsee_types::v2::error::{JsonRpcErrorCode, JsonRpcErrorObject, CALL_EXECUTION_FAILED_CODE};
use jsonrpsee_types::v2::params::{Id, JsonRpcNotificationParams, RpcParams, TwoPointZero};
use jsonrpsee_types::v2::response::JsonRpcSubscriptionResponse;

use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::value::to_raw_value;
use std::sync::Arc;
use std::convert::{TryFrom, TryInto};

/// A `Method` is an RPC endpoint, callable with a standard JSON-RPC request,
/// implemented as a function pointer to a `Fn` function taking four arguments:
/// the `id`, `params`, a channel the function uses to communicate the result (or error)
/// back to `jsonrpsee`, and the connection ID (useful for the websocket transport).
pub type Method = Box<dyn Send + Sync + Fn(Id, RpcParams, &MethodSink, ConnectionId) -> anyhow::Result<()>>;
/// A collection of registered [`Method`]s.
pub type Methods = FxHashMap<&'static str, Method>;
/// Connection ID, used for stateful protocol such as WebSockets.
/// For stateless protocols such as http it's unused, so feel free to set it some hardcoded value.
pub type ConnectionId = usize;
/// Subscription ID.
pub type SubscriptionId = u64;
/// Sink that is used to send back the result to the server for a specific method.
pub type MethodSink = mpsc::UnboundedSender<String>;

type Subscribers<P> = Arc<Mutex<FxHashMap<(ConnectionId, SubscriptionId), InnerSink<P>>>>;

/// Sets of JSON-RPC methods can be organized into a "module" that are in turn registered on server or,
/// alternatively, merged with other modules to construct a cohesive API.
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
	pub fn register_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
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
					Err(CallError::InvalidParams) => send_error(id, tx, JsonRpcErrorCode::InvalidParams.into()),
					Err(CallError::Failed(err)) => {
						log::error!("Call failed with: {}", err);
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

	/// Register a new RPC subscription, with subscribe and unsubscribe methods.
	pub fn register_subscription<P: DeserializeOwned + Send + Sync + 'static>(
		&mut self,
		subscribe_method_name: &'static str,
		unsubscribe_method_name: &'static str,
	) -> Result<SubscriptionSink<P>, Error> {
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
				Box::new(move |id, params, tx, conn| {
					let params = params.parse().or_else(|_| params.one().map_err(|_| CallError::InvalidParams)).ok();
					let sub_id = {
						const JS_NUM_MASK: SubscriptionId = !0 >> 11;

						let sub_id = rand::random::<SubscriptionId>() & JS_NUM_MASK;

						let inner = InnerSink { sink: tx.clone(), sub_id, params, method: subscribe_method_name };
						subscribers.lock().insert((conn, sub_id), inner);

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

	/// Convert a module into methods.
	pub fn into_methods(self) -> Methods {
		self.methods
	}

	/// Merge two [`RpcModule`]'s by adding all [`Method`]s from `other` into `self`.
	/// Fails if any of the methods in `other` is present already.
	pub fn merge(&mut self, other: RpcModule) -> Result<(), Error> {
		for name in other.methods.keys() {
			self.verify_method_name(name)?;
		}

		for (name, callback) in other.methods {
			self.methods.insert(name, callback);
		}

		Ok(())
	}
}

/// Similar to [`RpcModule`] but wraps an additional context argument that can be used
/// to access data during call execution.
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
	pub fn register_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
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
					Err(CallError::InvalidParams) => send_error(id, tx, JsonRpcErrorCode::InvalidParams.into()),
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

#[derive(Clone)]
pub struct SubscriptionSink<P> {
	method: &'static str,
	subscribers: Subscribers<P>,
}

impl<P> SubscriptionSink<P> {
	/// Send a message on all the subscriptions
	///
	/// If you have subscriptions with params/input you should most likely
	/// call `extract_with_input` to the process the input/params and send out
	/// the result on each subscription individually instead.
	pub fn send_all<T>(&mut self, result: &T) -> anyhow::Result<()>
	where
		T: Serialize,
	{
		let result = to_raw_value(result)?;

		let mut errored = Vec::new();
		let mut subs = self.subscribers.lock();

		for ((conn_id, sub_id), sender) in subs.iter() {
			let msg = serde_json::to_string(&JsonRpcSubscriptionResponse {
				jsonrpc: TwoPointZero,
				method: self.method,
				params: JsonRpcNotificationParams { subscription: *sub_id, result: &*result },
			})?;

			// Log broken connections
			if sender.sink.unbounded_send(msg).is_err() {
				errored.push((*conn_id, *sub_id));
			}
		}

		// Remove broken connections
		for entry in errored {
			subs.remove(&entry);
		}

		Ok(())
	}

	/// Extract subscriptions with input.
	/// Usually, you want process the input before sending back
	/// data on that subscription.
	pub fn extract_with_input(&self) -> Vec<InnerSinkWithParams<P>> {
		let mut subs = self.subscribers.lock();

		let mut input = Vec::new();
		*subs = std::mem::take(&mut *subs)
			.into_iter()
			.filter_map(|(k, v)| {
				if v.params.is_some() {
					let with_input = v.try_into().expect("is Some checked above; qed");
					input.push(with_input);
					None
				} else {
					Some((k, v))
				}
			})
			.collect();
		input
	}
}

pub struct InnerSink<P> {
	/// Sink.
	sink: mpsc::UnboundedSender<String>,
	/// Params.
	params: Option<P>,
	/// Subscription ID.
	sub_id: SubscriptionId,
	/// Method name
	method: &'static str,
}

pub struct InnerSinkWithParams<P> {
	/// Sink.
	sink: mpsc::UnboundedSender<String>,
	/// Params.
	params: P,
	/// Subscription ID.
	sub_id: SubscriptionId,
	/// Method name
	method: &'static str,
}

impl<P> TryFrom<InnerSink<P>> for InnerSinkWithParams<P> {
	type Error = ();

	fn try_from(other: InnerSink<P>) -> Result<Self, Self::Error> {
		match other.params {
			Some(params) => Ok(InnerSinkWithParams { sink: other.sink, params, sub_id: other.sub_id, method: other.method }),
			None => Err(())
		}
	}
}

impl<P> InnerSinkWithParams<P> {
	/// Send data on a specific subscription
	/// Note: a subscription can be "subscribed" to arbitary number of times with
	/// diffrent input/params.
	pub fn send<T>(&self, result: &T) -> anyhow::Result<()>
	where
		T: Serialize,
	{
		let result = to_raw_value(result)?;
		let msg = serde_json::to_string(&JsonRpcSubscriptionResponse {
			jsonrpc: TwoPointZero,
			method: self.method,
			params: JsonRpcNotificationParams { subscription: self.sub_id, result: &*result },
		})?;

		self.sink.unbounded_send(msg).map_err(|e| anyhow::anyhow!("{:?}", e))
	}

	/// Get the input/params of the subscrption.
	pub fn params(&self) -> &P {
		&self.params
	}
}
