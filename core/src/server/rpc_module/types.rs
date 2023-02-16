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

use std::fmt::{self, Debug};
use std::sync::Arc;
use std::future::Future;
use std::time::Duration;

use crate::error::{Error, SubscriptionAcceptRejectError};
use crate::server::helpers::MethodSink;
use crate::traits::IdProvider;
use futures_util::future::{BoxFuture, Either};
use jsonrpsee_types::response::SubscriptionError;
use jsonrpsee_types::{Id, Params, SubscriptionId as RpcSubscriptionId, SubscriptionResponse, ErrorObjectOwned};
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::{mpsc, oneshot};

use super::super::helpers::{MethodResponse, SubscriptionPermit};

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
///   - a [`mpsc::UnboundedReceiver<String>`] to receive future subscription results
///   - a [`crate::server::helpers::SubscriptionPermit`] to allow subscribers to notify their [`SubscriptionSink`] when they disconnect.
pub type RawRpcResponse = (MethodResponse, mpsc::Receiver<String>, SubscriptionPermit);

/// Error that may occur during [`SubscriptionSink::try_send`].
#[derive(Debug)]
pub enum TrySendError {
	/// The channel is closed.
	Closed(SubscriptionMessage),
	/// The channel is full.
	Full(SubscriptionMessage),
}

impl std::fmt::Display for TrySendError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let msg = match self {
			Self::Closed(_) => "closed",
			Self::Full(_) => "full",
		};
		f.write_str(msg)
	}
}

#[derive(Debug, Clone)]
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

/// Error that may occur during `MethodSink::send` or `SubscriptionSink::send`.
#[derive(Debug)]
pub struct DisconnectError(pub SubscriptionMessage);

impl std::fmt::Display for DisconnectError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("closed")
	}
}

/// Error that may occur during `SubscriptionSink::send_timeout`.
#[derive(Debug)]
pub enum SendTimeoutError {
	/// The data could not be sent because the timeout elapsed
	/// which most likely is that the channel is full.
	Timeout(SubscriptionMessage),
	/// The channel is full.
	Closed(SubscriptionMessage),
}

impl std::fmt::Display for SendTimeoutError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let msg = match self {
			Self::Timeout(_) => "timed out waiting on send operation",
			Self::Closed(_) => "closed",
		};
		f.write_str(msg)
	}
}

/// Helper struct to manage subscriptions.
pub struct ConnState<'a> {
	/// Connection ID
	pub conn_id: ConnectionId,
	/// ID provider.
	pub id_provider: &'a dyn IdProvider,
	/// Subscription limit
	pub subscription_permit: SubscriptionPermit,
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
		DisconnectError(SubscriptionMessage::from_complete_message(e.0))
	}
}

impl From<mpsc::error::TrySendError<String>> for TrySendError {
	fn from(e: mpsc::error::TrySendError<String>) -> Self {
		match e {
			mpsc::error::TrySendError::Closed(m) => Self::Closed(SubscriptionMessage::from_complete_message(m)),
			mpsc::error::TrySendError::Full(m) => Self::Full(SubscriptionMessage::from_complete_message(m)),
		}
	}
}

impl From<mpsc::error::SendTimeoutError<String>> for SendTimeoutError {
	fn from(e: mpsc::error::SendTimeoutError<String>) -> Self {
		match e {
			mpsc::error::SendTimeoutError::Closed(m) => Self::Closed(SubscriptionMessage::from_complete_message(m)),
			mpsc::error::SendTimeoutError::Timeout(m) => Self::Timeout(SubscriptionMessage::from_complete_message(m)),
		}
	}
}

impl<'a> std::fmt::Debug for ConnState<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ConnState").field("conn_id", &self.conn_id).finish()
	}
}

pub(crate) type Subscribers = Arc<Mutex<FxHashMap<SubscriptionKey, (MethodSink, mpsc::Receiver<()>)>>>;

/// This represent a response to a RPC call
/// and `Subscribe` calls are handled differently
/// because we want to prevent subscriptions to start
/// before the actual subscription call has been answered.
#[derive(Debug, Clone)]
pub enum CallOrSubscription {
	/// The subscription callback itself sends back the result
	/// so it must not be sent back again.
	Subscription(SubscriptionAnswered),

	/// Treat it as ordinary call.
	Call(MethodResponse),
}

impl CallOrSubscription {
	/// Extract the JSON-RPC response.
	pub fn as_response(&self) -> &MethodResponse {
		match &self {
			Self::Subscription(r) => match r {
				SubscriptionAnswered::Yes(r) => r,
				SubscriptionAnswered::No(r) => r,
			},
			Self::Call(r) => r,
		}
	}

	/// Convert the `CallOrSubscription` to JSON-RPC response.
	pub fn into_response(self) -> MethodResponse {
		match self {
			Self::Subscription(r) => match r {
				SubscriptionAnswered::Yes(r) => r,
				SubscriptionAnswered::No(r) => r,
			},
			Self::Call(r) => r,
		}
	}
}

/// A complete subscription message or partial subscription message.
#[derive(Debug, Clone)]
pub enum SubscriptionMessageInner {
	/// Complete JSON message.
	Complete(String),
	/// Need subscription ID and method name.
	NeedsData(String),
}

/// Subscription message.
#[derive(Debug, Clone)]
pub struct SubscriptionMessage(pub(crate) SubscriptionMessageInner);

impl SubscriptionMessage {
	/// Create a new subscription message from JSON.
	pub fn from_json(t: &impl Serialize) -> Result<Self, serde_json::Error> {
		serde_json::to_string(t).map(|json| SubscriptionMessage(SubscriptionMessageInner::NeedsData(json)))
	}

	pub(crate) fn from_complete_message(msg: String) -> Self {
		SubscriptionMessage(SubscriptionMessageInner::Complete(msg))
	}

	pub(crate) fn empty() -> Self {
		Self::from_complete_message(String::new())
	}
}

/// Represent a unique subscription entry based on [`RpcSubscriptionId`] and [`ConnectionId`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct SubscriptionKey {
	pub(crate) conn_id: ConnectionId,
	pub(crate) sub_id: RpcSubscriptionId<'static>,
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
	pub(crate) callback: MethodKind,
}

/// Result of a method, either direct value or a future of one.
pub enum MethodResult<T> {
	/// Result by value
	Sync(T),
	/// Future of a value
	Async(BoxFuture<'static, T>),
}

pub(crate) enum SubNotifResultOrError {
	Result,
	Error,
}

impl SubNotifResultOrError {
	pub(crate) const fn as_str(&self) -> &str {
		match self {
			Self::Result => "result",
			Self::Error => "error",
		}
	}
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
	/// New sync method callback.
	pub fn new_sync(callback: SyncMethod) -> Self {
		MethodCallback { callback: MethodKind::Sync(callback) }
	}

	/// New async method callback.
	pub fn new_async(callback: AsyncMethod<'static>) -> Self {
		MethodCallback { callback: MethodKind::Async(callback) }
	}

	/// New subscription method callback.
	pub fn new_subscription(callback: SubscriptionMethod<'static>) -> Self {
		MethodCallback { callback: MethodKind::Subscription(callback) }
	}

	/// New unsubscription method callback.
	pub fn new_unsubscription(callback: UnsubscriptionMethod) -> Self {
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

/// Wrapper struct that maintains a subscription "mainly" for testing.
#[derive(Debug)]
pub struct Subscription {
	pub(crate) rx: mpsc::Receiver<String>,
	pub(crate) sub_id: RpcSubscriptionId<'static>,
	pub(crate) _permit: SubscriptionPermit,
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

/// Represents a subscription until it is unsubscribed.
///
// NOTE: The reason why we use `mpsc` here is because it allows `IsUnsubscribed::unsubscribed`
// to be &self instead of &mut self.
#[derive(Debug, Clone)]
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
	pub(crate) inner: MethodSink,
	/// MethodCallback.
	pub(crate) method: &'static str,
	/// Shared Mutex of subscriptions for this method.
	pub(crate) subscribers: Subscribers,
	/// Unique subscription.
	pub(crate) uniq_sub: SubscriptionKey,
	/// ID of the `subscription call` (i.e. not the same as subscription id) which is used
	/// to reply to subscription method call and must only be used once.
	pub(crate) id: Id<'static>,
	/// Sender to answer the subscribe call.
	pub(crate) subscribe: oneshot::Sender<MethodResponse>,
	/// Subscription permit.
	pub(crate) permit: SubscriptionPermit,
}

impl PendingSubscriptionSink {
	/// Reject the subscription call with the error from [`jsonrpsee_types::error::ErrorObject`].
	pub async fn reject(self, err: impl Into<ErrorObjectOwned>) -> Result<(), SubscriptionAcceptRejectError> {
		let err = MethodResponse::error(self.id, err.into());
		self.inner.send(err.result.clone()).await.map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;
		self.subscribe.send(err).map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;

		Ok(())
	}

	/// Attempt to accept the subscription and respond the subscription method call.
	///
	/// Fails if the connection was closed or the message was too large.
	pub async fn accept(self) -> Result<SubscriptionSink, SubscriptionAcceptRejectError> {
		let response =
			MethodResponse::response(self.id, &self.uniq_sub.sub_id, self.inner.max_response_size() as usize);
		let success = response.success;
		self.inner.send(response.result.clone()).await.map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;
		self.subscribe.send(response).map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;

		if success {
			let (tx, rx) = mpsc::channel(1);
			self.subscribers.lock().insert(self.uniq_sub.clone(), (self.inner.clone(), rx));
			Ok(SubscriptionSink {
				inner: self.inner,
				method: self.method,
				subscribers: self.subscribers,
				uniq_sub: self.uniq_sub,
				unsubscribe: IsUnsubscribed(tx),
				_permit: Arc::new(self.permit),
			})
		} else {
			Err(SubscriptionAcceptRejectError::MessageTooLarge)
		}
	}
}

/// Represents a single subscription that hasn't been processed yet.
#[derive(Debug, Clone)]
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
	/// Subscription permit
	_permit: Arc<SubscriptionPermit>,
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

		let json = self.sub_message_to_json(msg, SubNotifResultOrError::Result);
		self.inner.send(json).await.map_err(Into::into)
	}

	/// Similar to to `SubscriptionSink::send` but only waits for a limited time.
	pub async fn send_timeout(&self, msg: SubscriptionMessage, timeout: Duration) -> Result<(), SendTimeoutError> {
		// Only possible to trigger when the connection is dropped.
		if self.is_closed() {
			return Err(SendTimeoutError::Closed(msg));
		}

		let json = self.sub_message_to_json(msg, SubNotifResultOrError::Result);
		self.inner.send_timeout(json, timeout).await.map_err(Into::into)
	}

	/// Attempts to immediately send out the message as JSON string to the subscribers but fails if the
	/// channel is full or the connection/subscription is closed
	///
	///
	/// This differs from [`SubscriptionSink::send`] where it will until there is capacity
	/// in the channel.
	pub fn try_send(&mut self, msg: SubscriptionMessage) -> Result<(), TrySendError> {
		// Only possible to trigger when the connection is dropped.
		if self.is_closed() {
			return Err(TrySendError::Closed(msg));
		}

		let json = self.sub_message_to_json(msg, SubNotifResultOrError::Result);
		self.inner.try_send(json).map_err(Into::into)
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

	fn sub_message_to_json(&self, msg: SubscriptionMessage, result_or_err: SubNotifResultOrError) -> String {
		let result_or_err = result_or_err.as_str();

		match msg.0 {
			SubscriptionMessageInner::Complete(msg) => msg,
			SubscriptionMessageInner::NeedsData(result) => {
				let sub_id = serde_json::to_string(&self.uniq_sub.sub_id).expect("valid JSON; qed");
				let method = self.method;
				format!(
					r#"{{"jsonrpc":"2.0","method":"{method}","params":{{"subscription":{sub_id},"{result_or_err}":{result}}}}}"#,
				)
			}
		}
	}

	/// Close the subscription, sending a notification with a special `error` field containing the provided close reason.
	///
	/// This can be used to signal that an subscription was closed because of some particular state
	/// and doesn't imply that subscription was closed because of an error occurred. Just
	/// a custom way to indicate to the client that the subscription was closed.
	///
	/// If you'd like to to close the subscription without sending an extra notification,
	/// just drop it and don't call this method.
	///
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
	pub fn close_with_error(self, msg: SubscriptionMessage) -> impl Future<Output = ()> {
		self.inner_close(msg, SubNotifResultOrError::Error)
	}

	fn inner_close(self, msg: SubscriptionMessage, result_or_err: SubNotifResultOrError) -> impl Future<Output = ()> {
		if self.is_active_subscription() {
			if let Some((sink, _)) = self.subscribers.lock().remove(&self.uniq_sub) {
				tracing::debug!("Closing subscription: {:?}", self.uniq_sub.sub_id);
				let msg = self.sub_message_to_json(msg, result_or_err);

				return Either::Right(async move {
					// This only fails if the connection was closed
					// Fine to ignore
					let _ = sink.send(msg).await;
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
