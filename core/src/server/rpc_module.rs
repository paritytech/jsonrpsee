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

use crate::error::{Error, SubscriptionClosed};
use crate::id_providers::RandomIntegerIdProvider;
use crate::server::helpers::MethodSink;
use crate::traits::{IdProvider, ToRpcParams};
use futures_channel::mpsc::TrySendError;
use futures_channel::{mpsc, oneshot};
use futures_util::future::Either;
use futures_util::pin_mut;
use futures_util::{future::BoxFuture, FutureExt, Stream, StreamExt, TryStream, TryStreamExt};
use jsonrpsee_types::error::{
	CallError, ErrorCode, ErrorObject, ErrorObjectOwned, SubscriptionAcceptRejectError, INTERNAL_ERROR_CODE,
	SUBSCRIPTION_CLOSED_WITH_ERROR,
};
use jsonrpsee_types::response::{SubscriptionError, SubscriptionPayloadError};
use jsonrpsee_types::{
	ErrorResponse, Id, Params, Request, Response, SubscriptionId as RpcSubscriptionId, SubscriptionPayload,
	SubscriptionResponse, SubscriptionResult,
};
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::{watch, Notify};

use super::helpers::MethodResponse;

/// A `MethodCallback` is an RPC endpoint, callable with a standard JSON-RPC request,
/// implemented as a function pointer to a `Fn` function taking four arguments:
/// the `id`, `params`, a channel the function uses to communicate the result (or error)
/// back to `jsonrpsee`, and the connection ID (useful for the websocket transport).
pub type SyncMethod = Arc<dyn Send + Sync + Fn(Id, Params, MaxResponseSize) -> MethodResponse>;
/// Similar to [`SyncMethod`], but represents an asynchronous handler and takes an additional argument containing a [`ResourceGuard`] if configured.
pub type AsyncMethod<'a> =
	Arc<dyn Send + Sync + Fn(Id<'a>, Params<'a>, ConnectionId, MaxResponseSize) -> BoxFuture<'a, MethodResponse>>;
/// Method callback for subscriptions.
pub type SubscriptionMethod<'a> =
	Arc<dyn Send + Sync + Fn(Id, Params, MethodSink, ConnState) -> BoxFuture<'a, MethodResponse>>;
// Method callback to unsubscribe.
type UnsubscriptionMethod = Arc<dyn Send + Sync + Fn(Id, Params, ConnectionId, MaxResponseSize) -> MethodResponse>;

/// Connection ID, used for stateful protocol such as WebSockets.
/// For stateless protocols such as http it's unused, so feel free to set it some hardcoded value.
pub type ConnectionId = usize;

/// Max response size.
pub type MaxResponseSize = usize;

/// Raw response from an RPC
/// A 3-tuple containing:
///   - Call result as a `String`,
///   - a [`mpsc::Receiver<String>`] to receive future subscription results
///   - a [`crate::server::helpers::SubscriptionPermit`] to allow subscribers to notify their [`SubscriptionSink`] when they disconnect.
pub type RawRpcResponse = (MethodResponse, mpsc::Receiver<String>, Arc<Notify>);

/// Helper struct to manage subscriptions.
pub struct ConnState<'a> {
	/// Connection ID
	pub conn_id: ConnectionId,
	/// Get notified when the connection to subscribers is closed.
	pub close_notify: Arc<Notify>,
	/// ID provider.
	pub id_provider: &'a dyn IdProvider,
}

/// Outcome of a successful terminated subscription.
#[derive(Debug, Copy, Clone)]
pub enum InnerSubscriptionResult {
	/// The subscription stream was executed successfully.
	Success,
	/// The subscription was aborted by the remote peer.
	Aborted,
}

/// Sending stuff via a subscription can either fail if the subscription failed or that
/// actual connection is full or disconnected.
#[derive(Debug, thiserror::Error)]
pub enum SubscriptionSinkError {
	/// Something failed during the init of the subscription.
	#[error("{0:?}")]
	Subscribe(SubscriptionAcceptRejectError),
	#[error("{0}")]
	/// Something failed when sending a message via the subscription.
	Send(#[from] SendError),
	#[error("{0}")]
	/// Something when trying to decode the message.
	Serialize(#[from] serde_json::Error),
}

impl<'a> std::fmt::Debug for ConnState<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ConnState").field("conn_id", &self.conn_id).field("close", &self.close_notify).finish()
	}
}

type Subscribers = Arc<Mutex<FxHashMap<SubscriptionKey, (MethodSink, watch::Sender<()>)>>>;

/// Represent a unique subscription entry based on [`RpcSubscriptionId`] and [`ConnectionId`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct SubscriptionKey {
	conn_id: ConnectionId,
	sub_id: RpcSubscriptionId<'static>,
}

/// Callback wrapper that can be either sync or async.
#[derive(Clone)]
pub enum MethodKind {
	/// Synchronous method handler.
	Sync(SyncMethod),
	/// Asynchronous method handler.
	Async(AsyncMethod<'static>),
	/// Subscription method handler.
	Subscription(SubscriptionMethod<'static>),
	/// Unsubscription method handler.
	Unsubscription(UnsubscriptionMethod),
}

/// Method callback wrapper that contains a sync or async closure,
#[derive(Clone, Debug)]
pub struct MethodCallback {
	callback: MethodKind,
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

impl MethodCallback {
	fn new_sync(callback: SyncMethod) -> Self {
		MethodCallback { callback: MethodKind::Sync(callback) }
	}

	fn new_async(callback: AsyncMethod<'static>) -> Self {
		MethodCallback { callback: MethodKind::Async(callback) }
	}

	fn new_subscription(callback: SubscriptionMethod<'static>) -> Self {
		MethodCallback { callback: MethodKind::Subscription(callback) }
	}

	fn new_unsubscription(callback: UnsubscriptionMethod) -> Self {
		MethodCallback { callback: MethodKind::Unsubscription(callback) }
	}

	/// Get handle to the callback.
	pub fn inner(&self) -> &MethodKind {
		&self.callback
	}
}

impl Debug for MethodKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Async(_) => write!(f, "Async"),
			Self::Sync(_) => write!(f, "Sync"),
			Self::Subscription(_) => write!(f, "Subscription"),
			Self::Unsubscription(_) => write!(f, "Unsubscription"),
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

	/// Helper to call a method on the `RPC module` without having to spin up a server.
	///
	/// The params must be serializable as JSON array, see [`ToRpcParams`] for further documentation.
	///
	/// Returns the decoded value of the `result field` in JSON-RPC response if successful.
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
		let req = Request::new(method.into(), params.as_ref().map(|p| p.as_ref()), Id::Number(0));
		tracing::trace!("[Methods::call] Method: {:?}, params: {:?}", method, params);
		let (resp, _, _) = self.inner_call(req).await;

		if resp.success {
			serde_json::from_str::<Response<T>>(&resp.result).map(|r| r.result).map_err(Into::into)
		} else {
			match serde_json::from_str::<ErrorResponse>(&resp.result) {
				Ok(err) => Err(Error::Call(CallError::Custom(err.error_object().clone().into_owned()))),
				Err(e) => Err(e.into()),
			}
		}
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
	///     let resp = serde_json::from_str::<Response<u64>>(&resp.result).unwrap();
	///     let sub_resp = stream.next().await.unwrap();
	///     assert_eq!(
	///         format!(r#"{{"jsonrpc":"2.0","method":"hi","params":{{"subscription":{},"result":"one answer"}}}}"#, resp.result),
	///         sub_resp
	///     );
	/// }
	/// ```
	pub async fn raw_json_request(&self, request: &str) -> Result<(MethodResponse, mpsc::Receiver<String>), Error> {
		tracing::trace!("[Methods::raw_json_request] Request: {:?}", request);
		let req: Request = serde_json::from_str(request)?;
		let (resp, rx, _) = self.inner_call(req).await;
		Ok((resp, rx))
	}

	/// Execute a callback.
	async fn inner_call(&self, req: Request<'_>) -> RawRpcResponse {
		let (tx_sink, mut rx_sink) = mpsc::channel(u32::MAX as usize / 2);
		let sink = MethodSink::new(tx_sink);
		let id = req.id.clone();
		let params = Params::new(req.params.map(|params| params.get()));
		let close_notify = Arc::new(Notify::new());

		let response = match self.method(&req.method).map(|c| &c.callback) {
			None => MethodResponse::error(req.id, ErrorObject::from(ErrorCode::MethodNotFound)),
			Some(MethodKind::Sync(cb)) => (cb)(id, params, usize::MAX),
			Some(MethodKind::Async(cb)) => (cb)(id.into_owned(), params.into_owned(), 0, usize::MAX).await,
			Some(MethodKind::Subscription(cb)) => {
				let conn_state =
					ConnState { conn_id: 0, close_notify: close_notify.clone(), id_provider: &RandomIntegerIdProvider };
				let res = (cb)(id, params, sink.clone(), conn_state).await;

				// This message is not used because it's used for metrics so we discard in other to
				// not read once this is used for subscriptions.
				//
				// The same information is part of `res` above.
				let _ = rx_sink.next().await.expect("Every call must at least produce one response; qed");

				res
			}
			Some(MethodKind::Unsubscription(cb)) => (cb)(id, params, 0, usize::MAX),
		};

		tracing::trace!("[Methods::inner_call] Method: {}, response: {:?}", req.method, response);

		(response, rx_sink, close_notify)
	}

	/// Helper to create a subscription on the `RPC module` without having to spin up a server.
	///
	/// The params must be serializable as JSON array, see [`ToRpcParams`] for further documentation.
	///
	/// Returns [`Subscription`] on success which can used to get results from the subscriptions.
	///
	/// # Examples
	///
	/// ```
	/// #[tokio::main]
	/// async fn main() {
	///     use jsonrpsee::{RpcModule, types::EmptyServerParams};
	///
	///     let mut module = RpcModule::new(());
	///     module.register_subscription("hi", "hi", "goodbye", |_, mut sink, _| {
	///         sink.send(&"one answer").unwrap();
	///         Ok(())
	///     }).unwrap();
	///
	///     let mut sub = module.subscribe("hi", EmptyServerParams::new()).await.unwrap();
	///     // In this case we ignore the subscription ID,
	///     let (sub_resp, _sub_id) = sub.next::<String>().await.unwrap().unwrap();
	///     assert_eq!(&sub_resp, "one answer");
	/// }
	/// ```
	pub async fn subscribe(&self, sub_method: &str, params: impl ToRpcParams) -> Result<Subscription, Error> {
		let params = params.to_rpc_params()?;
		let req = Request::new(sub_method.into(), params.as_ref().map(|p| p.as_ref()), Id::Number(0));

		tracing::trace!("[Methods::subscribe] Method: {}, params: {:?}", sub_method, params);

		let (response, rx, close_notify) = self.inner_call(req).await;

		let subscription_response = match serde_json::from_str::<Response<RpcSubscriptionId>>(&response.result) {
			Ok(r) => r,
			Err(_) => match serde_json::from_str::<ErrorResponse>(&response.result) {
				Ok(err) => return Err(Error::Call(CallError::Custom(err.error_object().clone().into_owned()))),
				Err(err) => return Err(err.into()),
			},
		};

		let sub_id = subscription_response.result.into_owned();
		let close_notify = Some(close_notify);

		Ok(Subscription { sub_id, rx, close_notify })
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

	/// Transform a module into an `RpcModule<()>` (unit context).
	pub fn remove_context(self) -> RpcModule<()> {
		let mut module = RpcModule::new(());
		module.methods = self.methods;
		module
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
	) -> Result<&mut MethodCallback, Error>
	where
		Context: Send + Sync + 'static,
		R: Serialize,
		F: Fn(Params, &Context) -> Result<R, Error> + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		self.methods.verify_and_insert(
			method_name,
			MethodCallback::new_sync(Arc::new(move |id, params, max_response_size| match callback(params, &*ctx) {
				Ok(res) => MethodResponse::response(id, res, max_response_size),
				Err(err) => MethodResponse::error(id, err),
			})),
		)
	}

	/// Register a new asynchronous RPC method, which computes the response with the given callback.
	pub fn register_async_method<R, Fun, Fut>(
		&mut self,
		method_name: &'static str,
		callback: Fun,
	) -> Result<&mut MethodCallback, Error>
	where
		R: Serialize + Send + Sync + 'static,
		Fut: Future<Output = Result<R, Error>> + Send,
		Fun: (Fn(Params<'static>, Arc<Context>) -> Fut) + Clone + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		self.methods.verify_and_insert(
			method_name,
			MethodCallback::new_async(Arc::new(move |id, params, _, max_response_size| {
				let ctx = ctx.clone();
				let callback = callback.clone();

				let future = async move {
					match callback(params, ctx).await {
						Ok(res) => MethodResponse::response(id, res, max_response_size),
						Err(err) => MethodResponse::error(id, err),
					}
				};
				future.boxed()
			})),
		)
	}

	/// Register a new **blocking** synchronous RPC method, which computes the response with the given callback.
	/// Unlike the regular [`register_method`](RpcModule::register_method), this method can block its thread and perform expensive computations.
	pub fn register_blocking_method<R, F>(
		&mut self,
		method_name: &'static str,
		callback: F,
	) -> Result<&mut MethodCallback, Error>
	where
		Context: Send + Sync + 'static,
		R: Serialize,
		F: Fn(Params, Arc<Context>) -> Result<R, Error> + Clone + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		let callback = self.methods.verify_and_insert(
			method_name,
			MethodCallback::new_async(Arc::new(move |id, params, _, max_response_size| {
				let ctx = ctx.clone();
				let callback = callback.clone();

				tokio::task::spawn_blocking(move || match callback(params, ctx) {
					Ok(result) => MethodResponse::response(id, result, max_response_size),
					Err(err) => MethodResponse::error(id, err),
				})
				.map(|result| match result {
					Ok(r) => r,
					Err(err) => {
						tracing::error!("Join error for blocking RPC method: {:?}", err);
						MethodResponse::error(Id::Null, ErrorObject::from(ErrorCode::InternalError))
					}
				})
				.boxed()
			})),
		)?;

		Ok(callback)
	}

	/// Register a new publish/subscribe interface using JSON-RPC notifications.
	///
	/// It implements the [ethereum pubsub specification](https://geth.ethereum.org/docs/rpc/pubsub)
	/// with an option to choose custom subscription ID generation.
	///
	/// Furthermore, it generates the `unsubscribe implementation` where a `bool` is used as
	/// the result to indicate whether the subscription was successfully unsubscribed to or not.
	/// For instance an `unsubscribe call` may fail if a non-existent subscriptionID is used in the call.
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
	/// * `callback` - A callback to invoke on each subscription; it takes three parameters:
	///     - [`Params`]: JSON-RPC parameters in the subscription call.
	///     - [`SubscriptionSink`]: A sink to send messages to the subscriber.
	///     - Context: Any type that can be embedded into the [`RpcModule`].
	///
	/// # Examples
	///
	/// ```no_run
	///
	/// use jsonrpsee_core::server::rpc_module::{RpcModule, SubscriptionSink};
	/// use jsonrpsee_core::Error;
	///
	/// let mut ctx = RpcModule::new(99_usize);
	/// ctx.register_subscription("sub", "notif_name", "unsub", |params, mut sink, ctx| {
	///     let x = match params.one::<usize>() {
	///         Ok(x) => x,
	///         Err(e) => {
	///             let err: Error = e.into();
	///             sink.reject(err);
	///             return Ok(());
	///         }
	///     };
	///     // Sink is accepted on the first `send` call.
	///     std::thread::spawn(move || {
	///         let sum = x + (*ctx);
	///         let _ = sink.send(&sum);
	///     });
	///
	///     Ok(())
	/// });
	/// ```
	pub fn register_subscription<F>(
		&mut self,
		subscribe_method_name: &'static str,
		notif_method_name: &'static str,
		unsubscribe_method_name: &'static str,
		callback: F,
	) -> Result<&mut MethodCallback, Error>
	where
		Context: Send + Sync + 'static,
		F: Fn(Params, SubscriptionSink, Arc<Context>) -> SubscriptionResult + Send + Sync + 'static,
	{
		if subscribe_method_name == unsubscribe_method_name {
			return Err(Error::SubscriptionNameConflict(subscribe_method_name.into()));
		}

		self.methods.verify_method_name(subscribe_method_name)?;
		self.methods.verify_method_name(unsubscribe_method_name)?;

		let ctx = self.ctx.clone();
		let subscribers = Subscribers::default();

		// Unsubscribe
		{
			let subscribers = subscribers.clone();
			self.methods.mut_callbacks().insert(
				unsubscribe_method_name,
				MethodCallback::new_unsubscription(Arc::new(move |id, params, conn_id, max_response_size| {
					let sub_id = match params.one::<RpcSubscriptionId>() {
						Ok(sub_id) => sub_id,
						Err(_) => {
							tracing::warn!(
								"Unsubscribe call `{}` failed: couldn't parse subscription id={:?} request id={:?}",
								unsubscribe_method_name,
								params,
								id
							);

							return MethodResponse::response(id, false, max_response_size);
						}
					};

					let key = SubscriptionKey { conn_id, sub_id: sub_id.into_owned() };
					let result = subscribers.lock().remove(&key).is_some();

					if !result {
						tracing::warn!(
							"Unsubscribe call `{}` subscription key={:?} not an active subscription",
							unsubscribe_method_name,
							key,
						);
					}

					MethodResponse::response(id, result, max_response_size)
				})),
			);
		}

		// Subscribe
		let callback = {
			self.methods.verify_and_insert(
				subscribe_method_name,
				MethodCallback::new_subscription(Arc::new(move |id, params, method_sink, conn| {
					let uniq_sub = SubscriptionKey { conn_id: conn.conn_id, sub_id: conn.id_provider.next_id() };

					// response to the subscription call.
					let (tx, rx) = oneshot::channel();

					let sink = SubscriptionSink {
						inner: method_sink,
						close_notify: conn.close_notify,
						method: notif_method_name,
						subscribers: subscribers.clone(),
						uniq_sub,
						id: Some((id.clone().into_owned(), tx)),
						unsubscribe: None,
					};

					// The callback returns a `SubscriptionResult` for better ergonomics and is not propagated further.
					if callback(params, sink, ctx.clone()).is_err() {
						tracing::warn!("Subscribe call `{}` failed", subscribe_method_name);
					}

					let id = id.clone().into_owned();

					let result = async move {
						match rx.await {
							Ok(result) => result,
							Err(_) => MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError)),
						}
					};

					Box::pin(result)
				})),
			)?
		};

		Ok(callback)
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

/// Returns once the unsubscribe method has been called.
type UnsubscribeCall = Option<watch::Receiver<()>>;

/// Represents a send error over a bounded channel.
#[derive(Debug, Copy, Clone, thiserror::Error)]
pub enum SendError {
	/// The channel was full.
	#[error("The channel is full")]
	Full,
	/// The channel was disconnected.
	#[error("The channel is disconnected")]
	Disconnected,
}

impl From<TrySendError<String>> for SendError {
	fn from(err: TrySendError<String>) -> Self {
		if err.is_full() {
			Self::Full
		} else {
			Self::Disconnected
		}
	}
}

impl From<SendError> for SubscriptionAcceptRejectError {
	fn from(err: SendError) -> Self {
		match err {
			SendError::Disconnected => Self::RemotePeerAborted,
			SendError::Full => Self::Full,
		}
	}
}

/// Represents a single subscription.
#[derive(Debug)]
pub struct SubscriptionSink {
	/// Sink.
	inner: MethodSink,
	/// Get notified when subscribers leave so we can exit
	close_notify: Arc<Notify>,
	/// MethodCallback.
	method: &'static str,
	/// Shared Mutex of subscriptions for this method.
	subscribers: Subscribers,
	/// Unique subscription.
	uniq_sub: SubscriptionKey,
	/// Id of the `subscription call` (i.e. not the same as subscription id) which is used
	/// to reply to subscription method call and must only be used once.
	///
	/// *Note*: Having some value means the subscription was not accepted or rejected yet.
	id: Option<(Id<'static>, oneshot::Sender<MethodResponse>)>,
	/// Having some value means the subscription was accepted.
	unsubscribe: UnsubscribeCall,
}

impl SubscriptionSink {
	/// Reject the subscription call from [`ErrorObject`].
	pub fn reject(&mut self, err: impl Into<ErrorObjectOwned>) -> Result<(), SubscriptionAcceptRejectError> {
		let (id, subscribe_call) = self.id.take().ok_or(SubscriptionAcceptRejectError::AlreadyCalled)?;

		let err = MethodResponse::error(id, err.into());

		self.answer_subscription(err, subscribe_call)?;
		Ok(())
	}

	/// Attempt to accept the subscription and respond the subscription method call.
	///
	/// Fails if the connection was closed, or if called multiple times.
	pub fn accept(&mut self) -> Result<(), SubscriptionAcceptRejectError> {
		let (id, subscribe_call) = self.id.take().ok_or(SubscriptionAcceptRejectError::AlreadyCalled)?;

		let response = MethodResponse::response(id, &self.uniq_sub.sub_id, self.inner.max_response_size() as usize);
		let success = response.success;

		self.answer_subscription(response, subscribe_call)?;

		if success {
			let (tx, rx) = watch::channel(());
			self.subscribers.lock().insert(self.uniq_sub.clone(), (self.inner.clone(), tx));
			self.unsubscribe = Some(rx);
			Ok(())
		} else {
			// TODO(niklasad1): this is wrong, the response was too big to be sent.
			Err(SubscriptionAcceptRejectError::RemotePeerAborted)
		}
	}

	/// Return the subscription ID if the the subscription was accepted.
	///
	/// [`SubscriptionSink::accept`] should be called prior to this method.
	pub fn subscription_id(&self) -> Option<RpcSubscriptionId<'static>> {
		if self.id.is_some() {
			// Subscription was not accepted.
			None
		} else {
			Some(self.uniq_sub.sub_id.clone())
		}
	}

	/// Send a message back to subscribers.
	///
	/// Returns
	/// - `Ok(true)` if the message could be send.
	/// - `Ok(false)` if the sink was closed (either because the subscription was closed or the connection was terminated),
	/// or the subscription could not be accepted.
	/// - `Err(err)` if the message could not be serialized.
	pub fn send<T: Serialize>(&mut self, result: &T) -> Result<(), SubscriptionSinkError> {
		match self.accept() {
			Ok(_) => (),
			Err(SubscriptionAcceptRejectError::AlreadyCalled) => (),
			Err(SubscriptionAcceptRejectError::Full) => return Err(SubscriptionSinkError::Send(SendError::Full)),
			Err(SubscriptionAcceptRejectError::RemotePeerAborted) => {
				return Err(SubscriptionSinkError::Send(SendError::Disconnected))
			}
		};

		self.send_without_accept(result)
	}

	/// Reads data from the `stream` and sends back data on the subscription
	/// when items gets produced by the stream.
	/// The underlying stream must produce `Result values, see [`futures_util::TryStream`] for further information.
	///
	/// Returns `Ok(())` if the stream or connection was terminated.
	/// Returns `Err(_)` immediately if the underlying stream returns an error or if an item from the stream could not be serialized.
	///
	/// # Examples
	///
	/// ```no_run
	///
	/// use jsonrpsee_core::server::rpc_module::RpcModule;
	/// use jsonrpsee_core::error::{Error, SubscriptionClosed};
	/// use jsonrpsee_types::ErrorObjectOwned;
	/// use anyhow::anyhow;
	///
	/// let mut m = RpcModule::new(());
	/// m.register_subscription("sub", "_", "unsub", |params, mut sink, _| {
	///     let stream = futures_util::stream::iter(vec![Ok(1_u32), Ok(2), Err("error on the stream")]);
	///     // This will return send `[Ok(1_u32), Ok(2_u32), Err(Error::SubscriptionClosed))]` to the subscriber
	///     // because after the `Err(_)` the stream is terminated.
	///     let stream = futures_util::stream::iter(vec![Ok(1_u32), Ok(2), Err("error on the stream")]);
	///
	///     tokio::spawn(async move {
	///
	///         // jsonrpsee doesn't send an error notification unless `close` is explicitly called.
	///         // If we pipe messages to the sink, we can inspect why it ended:
	///         match sink.pipe_from_try_stream(stream).await {
	///            SubscriptionClosed::Success => {
	///                let err_obj: ErrorObjectOwned = SubscriptionClosed::Success.into();
	///                sink.close(err_obj);
	///            }
	///            // we don't want to send close reason when the client is unsubscribed or disconnected.
	///            SubscriptionClosed::RemotePeerAborted => (),
	///            SubscriptionClosed::Failed(e) => {
	///                sink.close(e);
	///            }
	///         }
	///     });
	///     Ok(())
	/// });
	/// ```
	pub async fn pipe_from_try_stream<S, T, E>(&mut self, mut stream: S) -> SubscriptionClosed
	where
		S: TryStream<Ok = T, Error = E> + Unpin,
		T: Serialize,
		E: std::fmt::Display,
	{
		if let Err(SubscriptionAcceptRejectError::RemotePeerAborted) = self.accept() {
			return SubscriptionClosed::RemotePeerAborted;
		}

		let conn_closed = self.close_notify.clone();

		let mut sub_closed = match self.unsubscribe.as_ref() {
			Some(rx) => rx.clone(),
			_ => {
				return SubscriptionClosed::Failed(ErrorObject::owned(
					INTERNAL_ERROR_CODE,
					"Unsubscribe watcher not set after accepting the subscription".to_string(),
					None::<()>,
				))
			}
		};

		let sub_closed_fut = sub_closed.changed();

		let conn_closed_fut = conn_closed.notified();
		pin_mut!(conn_closed_fut);
		pin_mut!(sub_closed_fut);

		let mut stream_item = stream.try_next();
		let mut closed_fut = futures_util::future::select(conn_closed_fut, sub_closed_fut);

		loop {
			match futures_util::future::select(stream_item, closed_fut).await {
				// The app sent us a value to send back to the subscribers
				Either::Left((Ok(Some(result)), next_closed_fut)) => {
					match self.send_without_accept(&result) {
						Ok(()) => (),
						Err(SubscriptionSinkError::Send(SendError::Disconnected)) => {
							break SubscriptionClosed::RemotePeerAborted;
						}
						Err(SubscriptionSinkError::Send(SendError::Full)) => {
							break SubscriptionClosed::Full;
						}
						Err(err) => {
							let err = ErrorObject::owned(SUBSCRIPTION_CLOSED_WITH_ERROR, err.to_string(), None::<()>);
							break SubscriptionClosed::Failed(err);
						}
					};
					stream_item = stream.try_next();
					closed_fut = next_closed_fut;
				}
				// Stream canceled because of error.
				Either::Left((Err(err), _)) => {
					let err = ErrorObject::owned(SUBSCRIPTION_CLOSED_WITH_ERROR, err.to_string(), None::<()>);
					break SubscriptionClosed::Failed(err);
				}
				Either::Left((Ok(None), _)) => break SubscriptionClosed::Success,
				Either::Right((_, _)) => {
					break SubscriptionClosed::RemotePeerAborted;
				}
			}
		}
	}

	/// Similar to [`SubscriptionSink::pipe_from_try_stream`] but it doesn't require the stream return `Result`.
	///
	/// Warning: it's possible to pass in a stream that returns `Result` if `Result: Serialize` is satisfied
	/// but it won't cancel the stream when an error occurs. If you want the stream to be canceled when an
	/// error occurs use [`SubscriptionSink::pipe_from_try_stream`] instead.
	///
	/// # Examples
	///
	/// ```no_run
	///
	/// use jsonrpsee_core::server::rpc_module::RpcModule;
	///
	/// let mut m = RpcModule::new(());
	/// m.register_subscription("sub", "_", "unsub", |params, mut sink, _| {
	///     let stream = futures_util::stream::iter(vec![1_usize, 2, 3]);
	///     tokio::spawn(async move { sink.pipe_from_stream(stream).await; });
	///     Ok(())
	/// });
	/// ```
	pub async fn pipe_from_stream<S, T>(&mut self, stream: S) -> SubscriptionClosed
	where
		S: Stream<Item = T> + Unpin,
		T: Serialize,
	{
		self.pipe_from_try_stream::<_, _, Error>(stream.map(|item| Ok(item))).await
	}

	/// Returns whether the subscription is closed.
	pub fn is_closed(&self) -> bool {
		self.inner.is_closed() || !self.is_active_subscription()
	}

	/// Send a message back to subscribers.
	///
	/// This is similar to the [`SubscriptionSink::send`], but it does not try to accept
	/// the subscription prior to sending.
	#[inline]
	fn send_without_accept<T: Serialize>(&mut self, result: &T) -> Result<(), SubscriptionSinkError> {
		// Only possible to trigger when the connection is dropped.
		if self.is_closed() {
			return Err(SubscriptionSinkError::Send(SendError::Disconnected));
		}

		let msg = self.build_message(result)?;
		self.inner.send_raw(msg).map_err(SubscriptionSinkError::Send)
	}

	fn is_active_subscription(&self) -> bool {
		match self.unsubscribe.as_ref() {
			Some(unsubscribe) => unsubscribe.has_changed().is_ok(),
			_ => false,
		}
	}

	fn answer_subscription(
		&mut self,
		response: MethodResponse,
		subscribe_call: oneshot::Sender<MethodResponse>,
	) -> Result<(), SendError> {
		self.inner.send_raw(response.result.clone())?;
		subscribe_call.send(response).map_err(|_| SendError::Disconnected)?;

		Ok(())
	}

	fn build_message<T: Serialize>(&self, result: &T) -> Result<String, serde_json::Error> {
		serde_json::to_string(&SubscriptionResponse::new(
			self.method.into(),
			SubscriptionPayload { subscription: self.uniq_sub.sub_id.clone(), result },
		))
		.map_err(Into::into)
	}

	fn build_error_message<T: Serialize>(&self, error: &T) -> Result<String, serde_json::Error> {
		serde_json::to_string(&SubscriptionError::new(
			self.method.into(),
			SubscriptionPayloadError { subscription: self.uniq_sub.sub_id.clone(), error },
		))
		.map_err(Into::into)
	}

	/// Close the subscription, sending a notification with a special `error` field containing the provided error.
	///
	/// This can be used to signal an actual error, or just to signal that the subscription has been closed,
	/// depending on your preference.
	///
	/// If you'd like to to close the subscription without sending an error, just drop it and don't call this method.
	///
	///
	/// ```json
	/// {
	///  "jsonrpc": "2.0",
	///  "method": "<method>",
	///  "params": {
	///    "subscription": "<subscriptionID>",
	///    "error": { "code": <code from error>, "message": <message from error>, "data": <data from error> }
	///    }
	///  }
	/// }
	/// ```
	///
	pub fn close(self, err: impl Into<ErrorObjectOwned>) -> bool {
		if self.is_active_subscription() {
			if let Some((mut sink, _)) = self.subscribers.lock().remove(&self.uniq_sub) {
				tracing::debug!("Closing subscription: {:?}", self.uniq_sub.sub_id);

				let msg = self.build_error_message(&err.into()).expect("valid json infallible; qed");
				return sink.send_raw(msg).is_ok();
			}
		}
		false
	}
}

impl Drop for SubscriptionSink {
	fn drop(&mut self) {
		if let Some((id, subscribe_call)) = self.id.take() {
			// Subscription was never accepted / rejected. As such,
			// we default to assuming that the params were invalid,
			// because that's how the previous PendingSubscription logic
			// worked.
			let err = MethodResponse::error(id, ErrorObject::from(ErrorCode::InvalidParams));
			let _ = self.answer_subscription(err, subscribe_call);
		} else if self.is_active_subscription() {
			self.subscribers.lock().remove(&self.uniq_sub);
		}
	}
}

/// Wrapper struct that maintains a subscription "mainly" for testing.
#[derive(Debug)]
pub struct Subscription {
	close_notify: Option<Arc<Notify>>,
	rx: mpsc::Receiver<String>,
	sub_id: RpcSubscriptionId<'static>,
}

impl Subscription {
	/// Close the subscription channel.
	pub fn close(&mut self) {
		tracing::trace!("[Subscription::close] Notifying");
		if let Some(n) = self.close_notify.take() {
			n.notify_one()
		}
	}

	/// Get the subscription ID
	pub fn subscription_id(&self) -> &RpcSubscriptionId {
		&self.sub_id
	}

	/// Check whether the subscription is closed.
	pub fn is_closed(&self) -> bool {
		self.close_notify.is_none()
	}

	/// Returns `Some((val, sub_id))` for the next element of type T from the underlying stream,
	/// otherwise `None` if the subscription was closed.
	///
	/// # Panics
	///
	/// If the decoding the value as `T` fails.
	pub async fn next<T: DeserializeOwned>(&mut self) -> Option<Result<(T, RpcSubscriptionId<'static>), Error>> {
		if self.close_notify.is_none() {
			tracing::debug!("[Subscription::next] Closed.");
			return None;
		}
		let raw = self.rx.next().await?;

		tracing::debug!("[Subscription::next]: rx {}", raw);
		let res = match serde_json::from_str::<SubscriptionResponse<T>>(&raw) {
			Ok(r) => Some(Ok((r.params.result, r.params.subscription.into_owned()))),
			Err(e) => match serde_json::from_str::<SubscriptionError<serde_json::Value>>(&raw) {
				Ok(_) => None,
				Err(_) => Some(Err(e.into())),
			},
		};
		res
	}
}

impl Drop for Subscription {
	fn drop(&mut self) {
		self.close();
	}
}
