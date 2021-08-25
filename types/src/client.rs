use crate::{error::SubscriptionClosedError, v2::params::SubscriptionId, Error};
use core::marker::PhantomData;
use futures_channel::{mpsc, oneshot};
use futures_util::{future::FutureExt, sink::SinkExt, stream::StreamExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value as JsonValue;

/// Subscription kind
#[derive(Debug)]
#[non_exhaustive]
pub enum SubscriptionKind {
	/// Get notifications based on Subscription ID.
	Subscription(SubscriptionId),
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
	pub send_back: oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>,
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
	SubscriptionClosed(SubscriptionId),
}

impl<Notif> Subscription<Notif>
where
	Notif: DeserializeOwned,
{
	/// Returns the next notification from the stream.
	/// This may return `Ok(None)` if the subscription has been terminated,
	/// may happen if the channel becomes full or is dropped.
	pub async fn next(&mut self) -> Result<Option<Notif>, Error> {
		match self.notifs_rx.next().await {
			Some(n) => match serde_json::from_value::<NotifResponse<Notif>>(n) {
				Ok(NotifResponse::Ok(parsed)) => Ok(Some(parsed)),
				Ok(NotifResponse::Err(e)) => Err(Error::SubscriptionClosed(e)),
				Err(e) => Err(Error::ParseError(e)),
			},
			None => Ok(None),
		}
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
