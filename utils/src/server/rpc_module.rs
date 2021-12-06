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

use crate::server::helpers::MethodSink;
use crate::server::resource_limiting::{ResourceGuard, ResourceTable, ResourceVec, Resources};
use beef::Cow;
use futures_channel::{mpsc, oneshot};
use futures_util::{future::BoxFuture, FutureExt, StreamExt};
use jsonrpsee_types::to_json_raw_value;
use jsonrpsee_types::v2::error::{invalid_subscription_err, CALL_EXECUTION_FAILED_CODE};
use jsonrpsee_types::{
	error::{Error, SubscriptionClosedError},
	traits::ToRpcParams,
	v2::{
		ErrorCode, Id, Params, Request, Response, SubscriptionId as RpcSubscriptionId, SubscriptionPayload,
		SubscriptionResponse,
	},
	DeserializeOwned,
};

use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::Serialize;
use serde_json::value::RawValue;
use std::collections::hash_map::Entry;
use std::fmt::{self, Debug};
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// A `MethodCallback` is an RPC endpoint, callable with a standard JSON-RPC request,
/// implemented as a function pointer to a `Fn` function taking four arguments:
/// the `id`, `params`, a channel the function uses to communicate the result (or error)
/// back to `jsonrpsee`, and the connection ID (useful for the websocket transport).
pub type SyncMethod = Arc<dyn Send + Sync + Fn(Id, Params, &MethodSink, ConnectionId) -> bool>;
/// Similar to [`SyncMethod`], but represents an asynchronous handler and takes an additional argument containing a [`ResourceGuard`] if configured.
pub type AsyncMethod<'a> =
	Arc<dyn Send + Sync + Fn(Id<'a>, Params<'a>, MethodSink, Option<ResourceGuard>) -> BoxFuture<'a, bool>>;
/// Connection ID, used for stateful protocol such as WebSockets.
/// For stateless protocols such as http it's unused, so feel free to set it some hardcoded value.
pub type ConnectionId = usize;
/// Subscription ID.
pub type SubscriptionId = u64;
/// Raw RPC response.
pub type RawRpcResponse = (String, mpsc::UnboundedReceiver<String>, mpsc::UnboundedSender<String>);


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
		req: Request<'_>,
		conn_id: ConnectionId,
		claimed: Option<ResourceGuard>,
	) -> MethodResult<bool> {
		let id = req.id.clone();
		let params = Params::new(req.params.map(|params| params.get()));

		let result = match &self.callback {
			MethodKind::Sync(callback) => {
				tracing::trace!(
					"[MethodCallback::execute] Executing sync callback, params={:?}, req.id={:?}, conn_id={:?}",
					params,
					id,
					conn_id
				);

				let result = (callback)(id, params, sink, conn_id);

				// Release claimed resources
				drop(claimed);

				MethodResult::Sync(result)
			}
			MethodKind::Async(callback) => {
				let sink = sink.clone();
				let params = params.into_owned();
				let id = id.into_owned();
				tracing::trace!(
					"[MethodCallback::execute] Executing async callback, params={:?}, req.id={:?}, conn_id={:?}",
					params,
					id,
					conn_id
				);

				MethodResult::Async((callback)(id, params, sink, claimed))
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
	pub fn execute(&self, sink: &MethodSink, req: Request, conn_id: ConnectionId) -> MethodResult<bool> {
		tracing::trace!("[Methods::execute] Executing request: {:?}", req);
		match self.callbacks.get(&*req.method) {
			Some(callback) => callback.execute(sink, req, conn_id, None),
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
		req: Request<'r>,
		conn_id: ConnectionId,
		resources: &Resources,
	) -> Result<(&'static str, MethodResult<bool>), Cow<'r, str>> {
		tracing::trace!("[Methods::execute_with_resources] Executing request: {:?}", req);
		match self.callbacks.get_key_value(&*req.method) {
			Some((&name, callback)) => match callback.claim(&req.method, resources) {
				Ok(guard) => Ok((name, callback.execute(sink, req, conn_id, Some(guard)))),
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
	pub async fn call<Params: ToRpcParams, T: DeserializeOwned>(&self, method: &str, params: Params) -> Result<T, String> {
		let params = params.to_rpc_params().map_err(|e| e.to_string())?;
		let (resp, _, _) = self.raw_call(method, params).await;
		if let Ok(res) = serde_json::from_str::<Response<T>>(&resp) {
			return Ok(res.result);
		}
		Err(resp)
	}

	/// Perform one or more raw "in memory JSON-RPC method calls".
	/// 
	/// You can use this to support method calls and subscriptions
	///
	/// There are better variants than this method if you only want
	/// method calls or only subscriptions.
	///
	/// See [`Methods::test_subscription`] and [`Methods::call`] for
	/// for further documentation.
	///
	/// Returns a response to the actual method call and a stream to process
	/// for further notifications if a subscription was registered by the call.
	///
	/// ```
	/// #[tokio::main]
	/// async fn main() {
	///     use jsonrpsee::RpcModule;
	///     use jsonrpsee::types::{
	///         EmptyParams,
	///         v2::{Response, SubscriptionResponse},
	///         traits::ToRpcParams,
	///     };
	///     use futures_util::StreamExt;
	///
	///     let mut module = RpcModule::new(());
	///     module.register_subscription("hi", "hi", "goodbye", |_, mut sink, _| {
	///         sink.send(&"one answer").unwrap();
	///         Ok(())
	///     }).unwrap();
	///     let (resp, mut stream, _) = module.raw_call("hi", EmptyParams::new().to_rpc_params().unwrap()).await.unwrap();
	///     assert!(serde_json::from_str::<Response<u64>>(&resp).is_ok());
	///     let raw_sub_resp = stream.next().await.unwrap();
	///     let sub_resp: SubscriptionResponse<String> = serde_json::from_str(&raw_sub_resp).unwrap();
	///     assert_eq!(&sub_resp.params.result, "one answer");
	/// }
	/// ```
	pub async fn raw_call(&self, method: &str, params: Box<RawValue>) -> RawRpcResponse {
		let req = Request::new(method.into(),Some(&params), Id::Number(0));

		let (tx, mut rx) = mpsc::unbounded();
		let sink = MethodSink::new(tx.clone());

		if let MethodResult::Async(fut) = self.execute(&sink, req, 0) {
			fut.await;
		}

		let resp = rx.next().await.expect("tx and rx still alive; qed");
		(resp, rx, tx)
	}

	/// Test helper that sets up a subscription using the given `method`. Returns a tuple of the
	/// [`SubscriptionId`] and a channel on which subscription JSON payloads can be received.
	pub async fn test_subscription(&self, sub_method: &str, params: impl ToRpcParams) -> TestSubscription {
		let params = params.to_rpc_params().expect("valid JSON-RPC params");
		tracing::trace!("[Methods::test_subscription] Calling subscription method: {:?}, params: {:?}", sub_method, params);
		let (response, rx, tx) = self.raw_call(sub_method, params).await;
		let subscription_response = serde_json::from_str::<Response<SubscriptionId>>(&response)
			.unwrap_or_else(|_| panic!("Could not deserialize subscription response {:?}", response));
		let sub_id = subscription_response.result;
		TestSubscription { sub_id, rx, tx }
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
			MethodCallback::new_async(Arc::new(move |id, params, sink, claimed| {
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
			MethodCallback::new_async(Arc::new(move |id, params, sink, claimed| {
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
	/// use jsonrpsee_utils::server::rpc_module::RpcModule;
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
				MethodCallback::new_sync(Arc::new(move |id, params, method_sink, conn_id| {
					let (conn_tx, conn_rx) = oneshot::channel::<()>();
					let sub_id = {
						const JS_NUM_MASK: SubscriptionId = !0 >> 11;
						let sub_id = rand::random::<SubscriptionId>() & JS_NUM_MASK;
						let uniq_sub = SubscriptionKey { conn_id, sub_id };

						subscribers.lock().insert(uniq_sub, (method_sink.clone(), conn_rx));

						sub_id
					};

					method_sink.send_response(id.clone(), sub_id);

					let sink = SubscriptionSink {
						inner: method_sink.clone(),
						method: notif_method_name,
						subscribers: subscribers.clone(),
						uniq_sub: SubscriptionKey { conn_id, sub_id },
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
				MethodCallback::new_sync(Arc::new(move |id, params, sink, conn_id| {
					let sub_id = match params.one() {
						Ok(sub_id) => sub_id,
						Err(_) => {
							tracing::error!(
								"unsubscribe call '{}' failed: couldn't parse subscription id={:?} request id={:?}",
								unsubscribe_method_name,
								params,
								id
							);
							let err = to_json_raw_value(&"Invalid subscription ID type, must be integer").ok();
							return sink.send_error(id, invalid_subscription_err(err.as_deref()));
						}
					};

					if subscribers.lock().remove(&SubscriptionKey { conn_id, sub_id }).is_some() {
						sink.send_response(id, "Unsubscribed")
					} else {
						let err = to_json_raw_value(&format!("Invalid subscription ID={}", sub_id)).ok();
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
		serde_json::to_string(&SubscriptionResponse::new(
			self.method.into(),
			SubscriptionPayload { subscription: RpcSubscriptionId::Num(self.uniq_sub.sub_id), result },
		))
		.map_err(Into::into)
	}

	fn inner_send(&mut self, msg: String) -> Result<(), Error> {
		let res = match self.is_connected.as_ref() {
			Some(conn) if !conn.is_canceled() => {
				// unbounded send only fails if the receiver has been dropped.
				self.inner.send_raw(msg).map_err(|_| {
					Some(SubscriptionClosedError::new("Closed by the client (connection reset)", self.uniq_sub.sub_id))
				})
			}
			Some(_) => Err(Some(SubscriptionClosedError::new("Closed by unsubscribe call", self.uniq_sub.sub_id))),
			// NOTE(niklasad1): this should be unreachble, after the first error is detected the subscription is closed.
			None => Err(None),
		};

		if let Err(Some(e)) = &res {
			self.inner_close(e);
		}

		res.map_err(|e| {
			let err = e.unwrap_or_else(|| SubscriptionClosedError::new("Close reason unknown", self.uniq_sub.sub_id));
			Error::SubscriptionClosed(err)
		})
	}

	/// Close the subscription sink with a customized error message.
	pub fn close(&mut self, msg: &str) {
		let err = SubscriptionClosedError::new(msg, self.uniq_sub.sub_id);
		self.inner_close(&err);
	}

	fn inner_close(&mut self, err: &SubscriptionClosedError) {
		self.is_connected.take();
		if let Some((sink, _)) = self.subscribers.lock().remove(&self.uniq_sub) {
			tracing::debug!("Closing subscription: {:?}", self.uniq_sub.sub_id);
			let msg = self.build_message(err).expect("valid json infallible; qed");
			let _ = sink.send_raw(msg);
		}
	}
}

impl Drop for SubscriptionSink {
	fn drop(&mut self) {
		let err = SubscriptionClosedError::new("Closed by the server", self.uniq_sub.sub_id);
		self.inner_close(&err);
	}
}

/// Wrapper struct that maintains a subscription for testing.
#[derive(Debug)]
pub struct TestSubscription {
	tx: mpsc::UnboundedSender<String>,
	rx: mpsc::UnboundedReceiver<String>,
	sub_id: u64,
}

impl TestSubscription {
	/// Close the subscription channel by doing a unsubscribe call.
	pub fn close(&mut self) {
		self.tx.close_channel();
	}

	/// Get the subscription ID
	pub fn subscription_id(&self) -> u64 {
		self.sub_id
	}

	/// Returns `Some((val, sub_id))` for the next element of type T from the underlying stream,
	/// otherwise `None` if the subscruption was closed.
	///
	/// # Panics
	///
	/// If the decoding the value as `T` fails.
	pub async fn next<T: DeserializeOwned>(&mut self) -> Option<(T, jsonrpsee_types::v2::SubscriptionId)> {
		let raw = self.rx.next().await?;
		let val: SubscriptionResponse<T> =
			serde_json::from_str(&raw).expect("valid response in TestSubscription::next()");
		Some((val.params.result, val.params.subscription))
	}
}

impl Drop for TestSubscription {
	fn drop(&mut self) {
		self.close();
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use jsonrpsee_types::{v2, EmptyParams};
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
		let _subscription = cxmodule.register_subscription("hi", "hi", "goodbye", |_, _, _| Ok(()));

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

		let res: String = module.call("boo", EmptyParams::new()).await.unwrap();
		assert_eq!(&res, "boo!");

		// Call sync method with params
		module
			.register_method("foo", |params, _| {
				let n: u16 = params.one()?;
				Ok(n * 2)
			})
			.unwrap();
		let res: u64 = module.call("foo", [3_u64]).await.unwrap();
		assert_eq!(res, 6);

		// Call sync method with bad param
		let params = (false,).to_rpc_params().unwrap();
		let (result, _, _) = module.raw_call("foo", params).await;
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
				async move { Ok(ctx.roo(ns)) }
			})
			.unwrap();
		let res: u64 = module.call("roo", [12, 13]).await.unwrap();
		assert_eq!(res, 25);
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
		let res: bool = module.call("rebel_without_cause", EmptyParams::new()).await.unwrap();
		assert_eq!(res, false);

		// Call sync method with params
		let res: String = module.call("rebel", (Gun { shoots: true }, HashMap::<u8, u8>::default())).await.unwrap();
		assert_eq!(&res, "0 Gun { shoots: true }");

		// Call sync method with bad params
		let params = (Gun { shoots: true }, false).to_rpc_params().unwrap();
		let (result, _, _) = module.raw_call("rebel", params).await;
		assert_eq!(
			result,
			r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"invalid type: boolean `false`, expected a map at line 1 column 5"},"id":0}"#
		);

		// Call async method with params and context
		let result: String = module.call("revolution", (Beverage { ice: true }, vec![1, 2, 3])).await.unwrap();
		assert_eq!(&result, "drink: Beverage { ice: true }, phases: [1, 2, 3]");
	}

	#[tokio::test]
	async fn subscribing_without_server() {
		let mut module = RpcModule::new(());
		module
			.register_subscription("my_sub", "my_sub", "my_unsub", |_, mut sink, _| {
				let mut stream_data = vec!['0', '1', '2'];
				std::thread::spawn(move || loop {
					tracing::debug!("This is your friendly subscription sending data.");
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

		let mut my_sub: TestSubscription = module.test_subscription("my_sub", EmptyParams::new()).await;
		for i in (0..=2).rev() {
			let (val, id) = my_sub.next::<char>().await.unwrap();
			assert_eq!(val, std::char::from_digit(i, 10).unwrap());
			assert_eq!(id, v2::params::SubscriptionId::Num(my_sub.subscription_id()));
		}

		// The subscription is now closed by the server.
		let (sub_closed_err, _) = my_sub.next::<SubscriptionClosedError>().await.unwrap();
		assert_eq!(sub_closed_err.subscription_id(), my_sub.subscription_id());
		assert_eq!(sub_closed_err.close_reason(), "Closed by the server");
	}

	#[tokio::test]
	async fn close_test_subscribing_without_server() {
		let mut module = RpcModule::new(());
		module
			.register_subscription("my_sub", "my_sub", "my_unsub", |_, mut sink, _| {
				std::thread::spawn(move || loop {
					if let Err(Error::SubscriptionClosed(_)) = sink.send(&"lo") {
						return;
					}
					std::thread::sleep(std::time::Duration::from_millis(500));
				});
				Ok(())
			})
			.unwrap();

		let mut my_sub: TestSubscription = module.test_subscription("my_sub", EmptyParams::new()).await;
		let (val, id) = my_sub.next::<String>().await.unwrap();
		assert_eq!(&val, "lo");
		assert_eq!(id, v2::params::SubscriptionId::Num(my_sub.subscription_id()));

		// close the subscription to ensure it doesn't return any items.
		my_sub.close();
		assert_eq!(None, my_sub.next::<String>().await);
	}
}
