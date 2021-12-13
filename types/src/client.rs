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

use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::task;

use crate::{error::SubscriptionClosedError, Error, SubscriptionId};
use core::marker::PhantomData;
use futures_channel::{mpsc, oneshot};
use futures_util::{
	future::FutureExt,
	sink::SinkExt,
	stream::{Stream, StreamExt},
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Subscription kind
#[derive(Debug)]
#[non_exhaustive]
pub enum SubscriptionKind {
	/// Get notifications based on Subscription ID.
	Subscription(SubscriptionId<'static>),
	/// Get notifications based on method name.
	Method(String),
}

/// Internal type to detect whether a subscription response from
/// the server was a valid notification or should be treated as an error.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
enum NotifResponse<Notif> {
	Ok(Notif),
	Err(SubscriptionClosedError),
}

/// Active subscription on the client.
///
/// It will automatically unsubscribe in the [`Subscription::drop`] so no need to explicitly call
/// the `unsubscribe method` if it is an an subscription based on [`SubscriptionId`].
#[derive(Debug)]
pub struct Subscription<Notif> {
	/// Channel to send requests to the background task.
	to_back: mpsc::Sender<FrontToBack>,
	/// Channel from which we receive notifications from the server, as encoded `JsonValue`s.
	notifs_rx: mpsc::Receiver<JsonValue>,
	/// Callback kind.
	kind: SubscriptionKind,
	/// Marker in order to pin the `Notif` parameter.
	marker: PhantomData<Notif>,
}

// `Subscription` does not automatically implement this due to `PhantomData<Notif>`,
// but type type has no need to be pinned.
impl<Notif> std::marker::Unpin for Subscription<Notif> {}

impl<Notif> Subscription<Notif> {
	/// Create a new subscription.
	pub fn new(
		to_back: mpsc::Sender<FrontToBack>,
		notifs_rx: mpsc::Receiver<JsonValue>,
		kind: SubscriptionKind,
	) -> Self {
		Self { to_back, notifs_rx, kind, marker: PhantomData }
	}
}

/// Batch request message.
#[derive(Debug)]
pub struct BatchMessage {
	/// Serialized batch request.
	pub raw: String,
	/// Request IDs.
	pub ids: Vec<u64>,
	/// One-shot channel over which we send back the result of this request.
	pub send_back: oneshot::Sender<Result<Vec<JsonValue>, Error>>,
}

/// Request message.
#[derive(Debug)]
pub struct RequestMessage {
	/// Serialized message.
	pub raw: String,
	/// Request ID.
	pub id: u64,
	/// One-shot channel over which we send back the result of this request.
	pub send_back: Option<oneshot::Sender<Result<JsonValue, Error>>>,
}

/// Subscription message.
#[derive(Debug)]
pub struct SubscriptionMessage {
	/// Serialized message.
	pub raw: String,
	/// Request ID of the subscribe message.
	pub subscribe_id: u64,
	/// Request ID of the unsubscribe message.
	pub unsubscribe_id: u64,
	/// Method to use to unsubscribe later. Used if the channel unexpectedly closes.
	pub unsubscribe_method: String,
	/// If the subscription succeeds, we return a [`mpsc::Receiver`] that will receive notifications.
	/// When we get a response from the server about that subscription, we send the result over
	/// this channel.
	pub send_back: oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, SubscriptionId<'static>), Error>>,
}

/// RegisterNotification message.
#[derive(Debug)]
pub struct RegisterNotificationMessage {
	/// Method name this notification handler is attached to
	pub method: String,
	/// We return a [`mpsc::Receiver`] that will receive notifications.
	/// When we get a response from the server about that subscription, we send the result over
	/// this channel.
	pub send_back: oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, String), Error>>,
}

/// Message that the Client can send to the background task.
#[derive(Debug)]
pub enum FrontToBack {
	/// Send a batch request to the server.
	Batch(BatchMessage),
	/// Send a notification to the server.
	Notification(String),
	/// Send a request to the server.
	Request(RequestMessage),
	/// Send a subscription request to the server.
	Subscribe(SubscriptionMessage),
	/// Register a notification handler
	RegisterNotification(RegisterNotificationMessage),
	/// Unregister a notification handler
	UnregisterNotification(String),
	/// When a subscription channel is closed, we send this message to the background
	/// task to mark it ready for garbage collection.
	// NOTE: It is not possible to cancel pending subscriptions or pending requests.
	// Such operations will be blocked until a response is received or the background
	// thread has been terminated.
	SubscriptionClosed(SubscriptionId<'static>),
}

impl<Notif> Subscription<Notif>
where
	Notif: DeserializeOwned,
{
	/// Returns the next notification from the stream.
	/// This may return `None` if the subscription has been terminated,
	/// which may happen if the channel becomes full or is dropped.
	///
	/// **Note:** This has an identical signature to the [`StreamExt::next`]
	/// method (and delegates to that). Import [`StreamExt`] if you'd like
	/// access to other stream combinator methods.
	#[allow(clippy::should_implement_trait)]
	pub async fn next(&mut self) -> Option<Result<Notif, Error>> {
		StreamExt::next(self).await
	}
}

impl<Notif> Stream for Subscription<Notif>
where
	Notif: DeserializeOwned,
{
	type Item = Result<Notif, Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Option<Self::Item>> {
		let n = futures_util::ready!(self.notifs_rx.poll_next_unpin(cx));
		let res = n.map(|n| match serde_json::from_value::<NotifResponse<Notif>>(n) {
			Ok(NotifResponse::Ok(parsed)) => Ok(parsed),
			Ok(NotifResponse::Err(e)) => Err(Error::SubscriptionClosed(e)),
			Err(e) => Err(Error::ParseError(e)),
		});
		task::Poll::Ready(res)
	}
}

impl<Notif> Drop for Subscription<Notif> {
	fn drop(&mut self) {
		// We can't actually guarantee that this goes through. If the background task is busy, then
		// the channel's buffer will be full.
		// However, when a notification arrives, the background task will realize that the channel
		// to the `Callback` has been closed.
		let kind = std::mem::replace(&mut self.kind, SubscriptionKind::Subscription(SubscriptionId::Num(0)));

		let msg = match kind {
			SubscriptionKind::Method(notif) => FrontToBack::UnregisterNotification(notif),
			SubscriptionKind::Subscription(sub_id) => FrontToBack::SubscriptionClosed(sub_id),
		};
		let _ = self.to_back.send(msg).now_or_never();
	}
}

#[derive(Debug)]
/// Keep track of request IDs.
pub struct RequestIdManager {
	// Current pending requests.
	current_pending: Arc<()>,
	/// Max concurrent pending requests allowed.
	max_concurrent_requests: usize,
	/// Get the next request ID.
	current_id: AtomicU64,
}

impl RequestIdManager {
	/// Create a new `RequestIdGuard` with the provided concurrency limit.
	pub fn new(limit: usize) -> Self {
		Self { current_pending: Arc::new(()), max_concurrent_requests: limit, current_id: AtomicU64::new(0) }
	}

	fn get_slot(&self) -> Result<Arc<()>, Error> {
		// Strong count is 1 at start, so that's why we use `>` and not `>=`.
		if Arc::strong_count(&self.current_pending) > self.max_concurrent_requests {
			Err(Error::MaxSlotsExceeded)
		} else {
			Ok(self.current_pending.clone())
		}
	}

	/// Attempts to get the next request ID.
	///
	/// Fails if request limit has been exceeded.
	pub fn next_request_id(&self) -> Result<RequestIdGuard<u64>, Error> {
		let rc = self.get_slot()?;
		let id = self.current_id.fetch_add(1, Ordering::SeqCst);
		Ok(RequestIdGuard { _rc: rc, id })
	}

	/// Attempts to get the `n` number next IDs that only counts as one request.
	///
	/// Fails if request limit has been exceeded.
	pub fn next_request_ids(&self, len: usize) -> Result<RequestIdGuard<Vec<u64>>, Error> {
		let rc = self.get_slot()?;
		let mut ids = Vec::with_capacity(len);
		for _ in 0..len {
			ids.push(self.current_id.fetch_add(1, Ordering::SeqCst));
		}
		Ok(RequestIdGuard { _rc: rc, id: ids })
	}
}

/// Reference counted request ID.
#[derive(Debug)]
pub struct RequestIdGuard<T> {
	id: T,
	/// Reference count decreased when dropped.
	_rc: Arc<()>,
}

impl<T> RequestIdGuard<T> {
	/// Get the actual ID.
	pub fn inner(&self) -> &T {
		&self.id
	}
}

#[cfg(test)]
mod tests {
	use super::RequestIdManager;

	#[test]
	fn request_id_guard_works() {
		let manager = RequestIdManager::new(2);
		let _first = manager.next_request_id().unwrap();

		{
			let _second = manager.next_request_ids(13).unwrap();
			assert!(manager.next_request_id().is_err());
			// second dropped here.
		}

		assert!(manager.next_request_id().is_ok());
	}
}

/// What certificate store to use
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CertificateStore {
	/// Use the native system certificate store
	Native,
	/// Use WebPKI's certificate store
	WebPki,
}
