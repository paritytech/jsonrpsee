use crate::error::Error;
use crate::jsonrpc::{self, DeserializeOwned, JsonValue, Params, SubscriptionId};
use core::marker::PhantomData;
use futures::channel::{mpsc, oneshot};
use futures::prelude::*;

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

/// Message that the Client can send to the background task.
pub enum FrontToBack {
	/// Send a one-shot notification to the server. The server doesn't give back any feedback.
	Notification {
		/// Method for the notification.
		method: String,
		/// Parameters to send to the server.
		params: Params,
	},

	/// Send a request to the server.
	StartRequest {
		/// Method for the request.
		method: String,
		/// Parameters of the request.
		params: Params,
		/// One-shot channel over which we send back the result of this request.
		send_back: oneshot::Sender<Result<JsonValue, Error>>,
	},

	/// Send a subscription request to the server.
	Subscribe {
		/// Method for the subscription request.
		subscribe_method: String,
		/// Parameters to send for the subscription.
		params: Params,
		/// Method to use to unsubscribe later. Used if the channel unexpectedly closes.
		unsubscribe_method: String,
		/// When we get a response from the server about that subscription, we send the result on
		/// this channel. If the subscription succeeds, we return a [Receiver](futures::channel::mpsc::Receiver) that will receive
		/// notifications.
		send_back: oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>,
	},

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
	/// This may return `None` if the subscription has been terminated, may happen if the channel becomes full or dropped.
	///
	/// Ignores any malformed packet.
	pub async fn next(&mut self) -> Option<Notif> {
		loop {
			match self.notifs_rx.next().await {
				Some(n) => match jsonrpc::from_value(n) {
					Ok(parsed) => return Some(parsed),
					Err(e) => log::info!("Subscription response error: {:?}", e),
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
