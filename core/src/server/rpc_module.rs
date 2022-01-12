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

use std::collections::hash_map::Entry;
use std::fmt::{self, Debug};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

use crate::error::{Error, SubscriptionClosed, SubscriptionClosedReason};
use crate::id_providers::RandomIntegerIdProvider;
use crate::server::helpers::MethodSink;
use crate::server::resource_limiting::{ResourceGuard, ResourceTable, ResourceVec, Resources};
use crate::to_json_raw_value;
use crate::traits::{IdProvider, ToRpcParams};
use beef::Cow;
use futures_channel::{mpsc, oneshot};
use futures_util::{future::BoxFuture, FutureExt, Stream, StreamExt};
use jsonrpsee_types::error::{invalid_subscription_err, ErrorCode, CALL_EXECUTION_FAILED_CODE};
use jsonrpsee_types::{
	Id, Params, Request, Response, SubscriptionId as RpcSubscriptionId, SubscriptionPayload, SubscriptionResponse,
};
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize};

/// A `MethodCallback` is an RPC endpoint, callable with a standard JSON-RPC request,
/// implemented as a function pointer to a `Fn` function taking four arguments:
/// the `id`, `params`, a channel the function uses to communicate the result (or error)
/// back to `jsonrpsee`, and the connection ID (useful for the websocket transport).
pub type SyncMethod = Arc<dyn Send + Sync + Fn(Id, Params, &MethodSink, MaybeConnState) -> bool>;
/// Similar to [`SyncMethod`], but represents an asynchronous handler and takes an additional argument containing a [`ResourceGuard`] if configured.
pub type AsyncMethod<'a> = Arc<
	dyn Send + Sync + Fn(Id<'a>, Params<'a>, MethodSink, ConnectionId, Option<ResourceGuard>) -> BoxFuture<'a, bool>,
>;
/// Connection ID, used for stateful protocol such as WebSockets.
/// For stateless protocols such as http it's unused, so feel free to set it some hardcoded value.
pub type ConnectionId = usize;
/// Raw RPC response.
pub type RawRpcResponse = (String, mpsc::UnboundedReceiver<String>, mpsc::UnboundedSender<String>);
/// Connection state for stateful protocols such as WebSocket
/// This is used to keep track whether the connection has been closed.
pub type MaybeConnState<'a> = Option<ConnState<'a>>;

/// Data for stateful connections.
pub struct ConnState<'a> {
	/// Connection ID
	pub conn_id: ConnectionId,
	/// Channel to know whether the connection is closed or not.
	pub close: async_channel::Receiver<()>,
	/// ID provider.
	pub id_provider: &'a dyn IdProvider,
}

impl<'a> std::fmt::Debug for ConnState<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Server").field("conn_id", &self.conn_id).field("close", &self.close).finish()
	}
}

type Subscribers = Arc<Mutex<FxHashMap<SubscriptionKey, (MethodSink, oneshot::Receiver<()>)>>>;

/// Represent a unique subscription entry based on [`RpcSubscriptionId`] and [`ConnectionId`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SubscriptionKey {
	conn_id: ConnectionId,
	sub_id: RpcSubscriptionId<'static>,
}

/// Callback wrapper that can be either sync or async.
#[derive(Clone)]
enum MethodKind {
	/// Synchronous method handler.
	Sync(SyncMethod),
	/// Asynchronous method handler.
	Async(AsyncMethod<'static>),
}

/// Information about resources the method uses during its execution. Initialized when the the server starts.
#[derive(Clone, Debug)]
enum MethodResources {
	/// Uninitialized resource table, mapping string label to units.
	Uninitialized(Box<[(&'static str, u16)]>),
	/// Initialized resource table containing units for each `ResourceId`.
	Initialized(ResourceTable),
}

/// Method callback wrapper that contains a sync or async closure,
/// plus a table with resources it needs to claim to run
#[derive(Clone, Debug)]
pub struct MethodCallback {
	callback: MethodKind,
	resources: MethodResources,
}

/// Result of a method, either direct value or a future of one.
pub enum MethodResult<T> {
	/// Result by value
	Sync(T),
	/// Future of a value
	Async(BoxFuture<'static, T>),
}

impl<T: Debug> Debug for MethodResult<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			MethodResult::Sync(result) => result.fmt(f),
			MethodResult::Async(_) => f.write_str("<future>"),
		}
	}
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

	/// Attempt to claim resources prior to executing a method. On success returns a guard that releases
	/// claimed resources when dropped.
	pub fn claim(&self, name: &str, resources: &Resources) -> Result<ResourceGuard, Error> {
		match self.resources {
			MethodResources::Uninitialized(_) => Err(Error::UninitializedMethod(name.into())),
			MethodResources::Initialized(units) => resources.claim(units),
		}
	}

	/// Execute the callback, sending the resulting JSON (success or error) to the specified sink.
	pub fn execute(
		&self,
		sink: &MethodSink,
		conn_state: MaybeConnState<'_>,
		req: Request<'_>,
		claimed: Option<ResourceGuard>,
	) -> MethodResult<bool> {
		let id = req.id.clone();
		let params = Params::new(req.params.map(|params| params.get()));

		let result = match &self.callback {
			MethodKind::Sync(callback) => {
				tracing::trace!(
					"[MethodCallback::execute] Executing sync callback, params={:?}, req.id={:?}, conn_state={:?}",
					params,
					id,
					conn_state
				);

				let result = (callback)(id, params, sink, conn_state);

				// Release claimed resources
				drop(claimed);

				MethodResult::Sync(result)
			}
			MethodKind::Async(callback) => {
				let sink = sink.clone();
				let params = params.into_owned();
				let id = id.into_owned();
				let conn_id = conn_state.map(|s| s.conn_id).unwrap_or(0);
				tracing::trace!(
					"[MethodCallback::execute] Executing async callback, params={:?}, req.id={:?}, conn_state={:?}",
					params,
					id,
					conn_id,
				);

				MethodResult::Async((callback)(id, params, sink, conn_id, claimed))
			}
		};

		result
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
	/// On success it returns a mut reference to the [`MethodCallback`] just inserted.
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

					// If resource capacity set to `0`, we ignore the unit value of the method
					// and set it to `0` as well, effectively making the resource unlimited.
					if resources.capacities[idx] == 0 {
						map[idx] = 0;
					} else {
						map[idx] = units;
					}
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

	/// Merge two [`Methods`]'s by adding all [`MethodCallback`]s from `other` into `self`.
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

	/// Returns the method callback along with its name. The returned name is same as the
	/// `method_name`, but its lifetime bound is `'static`.
	pub fn method_with_name(&self, method_name: &str) -> Option<(&'static str, &MethodCallback)> {
		self.callbacks.get_key_value(method_name).map(|(k, v)| (*k, v))
	}

	/// Attempt to execute a callback, sending the resulting JSON (success or error) to the specified sink.
	pub fn execute(&self, sink: &MethodSink, conn_state: MaybeConnState, req: Request) -> MethodResult<bool> {
		tracing::trace!("[Methods::execute] Executing request: {:?}", req);
		match self.callbacks.get(&*req.method) {
			Some(callback) => callback.execute(sink, conn_state, req, None),
			None => {
				sink.send_error(req.id, ErrorCode::MethodNotFound.into());
				MethodResult::Sync(false)
			}
		}
	}

	/// Attempt to execute a callback while checking that the call does not exhaust the available resources,
	// sending the resulting JSON (success or error) to the specified sink.
	pub fn execute_with_resources<'r>(
		&self,
		sink: &MethodSink,
		conn_state: MaybeConnState<'r>,
		req: Request<'r>,
		resources: &Resources,
	) -> Result<(&'static str, MethodResult<bool>), Cow<'r, str>> {
		tracing::trace!("[Methods::execute_with_resources] Executing request: {:?}", req);
		match self.callbacks.get_key_value(&*req.method) {
			Some((&name, callback)) => match callback.claim(&req.method, resources) {
				Ok(guard) => Ok((name, callback.execute(sink, conn_state, req, Some(guard)))),
				Err(err) => {
					tracing::error!("[Methods::execute_with_resources] failed to lock resources: {:?}", err);
					sink.send_error(req.id, ErrorCode::ServerIsBusy.into());
					Ok((name, MethodResult::Sync(false)))
				}
			},
			None => {
				sink.send_error(req.id, ErrorCode::MethodNotFound.into());
				Err(req.method)
			}
		}
	}

	/// Helper to call a method on the `RPC module` without having to spin up a server.
	///
	/// The params must be serializable as JSON array, see [`ToRpcParams`] for further documentation.
	///
	/// Returns the decoded value of the `result field` in JSON-RPC response if succesful.
	///
	/// # Examples
	///
	/// ```
	/// #[tokio::main]
	/// async fn main() {
	///     use jsonrpsee::RpcModule;
	///
	///     let mut module = RpcModule::new(());
	///     module.register_method("echo_call", |params, _| {
	///         params.one::<u64>().map_err(Into::into)
	///     }).unwrap();
	///
	///     let echo: u64 = module.call("echo_call", [1_u64]).await.unwrap();
	///     assert_eq!(echo, 1);
	/// }
	/// ```
	pub async fn call<Params: ToRpcParams, T: DeserializeOwned>(
		&self,
		method: &str,
		params: Params,
	) -> Result<T, Error> {
		let params = params.to_rpc_params()?;
		let req = Request::new(method.into(), Some(&params), Id::Number(0));
		tracing::trace!("[Methods::call] Calling method: {:?}, params: {:?}", method, params);
		let (resp, _, _) = self.inner_call(req).await;
		if let Ok(res) = serde_json::from_str::<Response<T>>(&resp) {
			return Ok(res.result);
		}
		Err(Error::Request(resp))
	}

	/// Make a request (JSON-RPC method call or subscription) by using raw JSON.
	///
	/// Returns the raw JSON response to the call and a stream to receive notifications if the call was a subscription.
	///
	/// # Examples
	///
	/// ```
	/// #[tokio::main]
	/// async fn main() {
	///     use jsonrpsee::RpcModule;
	///     use jsonrpsee::types::Response;
	///     use futures_util::StreamExt;
	///
	///     let mut module = RpcModule::new(());
	///     module.register_subscription("hi", "hi", "goodbye", |_, mut sink, _| {
	///         sink.send(&"one answer").unwrap();
	///         Ok(())
	///     }).unwrap();
	///     let (resp, mut stream) = module.raw_json_request(r#"{"jsonrpc":"2.0","method":"hi","id":0}"#).await.unwrap();
	///     let resp = serde_json::from_str::<Response<u64>>(&resp).unwrap();
	///     let sub_resp = stream.next().await.unwrap();
	///     assert_eq!(
	///         format!(r#"{{"jsonrpc":"2.0","method":"hi","params":{{"subscription":{},"result":"one answer"}}}}"#, resp.result),
	///         sub_resp
	///     );
	/// }
	/// ```
	pub async fn raw_json_request(&self, call: &str) -> Result<(String, mpsc::UnboundedReceiver<String>), Error> {
		tracing::trace!("[Methods::raw_json_request] {:?}", call);
		let req: Request = serde_json::from_str(call)?;
		let (resp, rx, _) = self.inner_call(req).await;
		Ok((resp, rx))
	}

	/// Wrapper over [`Methods::execute`] to execute a callback.
	async fn inner_call(&self, req: Request<'_>) -> RawRpcResponse {
		let (tx, mut rx) = mpsc::unbounded();
		let sink = MethodSink::new(tx.clone());
		let (_tx, rx_2) = async_channel::unbounded();

		let conn_state = Some(ConnState { conn_id: 0, close: rx_2, id_provider: &RandomIntegerIdProvider });

		if let MethodResult::Async(fut) = self.execute(&sink, conn_state, req) {
			fut.await;
		}

		let resp = rx.next().await.expect("tx and rx still alive; qed");
		(resp, rx, tx)
	}

	/// Helper to create a subscription on the `RPC module` without having to spin up a server.
	///
	/// The params must be serializable as JSON array, see [`ToRpcParams`] for further documentation.
	///
	/// Returns [`Subscription`] on succes which can used to get results from the subscriptions.
	///
	/// # Examples
	///
	/// ```
	/// #[tokio::main]
	/// async fn main() {
	///     use jsonrpsee::{RpcModule, types::EmptyParams};
	///
	///     let mut module = RpcModule::new(());
	///     module.register_subscription("hi", "hi", "goodbye", |_, mut sink, _| {
	///         sink.send(&"one answer").unwrap();
	///         Ok(())
	///     }).unwrap();
	///
	///     let mut sub = module.subscribe("hi", EmptyParams::new()).await.unwrap();
	///     // In this case we ignore the subscription ID,
	///     let (sub_resp, _sub_id) = sub.next::<String>().await.unwrap().unwrap();
	///     assert_eq!(&sub_resp, "one answer");
	/// }
	/// ```
	pub async fn subscribe(&self, sub_method: &str, params: impl ToRpcParams) -> Result<Subscription, Error> {
		let params = params.to_rpc_params()?;
		let req = Request::new(sub_method.into(), Some(&params), Id::Number(0));
		tracing::trace!("[Methods::subscribe] Calling subscription method: {:?}, params: {:?}", sub_method, params);
		let (response, rx, tx) = self.inner_call(req).await;
		let subscription_response = serde_json::from_str::<Response<RpcSubscriptionId>>(&response)?;
		let sub_id = subscription_response.result.into_owned();
		Ok(Subscription { sub_id, rx, tx })
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
			MethodCallback::new_sync(Arc::new(move |id, params, sink, _| match callback(params, &*ctx) {
				Ok(res) => sink.send_response(id, res),
				Err(err) => sink.send_call_error(id, err),
			})),
		)?;

		Ok(MethodResourcesBuilder { build: ResourceVec::new(), callback })
	}

	/// Register a new asynchronous RPC method, which computes the response with the given callback.
	pub fn register_async_method<R, Fun, Fut>(
		&mut self,
		method_name: &'static str,
		callback: Fun,
	) -> Result<MethodResourcesBuilder, Error>
	where
		R: Serialize + Send + Sync + 'static,
		Fut: Future<Output = Result<R, Error>> + Send,
		Fun: (Fn(Params<'static>, Arc<Context>) -> Fut) + Copy + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		let callback = self.methods.verify_and_insert(
			method_name,
			MethodCallback::new_async(Arc::new(move |id, params, sink, _, claimed| {
				let ctx = ctx.clone();
				let future = async move {
					let result = match callback(params, ctx).await {
						Ok(res) => sink.send_response(id, res),
						Err(err) => sink.send_call_error(id, err),
					};

					// Release claimed resources
					drop(claimed);

					result
				};
				future.boxed()
			})),
		)?;

		Ok(MethodResourcesBuilder { build: ResourceVec::new(), callback })
	}

	/// Register a new **blocking** synchronous RPC method, which computes the response with the given callback.
	/// Unlike the regular [`register_method`](RpcModule::register_method), this method can block its thread and perform expensive computations.
	pub fn register_blocking_method<R, F>(
		&mut self,
		method_name: &'static str,
		callback: F,
	) -> Result<MethodResourcesBuilder, Error>
	where
		Context: Send + Sync + 'static,
		R: Serialize,
		F: Fn(Params, Arc<Context>) -> Result<R, Error> + Copy + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		let callback = self.methods.verify_and_insert(
			method_name,
			MethodCallback::new_async(Arc::new(move |id, params, sink, _, claimed| {
				let ctx = ctx.clone();

				tokio::task::spawn_blocking(move || {
					let result = match callback(params, ctx) {
						Ok(res) => sink.send_response(id, res),
						Err(err) => sink.send_call_error(id, err),
					};

					// Release claimed resources
					drop(claimed);

					result
				})
				.map(|result| match result {
					Ok(r) => r,
					Err(err) => {
						tracing::error!("Join error for blocking RPC method: {:?}", err);
						false
					}
				})
				.boxed()
			})),
		)?;

		Ok(MethodResourcesBuilder { build: ResourceVec::new(), callback })
	}

	/// Register a new RPC subscription that invokes s callback on every subscription call.
	///
	/// This method ensures that the `subscription_method_name` and `unsubscription_method_name` are unique.
	/// The `notif_method_name` argument sets the content of the `method` field in the JSON document that
	/// the server sends back to the client. The uniqueness of this value is not machine checked and it's up to
	/// the user to ensure it is not used in any other [`RpcModule`] used in the server.
	///
	/// # Arguments
	///
	/// * `subscription_method_name` - name of the method to call to initiate a subscription
	/// * `notif_method_name` - name of method to be used in the subscription payload (technically a JSON-RPC notification)
	/// * `unsubscription_method` - name of the method to call to terminate a subscription
	/// *  `callback` - A callback to invoke on each subscription; it takes three parameters:
	///     - [`Params`]: JSON-RPC parameters in the subscription call.
	///     - [`SubscriptionSink`]: A sink to send messages to the subscriber.
	///     - Context: Any type that can be embedded into the [`RpcModule`].
	///
	/// # Examples
	///
	/// ```no_run
	///
	/// use jsonrpsee_core::server::rpc_module::RpcModule;
	///
	/// let mut ctx = RpcModule::new(99_usize);
	/// ctx.register_subscription("sub", "notif_name", "unsub", |params, mut sink, ctx| {
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
		notif_method_name: &'static str,
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
				MethodCallback::new_sync(Arc::new(move |id, params, method_sink, conn| {
					let (conn_tx, conn_rx) = oneshot::channel::<()>();
					let c = conn.expect("conn must be Some; this is bug");

					let sub_id = {
						let sub_id: RpcSubscriptionId = c.id_provider.next_id().into_owned();
						let uniq_sub = SubscriptionKey { conn_id: c.conn_id, sub_id: sub_id.clone() };

						subscribers.lock().insert(uniq_sub, (method_sink.clone(), conn_rx));

						sub_id
					};

					method_sink.send_response(id.clone(), &sub_id);

					let sink = SubscriptionSink {
						inner: method_sink.clone(),
						close: c.close,
						method: notif_method_name,
						subscribers: subscribers.clone(),
						uniq_sub: SubscriptionKey { conn_id: c.conn_id, sub_id },
						is_connected: Some(conn_tx),
					};
					if let Err(err) = callback(params, sink, ctx.clone()) {
						tracing::error!(
							"subscribe call '{}' failed: {:?}, request id={:?}",
							subscribe_method_name,
							err,
							id
						);
						method_sink.send_error(id, ErrorCode::ServerError(CALL_EXECUTION_FAILED_CODE).into())
					} else {
						true
					}
				})),
			);
		}

		{
			self.methods.mut_callbacks().insert(
				unsubscribe_method_name,
				MethodCallback::new_sync(Arc::new(move |id, params, sink, conn_state| {
					let conn = conn_state.expect("conn must be Some; this is bug");

					let sub_id = match params.one::<RpcSubscriptionId>() {
						Ok(sub_id) => sub_id,
						Err(_) => {
							tracing::error!(
								"unsubscribe call '{}' failed: couldn't parse subscription id={:?} request id={:?}",
								unsubscribe_method_name,
								params,
								id
							);
							let err =
								to_json_raw_value(&"Invalid subscription ID type, must be Integer or String").ok();
							return sink.send_error(id, invalid_subscription_err(err.as_deref()));
						}
					};
					let sub_id = sub_id.into_owned();

					if subscribers
						.lock()
						.remove(&SubscriptionKey { conn_id: conn.conn_id, sub_id: sub_id.clone() })
						.is_some()
					{
						sink.send_response(id, "Unsubscribed")
					} else {
						let err = to_json_raw_value(&format!(
							"Invalid subscription ID={}",
							serde_json::to_string(&sub_id).expect("valid JSON; qed")
						))
						.ok();
						sink.send_error(id, invalid_subscription_err(err.as_deref()))
					}
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
	inner: MethodSink,
	/// Close
	close: async_channel::Receiver<()>,
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
		if self.is_closed() {
			return Err(Error::SubscriptionClosed(SubscriptionClosedReason::ConnectionReset.into()));
		}
		let msg = self.build_message(result)?;
		self.inner_send(msg).map_err(Into::into)
	}

	/// Consume the sink by passing a stream to be sent via the sink.
	pub async fn add_stream<S, T>(mut self, mut stream: S)
	where
		S: Stream<Item = T> + Unpin,
		T: Serialize,
	{
		loop {
			tokio::select! {
				Some(item) = stream.next() => {
					if let Err(Error::SubscriptionClosed(_)) = self.send(&item) {
						break;
					}
				},
				// No messages should be sent over this channel (just ignore and continue)
				Some(_) = self.close.next() => {},
				// Stream or connection was dropped => close stream.
				else => break,
			}
		}
	}

	/// Returns whether this channel is closed without needing a context.
	pub fn is_closed(&self) -> bool {
		self.inner.is_closed()
	}

	fn build_message<T: Serialize>(&self, result: &T) -> Result<String, Error> {
		serde_json::to_string(&SubscriptionResponse::new(
			self.method.into(),
			SubscriptionPayload { subscription: self.uniq_sub.sub_id.clone(), result },
		))
		.map_err(Into::into)
	}

	fn inner_send(&mut self, msg: String) -> Result<(), Error> {
		let res = match self.is_connected.as_ref() {
			Some(conn) if !conn.is_canceled() => {
				// unbounded send only fails if the receiver has been dropped.
				self.inner.send_raw(msg).map_err(|_| Some(SubscriptionClosedReason::ConnectionReset))
			}
			Some(_) => Err(Some(SubscriptionClosedReason::Unsubscribed)),
			// NOTE(niklasad1): this should be unreachable, after the first error is detected the subscription is closed.
			None => Err(None),
		};

		// The subscription was already closed by the client
		// Close down the subscription but don't send a message to the client.
		if res.is_err() {
			self.inner_close(None);
		}

		res.map_err(|e| {
			let err = e.unwrap_or_else(|| SubscriptionClosedReason::Server("Close reason unknown".to_string()));
			Error::SubscriptionClosed(err.into())
		})
	}

	/// Close the subscription sink with a customized error message.
	pub fn close(&mut self, msg: &str) {
		let close_reason = SubscriptionClosedReason::Server(msg.to_string()).into();
		self.inner_close(Some(&close_reason));
	}

	fn inner_close(&mut self, close_reason: Option<&SubscriptionClosed>) {
		self.is_connected.take();
		if let Some((sink, _)) = self.subscribers.lock().remove(&self.uniq_sub) {
			tracing::debug!("Closing subscription: {:?} reason: {:?}", self.uniq_sub.sub_id, close_reason);
			if let Some(close_reason) = close_reason {
				let msg = self.build_message(close_reason).expect("valid json infallible; qed");
				let _ = sink.send_raw(msg);
			}
		}
	}
}

impl Drop for SubscriptionSink {
	fn drop(&mut self) {
		let err = SubscriptionClosedReason::Server("No close reason provided".into()).into();
		self.inner_close(Some(&err));
	}
}

/// Wrapper struct that maintains a subscription "mainly" for testing.
#[derive(Debug)]
pub struct Subscription {
	tx: mpsc::UnboundedSender<String>,
	rx: mpsc::UnboundedReceiver<String>,
	sub_id: RpcSubscriptionId<'static>,
}

impl Subscription {
	/// Close the subscription channel.
	pub fn close(&mut self) {
		self.tx.close_channel();
	}

	/// Get the subscription ID
	pub fn subscription_id(&self) -> &RpcSubscriptionId {
		&self.sub_id
	}

	/// Returns `Some((val, sub_id))` for the next element of type T from the underlying stream,
	/// otherwise `None` if the subscription was closed.
	///
	/// # Panics
	///
	/// If the decoding the value as `T` fails.
	pub async fn next<T: DeserializeOwned>(&mut self) -> Option<Result<(T, RpcSubscriptionId<'static>), Error>> {
		let raw = self.rx.next().await?;
		let res = match serde_json::from_str::<SubscriptionResponse<T>>(&raw) {
			Ok(r) => Ok((r.params.result, r.params.subscription.into_owned())),
			Err(_) => match serde_json::from_str::<SubscriptionResponse<SubscriptionClosed>>(&raw) {
				Ok(e) => Err(Error::SubscriptionClosed(e.params.result)),
				Err(e) => Err(e.into()),
			},
		};
		Some(res)
	}
}

impl Drop for Subscription {
	fn drop(&mut self) {
		self.close();
	}
}
