use crate::server::helpers::{send_error, send_response};
use futures_channel::{mpsc, oneshot};
use jsonrpsee_types::error::{CallError, Error};
use jsonrpsee_types::traits::RpcMethod;
use jsonrpsee_types::v2::error::{JsonRpcErrorCode, JsonRpcErrorObject, CALL_EXECUTION_FAILED_CODE};
use jsonrpsee_types::v2::params::{Id, JsonRpcNotificationParams, RpcParams, TwoPointZero};
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
/// A collection of registered [`Method`]s.
pub type Methods = FxHashMap<&'static str, Method>;
/// Connection ID, used for stateful protocol such as WebSockets.
/// For stateless protocols such as http it's unused, so feel free to set it some hardcoded value.
pub type ConnectionId = usize;
/// Subscription ID.
pub type SubscriptionId = u64;
/// Sink that is used to send back the result to the server for a specific method.
pub type MethodSink = mpsc::UnboundedSender<String>;

type Subscribers = Arc<Mutex<FxHashMap<(ConnectionId, SubscriptionId), (MethodSink, oneshot::Receiver<()>)>>>;

/// Sets of JSON-RPC methods can be organized into a "module"s that are in turn registered on the server or,
/// alternatively, merged with other modules to construct a cohesive API.
#[derive(Default)]
pub struct RpcModule {
	methods: Methods,
	subscribers: Subscribers,
}

impl RpcModule {
	/// Instantiate a new `RpcModule`.
	pub fn new() -> Self {
		RpcModule { methods: Methods::default(), subscribers: Subscribers::default() }
	}

	/// Add context for this module, turning it into an `RpcContextModule`.
	pub fn with_context<Context>(self, ctx: Context) -> RpcContextModule<Context> {
		RpcContextModule { ctx: Arc::new(ctx), module: self, subscribers: Subscribers::default() }
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
	/// rpc_module.register_subscription("sub", "unsub", |params, mut sink| {
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
	/// ctx.register_subscription_with_context("sub", "unsub", |params, mut sink, ctx| {
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
				Box::new(move |id, params, method_sink, conn_id| {
					let (keep_alive_tx, keep_alive_rx) = oneshot::channel();
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

	/// Convert a module into methods. Consumes self.
	pub fn into_methods(self) -> Methods {
		self.into_module().into_methods()
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

	/// Close down the subscription by removing it from shared [`Subscribers`]
	/// and sends an unsubscribe response similar to how un-subscription requests are handled
	/// in the [`RpcModule`]
	///
	/// Doesn't do anything if the subscription has already been closed.
	fn close(&mut self) {
		if let Some(sink) = self.subscribers.lock().remove(&(self.conn_id, self.sub_id)) {
			send_response(Id::Number(self.sub_id), &sink.0, "Unsubscribed");
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
	fn rpc_context_modules_can_merge_with_rpc_module() {
		// Prove that we can merge an RpcContextModule with a RpcModule.
		let cx = Vec::<u8>::new();
		let mut cxmodule = RpcContextModule::new(cx);
		cxmodule.register_method("bla with context", |_: RpcParams, _| Ok(())).unwrap();
		let mut module = RpcModule::new();
		module.register_method("bla", |_: RpcParams| Ok(())).unwrap();

		// `merge` is a method on `RpcModule` => deref works
		cxmodule.merge(module).unwrap();
		let mut cx_methods = cxmodule.into_methods().keys().cloned().collect::<Vec<&str>>();
		cx_methods.sort();
		assert_eq!(cx_methods, vec!["bla", "bla with context"]);
	}

	#[test]
	fn rpc_context_modules_can_register_subscriptions() {
		let cx = ();
		let mut cxmodule = RpcContextModule::new(cx);
		let _subscription = cxmodule.register_subscription("hi", "goodbye", |_, _| Ok(()));

		let methods = cxmodule.into_methods().keys().cloned().collect::<Vec<&str>>();
		assert!(methods.contains(&"hi"));
		assert!(methods.contains(&"goodbye"));
	}
}
