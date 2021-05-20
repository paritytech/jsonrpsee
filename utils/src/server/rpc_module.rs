use crate::server::helpers::{send_error, send_response};
use futures_channel::{mpsc, oneshot};
use futures_util::{future::BoxFuture, FutureExt};
use jsonrpsee_types::error::{CallError, Error};
use jsonrpsee_types::traits::{AsyncRpcMethod, RpcMethod};
use jsonrpsee_types::v2::error::{JsonRpcErrorCode, JsonRpcErrorObject, CALL_EXECUTION_FAILED_CODE};
use jsonrpsee_types::v2::params::{Id, JsonRpcNotificationParams, OwnedId, OwnedRpcParams, RpcParams, TwoPointZero};
use jsonrpsee_types::v2::response::JsonRpcSubscriptionResponse;

use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::Serialize;
use serde_json::value::{to_raw_value, RawValue};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// A `Method` is an RPC endpoint, callable with a standard JSON-RPC request,
/// implemented as a function pointer to a `Fn` function taking four arguments:
/// the `id`, `params`, a channel the function uses to communicate the result (or error)
/// back to `jsonrpsee`, and the connection ID (useful for the websocket transport).
pub type Method = Box<dyn Send + Sync + Fn(Id, RpcParams, &MethodSink, ConnectionId) -> Result<(), Error>>;
/// Similar to [`Method`], but represents an asynchronous handler.
pub type AsyncMethod = Box<
	dyn Send + Sync + Fn(OwnedId, OwnedRpcParams, MethodSink, ConnectionId) -> BoxFuture<'static, Result<(), Error>>,
>;
/// A collection of registered [`Method`]s.
pub type Methods = FxHashMap<&'static str, Method>;
/// A collection of registered [`AsyncMethod`]s.
pub type AsyncMethods = FxHashMap<&'static str, AsyncMethod>;
/// Connection ID, used for stateful protocol such as WebSockets.
/// For stateless protocols such as http it's unused, so feel free to set it some hardcoded value.
pub type ConnectionId = usize;
/// Subscription ID.
pub type SubscriptionId = u64;
/// Sink that is used to send back the result to the server for a specific method.
pub type MethodSink = mpsc::UnboundedSender<String>;

type Subscribers = Arc<Mutex<FxHashMap<(ConnectionId, SubscriptionId), (MethodSink, oneshot::Receiver<()>)>>>;

/// Identifier of the method type.
#[derive(Debug, Copy, Clone)]
pub enum MethodType {
	/// Synchronous method handler.
	Sync,
	/// Asynchronous method handler.
	Async,
}

/// Sets of JSON-RPC methods can be organized into a "module"s that are in turn registered on the server or,
/// alternatively, merged with other modules to construct a cohesive API.
#[derive(Default)]
pub struct RpcModule {
	method_types: FxHashMap<&'static str, MethodType>,
	methods: Methods,
	async_methods: AsyncMethods,
	subscribers: Subscribers,
}

impl RpcModule {
	/// Instantiate a new `RpcModule`.
	pub fn new() -> Self {
		Self::default()
	}

	/// Add context for this module, turning it into an `RpcContextModule`.
	pub fn with_context<Context>(self, ctx: Context) -> RpcContextModule<Context> {
		RpcContextModule { ctx: Arc::new(ctx), module: self, subscribers: Subscribers::default() }
	}

	fn verify_method_name(&mut self, name: &str) -> Result<(), Error> {
		if self.methods.get(name).is_some() || self.async_methods.get(name).is_some() {
			return Err(Error::MethodAlreadyRegistered(name.into()));
		}

		Ok(())
	}

	/// Returns the type of the method handler, if any.
	pub fn method_type(&self, method_name: &str) -> Option<MethodType> {
		self.method_types.get(method_name).copied()
	}

	/// Returns the synchronous method.
	pub fn method(&self, method_name: &str) -> Option<&Method> {
		self.methods.get(method_name)
	}

	/// Returns the asynchronous method.
	pub fn async_method(&self, method_name: &str) -> Option<&AsyncMethod> {
		self.async_methods.get(method_name)
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
		self.method_types.insert(method_name, MethodType::Sync);

		Ok(())
	}

	/// Register a new asynchronous RPC method, which responds with a given callback.
	pub fn register_async_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize + Send + Sync + 'static,
		F: AsyncRpcMethod<R, CallError> + Copy + Send + Sync + 'static,
	{
		self.verify_method_name(method_name)?;

		self.async_methods.insert(
			method_name,
			Box::new(move |id, params, tx, _| {
				let future = async move {
					let params = params.borrowed();
					let id = id.borrowed();
					match callback(params).await {
						Ok(res) => send_response(id, &tx, res),
						Err(CallError::InvalidParams) => send_error(id, &tx, JsonRpcErrorCode::InvalidParams.into()),
						Err(CallError::Failed(err)) => {
							log::error!("Call failed with: {}", err);
							let err = JsonRpcErrorObject {
								code: JsonRpcErrorCode::ServerError(CALL_EXECUTION_FAILED_CODE),
								message: &err.to_string(),
								data: None,
							};
							send_error(id, &tx, err)
						}
					};
					Ok(())
				};
				future.boxed()
			}),
		);
		self.method_types.insert(method_name, MethodType::Async);

		Ok(())
	}

	/// Register a new RPC subscription that invokes callback on every subscription request.
	/// The callback itself takes two parameters:
	///     - RpcParams: JSONRPC parameters in the subscription request.
	///     - SubscriptionSink: A sink to send messages to the subscriber.
	///
	/// # Examples
	///
	/// ```no_run
	///
	/// use jsonrpsee_utils::server::rpc_module::RpcModule;
	///
	/// let mut rpc_module = RpcModule::new();
	/// rpc_module.register_subscription("sub", "unsub", |params, sink| {
	///     let x: usize = params.one()?;
	///     std::thread::spawn(move || {
	///         sink.send(&x)
	///     });
	///     Ok(())
	/// });
	/// ```
	pub fn register_subscription<F>(
		&mut self,
		subscribe_method_name: &'static str,
		unsubscribe_method_name: &'static str,
		callback: F,
	) -> Result<(), Error>
	where
		F: Fn(RpcParams, SubscriptionSink) -> Result<(), Error> + Send + Sync + 'static,
	{
		if subscribe_method_name == unsubscribe_method_name {
			return Err(Error::SubscriptionNameConflict(subscribe_method_name.into()));
		}

		self.verify_method_name(subscribe_method_name)?;
		self.verify_method_name(unsubscribe_method_name)?;

		{
			let subscribers = self.subscribers.clone();
			self.methods.insert(
				subscribe_method_name,
				Box::new(move |id, params, method_sink, conn| {
					let (online_tx, online_rx) = oneshot::channel::<()>();
					let sub_id = {
						const JS_NUM_MASK: SubscriptionId = !0 >> 11;
						let sub_id = rand::random::<SubscriptionId>() & JS_NUM_MASK;

						subscribers.lock().insert((conn, sub_id), (method_sink.clone(), online_rx));

						sub_id
					};

					send_response(id, method_sink, sub_id);
					let sink = SubscriptionSink {
						inner: method_sink.clone(),
						method: subscribe_method_name,
						sub_id,
						is_online: online_tx,
					};
					callback(params, sink)
				}),
			);
		}

		{
			let subscribers = self.subscribers.clone();
			self.methods.insert(
				unsubscribe_method_name,
				Box::new(move |id, params, tx, conn| {
					let sub_id = params.one()?;
					subscribers.lock().remove(&(conn, sub_id));
					send_response(id, tx, "Unsubscribed");

					Ok(())
				}),
			);
		}

		Ok(())
	}

	/// Merge two [`RpcModule`]'s by adding all [`Method`]s from `other` into `self`.
	/// Fails if any of the methods in `other` is present already.
	pub fn merge(&mut self, other: RpcModule) -> Result<(), Error> {
		for name in other.method_types.keys() {
			self.verify_method_name(name)?;
		}

		for (name, callback) in other.methods {
			self.methods.insert(name, callback);
			self.method_types.insert(name, MethodType::Sync);
		}

		for (name, callback) in other.async_methods {
			self.async_methods.insert(name, callback);
			self.method_types.insert(name, MethodType::Async);
		}

		Ok(())
	}
}

/// Similar to [`RpcModule`] but wraps an additional context argument that can be used
/// to access data during call execution.
pub struct RpcContextModule<Context> {
	ctx: Arc<Context>,
	module: RpcModule,
	subscribers: Subscribers,
}

impl<Context> RpcContextModule<Context> {
	/// Create a new module with a given shared `Context`.
	pub fn new(ctx: Context) -> Self {
		RpcContextModule { ctx: Arc::new(ctx), module: RpcModule::new(), subscribers: Subscribers::default() }
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

		self.module.method_types.insert(method_name, MethodType::Async);
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

	/// Register a new asynchronous RPC method, which responds with a given callback.
	pub fn register_async_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize + Send + Sync + 'static,
		F: Fn(RpcParams, Arc<Context>) -> BoxFuture<'static, Result<R, CallError>> + Copy + Send + Sync + 'static,
		Context: Sync + Send + 'static,
	{
		self.module.verify_method_name(method_name)?;
		let ctx = self.ctx.clone();

		self.module.method_types.insert(method_name, MethodType::Async);
		self.module.async_methods.insert(
			method_name,
			Box::new(move |id, params, tx, _| {
				let ctx = ctx.clone();
				let future = async move {
					let params = params.borrowed();
					let id = id.borrowed();
					match callback(params, ctx).await {
						Ok(res) => send_response(id, &tx, res),
						Err(CallError::InvalidParams) => send_error(id, &tx, JsonRpcErrorCode::InvalidParams.into()),
						Err(CallError::Failed(err)) => {
							log::error!("Call failed with: {}", err);
							let err = JsonRpcErrorObject {
								code: JsonRpcErrorCode::ServerError(CALL_EXECUTION_FAILED_CODE),
								message: &err.to_string(),
								data: None,
							};
							send_error(id, &tx, err)
						}
					};
					Ok(())
				};
				future.boxed()
			}),
		);

		Ok(())
	}

	/// Register a new RPC subscription that invokes callback on every subscription request.
	/// The callback itself takes three parameters:
	///     - RpcParams: JSONRPC parameters in the subscription request.
	///     - SubscriptionSink: A sink to send messages to the subscriber.
	///     - Context: Any type that can be embedded into the RpcContextModule.
	///
	/// # Examples
	///
	/// ```no_run
	///
	/// use jsonrpsee_utils::server::rpc_module::RpcContextModule;
	///
	/// let mut ctx = RpcContextModule::new(99_usize);
	/// ctx.register_subscription_with_context("sub", "unsub", |params, sink, ctx| {
	///     let x: usize = params.one()?;
	///     std::thread::spawn(move || {
	///         let sum = x + (*ctx);
	///         sink.send(&sum)
	///     });
	///     Ok(())
	/// });
	/// ```
	pub fn register_subscription_with_context<F>(
		&mut self,
		subscribe_method_name: &'static str,
		unsubscribe_method_name: &'static str,
		callback: F,
	) -> Result<(), Error>
	where
		Context: Send + Sync + 'static,
		F: Fn(RpcParams, SubscriptionSink, Arc<Context>) -> Result<(), Error> + Send + Sync + 'static,
	{
		if subscribe_method_name == unsubscribe_method_name {
			return Err(Error::SubscriptionNameConflict(subscribe_method_name.into()));
		}

		self.verify_method_name(subscribe_method_name)?;
		self.verify_method_name(unsubscribe_method_name)?;
		let ctx = self.ctx.clone();

		{
			let subscribers = self.subscribers.clone();
			self.methods.insert(
				subscribe_method_name,
				Box::new(move |id, params, method_sink, conn| {
					let (online_tx, online_rx) = oneshot::channel::<()>();
					let sub_id = {
						const JS_NUM_MASK: SubscriptionId = !0 >> 11;
						let sub_id = rand::random::<SubscriptionId>() & JS_NUM_MASK;

						subscribers.lock().insert((conn, sub_id), (method_sink.clone(), online_rx));

						sub_id
					};

					send_response(id, method_sink, sub_id);
					let sink = SubscriptionSink {
						inner: method_sink.clone(),
						method: subscribe_method_name,
						sub_id,
						is_online: online_tx,
					};
					callback(params, sink, ctx.clone())
				}),
			);
		}

		{
			let subscribers = self.subscribers.clone();
			self.methods.insert(
				unsubscribe_method_name,
				Box::new(move |id, params, tx, conn| {
					let sub_id = params.one()?;
					subscribers.lock().remove(&(conn, sub_id));
					send_response(id, tx, "Unsubscribed");

					Ok(())
				}),
			);
		}

		Ok(())
	}

	/// Convert this `RpcContextModule` into a regular `RpcModule` that can be registered on the `Server`.
	pub fn into_module(self) -> RpcModule {
		self.module
	}
}

impl<Cx> Deref for RpcContextModule<Cx> {
	type Target = RpcModule;
	fn deref(&self) -> &Self::Target {
		&self.module
	}
}

impl<Cx> DerefMut for RpcContextModule<Cx> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.module
	}
}

/// Represents a single subscription.
pub struct SubscriptionSink {
	/// Sink.
	inner: mpsc::UnboundedSender<String>,
	/// Method.
	method: &'static str,
	/// SubscriptionID,
	sub_id: SubscriptionId,
	/// Whether the subscriber is still alive (to avoid send messages that the subscriber is not interested in).
	is_online: oneshot::Sender<()>,
}

impl SubscriptionSink {
	/// Send message on this subscription.
	pub fn send<T: Serialize>(&self, result: &T) -> Result<(), Error> {
		let result = to_raw_value(result)?;
		self.send_raw_value(&result)
	}

	fn send_raw_value(&self, result: &RawValue) -> Result<(), Error> {
		let msg = serde_json::to_string(&JsonRpcSubscriptionResponse {
			jsonrpc: TwoPointZero,
			method: self.method,
			params: JsonRpcNotificationParams { subscription: self.sub_id, result: &*result },
		})?;

		self.inner_send(msg).map_err(Into::into)
	}

	fn inner_send(&self, msg: String) -> Result<(), Error> {
		if self.is_online() {
			self.inner.unbounded_send(msg).map_err(|e| Error::Internal(e.into_send_error()))
		} else {
			Err(Error::Custom("Subscription canceled".into()))
		}
	}

	fn is_online(&self) -> bool {
		!self.is_online.is_canceled()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn rpc_context_modules_can_merge_with_rpc_module() {
		// Prove that we can merge an RpcContextModule with a RpcModule.
		let cx = Vec::<u8>::new();
		let mut cxmodule = RpcContextModule::new(cx);
		cxmodule.register_method("bla with context", |_: RpcParams, _| Ok(())).unwrap();
		let mut module = RpcModule::new();
		module.register_method("bla", |_: RpcParams| Ok(())).unwrap();

		// `merge` is a method on `RpcModule` => deref works
		cxmodule.merge(module).unwrap();

		assert!(cxmodule.method(&"bla").is_some());
		assert!(cxmodule.method(&"bla with context").is_some());
	}

	#[test]
	fn rpc_context_modules_can_register_subscriptions() {
		let cx = ();
		let mut cxmodule = RpcContextModule::new(cx);
		let _subscription = cxmodule.register_subscription("hi", "goodbye", |_, _| Ok(()));

		assert!(cxmodule.method(&"hi").is_some());
		assert!(cxmodule.method(&"goodbye").is_some());
	}
}
