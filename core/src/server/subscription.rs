//! Subscription related types and traits for server implementations.

use super::helpers::{MethodResponse, MethodSink};
use crate::server::error::{DisconnectError, SendTimeoutError, SubscriptionAcceptRejectError, TrySendError};
use crate::server::rpc_module::ConnectionId;
use crate::traits::IdProvider;
use crate::Error;
use jsonrpsee_types::{response::SubscriptionError, ErrorObjectOwned, Id, SubscriptionId, SubscriptionResponse};
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize};
use std::{sync::Arc, time::Duration};
use tokio::sync::{mpsc, Notify, OwnedSemaphorePermit, Semaphore};

/// Type-alias for subscribers.
pub type Subscribers = Arc<Mutex<FxHashMap<SubscriptionKey, (MethodSink, mpsc::Receiver<()>)>>>;

/// Convert something into a subscription close notification
/// before a subscription is terminated.
pub trait IntoSubscriptionResponse {
	/// Convert something into a subscription response
	fn into_response(self) -> SubscriptionCloseResponse;
}

/// Represents what action that will sent when a subscription callback returns.
#[derive(Debug)]
pub enum SubscriptionCloseResponse {
	/// No further message will be sent.
	None,
	/// Send a ordinary subscription response.
	Some(SubscriptionMessage),
	/// Send a subscription error response.
	Err(SubscriptionMessage),
}

impl<T> IntoSubscriptionResponse for Option<T>
where
	T: serde::Serialize,
{
	fn into_response(self) -> SubscriptionCloseResponse {
		match self {
			Some(msg) => match SubscriptionMessage::from_json(&msg) {
				Ok(m) => SubscriptionCloseResponse::Some(m),
				Err(e) => SubscriptionCloseResponse::Err(e.to_string().into()),
			},
			None => SubscriptionCloseResponse::None,
		}
	}
}

impl<T, E> IntoSubscriptionResponse for Result<T, E>
where
	T: serde::Serialize,
	E: ToString,
{
	fn into_response(self) -> SubscriptionCloseResponse {
		match self {
			Ok(msg) => match SubscriptionMessage::from_json(&msg) {
				Ok(m) => SubscriptionCloseResponse::Some(m),
				Err(e) => SubscriptionCloseResponse::Err(e.to_string().into()),
			},
			Err(e) => SubscriptionCloseResponse::Err(e.to_string().into()),
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
	///
	/// Fails if the value couldn't be serialized.
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

impl<T> From<T> for SubscriptionMessage
where
	T: AsRef<str>,
{
	fn from(msg: T) -> Self {
		SubscriptionMessage(SubscriptionMessageInner::NeedsData(format!("\"{}\"", msg.as_ref())))
	}
}

/// Represent a unique subscription entry based on [`SubscriptionId`] and [`ConnectionId`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubscriptionKey {
	pub(crate) conn_id: ConnectionId,
	pub(crate) sub_id: SubscriptionId<'static>,
}

#[derive(Debug, Clone, Copy)]
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

/// Represents a subscription until it is unsubscribed.
///
// NOTE: The reason why we use `mpsc` here is because it allows `IsUnsubscribed::unsubscribed`
// to be &self instead of &mut self.
#[derive(Debug, Clone)]
pub struct IsUnsubscribed(mpsc::Sender<()>);

impl IsUnsubscribed {
	/// Returns true if the unsubscribe method has been invoked or the subscription has been canceled.
	///
	/// This can be called multiple times as the element in the channel is never
	/// removed.
	pub fn is_unsubscribed(&self) -> bool {
		self.0.is_closed()
	}

	/// Wrapper over [`tokio::sync::mpsc::Sender::closed`]
	///
	/// # Cancel safety
	///
	/// This method is cancel safe. Once the channel is closed,
	/// it stays closed forever and all future calls to closed will return immediately.
	pub async fn unsubscribed(&self) {
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
	pub(crate) subscribe: mpsc::Sender<MethodResponse>,
	/// Subscription permit.
	pub(crate) permit: SubscriptionPermit,
}

impl PendingSubscriptionSink {
	/// Reject the subscription call with the error from [`ErrorObject`].
	pub async fn reject(self, err: impl Into<ErrorObjectOwned>) {
		let err = MethodResponse::error(self.id, err.into());
		_ = self.inner.send(err.result.clone()).await;
		_ = self.subscribe.send(err).await;
	}

	/// Attempt to accept the subscription and respond the subscription method call.
	///
	/// Returns `None` if the connection is already closed, `Some(SubscriptionSink)` otherwise.
	///
	/// # Panics
	///
	/// Panics if the subscription response exceeded the `max_response_size`.
	pub async fn accept(self) -> Result<SubscriptionSink, SubscriptionAcceptRejectError> {
		let response =
			MethodResponse::response(self.id, &self.uniq_sub.sub_id, self.inner.max_response_size() as usize);
		let success = response.success;
		self.inner.send(response.result.clone()).await.map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;
		self.subscribe.send(response).await.map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;

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
	pub fn subscription_id(&self) -> SubscriptionId<'static> {
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

		let json = sub_message_to_json(msg, SubNotifResultOrError::Result, &self.uniq_sub.sub_id, self.method);
		self.inner.send(json).await.map_err(Into::into)
	}

	/// Similar to to `SubscriptionSink::send` but only waits for a limited time.
	pub async fn send_timeout(&self, msg: SubscriptionMessage, timeout: Duration) -> Result<(), SendTimeoutError> {
		// Only possible to trigger when the connection is dropped.
		if self.is_closed() {
			return Err(SendTimeoutError::Closed(msg));
		}

		let json = sub_message_to_json(msg, SubNotifResultOrError::Result, &self.uniq_sub.sub_id, self.method);
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

		let json = sub_message_to_json(msg, SubNotifResultOrError::Result, &self.uniq_sub.sub_id, self.method);
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
	pub(crate) rx: mpsc::Receiver<String>,
	pub(crate) sub_id: SubscriptionId<'static>,
	pub(crate) _permit: SubscriptionPermit,
}

impl Subscription {
	/// Close the subscription channel.
	pub fn close(&mut self) {
		tracing::trace!("[Subscription::close] Notifying");
		self.rx.close();
	}

	/// Get the subscription ID
	pub fn subscription_id(&self) -> &SubscriptionId {
		&self.sub_id
	}

	/// Returns `Some((val, sub_id))` for the next element of type T from the underlying stream,
	/// otherwise `None` if the subscription was closed.
	///
	/// # Panics
	///
	/// If the decoding the value as `T` fails.
	pub async fn next<T: DeserializeOwned>(&mut self) -> Option<Result<(T, SubscriptionId<'static>), Error>> {
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

/// A permitted subscription.
#[derive(Debug)]
pub struct SubscriptionPermit {
	_permit: OwnedSemaphorePermit,
	resource: Arc<Notify>,
}

impl SubscriptionPermit {
	/// Get the handle to [`tokio::sync::Notify`].
	pub fn handle(&self) -> Arc<Notify> {
		self.resource.clone()
	}
}

/// Wrapper over [`tokio::sync::Notify`] with bounds check.
#[derive(Debug, Clone)]
pub struct BoundedSubscriptions {
	resource: Arc<Notify>,
	guard: Arc<Semaphore>,
	max: u32,
}

impl BoundedSubscriptions {
	/// Create a new bounded subscription.
	pub fn new(max_subscriptions: u32) -> Self {
		Self {
			resource: Arc::new(Notify::new()),
			guard: Arc::new(Semaphore::new(max_subscriptions as usize)),
			max: max_subscriptions,
		}
	}

	/// Attempts to acquire a subscription slot.
	///
	/// Fails if `max_subscriptions` have been exceeded.
	pub fn acquire(&self) -> Option<SubscriptionPermit> {
		Arc::clone(&self.guard)
			.try_acquire_owned()
			.ok()
			.map(|p| SubscriptionPermit { _permit: p, resource: self.resource.clone() })
	}

	/// Get the maximum number of permitted subscriptions.
	pub const fn max(&self) -> u32 {
		self.max
	}

	/// Close all subscriptions.
	pub fn close(&self) {
		self.resource.notify_waiters();
	}
}

#[derive(Debug)]
/// Helper struct to manage subscriptions.
pub struct SubscriptionState<'a> {
	/// Connection ID
	pub conn_id: ConnectionId,
	/// ID provider.
	pub id_provider: &'a dyn IdProvider,
	/// Subscription limit
	pub subscription_permit: SubscriptionPermit,
}

pub(crate) fn sub_message_to_json(
	msg: SubscriptionMessage,
	result_or_err: SubNotifResultOrError,
	sub_id: &SubscriptionId,
	method: &str,
) -> String {
	let result_or_err = result_or_err.as_str();

	match msg.0 {
		SubscriptionMessageInner::Complete(msg) => msg,
		SubscriptionMessageInner::NeedsData(result) => {
			let sub_id = serde_json::to_string(&sub_id).expect("valid JSON; qed");
			format!(
				r#"{{"jsonrpc":"2.0","method":"{method}","params":{{"subscription":{sub_id},"{result_or_err}":{result}}}}}"#,
			)
		}
	}
}
