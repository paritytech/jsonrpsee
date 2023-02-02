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

use crate::error::{Error, SubscriptionAcceptRejectError};
use crate::id_providers::RandomIntegerIdProvider;
use crate::server::helpers::MethodSink;
use crate::traits::{IdProvider, ToRpcParams};
use crate::{SubscriptionClosed, SubscriptionResult};
use futures_util::future::Either;
use futures_util::{future::BoxFuture, FutureExt};
use futures_util::{Stream, StreamExt, TryStream, TryStreamExt};
use jsonrpsee_types::error::{CallError, ErrorCode, ErrorObject, ErrorObjectOwned, SUBSCRIPTION_CLOSED_WITH_ERROR};
use jsonrpsee_types::response::{SubscriptionError, SubscriptionPayloadError};
use jsonrpsee_types::{
	ErrorResponse, Id, Params, Request, Response, SubscriptionId as RpcSubscriptionId, SubscriptionPayload,
	SubscriptionResponse,
};
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::{mpsc, oneshot};

use super::helpers::{MethodResponse, MethodSinkPermit};

/// A `MethodCallback` is an RPC endpoint, callable with a standard JSON-RPC request,
/// implemented as a function pointer to a `Fn` function taking four arguments:
/// the `id`, `params`, a channel the function uses to communicate the result (or error)
/// back to `jsonrpsee`, and the connection ID (useful for the websocket transport).
pub type SyncMethod = Arc<dyn Send + Sync + Fn(Id, Params, MaxResponseSize) -> MethodResponse>;
/// Similar to [`SyncMethod`], but represents an asynchronous handler.
pub type AsyncMethod<'a> =
	Arc<dyn Send + Sync + Fn(Id<'a>, Params<'a>, ConnectionId, MaxResponseSize) -> BoxFuture<'a, MethodResponse>>;
/// Method callback for subscriptions.
pub type SubscriptionMethod<'a> =
	Arc<dyn Send + Sync + Fn(Id, Params, MethodSink, ConnState) -> BoxFuture<'a, SubscriptionAnswered>>;
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
pub type RawRpcResponse = (MethodResponse, mpsc::Receiver<String>, MethodSink);

/// TrySendError
#[derive(Debug)]
pub enum TrySendError {
	/// The channel is closed.
	Closed(SubscriptionMessage),
	/// The channel is full.
	Full(SubscriptionMessage),
}

#[derive(Debug)]
/// Represents whether a subscription was answered or not.
pub enum SubscriptionAnswered {
	/// The subscription was already answered and doesn't need to answered again.
	/// The response is kept to be logged.
	Yes(MethodResponse),
	/// The subscription was never answered and needs to be answered.
	///
	/// This may occur if a subscription dropped without calling `PendingSubscriptionSink::accept` or `PendingSubscriptionSink::reject`.
	No(MethodResponse),
}

/// Disconnect error
#[derive(Debug)]
pub struct DisconnectError(pub SubscriptionMessage);

/// Helper struct to manage subscriptions.
pub struct ConnState<'a> {
	/// Connection ID
	pub conn_id: ConnectionId,
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

impl From<mpsc::error::SendError<String>> for DisconnectError {
	fn from(e: mpsc::error::SendError<String>) -> Self {
		DisconnectError(SubscriptionMessage(e.0))
	}
}

impl From<mpsc::error::TrySendError<String>> for TrySendError {
	fn from(e: mpsc::error::TrySendError<String>) -> Self {
		match e {
			mpsc::error::TrySendError::Closed(m) => Self::Closed(SubscriptionMessage(m)),
			mpsc::error::TrySendError::Full(m) => Self::Full(SubscriptionMessage(m)),
		}
	}
}

impl<'a> std::fmt::Debug for ConnState<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ConnState").field("conn_id", &self.conn_id).finish()
	}
}

type Subscribers = Arc<Mutex<FxHashMap<SubscriptionKey, (MethodSink, mpsc::Receiver<()>)>>>;

/// Subscription message.
#[derive(Debug, Clone)]
pub struct SubscriptionMessage(pub(crate) String);

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
		let (tx, _rx) = mpsc::channel(1);
		let resp = self.inner_call(req, tx).await;

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
	/// This is a low-level API where you might be able build "in-memory" client
	/// but it's not ergonomic to use, see [`RpcModule::call`] and [`RpcModule::subscribe_bounded`]
	/// for more ergonomic APIs to perform method calls or subscriptions for testing or other purposes.
	///
	/// Returns the raw JSON response to the call, additional responses are sent out
	/// on the the receiver connected to the sender as passed to this method.
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
	///
	///     module.register_subscription("hi", "hi", "goodbye", |_, pending, _| async move {
	///          let sink = pending.accept().await?;
	///          let msg = sink.build_message(&"one answer").unwrap();
	///          sink.send(msg).await.unwrap();
	///
	///          Ok(())
	///     }).unwrap();
	///
	///     let (tx, mut rx) = tokio::sync::mpsc::channel(16);
	///     let resp = module.raw_json_request(r#"{"jsonrpc":"2.0","method":"hi","id":0}"#, tx).await.unwrap();
	///     let resp = serde_json::from_str::<Response<u64>>(&resp.result).unwrap();
	///     // ignore duplicated subscription call response.
	///     _ = rx.recv().await.unwrap();
	///     let sub_resp = rx.recv().await.unwrap();
	///     assert_eq!(
	///         format!(r#"{{"jsonrpc":"2.0","method":"hi","params":{{"subscription":{},"result":"one answer"}}}}"#, resp.result),
	///         sub_resp
	///     );
	/// }
	/// ```
	pub async fn raw_json_request(&self, request: &str, tx: mpsc::Sender<String>) -> Result<MethodResponse, Error> {
		tracing::trace!("[Methods::raw_json_request] Request: {:?}", request);
		let req: Request = serde_json::from_str(request)?;
		let resp = self.inner_call(req, tx).await;
		Ok(resp)
	}

	/// Execute a callback.
	async fn inner_call(&self, req: Request<'_>, tx: mpsc::Sender<String>) -> MethodResponse {
		let id = req.id.clone();
		let params = Params::new(req.params.map(|params| params.get()));

		let response = match self.method(&req.method).map(|c| &c.callback) {
			None => MethodResponse::error(req.id, ErrorObject::from(ErrorCode::MethodNotFound)),
			Some(MethodKind::Sync(cb)) => (cb)(id, params, usize::MAX),
			Some(MethodKind::Async(cb)) => (cb)(id.into_owned(), params.into_owned(), 0, usize::MAX).await,
			Some(MethodKind::Subscription(cb)) => {
				let conn_state = ConnState { conn_id: 0, id_provider: &RandomIntegerIdProvider };
				let res = (cb)(id, params, MethodSink::new(tx), conn_state).await;

				match res {
					SubscriptionAnswered::Yes(r) => r,
					SubscriptionAnswered::No(r) => r,
				}
			}
			Some(MethodKind::Unsubscription(cb)) => (cb)(id, params, 0, usize::MAX),
		};

		tracing::trace!("[Methods::inner_call] Method: {}, response: {:?}", req.method, response);

		response
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
	///     use jsonrpsee::{RpcModule, core::EmptyServerParams};
	///
	///     let mut module = RpcModule::new(());
	///     module.register_subscription("hi", "hi", "goodbye", |_, pending, _| async move {
	///         let sink = pending.accept().await?;
	///
	///         let msg = sink.build_message(&"one answer")?;
	///         sink.send(msg).await?;
	///         Ok(())
	///
	///     }).unwrap();
	///
	///     let mut sub = module.subscribe_unbounded("hi", EmptyServerParams::new()).await.unwrap();
	///     // In this case we ignore the subscription ID,
	///     let (sub_resp, _sub_id) = sub.next::<String>().await.unwrap().unwrap();
	///     assert_eq!(&sub_resp, "one answer");
	/// }
	/// ```
	pub async fn subscribe_unbounded(&self, sub_method: &str, params: impl ToRpcParams) -> Result<Subscription, Error> {
		self.subscribe_bounded(sub_method, params, u32::MAX as usize).await
	}

	/// Similar to `RpcMethods::subscribe_unbounded` but it's a using bounded channel.
	pub async fn subscribe_bounded(
		&self,
		sub_method: &str,
		params: impl ToRpcParams,
		buf_size: usize,
	) -> Result<Subscription, Error> {
		let params = params.to_rpc_params()?;
		let req = Request::new(sub_method.into(), params.as_ref().map(|p| p.as_ref()), Id::Number(0));
		let (tx, mut rx) = mpsc::channel(buf_size);

		tracing::trace!("[Methods::subscribe] Method: {}, params: {:?}", sub_method, params);

		let response = self.inner_call(req, tx.clone()).await;

		let subscription_response = match serde_json::from_str::<Response<RpcSubscriptionId>>(&response.result) {
			Ok(r) => r,
			Err(_) => match serde_json::from_str::<ErrorResponse>(&response.result) {
				Ok(err) => return Err(Error::Call(CallError::Custom(err.error_object().clone().into_owned()))),
				Err(err) => return Err(err.into()),
			},
		};

		let sub_id = subscription_response.result.into_owned();
		// throw away the response to the subscription call.
		_ = rx.recv().await;

		Ok(Subscription { sub_id, rx, tx: MethodSink::new(tx) })
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

/// Sets of JSON-RPC methods can be organized into "module"s that are in turn registered on the server or,
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
	pub fn register_async_method<R, E, Fun, Fut>(
		&mut self,
		method_name: &'static str,
		callback: Fun,
	) -> Result<&mut MethodCallback, Error>
	where
		R: Serialize + Send + Sync + 'static,
		E: Into<Error>,
		Fut: Future<Output = Result<R, E>> + Send,
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
						Err(err) => MethodResponse::error(id, err.into()),
					}
				};
				future.boxed()
			})),
		)
	}

	/// Register a new **blocking** synchronous RPC method, which computes the response with the given callback.
	/// Unlike the regular [`register_method`](RpcModule::register_method), this method can block its thread and perform expensive computations.
	pub fn register_blocking_method<R, E, F>(
		&mut self,
		method_name: &'static str,
		callback: F,
	) -> Result<&mut MethodCallback, Error>
	where
		Context: Send + Sync + 'static,
		R: Serialize,
		E: Into<Error>,
		F: Fn(Params, Arc<Context>) -> Result<R, E> + Clone + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		let callback = self.methods.verify_and_insert(
			method_name,
			MethodCallback::new_async(Arc::new(move |id, params, _, max_response_size| {
				let ctx = ctx.clone();
				let callback = callback.clone();

				tokio::task::spawn_blocking(move || match callback(params, ctx) {
					Ok(result) => MethodResponse::response(id, result, max_response_size),
					Err(err) => MethodResponse::error(id, err.into()),
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
	/// ctx.register_subscription("sub", "notif_name", "unsub", |params, pending, ctx| async move {
	///     let x = params.one::<usize>()?;
	///
	///     // mark the subscription is accepted after the params has been parsed successful.
	///     let sink = pending.accept().await?;
	///
	///     let sum = x + (*ctx);
	///     let msg = sink.build_message(&sum)?;
	///     sink.send(msg).await?;
	///
	///     Ok(())
	/// });
	/// ```
	pub fn register_subscription<F, Fut>(
		&mut self,
		subscribe_method_name: &'static str,
		notif_method_name: &'static str,
		unsubscribe_method_name: &'static str,
		callback: F,
	) -> Result<&mut MethodCallback, Error>
	where
		Context: Send + Sync + 'static,
		F: (Fn(Params<'static>, PendingSubscriptionSink, Arc<Context>) -> Fut) + Send + Sync + Clone + 'static,
		Fut: Future<Output = SubscriptionResult> + Send + 'static,
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

					let sink = PendingSubscriptionSink {
						inner: method_sink,
						method: notif_method_name,
						subscribers: subscribers.clone(),
						uniq_sub,
						id: id.clone().into_owned(),
						subscribe: tx,
					};

					// The subscription callback is a future from the subscription
					// definition and not the as same when the subscription call has been completed.
					//
					// This runs until the subscription callback has completed.
					let sub_fut = callback(params.into_owned(), sink, ctx.clone());

					tokio::spawn(async move {
						if sub_fut.await.is_err() {
							tracing::warn!("Subscribe call `{subscribe_method_name}` closed");
						}
					});

					let id = id.clone().into_owned();

					let result = async move {
						match rx.await {
							Ok(r) => SubscriptionAnswered::Yes(r),
							Err(_) => {
								let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError));
								SubscriptionAnswered::No(response)
							}
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

/// Represents a subscription until it is unsubscribed.
///
// NOTE: The reason why we use `mpsc` here is because it allows `IsUnsubscribed::unsubscribed`
// to be &self instead of &mut self.
#[derive(Debug)]
struct IsUnsubscribed(mpsc::Sender<()>);

impl IsUnsubscribed {
	/// Returns true if the unsubscribe method has been invoked or the subscription has been canceled.
	///
	/// This can be called multiple times as the element in the channel is never
	/// removed.
	fn is_unsubscribed(&self) -> bool {
		self.0.is_closed()
	}

	/// Wrapper over [`tokio::sync::mpsc::Sender::closed`]
	///
	/// # Cancel safety
	///
	/// This method is cancel safe. Once the channel is closed,
	/// it stays closed forever and all future calls to closed will return immediately.
	async fn unsubscribed(&self) {
		self.0.closed().await;
	}
}

/// Represents a single subscription that is waiting to be accepted or rejected.
///
/// If this is dropped without calling `PendingSubscription::reject` or `PendingSubscriptionSink::accept`
/// a default error is sent out as response to the subscription call.
///
/// Thus, if you want a customized error message then `PendingSubscription::reject` must be called.
#[derive(Debug)]
#[must_use = "PendningSubscriptionSink does nothing unless `accept` or `reject` is called"]
pub struct PendingSubscriptionSink {
	/// Sink.
	inner: MethodSink,
	/// MethodCallback.
	method: &'static str,
	/// Shared Mutex of subscriptions for this method.
	subscribers: Subscribers,
	/// Unique subscription.
	uniq_sub: SubscriptionKey,
	/// ID of the `subscription call` (i.e. not the same as subscription id) which is used
	/// to reply to subscription method call and must only be used once.
	id: Id<'static>,
	/// Sender to answer the subscribe call.
	subscribe: oneshot::Sender<MethodResponse>,
}

impl PendingSubscriptionSink {
	/// Reject the subscription call with the error from [`ErrorObject`].
	pub async fn reject(self, err: impl Into<ErrorObjectOwned>) -> Result<(), SubscriptionAcceptRejectError> {
		let err = MethodResponse::error(self.id, err.into());
		let permit = self.inner.reserve().await.map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;

		Self::answer_subscription(permit, err, self.subscribe).await?;
		Ok(())
	}

	/// Attempt to accept the subscription and respond the subscription method call.
	///
	/// Fails if the connection was closed or the message was too large.
	pub async fn accept(self) -> Result<SubscriptionSink, SubscriptionAcceptRejectError> {
		let response =
			MethodResponse::response(self.id, &self.uniq_sub.sub_id, self.inner.max_response_size() as usize);
		let success = response.success;
		let permit = self.inner.reserve().await.map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;

		Self::answer_subscription(permit, response, self.subscribe).await?;
		tracing::info!("accepted");

		if success {
			let (tx, rx) = mpsc::channel(1);
			self.subscribers.lock().insert(self.uniq_sub.clone(), (self.inner.clone(), rx));
			Ok(SubscriptionSink {
				inner: self.inner,
				method: self.method,
				subscribers: self.subscribers,
				uniq_sub: self.uniq_sub,
				unsubscribe: IsUnsubscribed(tx),
			})
		} else {
			Err(SubscriptionAcceptRejectError::MessageTooLarge)
		}
	}

	async fn answer_subscription(
		permit: MethodSinkPermit<'_>,
		response: MethodResponse,
		subscribe_call: oneshot::Sender<MethodResponse>,
	) -> Result<(), SubscriptionAcceptRejectError> {
		permit.send_raw(response.result.clone());
		subscribe_call.send(response).map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)
	}
}

/// Represents a single subscription that hasn't been processed yet.
#[derive(Debug)]
pub struct SubscriptionSink {
	/// Sink.
	inner: MethodSink,
	/// MethodCallback.
	method: &'static str,
	/// Shared Mutex of subscriptions for this method.
	subscribers: Subscribers,
	/// Unique subscription.
	uniq_sub: SubscriptionKey,
	/// A future to that fires once the unsubscribe method has been called.
	unsubscribe: IsUnsubscribed,
}

impl SubscriptionSink {
	/// Get the subscription ID.
	pub fn subscription_id(&self) -> RpcSubscriptionId<'static> {
		self.uniq_sub.sub_id.clone()
	}

	/// Get the method name.
	pub fn method_name(&self) -> &str {
		self.method
	}

	/// Send out a response on the subscription and wait until there is capacity.
	///
	///
	/// Returns
	/// - `Ok(())` if the message could be sent.
	/// - `Err(err)` if the connection or subscription was closed.
	///
	/// # Cancel safety
	///
	/// This method is cancel-safe and dropping a future loses its spot in the waiting queue.
	pub async fn send(&self, msg: SubscriptionMessage) -> Result<(), DisconnectError> {
		// Only possible to trigger when the connection is dropped.
		if self.is_closed() {
			return Err(DisconnectError(msg));
		}

		self.inner.send(msg.0).await.map_err(Into::into)
	}

	/// Attempts to immediately send out the message as JSON string to the subscribers but fails if the
	/// channel is full or the connection/subscription is closed
	///
	///
	/// This differs from [`SubscriptionSink::send`] as it will until there is capacity
	/// in the channel.
	pub fn try_send(&mut self, msg: SubscriptionMessage) -> Result<(), TrySendError> {
		// Only possible to trigger when the connection is dropped.
		if self.is_closed() {
			return Err(TrySendError::Closed(msg));
		}

		self.inner.try_send(msg.0).map_err(Into::into)
	}

	/// Returns whether the subscription is closed.
	pub fn is_closed(&self) -> bool {
		self.inner.is_closed() || !self.is_active_subscription()
	}

	/// Completes when the subscription has been closed.
	pub async fn closed(&self) {
		// Both are cancel-safe thus ok to use select here.
		tokio::select! {
			_ = self.inner.closed() => (),
			_ = self.unsubscribe.unsubscribed() => (),
		}
	}

	/// Build message that will be serialized as JSON-RPC notification.
	///
	/// You need to call this method prior to send out a subscription notification.
	pub fn build_message<T: Serialize>(&self, result: &T) -> Result<SubscriptionMessage, serde_json::Error> {
		serde_json::to_string(&SubscriptionResponse::new(
			self.method.into(),
			SubscriptionPayload { subscription: self.uniq_sub.sub_id.clone(), result },
		))
		.map(SubscriptionMessage)
		.map_err(Into::into)
	}

	/// Reads data from the `stream` and sends back data on the subscription
	/// when items gets produced by the stream.
	///
	/// The underlying sink is a bounded channel which may not have room for new items
	/// if the client can't keep up with all the messages. Thus, you need to provide a closure
	/// with a policy what to then the underlying channel has not space to "buffer" more messages.
	///
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
	/// m.register_subscription("sub", "_", "unsub", |params, pending, _| async move {
	///     let mut sink = pending.accept().await?;
	///
	///     let stream = futures_util::stream::iter(vec![Ok(1_u32), Ok(2), Err("error on the stream")]);
	///     // This will return send `[Ok(1_u32), Ok(2_u32), Err(Error::SubscriptionClosed))]` to the subscriber
	///     // because after the `Err(_)` then the stream is terminated.
	///     let stream = futures_util::stream::iter(vec![Ok(1_u32), Ok(2), Err("error on the stream")]);
	///
	///     // jsonrpsee doesn't send an error notification unless `close` is explicitly called.
	///     // If we pipe messages to the sink, we can inspect why it ended:
	///     match sink.pipe_from_try_stream(|last, next| last.unwrap_or(next), stream).await {
	///         SubscriptionClosed::Success => {
	///                let err_obj: ErrorObjectOwned = SubscriptionClosed::Success.into();
	///                sink.close(err_obj).await;
	///         }
	///         // we don't want to send close reason when the client is unsubscribed or disconnected.
	///         SubscriptionClosed::RemotePeerAborted => (),
	///         SubscriptionClosed::Failed(e) => {
	///           sink.close(e).await;
	///         }
	///     }
	///     Ok(())
	/// });
	/// ```
	pub async fn pipe_from_try_stream<S, T, F, E>(&mut self, f: F, mut stream: S) -> SubscriptionClosed
	where
		F: Fn(Option<SubscriptionMessage>, SubscriptionMessage) -> SubscriptionMessage,
		S: TryStream<Ok = T, Error = E> + Unpin,
		T: Serialize,
		E: std::fmt::Display,
	{
		fn err(err: impl std::fmt::Display) -> SubscriptionClosed {
			let err = ErrorObject::owned(SUBSCRIPTION_CLOSED_WITH_ERROR, err.to_string(), None::<()>);
			SubscriptionClosed::Failed(err)
		}

		let mut last = None;

		loop {
			tokio::select! {
				// add priority for the closed future
				// if that fires there's no point reading the stream
				biased;
				_ = self.closed() => {
					break SubscriptionClosed::RemotePeerAborted
				}

				item = stream.try_next() => {
					let item = match item {
						Ok(Some(item)) => item,
						Ok(None) => break SubscriptionClosed::Success,
						Err(e) => break err(e),
					};

					let next = match self.build_message(&item) {
						Ok(msg) => msg,
						Err(e) => break err(e),
					};

					let msg = f(last.take(), next);

					match self.try_send(msg) {
						Ok(_) => (),
						Err(TrySendError::Closed(_)) => break SubscriptionClosed::RemotePeerAborted,
						Err(TrySendError::Full(m)) => {
							tracing::debug!("Failed to send message the buffer is full");
							last = Some(m);
						}
					}
				},
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
	/// m.register_subscription("sub", "_", "unsub", |params, pending, _| async move {
	///    let stream = futures_util::stream::iter(vec![1_usize, 2, 3]);
	///
	///    let mut sink = pending.accept().await?;
	///    sink.pipe_from_stream(|_last, next| next, stream).await;
	///    Ok(())
	/// });
	/// ```
	pub async fn pipe_from_stream<S, T, F>(&mut self, f: F, stream: S) -> SubscriptionClosed
	where
		F: Fn(Option<SubscriptionMessage>, SubscriptionMessage) -> SubscriptionMessage,
		S: Stream<Item = T> + Unpin,
		T: Serialize,
	{
		self.pipe_from_try_stream::<_, _, _, Error>(f, stream.map(|item| Ok(item))).await
	}

	fn build_error_message<T: Serialize>(&self, error: &T) -> Result<SubscriptionMessage, serde_json::Error> {
		serde_json::to_string(&SubscriptionError::new(
			self.method.into(),
			SubscriptionPayloadError { subscription: self.uniq_sub.sub_id.clone(), error },
		))
		.map(SubscriptionMessage)
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
	pub fn close(self, err: impl Into<ErrorObjectOwned>) -> impl Future<Output = ()> {
		if self.is_active_subscription() {
			if let Some((sink, _)) = self.subscribers.lock().remove(&self.uniq_sub) {
				tracing::debug!("Closing subscription: {:?}", self.uniq_sub.sub_id);

				let msg = self.build_error_message(&err.into()).expect("valid json infallible; qed");

				return Either::Right(async move {
					let _ = sink.send(msg.0).await;
				});
			}
		}
		Either::Left(futures_util::future::ready(()))
	}

	fn is_active_subscription(&self) -> bool {
		!self.unsubscribe.is_unsubscribed()
	}
}

impl Drop for SubscriptionSink {
	fn drop(&mut self) {
		if self.is_active_subscription() {
			self.subscribers.lock().remove(&self.uniq_sub);
		}
	}
}

/// Wrapper struct that maintains a subscription "mainly" for testing.
#[derive(Debug)]
pub struct Subscription {
	tx: MethodSink,
	rx: mpsc::Receiver<String>,
	sub_id: RpcSubscriptionId<'static>,
}

impl Subscription {
	/// Close the subscription channel.
	pub fn close(&mut self) {
		tracing::trace!("[Subscription::close] Notifying");
		self.rx.close();
	}

	/// Get the subscription ID
	pub fn subscription_id(&self) -> &RpcSubscriptionId {
		&self.sub_id
	}

	/// Check whether the subscription is closed.
	pub fn is_closed(&self) -> bool {
		self.tx.is_closed()
	}

	/// Returns `Some((val, sub_id))` for the next element of type T from the underlying stream,
	/// otherwise `None` if the subscription was closed.
	///
	/// # Panics
	///
	/// If the decoding the value as `T` fails.
	pub async fn next<T: DeserializeOwned>(&mut self) -> Option<Result<(T, RpcSubscriptionId<'static>), Error>> {
		let raw = self.rx.recv().await?;

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
