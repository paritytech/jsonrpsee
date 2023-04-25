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

use crate::error::Error;
use crate::id_providers::RandomIntegerIdProvider;
use crate::server::helpers::{MethodResponse, MethodSink};
use crate::server::subscription::{
	sub_message_to_json, BoundedSubscriptions, IntoSubscriptionCloseResponse, PendingSubscriptionSink,
	SubNotifResultOrError, Subscribers, Subscription, SubscriptionCloseResponse, SubscriptionKey, SubscriptionPermit,
	SubscriptionState,
};
use crate::traits::ToRpcParams;
use futures_util::{future::BoxFuture, FutureExt};
use jsonrpsee_types::error::{ErrorCode, ErrorObject};
use jsonrpsee_types::{
	Id, Params, Request, Response, ResponsePayload, ResponseSuccess, SubscriptionId as RpcSubscriptionId,
};
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use tokio::sync::{mpsc, oneshot};

use super::IntoResponse;

/// A `MethodCallback` is an RPC endpoint, callable with a standard JSON-RPC request,
/// implemented as a function pointer to a `Fn` function taking four arguments:
/// the `id`, `params`, a channel the function uses to communicate the result (or error)
/// back to `jsonrpsee`, and the connection ID (useful for the websocket transport).
pub type SyncMethod = Arc<dyn Send + Sync + Fn(Id, Params, MaxResponseSize) -> MethodResponse>;
/// Similar to [`SyncMethod`], but represents an asynchronous handler.
pub type AsyncMethod<'a> =
	Arc<dyn Send + Sync + Fn(Id<'a>, Params<'a>, ConnectionId, MaxResponseSize) -> BoxFuture<'a, MethodResponse>>;
/// Method callback for subscriptions.
pub type SubscriptionMethod<'a> = Arc<
	dyn Send + Sync + Fn(Id, Params, MethodSink, SubscriptionState) -> BoxFuture<'a, Result<MethodResponse, Id<'a>>>,
>;
// Method callback to unsubscribe.
type UnsubscriptionMethod = Arc<dyn Send + Sync + Fn(Id, Params, ConnectionId, MaxResponseSize) -> MethodResponse>;

/// Connection ID, used for stateful protocol such as WebSockets.
/// For stateless protocols such as http it's unused, so feel free to set it some hardcoded value.
pub type ConnectionId = usize;

/// Max response size.
pub type MaxResponseSize = usize;

/// Raw response from an RPC
/// A tuple containing:
///   - Call result as a `String`,
///   - a [`mpsc::UnboundedReceiver<String>`] to receive future subscription results
pub type RawRpcResponse = (MethodResponse, mpsc::Receiver<String>);

/// This represent a response to a RPC call
/// and `Subscribe` calls are handled differently
/// because we want to prevent subscriptions to start
/// before the actual subscription call has been answered.
#[derive(Debug, Clone)]
pub enum CallOrSubscription {
	/// The subscription callback itself sends back the result
	/// so it must not be sent back again.
	Subscription(MethodResponse),

	/// Treat it as ordinary call.
	Call(MethodResponse),
}

impl CallOrSubscription {
	/// Extract the JSON-RPC response.
	pub fn as_response(&self) -> &MethodResponse {
		match &self {
			Self::Subscription(r) => r,
			Self::Call(r) => r,
		}
	}

	/// Extract the JSON-RPC response.
	pub fn into_response(self) -> MethodResponse {
		match self {
			Self::Subscription(r) => r,
			Self::Call(r) => r,
		}
	}
}

/// Callback wrapper that can be either sync or async.
#[derive(Clone)]
pub enum MethodCallback {
	/// Synchronous method handler.
	Sync(SyncMethod),
	/// Asynchronous method handler.
	Async(AsyncMethod<'static>),
	/// Subscription method handler.
	Subscription(SubscriptionMethod<'static>),
	/// Unsubscription method handler.
	Unsubscription(UnsubscriptionMethod),
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

impl Debug for MethodCallback {
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

	/// Verifies that the method name is not already taken, and returns an error if it is.
	pub fn verify_method_name(&mut self, name: &'static str) -> Result<(), Error> {
		if self.callbacks.contains_key(name) {
			return Err(Error::MethodAlreadyRegistered(name.into()));
		}

		Ok(())
	}

	/// Inserts the method callback for a given name, or returns an error if the name was already taken.
	/// On success it returns a mut reference to the [`MethodCallback`] just inserted.
	pub fn verify_and_insert(
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
	///     use jsonrpsee::{RpcModule, IntoResponse};
	///     use jsonrpsee::core::RpcResult;
	///
	///     let mut module = RpcModule::new(());
	///     module.register_method::<RpcResult<u64>, _>("echo_call", |params, _| {
	///         params.one::<u64>().map_err(Into::into)
	///     }).unwrap();
	///
	///     let echo: u64 = module.call("echo_call", [1_u64]).await.unwrap();
	///     assert_eq!(echo, 1);
	/// }
	/// ```
	pub async fn call<Params: ToRpcParams, T: DeserializeOwned + Clone>(
		&self,
		method: &str,
		params: Params,
	) -> Result<T, Error> {
		let params = params.to_rpc_params()?;
		let req = Request::new(method.into(), params.as_ref().map(|p| p.as_ref()), Id::Number(0));
		tracing::trace!("[Methods::call] Method: {:?}, params: {:?}", method, params);
		let (resp, _) = self.inner_call(req, 1, mock_subscription_permit()).await;
		let rp = serde_json::from_str::<Response<T>>(&resp.result)?;
		ResponseSuccess::try_from(rp).map(|s| s.result).map_err(Error::Call)
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
	///     use jsonrpsee::{RpcModule, SubscriptionMessage};
	///     use jsonrpsee::types::{response::Success, Response};
	///     use futures_util::StreamExt;
	///
	///     let mut module = RpcModule::new(());
	///     module.register_subscription("hi", "hi", "goodbye", |_, pending, _| async {
	///         let sink = pending.accept().await?;
	///
	///         // see comment above.
	///         sink.send("one answer".into()).await?;
	///
	///         Ok(())
	///     }).unwrap();
	///     let (resp, mut stream) = module.raw_json_request(r#"{"jsonrpc":"2.0","method":"hi","id":0}"#, 1).await.unwrap();
	///     let resp: Success<u64> = serde_json::from_str::<Response<u64>>(&resp.result).unwrap().try_into().unwrap();    
	///     let sub_resp = stream.recv().await.unwrap();
	///     assert_eq!(
	///         format!(r#"{{"jsonrpc":"2.0","method":"hi","params":{{"subscription":{},"result":"one answer"}}}}"#, resp.result),
	///         sub_resp
	///     );
	/// }
	/// ```
	pub async fn raw_json_request(
		&self,
		request: &str,
		buf_size: usize,
	) -> Result<(MethodResponse, mpsc::Receiver<String>), Error> {
		tracing::trace!("[Methods::raw_json_request] Request: {:?}", request);
		let req: Request = serde_json::from_str(request)?;
		let (resp, rx) = self.inner_call(req, buf_size, mock_subscription_permit()).await;

		Ok((resp, rx))
	}

	/// Execute a callback.
	async fn inner_call(
		&self,
		req: Request<'_>,
		buf_size: usize,
		subscription_permit: SubscriptionPermit,
	) -> RawRpcResponse {
		let (tx, mut rx) = mpsc::channel(buf_size);
		let id = req.id.clone();
		let params = Params::new(req.params.map(|params| params.get()));

		let response = match self.method(&req.method) {
			None => MethodResponse::error(req.id, ErrorObject::from(ErrorCode::MethodNotFound)),
			Some(MethodCallback::Sync(cb)) => (cb)(id, params, usize::MAX),
			Some(MethodCallback::Async(cb)) => (cb)(id.into_owned(), params.into_owned(), 0, usize::MAX).await,
			Some(MethodCallback::Subscription(cb)) => {
				let conn_state =
					SubscriptionState { conn_id: 0, id_provider: &RandomIntegerIdProvider, subscription_permit };
				let res = match (cb)(id, params, MethodSink::new(tx.clone()), conn_state).await {
					Ok(rp) => rp,
					Err(id) => MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError)),
				};

				// This message is not used because it's used for metrics so we discard in other to
				// not read once this is used for subscriptions.
				//
				// The same information is part of `res` above.
				let _ = rx.recv().await.expect("Every call must at least produce one response; qed");

				res
			}
			Some(MethodCallback::Unsubscription(cb)) => (cb)(id, params, 0, usize::MAX),
		};

		tracing::trace!("[Methods::inner_call] Method: {}, response: {:?}", req.method, response);

		(response, rx)
	}

	/// Helper to create a subscription on the `RPC module` without having to spin up a server.
	///
	/// The params must be serializable as JSON array, see [`ToRpcParams`] for further documentation.
	///
	/// Returns [`Subscription`] on success which can used to get results from the subscription.
	///
	/// # Examples
	///
	/// ```
	/// #[tokio::main]
	/// async fn main() {
	///     use jsonrpsee::{RpcModule, SubscriptionMessage};
	///     use jsonrpsee::core::{Error, EmptyServerParams, RpcResult};
	///
	///     let mut module = RpcModule::new(());
	///     module.register_subscription("hi", "hi", "goodbye", |_, pending, _| async move {
	///         let sink = pending.accept().await?;
	///         sink.send("one answer".into()).await?;
	///         Ok(())
	///     }).unwrap();
	///
	///     let mut sub = module.subscribe_unbounded("hi", EmptyServerParams::new()).await.unwrap();
	///     // In this case we ignore the subscription ID,
	///     let (sub_resp, _sub_id) = sub.next::<String>().await.unwrap().unwrap();
	///     assert_eq!(&sub_resp, "one answer");
	/// }
	/// ```
	pub async fn subscribe_unbounded(&self, sub_method: &str, params: impl ToRpcParams) -> Result<Subscription, Error> {
		self.subscribe(sub_method, params, u32::MAX as usize).await
	}

	/// Similar to [`Methods::subscribe_unbounded`] but it's using a bounded channel and the buffer capacity must be
	/// provided.
	pub async fn subscribe(
		&self,
		sub_method: &str,
		params: impl ToRpcParams,
		buf_size: usize,
	) -> Result<Subscription, Error> {
		let params = params.to_rpc_params()?;
		let req = Request::new(sub_method.into(), params.as_ref().map(|p| p.as_ref()), Id::Number(0));

		tracing::trace!("[Methods::subscribe] Method: {}, params: {:?}", sub_method, params);

		let (resp, rx) = self.inner_call(req, buf_size, mock_subscription_permit()).await;

		// TODO: hack around the lifetime on the `SubscriptionId` by deserialize first to serde_json::Value.
		let as_success: ResponseSuccess<serde_json::Value> =
			serde_json::from_str::<Response<_>>(&resp.result)?.try_into()?;

		let sub_id = as_success.result.try_into().map_err(|_| Error::InvalidSubscriptionId)?;

		Ok(Subscription { sub_id, rx })
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
		R: IntoResponse + 'static,
		F: Fn(Params, &Context) -> R + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		self.methods.verify_and_insert(
			method_name,
			MethodCallback::Sync(Arc::new(move |id, params, max_response_size| {
				let rp = callback(params, &*ctx).into_response();
				MethodResponse::response(id, rp, max_response_size)
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
		R: IntoResponse + 'static,
		Fut: Future<Output = R> + Send,
		Fun: (Fn(Params<'static>, Arc<Context>) -> Fut) + Clone + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		self.methods.verify_and_insert(
			method_name,
			MethodCallback::Async(Arc::new(move |id, params, _, max_response_size| {
				let ctx = ctx.clone();
				let callback = callback.clone();

				let future = async move {
					let rp = callback(params, ctx).await.into_response();
					MethodResponse::response(id, rp, max_response_size)
				};
				future.boxed()
			})),
		)
	}

	/// Register a new **blocking** synchronous RPC method, which computes the response with the given callback.
	/// Unlike the regular [`register_method`](RpcModule::register_method), this method can block its thread and perform
	/// expensive computations.
	pub fn register_blocking_method<R, F>(
		&mut self,
		method_name: &'static str,
		callback: F,
	) -> Result<&mut MethodCallback, Error>
	where
		Context: Send + Sync + 'static,
		R: IntoResponse + 'static,
		F: Fn(Params, Arc<Context>) -> R + Clone + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		let callback = self.methods.verify_and_insert(
			method_name,
			MethodCallback::Async(Arc::new(move |id, params, _, max_response_size| {
				let ctx = ctx.clone();
				let callback = callback.clone();

				tokio::task::spawn_blocking(move || {
					let rp = callback(params, ctx).into_response();
					MethodResponse::response(id, rp, max_response_size)
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
	/// For instance an `unsubscribe call` may fail if a non-existent subscription ID is used in the call.
	///
	/// This method ensures that the `subscription_method_name` and `unsubscription_method_name` are unique.
	/// The `notif_method_name` argument sets the content of the `method` field in the JSON document that
	/// the server sends back to the client. The uniqueness of this value is not machine checked and it's up to
	/// the user to ensure it is not used in any other [`RpcModule`] used in the server.
	///
	/// # Arguments
	///
	/// * `subscription_method_name` - name of the method to call to initiate a subscription
	/// * `notif_method_name` - name of method to be used in the subscription payload (technically a JSON-RPC
	///   notification)
	/// * `unsubscription_method` - name of the method to call to terminate a subscription
	/// * `callback` - A callback to invoke on each subscription; it takes three parameters:
	///     - [`Params`]: JSON-RPC parameters in the subscription call.
	///     - [`PendingSubscriptionSink`]: A pending subscription waiting to be accepted, in order to send out messages
	///       on the subscription
	///     - Context: Any type that can be embedded into the [`RpcModule`].
	///
	/// # Returns
	///
	/// An async block which returns something that implements [`crate::server::IntoSubscriptionCloseResponse`] which
	/// decides what action to take when the subscription ends whether such as to sent out another message
	/// on the subscription stream before closing down it.
	///
	/// NOTE: The return value is ignored if [`PendingSubscriptionSink`] hasn't been called or is unsuccessful, as the subscription
	/// is not allowed to send out subscription notifications before the actual subscription has been established.
	///
	/// This is implemented for `Result<T, E>` and `()`.
	///
	/// It's recommended to use `Result` if you want to propagate the error as special error notification
	/// Another option is to implement [`crate::server::IntoSubscriptionCloseResponse`] if you want customized behaviour.
	///
	/// The error notification has the following format:
	///
	/// ```json
	/// {
	///  "jsonrpc": "2.0",
	///  "method": "<method>",
	///  "params": {
	///    "subscription": "<subscriptionID>",
	///    "error": <your msg>
	///    }
	///  }
	/// }
	/// ```
	///
	/// # Examples
	///
	/// ```no_run
	///
	/// use jsonrpsee_core::server::{RpcModule, SubscriptionSink, SubscriptionMessage};
	/// use jsonrpsee_types::ErrorObjectOwned;
	///
	/// let mut ctx = RpcModule::new(99_usize);
	/// ctx.register_subscription("sub", "notif_name", "unsub", |params, pending, ctx| async move {
	///
	///     let x = match params.one::<usize>() {
	///         Ok(x) => x,
	///         Err(e) => {
	///            pending.reject(ErrorObjectOwned::from(e)).await;
	///            // If the subscription has not been "accepted" then
	///            // the return value will be "ignored" as it's not
	///            // allowed to send out any further notifications on
	///            // on the subscription.
	///            return Ok(());
	///         }
	///     };
	///
	///     // Mark the subscription is accepted after the params has been parsed successful.
	///     // This is actually responds the underlying RPC method call and may fail if the
	///     // connection is closed.
	///     let sink = pending.accept().await?;
	///     let sum = x + (*ctx);
	///
	///     // This will send out an error notification if it fails.
	///     //
	///     // If you need some other behavior implement or custom format of the error field
	///     // you need to manually handle that.
	///     let msg = SubscriptionMessage::from_json(&sum)?;
	///
	///     // This fails only if the connection is closed
	///     sink.send(msg).await?;
	///
	///     Ok(())
	/// });
	/// ```
	pub fn register_subscription<R, F, Fut>(
		&mut self,
		subscribe_method_name: &'static str,
		notif_method_name: &'static str,
		unsubscribe_method_name: &'static str,
		callback: F,
	) -> Result<&mut MethodCallback, Error>
	where
		Context: Send + Sync + 'static,
		F: (Fn(Params<'static>, PendingSubscriptionSink, Arc<Context>) -> Fut) + Send + Sync + Clone + 'static,
		Fut: Future<Output = R> + Send + 'static,
		R: IntoSubscriptionCloseResponse + Send,
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
				MethodCallback::Unsubscription(Arc::new(move |id, params, conn_id, max_response_size| {
					let sub_id = match params.one::<RpcSubscriptionId>() {
						Ok(sub_id) => sub_id,
						Err(_) => {
							tracing::warn!(
								"Unsubscribe call `{}` failed: couldn't parse subscription id={:?} request id={:?}",
								unsubscribe_method_name,
								params,
								id
							);

							return MethodResponse::response(id, ResponsePayload::result(false), max_response_size);
						}
					};

					let key = SubscriptionKey { conn_id, sub_id: sub_id.into_owned() };
					let result = subscribers.lock().remove(&key).is_some();

					if !result {
						tracing::debug!(
							"Unsubscribe call `{}` subscription key={:?} not an active subscription",
							unsubscribe_method_name,
							key,
						);
					}

					MethodResponse::response(id, ResponsePayload::result(result), max_response_size)
				})),
			);
		}

		// Subscribe
		let callback = {
			self.methods.verify_and_insert(
				subscribe_method_name,
				MethodCallback::Subscription(Arc::new(move |id, params, method_sink, conn| {
					let uniq_sub = SubscriptionKey { conn_id: conn.conn_id, sub_id: conn.id_provider.next_id() };

					// response to the subscription call.
					let (tx, rx) = oneshot::channel();
					let (accepted_tx, accepted_rx) = oneshot::channel();

					let sub_id = uniq_sub.sub_id.clone();
					let method = notif_method_name;

					let sink = PendingSubscriptionSink {
						inner: method_sink.clone(),
						method: notif_method_name,
						subscribers: subscribers.clone(),
						uniq_sub,
						id: id.clone().into_owned(),
						subscribe: tx,
						permit: conn.subscription_permit,
					};

					// The subscription callback is a future from the subscription
					// definition and not the as same when the subscription call has been completed.
					//
					// This runs until the subscription callback has completed.
					let sub_fut = callback(params.into_owned(), sink, ctx.clone());

					tokio::spawn(async move {
						// This will wait for the subscription future to be resolved
						let response = match futures_util::try_join!(sub_fut.map(|f| Ok(f)), accepted_rx) {
							Ok((r, _)) => r.into_response(),
							// The accept call failed i.e, the subscription was not accepted.
							Err(_) => return,
						};

						match response {
							SubscriptionCloseResponse::Notif(msg) => {
								let json = sub_message_to_json(msg, SubNotifResultOrError::Result, &sub_id, method);
								let _ = method_sink.send(json).await;
							}
							SubscriptionCloseResponse::NotifErr(msg) => {
								let json = sub_message_to_json(msg, SubNotifResultOrError::Error, &sub_id, method);
								let _ = method_sink.send(json).await;
							}
							SubscriptionCloseResponse::None => (),
						}
					});

					let id = id.clone().into_owned();

					Box::pin(async move {
						match rx.await {
							Ok(msg) => {
								// If the subscription was accepted then send a message
								// to subscription task otherwise rely on the drop impl.
								if msg.success {
									let _ = accepted_tx.send(());
								}
								Ok(msg)
							}
							Err(_) => Err(id),
						}
					})
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

fn mock_subscription_permit() -> SubscriptionPermit {
	BoundedSubscriptions::new(1).acquire().expect("1 permit should exist; qed")
}
