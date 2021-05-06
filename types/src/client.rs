use crate::{v2::params::SubscriptionId, Error};
use core::marker::PhantomData;
use futures_channel::{mpsc, oneshot};
use futures_util::{future::FutureExt, sink::SinkExt, stream::StreamExt};
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;

/// Active subscription on a Client.
pub struct Subscription<Notif> {
	/// Channel to send requests to the background task.
	pub to_back: mpsc::Sender<FrontToBack>,
	/// Channel from which we receive notifications from the server, as encoded `JsonValue`s.
	pub notifs_rx: mpsc::Receiver<JsonValue>,
	/// Subscription ID,
	pub id: SubscriptionId,
	/// Marker in order to pin the `Notif` parameter.
	pub marker: PhantomData<Notif>,
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

/// OnNotification message.
#[derive(Debug)]
pub struct OnNotificationMessage {
	/// Request ID of the subscribe message.
	pub req_id: u64,
	/// SubscriptionId the method name this notification handler is attached to
	pub sub_id: SubscriptionId,
	/// We return a [`mpsc::Receiver`] that will receive notifications.
	/// When we get a response from the server about that subscription, we send the result over
	/// this channel.
	pub send_back: oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>,
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
	/// Create a notification handler subscription
	OnNotification(OnNotificationMessage),
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
	/// Returns the next notification from the stream
	/// This may return `None` if the subscription has been terminated,
	/// may happen if the channel becomes full or is dropped.
	///
	/// Ignores any malformed packet.
	pub async fn next(&mut self) -> Option<Notif> {
		loop {
			match self.notifs_rx.next().await {
				Some(n) => match serde_json::from_value(n) {
					Ok(parsed) => return Some(parsed),
					Err(e) => log::debug!("Subscription response error: {:?}", e),
				},
				None => return None,
			}
		}
	}
}

impl<Notif> Drop for Subscription<Notif> {
	fn drop(&mut self) {
		// We can't actually guarantee that this goes through. If the background task is busy, then
		// the channel's buffer will be full, and our unsubscription request will never make it.
		// However, when a notification arrives, the background task will realize that the channel
		// to the `Subscription` has been closed, and will perform the unsubscribe.
		let id = std::mem::replace(&mut self.id, SubscriptionId::Num(0));
		let _ = self.to_back.send(FrontToBack::SubscriptionClosed(id)).now_or_never();
	}
}
