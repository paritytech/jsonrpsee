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

use crate::error::Error;
use crate::server::helpers::MethodSink;
use crate::traits::IdProvider;
use futures_util::future::BoxFuture;
use jsonrpsee_types::response::SubscriptionError;
use jsonrpsee_types::{Id, Params, SubscriptionId as RpcSubscriptionId, SubscriptionResponse};
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::mpsc;

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
