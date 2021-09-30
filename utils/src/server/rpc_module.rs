// Copyright 2019-2021 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::server::helpers::{send_error, send_response};
use crate::server::resource_limiting::{ResourceGuard, ResourceTable, ResourceVec, Resources};
use beef::Cow;
use futures_channel::{mpsc, oneshot};
use futures_util::{future::BoxFuture, FutureExt, StreamExt};
use jsonrpsee_types::{
	error::{CallError, Error, SubscriptionClosedError},
	traits::ToRpcParams,
	v2::{
		error::{CALL_EXECUTION_FAILED_CODE, UNKNOWN_ERROR_CODE},
		ErrorCode, ErrorObject, Id, Params, Request, Response, SubscriptionId as RpcSubscriptionId,
		SubscriptionPayload, SubscriptionResponse, TwoPointZero,
	},
};

use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::Serialize;
use serde_json::value::RawValue;
use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// A `MethodCallback` is an RPC endpoint, callable with a standard JSON-RPC request,
/// implemented as a function pointer to a `Fn` function taking four arguments:
/// the `id`, `params`, a channel the function uses to communicate the result (or error)
/// back to `jsonrpsee`, and the connection ID (useful for the websocket transport).
pub type SyncMethod = Arc<dyn Send + Sync + Fn(Id, Params, &MethodSink, ConnectionId)>;
/// Similar to [`SyncMethod`], but represents an asynchronous handler.
pub type AsyncMethod<'a> = Arc<dyn Send + Sync + Fn(Id<'a>, Params<'a>, MethodSink, ConnectionId, Option<ResourceGuard>) -> BoxFuture<'a, ()>>;
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
enum MethodKind {
	/// Synchronous method handler.
	Sync(SyncMethod),
	/// Asynchronous method handler.
	Async(AsyncMethod<'static>),
}

/// Information about resources the method uses during its execution.
#[derive(Clone, Debug)]
enum MethodResources {
	/// Unintialized resource table, mapping string label to units.
	Uninitialized(Box<[(&'static str, u16)]>),
	/// Intialized resource table containing units for each `ResourceId`.
	Initialized(ResourceTable<u16>),
}

/// Method callback wrapper that contains a sync or async closure,
/// plus a table with resources it needs to claim to run
#[derive(Clone, Debug)]
pub struct MethodCallback {
	callback: MethodKind,
	resources: MethodResources,
}

/// Builder for configuring resources used by a method.
#[derive(Debug)]
pub struct MethodResourcesBuilder<'a> {
	build: ResourceVec<(&'static str, u16)>,
	callback: &'a mut MethodCallback,
}

impl<'a> MethodResourcesBuilder<'a> {
	/// Define how many units of a given named resource the method uses during its execution.
	pub fn resource(mut self, label: &'static str, units: u16) -> Result<Self, Error> {
		self.build.try_push((label, units)).map_err(|_| Error::MaxResourcesReached)?;
		Ok(self)
	}
}

impl<'a> Drop for MethodResourcesBuilder<'a> {
	fn drop(&mut self) {
		self.callback.resources = MethodResources::Uninitialized(self.build[..].into());
	}
}

impl MethodCallback {
	fn new_sync(callback: SyncMethod) -> Self {
		MethodCallback { callback: MethodKind::Sync(callback), resources: MethodResources::Uninitialized([].into()) }
	}

	fn new_async(callback: AsyncMethod<'static>) -> Self {
		MethodCallback { callback: MethodKind::Async(callback), resources: MethodResources::Uninitialized([].into()) }
	}

	/// Execute the callback, sending the resulting JSON (success or error) to the specified sink.
	pub fn execute(&self, tx: &MethodSink, req: Request<'_>, conn_id: ConnectionId, claimed: Option<ResourceGuard>) -> Option<BoxFuture<'static, ()>> {
		let id = req.id.clone();
		let params = Params::new(req.params.map(|params| params.get()));

		match &self.callback {
			MethodKind::Sync(callback) => {
				log::trace!(
					"[MethodCallback::execute] Executing sync callback, params={:?}, req.id={:?}, conn_id={:?}",
					params,
					id,
					conn_id
				);
				(callback)(id, params, tx, conn_id);

				None
			}
			MethodKind::Async(callback) => {
				let tx = tx.clone();
				let params = params.into_owned();
				let id = id.into_owned();
				log::trace!(
					"[MethodCallback::execute] Executing async callback, params={:?}, req.id={:?}, conn_id={:?}",
					params,
					id,
					conn_id
				);

				Some((callback)(id, params, tx, conn_id, claimed))
			}
		}
	}
}

impl Debug for MethodKind {
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

	fn verify_method_name(&mut self, name: &'static str) -> Result<(), Error> {
		if self.callbacks.contains_key(name) {
			return Err(Error::MethodAlreadyRegistered(name.into()));
		}

		Ok(())
	}

	/// Inserts the method callback for a given name, or returns an error if the name was already taken.
	/// On success returns a mut reference to just inserted callback.
	fn verify_and_insert(
		&mut self,
		name: &'static str,
		callback: MethodCallback,
	) -> Result<&mut MethodCallback, Error> {
		match self.mut_callbacks().entry(name) {
			Entry::Occupied(_) => Err(Error::MethodAlreadyRegistered(name.into())),
			Entry::Vacant(vacant) => Ok(vacant.insert(callback)),
		}
	}

	/// Initialize resources for all methods in this collection. This method has no effect if called more than once.
	pub fn initialize_resources(mut self, resources: &Resources) -> Result<Self, Error> {
		let callbacks = self.mut_callbacks();

		for (&method_name, callback) in callbacks.iter_mut() {
			if let MethodResources::Uninitialized(uninit) = &callback.resources {
				let mut map = resources.defaults;

				for &(label, units) in uninit.iter() {
					let idx = match resources.labels.iter().position(|&l| l == label) {
						Some(idx) => idx,
						None => return Err(Error::ResourceNameNotFoundForMethod(label, method_name)),
					};

					map[idx] = units;
				}

				callback.resources = MethodResources::Initialized(map);
			}
		}

		Ok(self)
	}

	/// Helper for obtaining a mut ref to the callbacks HashMap.
	fn mut_callbacks(&mut self) -> &mut FxHashMap<&'static str, MethodCallback> {
		Arc::make_mut(&mut self.callbacks)
	}

	/// Merge two [`Methods`]'s by adding all [`MethodKind`]s from `other` into `self`.
	/// Fails if any of the methods in `other` is present already.
	pub fn merge(&mut self, other: impl Into<Methods>) -> Result<(), Error> {
		let mut other = other.into();

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
	pub fn execute(&self, tx: &MethodSink, req: Request<'_>, conn_id: ConnectionId) -> Option<BoxFuture<'static, ()>> {
		log::trace!("[Methods::execute] Executing request: {:?}", req);
		match self.callbacks.get(&*req.method) {
			Some(callback) => callback.execute(tx, req, conn_id, None),
			None => {
				send_error(req.id, tx, ErrorCode::MethodNotFound.into());
				None
			}
		}
	}

	/// Helper to call a method on the `RPC module` without having to spin up a server.
	///
	/// The params must be serializable as JSON array, see [`ToRpcParams`] for further documentation.
	pub async fn call_with<Params: ToRpcParams>(&self, method: &str, params: Params) -> Option<String> {
		let params = params.to_rpc_params().ok();
		self.call(method, params).await
	}

	/// Helper alternative to `execute`, useful for writing unit tests without having to spin
	/// a server up.
	pub async fn call(&self, method: &str, params: Option<Box<RawValue>>) -> Option<String> {
		let req = Request {
			jsonrpc: TwoPointZero,
			id: Id::Number(0),
			method: Cow::borrowed(method),
			params: params.as_deref(),
		};

		let (tx, mut rx) = mpsc::unbounded();

		if let Some(fut) = self.execute(&tx, req, 0) {
			fut.await;
		}

		rx.next().await
	}

	/// Test helper that sets up a subscription using the given `method`. Returns a tuple of the
	/// [`SubscriptionId`] and a channel on which subscription JSON payloads can be received.
	pub async fn test_subscription(
		&self,
		method: &str,
		params: Option<Box<RawValue>>,
	) -> (SubscriptionId, mpsc::UnboundedReceiver<String>) {
		log::trace!("[Methods::test_subscription] Calling subscription method: {:?}, params: {:?}", method, params);
		let req = Request {
			jsonrpc: TwoPointZero,
			id: Id::Number(0),
			method: Cow::borrowed(method),
			params: params.as_deref(),
		};

		let (tx, mut rx) = mpsc::unbounded();

		if let Some(fut) = self.execute(&tx, req, 0) {
			fut.await;
		}
		let response = rx.next().await.expect("Could not establish subscription.");
		let subscription_response = serde_json::from_str::<Response<SubscriptionId>>(&response)
			.unwrap_or_else(|_| panic!("Could not deserialize subscription response {:?}", response));
		let sub_id = subscription_response.result;
		(sub_id, rx)
	}

	/// Returns an `Iterator` with all the method names registered on this server.
	pub fn method_names(&self) -> impl Iterator<Item = &'static str> + '_ {
		self.callbacks.keys().copied()
	}
}

impl<Context> Deref for RpcModule<Context> {
	type Target = Methods;

	fn deref(&self) -> &Methods {
		&self.methods
	}
}

impl<Context> DerefMut for RpcModule<Context> {
	fn deref_mut(&mut self) -> &mut Methods {
		&mut self.methods
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
}

impl<Context> From<RpcModule<Context>> for Methods {
	fn from(module: RpcModule<Context>) -> Methods {
		module.methods
	}
}

impl<Context: Send + Sync + 'static> RpcModule<Context> {
	/// Register a new synchronous RPC method, which computes the response with the given callback.
	pub fn register_method<R, F>(
		&mut self,
		method_name: &'static str,
		callback: F,
	) -> Result<MethodResourcesBuilder, Error>
	where
		Context: Send + Sync + 'static,
		R: Serialize,
		F: Fn(Params, &Context) -> Result<R, Error> + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		let callback = self.methods.verify_and_insert(
			method_name,
			MethodCallback::new_sync(Arc::new(move |id, params, tx, _| {
				match callback(params, &*ctx) {
					Ok(res) => send_response(id, tx, res),
					Err(Error::Call(CallError::InvalidParams(e))) => {
						let error = ErrorObject { code: ErrorCode::InvalidParams, message: &e.to_string(), data: None };
						send_error(id, tx, error)
					}
					Err(Error::Call(CallError::Failed(e))) => {
						let err = ErrorObject {
							code: ErrorCode::ServerError(CALL_EXECUTION_FAILED_CODE),
							message: &e.to_string(),
							data: None,
						};
						send_error(id, tx, err)
					}
					Err(Error::Call(CallError::Custom { code, message, data })) => {
						let err = ErrorObject { code: code.into(), message: &message, data: data.as_deref() };
						send_error(id, tx, err)
					}
					// This should normally not happen because the most common use case is to
					// return `Error::Call` in `register_method`.
					Err(e) => {
						let err = ErrorObject {
							code: ErrorCode::ServerError(UNKNOWN_ERROR_CODE),
							message: &e.to_string(),
							data: None,
						};
						send_error(id, tx, err)
					}
				};
			})),
		)?;

		Ok(MethodResourcesBuilder { build: ResourceVec::new(), callback })
	}

	/// Register a new asynchronous RPC method, which computes the response with the given callback.
	pub fn register_async_method<R, F>(
		&mut self,
		method_name: &'static str,
		callback: F,
	) -> Result<MethodResourcesBuilder, Error>
	where
		R: Serialize + Send + Sync + 'static,
		F: Fn(Params<'static>, Arc<Context>) -> BoxFuture<'static, Result<R, Error>> + Copy + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		let callback = self.methods.verify_and_insert(
			method_name,
			MethodCallback::new_async(Arc::new(move |id, params, tx, claimed, _| {
				let ctx = ctx.clone();
				let future = async move {
					match callback(params, ctx).await {
						Ok(res) => send_response(id, &tx, res),
						Err(Error::Call(CallError::InvalidParams(e))) => {
							let error =
								ErrorObject { code: ErrorCode::InvalidParams, message: &e.to_string(), data: None };
							send_error(id, &tx, error)
						}
						Err(Error::Call(CallError::Failed(e))) => {
							let err = ErrorObject {
								code: ErrorCode::ServerError(CALL_EXECUTION_FAILED_CODE),
								message: &e.to_string(),
								data: None,
							};
							send_error(id, &tx, err)
						}
						Err(Error::Call(CallError::Custom { code, message, data })) => {
							let err = ErrorObject { code: code.into(), message: &message, data: data.as_deref() };
							send_error(id, &tx, err)
						}
						// This should normally not happen because the most common use case is to
						// return `Error::Call` in `register_async_method`.
						Err(e) => {
							let err = ErrorObject {
								code: ErrorCode::ServerError(UNKNOWN_ERROR_CODE),
								message: &e.to_string(),
								data: None,
							};
							send_error(id, &tx, err)
						}
					};

					drop(claimed);
				};
				future.boxed()
			})),
		)?;

		Ok(MethodResourcesBuilder { build: ResourceVec::new(), callback })
	}

	/// Register a new RPC subscription that invokes callback on every subscription request.
	/// The callback itself takes three parameters:
	///     - [`Params`]: JSONRPC parameters in the subscription request.
	///     - [`SubscriptionSink`]: A sink to send messages to the subscriber.
	///     - Context: Any type that can be embedded into the [`RpcModule`].
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
		F: Fn(Params, SubscriptionSink, Arc<Context>) -> Result<(), Error> + Send + Sync + 'static,
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
				MethodCallback::new_sync(Arc::new(move |id, params, method_sink, conn_id| {
					let (conn_tx, conn_rx) = oneshot::channel::<()>();
					let sub_id = {
						const JS_NUM_MASK: SubscriptionId = !0 >> 11;
						let sub_id = rand::random::<SubscriptionId>() & JS_NUM_MASK;
						let uniq_sub = SubscriptionKey { conn_id, sub_id };

						subscribers.lock().insert(uniq_sub, (method_sink.clone(), conn_rx));

						sub_id
					};

					send_response(id.clone(), method_sink, sub_id);

					let sink = SubscriptionSink {
						inner: method_sink.clone(),
						method: subscribe_method_name,
						subscribers: subscribers.clone(),
						uniq_sub: SubscriptionKey { conn_id, sub_id },
						is_connected: Some(conn_tx),
					};
					if let Err(err) = callback(params, sink, ctx.clone()) {
						log::error!(
							"subscribe call '{}' failed: {:?}, request id={:?}",
							subscribe_method_name,
							err,
							id
						);
						send_error(id, method_sink, ErrorCode::ServerError(-1).into());
					}
				})),
			);
		}

		{
			self.methods.mut_callbacks().insert(
				unsubscribe_method_name,
				MethodCallback::new_sync(Arc::new(move |id, params, tx, conn_id| {
					let sub_id = match params.one() {
						Ok(sub_id) => sub_id,
						Err(_) => {
							log::error!(
								"unsubscribe call '{}' failed: couldn't parse subscription id, request id={:?}",
								unsubscribe_method_name,
								id
							);
							send_error(id, tx, ErrorCode::ServerError(-1).into());
							return;
						}
					};
					subscribers.lock().remove(&SubscriptionKey { conn_id, sub_id });
					send_response(id, tx, "Unsubscribed");
				})),
			);
		}

		Ok(())
	}

	/// Register an alias for an existing_method. Alias uniqueness is enforced.
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
	/// MethodCallback.
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
	/// Send a message back to subscribers.
	pub fn send<T: Serialize>(&mut self, result: &T) -> Result<(), Error> {
		let msg = self.build_message(result)?;
		self.inner_send(msg).map_err(Into::into)
	}

	fn build_message<T: Serialize>(&self, result: &T) -> Result<String, Error> {
		serde_json::to_string(&SubscriptionResponse {
			jsonrpc: TwoPointZero,
			method: self.method,
			params: SubscriptionPayload { subscription: RpcSubscriptionId::Num(self.uniq_sub.sub_id), result },
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
		self.close(format!("Subscription: {} closed", self.uniq_sub.sub_id));
	}
}

fn subscription_closed_err(sub_id: u64) -> Error {
	Error::SubscriptionClosed(format!("Subscription {} closed", sub_id).into())
}

#[cfg(test)]
mod tests {
	use super::*;
	use jsonrpsee_types::v2;
	use serde::Deserialize;
	use std::collections::HashMap;

	#[test]
	fn rpc_modules_with_different_contexts_can_be_merged() {
		let cx = Vec::<u8>::new();
		let mut mod1 = RpcModule::new(cx);
		mod1.register_method("bla with Vec context", |_: Params, _| Ok(())).unwrap();
		let mut mod2 = RpcModule::new(String::new());
		mod2.register_method("bla with String context", |_: Params, _| Ok(())).unwrap();

		mod1.merge(mod2).unwrap();

		assert!(mod1.method("bla with Vec context").is_some());
		assert!(mod1.method("bla with String context").is_some());
	}

	#[test]
	fn rpc_context_modules_can_register_subscriptions() {
		let cx = ();
		let mut cxmodule = RpcModule::new(cx);
		let _subscription = cxmodule.register_subscription("hi", "goodbye", |_, _, _| Ok(()));

		assert!(cxmodule.method("hi").is_some());
		assert!(cxmodule.method("goodbye").is_some());
	}

	#[test]
	fn rpc_register_alias() {
		let mut module = RpcModule::new(());

		module.register_method("hello_world", |_: Params, _| Ok(())).unwrap();
		module.register_alias("hello_foobar", "hello_world").unwrap();

		assert!(module.method("hello_world").is_some());
		assert!(module.method("hello_foobar").is_some());
	}

	#[tokio::test]
	async fn calling_method_without_server() {
		// Call sync method with no params
		let mut module = RpcModule::new(());
		module.register_method("boo", |_: Params, _| Ok(String::from("boo!"))).unwrap();

		let result = module.call("boo", None).await.unwrap();
		assert_eq!(result, r#"{"jsonrpc":"2.0","result":"boo!","id":0}"#);

		// Call sync method with params
		module
			.register_method("foo", |params, _| {
				let n: u16 = params.one()?;
				Ok(n * 2)
			})
			.unwrap();
		let result = module.call_with("foo", [3]).await.unwrap();
		assert_eq!(result, r#"{"jsonrpc":"2.0","result":6,"id":0}"#);

		// Call sync method with bad param
		let result = module.call_with("foo", (false,)).await.unwrap();
		assert_eq!(
			result,
			r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"invalid type: boolean `false`, expected u16 at line 1 column 6"},"id":0}"#
		);

		// Call async method with params and context
		struct MyContext;
		impl MyContext {
			fn roo(&self, things: Vec<u8>) -> u16 {
				things.iter().sum::<u8>().into()
			}
		}
		let mut module = RpcModule::new(MyContext);
		module
			.register_async_method("roo", |params, ctx| {
				let ns: Vec<u8> = params.parse().expect("valid params please");
				async move { Ok(ctx.roo(ns)) }.boxed()
			})
			.unwrap();
		let result = module.call_with("roo", vec![12, 13]).await.unwrap();
		assert_eq!(result, r#"{"jsonrpc":"2.0","result":25,"id":0}"#);
	}

	#[tokio::test]
	async fn calling_method_without_server_using_proc_macro() {
		use jsonrpsee::{proc_macros::rpc, types::async_trait};
		// Setup
		#[derive(Debug, Deserialize, Serialize)]
		#[allow(unreachable_pub)]
		pub struct Gun {
			shoots: bool,
		}

		#[derive(Debug, Deserialize, Serialize)]
		#[allow(unreachable_pub)]
		pub struct Beverage {
			ice: bool,
		}

		#[rpc(server)]
		pub trait Cool {
			/// Sync method, no params.
			#[method(name = "rebel_without_cause")]
			fn rebel_without_cause(&self) -> Result<bool, Error>;

			/// Sync method.
			#[method(name = "rebel")]
			fn rebel(&self, gun: Gun, map: HashMap<u8, u8>) -> Result<String, Error>;

			/// Async method.
			#[method(name = "revolution")]
			async fn can_have_any_name(&self, beverage: Beverage, some_bytes: Vec<u8>) -> Result<String, Error>;
		}

		struct CoolServerImpl;

		#[async_trait]
		impl CoolServer for CoolServerImpl {
			fn rebel_without_cause(&self) -> Result<bool, Error> {
				Ok(false)
			}

			fn rebel(&self, gun: Gun, map: HashMap<u8, u8>) -> Result<String, Error> {
				Ok(format!("{} {:?}", map.values().len(), gun))
			}

			async fn can_have_any_name(&self, beverage: Beverage, some_bytes: Vec<u8>) -> Result<String, Error> {
				Ok(format!("drink: {:?}, phases: {:?}", beverage, some_bytes))
			}
		}
		let module = CoolServerImpl.into_rpc();

		// Call sync method with no params
		let result = module.call("rebel_without_cause", None).await.unwrap();
		assert_eq!(result, r#"{"jsonrpc":"2.0","result":false,"id":0}"#);

		// Call sync method with no params, alternative way.
		let result = module.call_with::<[u8; 0]>("rebel_without_cause", []).await.unwrap();
		assert_eq!(result, r#"{"jsonrpc":"2.0","result":false,"id":0}"#);

		// Call sync method with params
		let result = module.call_with("rebel", (Gun { shoots: true }, HashMap::<u8, u8>::default())).await.unwrap();
		assert_eq!(result, r#"{"jsonrpc":"2.0","result":"0 Gun { shoots: true }","id":0}"#);

		// Call sync method with bad params
		let result = module.call_with("rebel", (Gun { shoots: true }, false)).await.unwrap();
		assert_eq!(
			result,
			r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"invalid type: boolean `false`, expected a map at line 1 column 5"},"id":0}"#
		);

		// Call async method with params and context
		let result = module.call_with("revolution", (Beverage { ice: true }, vec![1, 2, 3])).await.unwrap();
		assert_eq!(result, r#"{"jsonrpc":"2.0","result":"drink: Beverage { ice: true }, phases: [1, 2, 3]","id":0}"#);
	}

	#[tokio::test]
	async fn subscribing_without_server() {
		let mut module = RpcModule::new(());
		module
			.register_subscription("my_sub", "my_unsub", |_, mut sink, _| {
				let mut stream_data = vec!['0', '1', '2'];
				std::thread::spawn(move || loop {
					log::debug!("This is your friendly subscription sending data.");
					if let Some(letter) = stream_data.pop() {
						if let Err(Error::SubscriptionClosed(_)) = sink.send(&letter) {
							return;
						}
					} else {
						return;
					}
					std::thread::sleep(std::time::Duration::from_millis(500));
				});
				Ok(())
			})
			.unwrap();

		let (sub_id, mut my_sub_stream) = module.test_subscription("my_sub", None).await;
		for i in (0..=2).rev() {
			let my_sub = my_sub_stream.next().await.unwrap();
			let my_sub = serde_json::from_str::<SubscriptionResponse<char>>(&my_sub).unwrap();
			assert_eq!(my_sub.params.result, std::char::from_digit(i, 10).unwrap());
			assert_eq!(my_sub.params.subscription, v2::params::SubscriptionId::Num(sub_id));
		}

		// The subscription is now closed
		let my_sub = my_sub_stream.next().await.unwrap();
		let my_sub = serde_json::from_str::<SubscriptionResponse<SubscriptionClosedError>>(&my_sub).unwrap();
		assert_eq!(my_sub.params.result, format!("Subscription: {} closed", sub_id).into());
	}
}
