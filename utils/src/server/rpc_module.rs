use crate::server::helpers::{send_error, send_response};
use futures_channel::{mpsc, oneshot};
use futures_util::{future::BoxFuture, FutureExt};
use jsonrpsee_types::error::{CallError, Error};
use jsonrpsee_types::v2::error::{JsonRpcErrorCode, JsonRpcErrorObject, CALL_EXECUTION_FAILED_CODE};
use jsonrpsee_types::v2::params::{Id, JsonRpcNotificationParams, OwnedId, OwnedRpcParams, RpcParams, TwoPointZero};
use jsonrpsee_types::v2::response::JsonRpcSubscriptionResponse;

use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::Serialize;
use serde_json::value::{to_raw_value, RawValue};
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

/// Collection of synchronous and asynchronous methods.
#[derive(Default)]
pub struct MethodsHolder {
	method_types: FxHashMap<&'static str, MethodType>,
	methods: Methods,
	async_methods: AsyncMethods,
}

impl MethodsHolder {
	/// Creates a new empty [`MethodsHolder`].
	pub fn new() -> Self {
		Self::default()
	}

	fn verify_method_name(&mut self, name: &str) -> Result<(), Error> {
		if self.methods.get(name).is_some() || self.async_methods.get(name).is_some() {
			return Err(Error::MethodAlreadyRegistered(name.into()));
		}

		Ok(())
	}

	/// Merge two [`MethodsHolder`]'s by adding all [`Method`]s and [`AsyncMethod`]s from `other` into `self`.
	/// Fails if any of the methods in `other` is present already.
	pub fn merge(&mut self, other: MethodsHolder) -> Result<(), Error> {
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

	/// Returns a `Vec` with all the method names registered on this server.
	pub fn method_names(&self) -> Vec<String> {
		self.method_types.keys().map(|name| name.to_string()).collect()
	}
}

/// Sets of JSON-RPC methods can be organized into a "module"s that are in turn registered on the server or,
/// alternatively, merged with other modules to construct a cohesive API. [`RpcModule`] wraps an additional context
/// argument that can be used to access data during call execution.
pub struct RpcModule<Context> {
	ctx: Arc<Context>,
	methods: MethodsHolder,
	subscribers: Subscribers,
}

impl<Context: Send + Sync + 'static> RpcModule<Context> {
	/// Create a new module with a given shared `Context`.
	pub fn new(ctx: Context) -> Self {
		Self { ctx: Arc::new(ctx), methods: Default::default(), subscribers: Default::default() }
	}
	/// Register a new RPC method, which responds with a given callback.
	pub fn register_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		Context: Send + Sync + 'static,
		R: Serialize,
		F: Fn(RpcParams, &Context) -> Result<R, CallError> + Send + Sync + 'static,
	{
		self.methods.verify_method_name(method_name)?;

		let ctx = self.ctx.clone();

		self.methods.methods.insert(
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
		self.methods.method_types.insert(method_name, MethodType::Sync);

		Ok(())
	}

	/// Register a new asynchronous RPC method, which responds with a given callback.
	pub fn register_async_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize + Send + Sync + 'static,
		F: Fn(RpcParams, Arc<Context>) -> BoxFuture<'static, Result<R, CallError>> + Copy + Send + Sync + 'static,
	{
		self.methods.verify_method_name(method_name)?;

		let ctx = self.ctx.clone();

		self.methods.async_methods.insert(
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
		self.methods.method_types.insert(method_name, MethodType::Async);

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
	/// use jsonrpsee_utils::server::rpc_module::RpcModule;
	///
	/// let mut ctx = RpcModule::new(99_usize);
	/// ctx.register_subscription("sub", "unsub", |params, mut sink, ctx| {
	///     let x: usize = params.one()?;
	///     std::thread::spawn(move || {
	///         let sum = x + (*ctx);
	///         sink.send(&sum)
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
		Context: Send + Sync + 'static,
		F: Fn(RpcParams, SubscriptionSink, Arc<Context>) -> Result<(), Error> + Send + Sync + 'static,
	{
		if subscribe_method_name == unsubscribe_method_name {
			return Err(Error::SubscriptionNameConflict(subscribe_method_name.into()));
		}

		self.methods.verify_method_name(subscribe_method_name)?;
		self.methods.verify_method_name(unsubscribe_method_name)?;
		self.methods.method_types.insert(subscribe_method_name, MethodType::Sync);
		self.methods.method_types.insert(unsubscribe_method_name, MethodType::Sync);
		let ctx = self.ctx.clone();

		let subscribers = Subscribers::default();

		{
			let subscribers = self.subscribers.clone();
			self.methods.methods.insert(
				subscribe_method_name,
				Box::new(move |id, params, method_sink, conn_id| {
					let (keep_alive_tx, keep_alive_rx) = oneshot::channel::<()>();
					let sub_id = {
						const JS_NUM_MASK: SubscriptionId = !0 >> 11;
						let sub_id = rand::random::<SubscriptionId>() & JS_NUM_MASK;

						subscribers.lock().insert((conn_id, sub_id), (method_sink.clone(), keep_alive_rx));

						sub_id
					};

					send_response(id, method_sink, sub_id);
					let sink = SubscriptionSink {
						inner: method_sink.clone(),
						method: subscribe_method_name,
						sub_id,
						keep_alive: Some(KeepAlive {
							subscribers: subscribers.clone(),
							sub_id,
							conn_id,
							keep_alive: keep_alive_tx,
						}),
					};
					callback(params, sink, ctx.clone())
				}),
			);
		}

		{
			let subscribers = subscribers.clone();
			self.methods.methods.insert(
				unsubscribe_method_name,
				Box::new(move |id, params, _, conn_id| {
					let sub_id = params.one()?;
					if let Some(sink) = subscribers.lock().remove(&(conn_id, sub_id)) {
						send_response(id, &sink.0, "Unsubscribe");
					}

					Ok(())
				}),
			);
		}

		Ok(())
	}

	/// Convert a module into methods. Consumes self.
	pub fn into_methods(self) -> MethodsHolder {
		self.methods
	}

	/// Merge two [`RpcModule`]'s by adding all [`Method`]s from `other` into `self`.
	/// Fails if any of the methods in `other` is present already.
	pub fn merge<Context2>(&mut self, other: RpcModule<Context2>) -> Result<(), Error> {
		self.methods.merge(other.methods)?;

		Ok(())
	}
}

/// Represents a single subscription.
pub struct SubscriptionSink {
	/// Sink.
	inner: mpsc::UnboundedSender<String>,
	/// Method.
	method: &'static str,
	/// Subscription ID.
	sub_id: SubscriptionId,
	/// Whether the subscription is still connected or not.
	keep_alive: Option<KeepAlive>,
}

impl SubscriptionSink {
	/// Send message on this subscription.
	pub fn send<T: Serialize>(&mut self, result: &T) -> Result<(), Error> {
		let result = to_raw_value(result)?;
		self.send_raw_value(&result)
	}

	/// Close down the subscription if it's still online.
	///
	/// Note, this doesn't actual send an unsubscribe response because we can't
	/// map it to an actual request.
	pub fn close(&mut self) {
		self.keep_alive.as_mut().map(|k| k.close());
	}

	fn send_raw_value(&mut self, result: &RawValue) -> Result<(), Error> {
		let msg = serde_json::to_string(&JsonRpcSubscriptionResponse {
			jsonrpc: TwoPointZero,
			method: self.method,
			params: JsonRpcNotificationParams { subscription: self.sub_id, result: &*result },
		})?;

		self.inner_send(msg).map_err(Into::into)
	}

	fn inner_send(&mut self, msg: String) -> Result<(), Error> {
		let res = if let Some(online) = self.keep_alive.as_ref() {
			if online.is_canceled() {
				return Err(Error::SubscriptionClosed);
			}
			match self.inner.unbounded_send(msg) {
				Ok(()) => Ok(()),
				// Unbounded channel can only fail when the receiver has been dropped.
				Err(_e) => Err(Error::SubscriptionClosed),
			}
		} else {
			Err(Error::SubscriptionClosed)
		};

		if res.is_err() {
			self.keep_alive.take();
		}

		res
	}
}

/// A type to keep tracks whether the subscription has been canceled.
struct KeepAlive {
	subscribers: Subscribers,
	keep_alive: oneshot::Sender<()>,
	conn_id: ConnectionId,
	sub_id: SubscriptionId,
}

impl KeepAlive {
	fn is_canceled(&self) -> bool {
		self.keep_alive.is_canceled()
	}

	/// Close down the subscription by removing it from shared [`Subscribers`].
	///
	/// Note, this doesn't actual send an unsubscribe response because we can't
	// map it to an actual request.
	fn close(&mut self) {
		if let Some((sink, _)) = self.subscribers.lock().remove(&(self.conn_id, self.sub_id)) {
			// TODO: better way to handle this?!
			let _ = sink.unbounded_send(format!("Subscription: {} is closed", self.sub_id));
		}
	}
}

impl Drop for KeepAlive {
	fn drop(&mut self) {
		self.close();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn rpc_modules_with_different_contexts_can_be_merged() {
		let cx = Vec::<u8>::new();
		let mut mod1 = RpcModule::new(cx);
		mod1.register_method("bla with Vec context", |_: RpcParams, _| Ok(())).unwrap();
		let mut mod2 = RpcModule::new(String::new());
		mod2.register_method("bla with String context", |_: RpcParams, _| Ok(())).unwrap();

		mod1.merge(mod2).unwrap();

		let methods = mod1.into_methods();
		assert!(methods.method(&"bla with Vec context").is_some());
		assert!(methods.method(&"bla with String context").is_some());
	}

	#[test]
	fn rpc_context_modules_can_register_subscriptions() {
		let cx = ();
		let mut cxmodule = RpcModule::new(cx);
		let _subscription = cxmodule.register_subscription("hi", "goodbye", |_, _, _| Ok(()));

		let methods = cxmodule.into_methods();
		assert!(methods.method(&"hi").is_some());
		assert!(methods.method(&"goodbye").is_some());
	}
}
