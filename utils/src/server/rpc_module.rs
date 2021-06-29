use crate::server::helpers::{send_error, send_response};
use futures_channel::{mpsc, oneshot};
use futures_util::{future::BoxFuture, FutureExt};
use jsonrpsee_types::error::{CallError, Error, SubscriptionClosedError};
use jsonrpsee_types::v2::error::{JsonRpcErrorCode, JsonRpcErrorObject, CALL_EXECUTION_FAILED_CODE};
use jsonrpsee_types::v2::params::{
	Id, JsonRpcSubscriptionParams, OwnedId, OwnedRpcParams, RpcParams, SubscriptionId as JsonRpcSubscriptionId,
	TwoPointZero,
};
use jsonrpsee_types::v2::request::{JsonRpcNotification, JsonRpcRequest};

use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::Serialize;
use std::fmt::Debug;
use std::sync::Arc;

/// A `Method` is an RPC endpoint, callable with a standard JSON-RPC request,
/// implemented as a function pointer to a `Fn` function taking four arguments:
/// the `id`, `params`, a channel the function uses to communicate the result (or error)
/// back to `jsonrpsee`, and the connection ID (useful for the websocket transport).
pub type SyncMethod = Arc<dyn Send + Sync + Fn(Id, RpcParams, &MethodSink, ConnectionId) -> Result<(), Error>>;
/// Similar to [`SyncMethod`], but represents an asynchronous handler.
pub type AsyncMethod = Arc<
	dyn Send + Sync + Fn(OwnedId, OwnedRpcParams, MethodSink, ConnectionId) -> BoxFuture<'static, Result<(), Error>>,
>;
/// Connection ID, used for stateful protocol such as WebSockets.
/// For stateless protocols such as http it's unused, so feel free to set it some hardcoded value.
pub type ConnectionId = usize;
/// Subscription ID.
pub type SubscriptionId = u64;
/// Sink that is used to send back the result to the server for a specific method.
pub type MethodSink = mpsc::UnboundedSender<String>;

type Subscribers = Arc<Mutex<FxHashMap<SubscriptionKey, (MethodSink, oneshot::Receiver<()>)>>>;

/// Represent a unique subscription entry based on [`SubscriptionId`] and [`ConnectionId`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct SubscriptionKey {
	conn_id: ConnectionId,
	sub_id: SubscriptionId,
}

/// Callback wrapper that can be either sync or async.
#[derive(Clone)]
pub enum MethodCallback {
	/// Synchronous method handler.
	Sync(SyncMethod),
	/// Asynchronous method handler.
	Async(AsyncMethod),
}

impl MethodCallback {
	/// Execute the callback, sending the resulting JSON (success or error) to the specified sink.
	pub async fn execute(&self, tx: &MethodSink, req: JsonRpcRequest<'_>, conn_id: ConnectionId) {
		let id = req.id.clone();
		let params = RpcParams::new(req.params.map(|params| params.get()));

		let result = match self {
			MethodCallback::Sync(callback) => (callback)(req.id.clone(), params, tx, conn_id),
			MethodCallback::Async(callback) => {
				let tx = tx.clone();
				let params = OwnedRpcParams::from(params);
				let id = OwnedId::from(req.id);

				(callback)(id, params, tx, conn_id).await
			}
		};

		if let Err(err) = result {
			log::error!("execution of method call '{}' failed: {:?}, request id={:?}", req.method, err, id);
			send_error(id, &tx, JsonRpcErrorCode::ServerError(-1).into());
		}
	}
}

impl Debug for MethodCallback {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Async(_) => write!(f, "Async"),
			Self::Sync(_) => write!(f, "Sync"),
		}
	}
}

/// Reference-counted, clone-on-write collection of synchronous and asynchronous methods.
#[derive(Default, Debug, Clone)]
pub struct Methods {
	callbacks: Arc<FxHashMap<&'static str, MethodCallback>>,
}

impl Methods {
	/// Creates a new empty [`Methods`].
	pub fn new() -> Self {
		Self::default()
	}

	fn verify_method_name(&mut self, name: &str) -> Result<(), Error> {
		if self.callbacks.contains_key(name) {
			return Err(Error::MethodAlreadyRegistered(name.into()));
		}

		Ok(())
	}

	/// Helper for obtaining a mut ref to the callbacks HashMap.
	fn mut_callbacks(&mut self) -> &mut FxHashMap<&'static str, MethodCallback> {
		Arc::make_mut(&mut self.callbacks)
	}

	/// Merge two [`Methods`]'s by adding all [`MethodCallback`]s from `other` into `self`.
	/// Fails if any of the methods in `other` is present already.
	pub fn merge(&mut self, mut other: Methods) -> Result<(), Error> {
		for name in other.callbacks.keys() {
			self.verify_method_name(name)?;
		}

		let callbacks = self.mut_callbacks();

		for (name, callback) in other.mut_callbacks().drain() {
			callbacks.insert(name, callback);
		}

		Ok(())
	}

	/// Returns the method callback.
	pub fn method(&self, method_name: &str) -> Option<&MethodCallback> {
		self.callbacks.get(method_name)
	}

	/// Attempt to execute a callback, sending the resulting JSON (success or error) to the specified sink.
	pub async fn execute(&self, tx: &MethodSink, req: JsonRpcRequest<'_>, conn_id: ConnectionId) {
		match self.callbacks.get(&*req.method) {
			Some(callback) => callback.execute(tx, req, conn_id).await,
			None => send_error(req.id, tx, JsonRpcErrorCode::MethodNotFound.into()),
		}
	}

	/// Returns a `Vec` with all the method names registered on this server.
	pub fn method_names(&self) -> Vec<&'static str> {
		self.callbacks.keys().copied().collect()
	}
}

/// Sets of JSON-RPC methods can be organized into a "module"s that are in turn registered on the server or,
/// alternatively, merged with other modules to construct a cohesive API. [`RpcModule`] wraps an additional context
/// argument that can be used to access data during call execution.
#[derive(Debug, Clone)]
pub struct RpcModule<Context> {
	ctx: Arc<Context>,
	methods: Methods,
}

impl<Context> RpcModule<Context> {
	/// Create a new module with a given shared `Context`.
	pub fn new(ctx: Context) -> Self {
		Self { ctx: Arc::new(ctx), methods: Default::default() }
	}

	/// Convert a module into methods. Consumes self.
	pub fn into_methods(self) -> Methods {
		self.methods
	}

	/// Merge two [`RpcModule`]'s by adding all [`Methods`] `other` into `self`.
	/// Fails if any of the methods in `other` is present already.
	pub fn merge<Context2>(&mut self, other: RpcModule<Context2>) -> Result<(), Error> {
		self.methods.merge(other.methods)?;

		Ok(())
	}
}

impl<Context: Send + Sync + 'static> RpcModule<Context> {
	/// Register a new synchronous RPC method, which computes the response with the given callback.
	pub fn register_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		Context: Send + Sync + 'static,
		R: Serialize,
		F: Fn(RpcParams, &Context) -> Result<R, CallError> + Send + Sync + 'static,
	{
		self.methods.verify_method_name(method_name)?;

		let ctx = self.ctx.clone();

		self.methods.mut_callbacks().insert(
			method_name,
			MethodCallback::Sync(Arc::new(move |id, params, tx, _| {
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
			})),
		);

		Ok(())
	}

	/// Register a new asynchronous RPC method, which computes the response with the given callback.
	pub fn register_async_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize + Send + Sync + 'static,
		F: Fn(RpcParams, Arc<Context>) -> BoxFuture<'static, Result<R, CallError>> + Copy + Send + Sync + 'static,
	{
		self.methods.verify_method_name(method_name)?;

		let ctx = self.ctx.clone();

		self.methods.mut_callbacks().insert(
			method_name,
			MethodCallback::Async(Arc::new(move |id, params, tx, _| {
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
			})),
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
		let ctx = self.ctx.clone();

		let subscribers = Subscribers::default();

		{
			let subscribers = subscribers.clone();
			self.methods.mut_callbacks().insert(
				subscribe_method_name,
				MethodCallback::Sync(Arc::new(move |id, params, method_sink, conn_id| {
					let (conn_tx, conn_rx) = oneshot::channel::<()>();
					let sub_id = {
						const JS_NUM_MASK: SubscriptionId = !0 >> 11;
						let sub_id = rand::random::<SubscriptionId>() & JS_NUM_MASK;
						let uniq_sub = SubscriptionKey { conn_id, sub_id };

						subscribers.lock().insert(uniq_sub, (method_sink.clone(), conn_rx));

						sub_id
					};

					send_response(id, method_sink, sub_id);
					let sink = SubscriptionSink {
						inner: method_sink.clone(),
						method: subscribe_method_name,
						subscribers: subscribers.clone(),
						uniq_sub: SubscriptionKey { conn_id, sub_id },
						is_connected: Some(conn_tx),
					};
					callback(params, sink, ctx.clone())
				})),
			);
		}

		{
			self.methods.mut_callbacks().insert(
				unsubscribe_method_name,
				MethodCallback::Sync(Arc::new(move |id, params, tx, conn_id| {
					let sub_id = params.one()?;
					subscribers.lock().remove(&SubscriptionKey { conn_id, sub_id });
					send_response(id, &tx, "Unsubscribed");

					Ok(())
				})),
			);
		}

		Ok(())
	}

	/// Register an `alias` name for an `existing_method`.
	pub fn register_alias(&mut self, alias: &'static str, existing_method: &'static str) -> Result<(), Error> {
		self.methods.verify_method_name(alias)?;

		let callback = match self.methods.callbacks.get(existing_method) {
			Some(callback) => callback.clone(),
			None => return Err(Error::MethodNotFound(existing_method.into())),
		};

		self.methods.mut_callbacks().insert(alias, callback);

		Ok(())
	}
}

/// Represents a single subscription.
#[derive(Debug)]
pub struct SubscriptionSink {
	/// Sink.
	inner: mpsc::UnboundedSender<String>,
	/// Method.
	method: &'static str,
	/// Unique subscription.
	uniq_sub: SubscriptionKey,
	/// Shared Mutex of subscriptions for this method.
	subscribers: Subscribers,
	/// A type to track whether the subscription is active (the subscriber is connected).
	///
	/// None - implies that the subscription as been closed.
	is_connected: Option<oneshot::Sender<()>>,
}

impl SubscriptionSink {
	/// Send message on this subscription.
	pub fn send<T: Serialize>(&mut self, result: &T) -> Result<(), Error> {
		let msg = self.build_message(result)?;
		self.inner_send(msg).map_err(Into::into)
	}

	fn build_message<T: Serialize>(&self, result: &T) -> Result<String, Error> {
		serde_json::to_string(&JsonRpcNotification {
			jsonrpc: TwoPointZero,
			method: self.method,
			params: JsonRpcSubscriptionParams {
				subscription: JsonRpcSubscriptionId::Num(self.uniq_sub.sub_id),
				result,
			},
		})
		.map_err(Into::into)
	}

	fn inner_send(&mut self, msg: String) -> Result<(), Error> {
		let res = if let Some(conn) = self.is_connected.as_ref() {
			if !conn.is_canceled() {
				// unbounded send only fails if the receiver has been dropped.
				self.inner.unbounded_send(msg).map_err(|_| subscription_closed_err(self.uniq_sub.sub_id))
			} else {
				Err(subscription_closed_err(self.uniq_sub.sub_id))
			}
		} else {
			Err(subscription_closed_err(self.uniq_sub.sub_id))
		};

		if let Err(e) = &res {
			self.close(e.to_string());
		}

		res
	}

	/// Close the subscription sink with a customized error message.
	pub fn close(&mut self, close_reason: String) {
		self.is_connected.take();
		if let Some((sink, _)) = self.subscribers.lock().remove(&self.uniq_sub) {
			let msg =
				self.build_message(&SubscriptionClosedError::from(close_reason)).expect("valid json infallible; qed");
			let _ = sink.unbounded_send(msg);
		}
	}
}

impl Drop for SubscriptionSink {
	fn drop(&mut self) {
		self.close(format!("Subscription: {} is closed and dropped", self.uniq_sub.sub_id));
	}
}

fn subscription_closed_err(sub_id: u64) -> Error {
	Error::SubscriptionClosed(format!("Subscription {} is closed but not yet dropped", sub_id).into())
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
		assert!(methods.method("bla with Vec context").is_some());
		assert!(methods.method("bla with String context").is_some());
	}

	#[test]
	fn rpc_context_modules_can_register_subscriptions() {
		let cx = ();
		let mut cxmodule = RpcModule::new(cx);
		let _subscription = cxmodule.register_subscription("hi", "goodbye", |_, _, _| Ok(()));

		let methods = cxmodule.into_methods();
		assert!(methods.method("hi").is_some());
		assert!(methods.method("goodbye").is_some());
	}

	#[test]
	fn rpc_register_alias() {
		let mut module = RpcModule::new(());

		module.register_method("hello_world", |_: RpcParams, _| Ok(())).unwrap();
		module.register_alias("hello_foobar", "hello_world").unwrap();

		let methods = module.into_methods();

		assert!(methods.method("hello_world").is_some());
		assert!(methods.method("hello_foobar").is_some());
	}
}
