use crate::error::Error;
use crate::jsonrpc::{self, JsonValue, Params, SubscriptionId};
use crate::traits::{TransportReceiver, TransportSender};

use core::marker::PhantomData;
use futures::{
	channel::{mpsc, oneshot},
	future::Either,
	pin_mut,
	prelude::*,
};

/// Client that can be cloned.
///
/// > **Note**: This struct is designed to be easy to use, but it works by maintaining a background
/// >           task running in parallel. If this is not desirable, you are encouraged to use the
/// >           [`RawClient`] struct instead.
#[derive(Clone)]
pub struct Client {
	/// Channel to send requests to the background task.
	to_back: mpsc::Sender<FrontToBack>,
}
/// Active subscription on a [`WsClient`].
pub struct Subscription<Notif> {
	/// Channel to send requests to the background task.
	to_back: mpsc::Sender<FrontToBack>,
	/// Channel from which we receive notifications from the server, as undecoded `JsonValue`s.
	notifs_rx: mpsc::Receiver<JsonValue>,
	/// Subscription ID,
	id: SubscriptionId,
	/// Marker in order to pin the `Notif` parameter.
	marker: PhantomData<Notif>,
}

/// Message that the [`Client`] can send to the background task.
enum FrontToBack {
	/// Send a one-shot notification to the server. The server doesn't give back any feedback.
	Notification {
		/// Method for the notification.
		method: String,
		/// Parameters to send to the server.
		params: jsonrpc::Params,
	},

	/// Send a request to the server.
	StartRequest {
		/// Method for the request.
		method: String,
		/// Parameters of the request.
		params: jsonrpc::Params,
		/// One-shot channel where to send back the outcome of that request.
		send_back: oneshot::Sender<Result<JsonValue, Error>>,
	},

	/// Send a subscription request to the server.
	Subscribe {
		/// Method for the subscription request.
		subscribe_method: String,
		/// Parameters to send for the subscription.
		params: jsonrpc::Params,
		/// Method to use to later unsubscription. Used if the channel unexpectedly closes.
		unsubscribe_method: String,
		/// When we get a response from the server about that subscription, we send the result on
		/// this channel. If the subscription succeeds, we return a `Receiver` that will receive
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

impl Client {
	/// Initializes a new client based upon this raw client.
	pub fn new<S, R>(sender: S, receiver: R) -> Client
	where
		S: TransportSender + Send + Sync + 'static,
		R: TransportReceiver + Send + Sync + 'static,
	{
		let (to_back, from_front) = mpsc::channel(16);
		async_std::task::spawn(async move {
			background_task(sender, receiver, from_front, 16).await;
		});
		Client { to_back }
	}

	/// Send a notification to the server.
	pub async fn notification(&self, method: impl Into<String>, params: impl Into<Params>) {
		todo!();
	}

	/// Perform a request towards the server.
	pub async fn request<Ret>(&self, method: impl Into<String>, params: impl Into<Params>) -> Result<Ret, Error>
	where
		Ret: jsonrpc::DeserializeOwned,
	{
		todo!();
	}

	/// Send a subscription request to the server.
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server. The `unsubscribe_method` is used to close the subscription.
	pub async fn subscribe<Notif>(
		&self,
		subscribe_method: impl Into<String>,
		params: impl Into<Params>,
		unsubscribe_method: impl Into<String>,
	) -> Result<Subscription<Notif>, Error> {
		todo!();
	}
}

impl<Notif> Subscription<Notif>
where
	Notif: jsonrpc::DeserializeOwned,
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
					Err(e) => log::error!("Subscription response error: {:?}", e),
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
		let id = core::mem::replace(&mut self.id, SubscriptionId::Num(0));
		let _ = self.to_back.send(FrontToBack::SubscriptionClosed(id)).now_or_never();
	}
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task<S, R>(
	mut sender: S,
	receiver: R,
	mut frontend: mpsc::Receiver<FrontToBack>,
	max_capacity_per_subscription: usize,
) where
	S: TransportSender + Send,
	R: TransportReceiver + Send,
{
	let backend_event = futures::stream::unfold(receiver, |mut receiver| async {
		let res = receiver.receive().await;
		Some((res, receiver))
	});

	pin_mut!(backend_event);

	loop {
		let request = jsonrpc::Request::Single(jsonrpc::Call::MethodCall(jsonrpc::MethodCall {
			jsonrpc: jsonrpc::Version::V2,
			method: "fooo".into(),
			params: Params::None,
			id: jsonrpc::Id::Num(0),
		}));
		let from_front = frontend.next();
		let next_response = backend_event.next();

		pin_mut!(from_front);
		pin_mut!(next_response);
		let rdy = match future::select(from_front, next_response).await {
			Either::Left((v, _)) => Either::Left(v),
			Either::Right((v, _)) => Either::Right(v),
		};

		let _ = sender.send(request).await;
	}
}
