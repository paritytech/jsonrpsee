// Copyright 2019 Parity Technologies (UK) Ltd.
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

use crate::client::ws::transport::WsConnectError;
use crate::client::ws::{RawClient, RawClientError, RawClientEvent, RawClientRequestId, WsTransportClient};
use crate::types::error::Error;
use crate::types::jsonrpc::{self, JsonValue};
// NOTE: this is a sign of a leaky abstraction to expose transport related details
// Should be removed after https://github.com/paritytech/jsonrpsee/issues/154
use soketto::connection::Error as SokettoError;

use futures::{
	channel::{mpsc, oneshot},
	future::Either,
	pin_mut,
	prelude::*,
	sink::SinkExt,
};
use std::{collections::HashMap, io, marker::PhantomData};

/// Client that can be cloned.
///
/// > **Note**: This struct is designed to be easy to use, but it works by maintaining a background
/// >           task running in parallel. If this is not desirable, you are encouraged to use the
/// >           [`RawClient`] struct instead.
#[derive(Clone)]
pub struct Client {
	/// Channel to send requests to the background task.
	to_back: mpsc::Sender<FrontToBack>,
	/// Config.
	config: Config,
}

#[derive(Copy, Clone, Debug)]
/// Configuration.
pub struct Config {
	/// Backend channel for serving requests and notifications.
	pub request_channel_capacity: usize,
	/// Backend channel for each unique subscription.
	pub subscription_channel_capacity: usize,
	/// Max request body size
	pub max_request_body_size: usize,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			request_channel_capacity: 100,
			subscription_channel_capacity: 4,
			max_request_body_size: 10 * 1024 * 1024,
		}
	}
}

/// Active subscription on a [`Client`].
pub struct Subscription<Notif> {
	/// Channel to send requests to the background task.
	to_back: mpsc::Sender<FrontToBack>,
	/// Channel from which we receive notifications from the server, as undecoded `JsonValue`s.
	notifs_rx: mpsc::Receiver<JsonValue>,
	/// Subscription ID,
	id: RawClientRequestId,
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
		send_back: oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, RawClientRequestId), Error>>,
	},

	/// When a subscription channel is closed, we send this message to the background
	/// task to mark it ready for garbage collection.
	// NOTE: It is not possible to cancel pending subscriptions or pending requests.
	// Such operations will be blocked until a response is received or the background
	// thread has been terminated.
	SubscriptionClosed(RawClientRequestId),
}

impl Client {
	/// Initializes a new WebSocket client
	///
	/// Fails when the URL is invalid.
	pub async fn new(target: impl AsRef<str>, config: Config) -> Result<Self, Error> {
		let transport = WsTransportClient::new(target).await.map_err(|e| Error::TransportError(Box::new(e)))?;
		let client = RawClient::new(transport);

		let (to_back, from_front) = mpsc::channel(config.request_channel_capacity);

		async_std::task::spawn(async move {
			background_task(client, from_front, config).await;
		});
		Ok(Client { to_back, config })
	}

	/// Send a notification to the server.
	pub async fn notification(
		&self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<(), Error> {
		let method = method.into();
		let params = params.into();
		log::trace!("[frontend]: send notification: method={:?}, params={:?}", method, params);
		self.to_back.clone().send(FrontToBack::Notification { method, params }).await.map_err(Error::Internal)
	}

	/// Perform a request towards the server.
	pub async fn request<Ret>(
		&self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<Ret, Error>
	where
		Ret: jsonrpc::DeserializeOwned,
	{
		let method = method.into();
		let params = params.into();
		log::trace!("[frontend]: send request: method={:?}, params={:?}", method, params);
		let (send_back_tx, send_back_rx) = oneshot::channel();
		self.to_back
			.clone()
			.send(FrontToBack::StartRequest { method, params, send_back: send_back_tx })
			.await
			.map_err(Error::Internal)?;

		let json_value = match send_back_rx.await {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => {
				let err = io::Error::new(io::ErrorKind::Other, "background task closed");
				return Err(Error::TransportError(Box::new(err)));
			}
		};
		jsonrpc::from_value(json_value).map_err(Error::ParseError)
	}

	/// Send a subscription request to the server.
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server. The `unsubscribe_method` is used to close the subscription.
	pub async fn subscribe<Notif>(
		&self,
		subscribe_method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
		unsubscribe_method: impl Into<String>,
	) -> Result<Subscription<Notif>, Error> {
		let subscribe_method = subscribe_method.into();
		let unsubscribe_method = unsubscribe_method.into();

		if subscribe_method == unsubscribe_method {
			return Err(Error::Subscription(subscribe_method, unsubscribe_method));
		}

		log::trace!("[frontend]: subscribe: {:?}, unsubscribe: {:?}", subscribe_method, unsubscribe_method);
		let (send_back_tx, send_back_rx) = oneshot::channel();
		self.to_back
			.clone()
			.send(FrontToBack::Subscribe {
				subscribe_method,
				unsubscribe_method,
				params: params.into(),
				send_back: send_back_tx,
			})
			.await
			.map_err(Error::Internal)?;

		let (notifs_rx, id) = match send_back_rx.await {
			Ok(Ok(val)) => val,
			Ok(Err(err)) => return Err(err),
			Err(_) => {
				let err = io::Error::new(io::ErrorKind::Other, "background task closed");
				return Err(Error::TransportError(Box::new(err)));
			}
		};

		Ok(Subscription { to_back: self.to_back.clone(), notifs_rx, marker: PhantomData, id })
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
		let _ = self.to_back.send(FrontToBack::SubscriptionClosed(self.id)).now_or_never();
	}
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task(mut client: RawClient, mut from_front: mpsc::Receiver<FrontToBack>, config: Config) {
	// List of subscription requests that have been sent to the server, with the method name to
	// unsubscribe.
	let mut pending_subscriptions: HashMap<RawClientRequestId, (oneshot::Sender<_>, _)> = HashMap::new();
	// List of subscription that are active on the server, with the method name to unsubscribe.
	let mut active_subscriptions: HashMap<RawClientRequestId, (mpsc::Sender<JsonValue>, _)> = HashMap::new();
	// List of requests that the server must answer.
	let mut ongoing_requests: HashMap<RawClientRequestId, oneshot::Sender<Result<_, _>>> = HashMap::new();

	loop {
		// We need to do a little transformation in order to destroy the borrow to `client`
		// and `from_front`.
		let outcome = {
			let next_message = from_front.next();
			let next_event = client.next_event();
			pin_mut!(next_message);
			pin_mut!(next_event);
			match future::select(next_message, next_event).await {
				Either::Left((v, _)) => Either::Left(v),
				Either::Right((v, _)) => Either::Right(v),
			}
		};

		match outcome {
			// If the channel is closed, then the `Client` has been destroyed and we
			// stop this task.
			Either::Left(None) => {
				log::trace!("[backend]: client terminated");
				return;
			}

			// User called `notification` on the front-end.
			Either::Left(Some(FrontToBack::Notification { method, params })) => {
				log::trace!("[backend]: client send notification");
				let _ = client.send_notification(method, params).await;
			}

			// User called `request` on the front-end.
			Either::Left(Some(FrontToBack::StartRequest { method, params, send_back })) => {
				log::trace!("[backend]: client prepare to send request={:?}", method);
				match client.start_request(method, params).await {
					Ok(id) => {
						ongoing_requests.insert(id, send_back);
					}
					Err(err) => {
						log::warn!("[backend]: client send request failed: {:?}", err);
						let _ = send_back.send(Err(Error::TransportError(Box::new(err))));
					}
				}
			}
			// User called `subscribe` on the front-end.
			Either::Left(Some(FrontToBack::Subscribe { subscribe_method, unsubscribe_method, params, send_back })) => {
				log::trace!(
					"[backend]: client prepare to start subscription, subscribe_method={:?} unsubscribe_method:{:?}",
					subscribe_method,
					unsubscribe_method
				);
				match client.start_subscription(subscribe_method, params).await {
					Ok(id) => {
						pending_subscriptions.insert(id, (send_back, unsubscribe_method));
					}
					Err(err) => {
						log::warn!("[backend]: client start subscription failed: {:?}", err);
						let _ = send_back.send(Err(Error::TransportError(Box::new(err))));
					}
				}
			}
			Either::Left(Some(FrontToBack::SubscriptionClosed(id))) => {
				if let Some((_, unsubscribe_method)) = active_subscriptions.remove(&id) {
					close_subscription(&mut client, id, unsubscribe_method).await;
				}
			}

			// Received a response to a request from the server.
			Either::Right(Ok(RawClientEvent::Response { request_id, result })) => {
				log::trace!("[backend] client received response to req={:?}, result={:?}", request_id, result);
				match ongoing_requests.remove(&request_id) {
					Some(r) => {
						if let Err(e) = r.send(result.map_err(Error::Request)) {
							log::error!("Could not dispatch pending request ID: {:?}, error: {:?}", request_id, e);
						}
					}
					None => log::error!("No pending response found for request ID {:?}", request_id),
				}
			}

			// Received a response from the server that a subscription is registered.
			Either::Right(Ok(RawClientEvent::SubscriptionResponse { request_id, result })) => {
				log::trace!("[backend]: client received response to subscription: {:?}", result);
				let (send_back, unsubscribe) = pending_subscriptions.remove(&request_id).unwrap();
				if let Err(err) = result {
					let _ = send_back.send(Err(Error::Request(err)));
				} else {
					let (notifs_tx, notifs_rx) = mpsc::channel(config.subscription_channel_capacity);

					// Send receiving end of `subscription channel` to the frontend
					if send_back.send(Ok((notifs_rx, request_id))).is_ok() {
						active_subscriptions.insert(request_id, (notifs_tx, unsubscribe));
					} else {
						close_subscription(&mut client, request_id, unsubscribe).await;
					}
				}
			}

			// Received a response on a subscription.
			Either::Right(Ok(RawClientEvent::SubscriptionNotif { request_id, result })) => {
				let notifs_tx = match active_subscriptions.get_mut(&request_id) {
					Some((notifs_tx, _)) => notifs_tx,
					None => {
						log::error!("Received notification on unknown subscription: {:?}", request_id);
						continue;
					}
				};

				match notifs_tx.try_send(result) {
					Ok(()) => (),
					// Channel is either full or disconnected, close it.
					Err(e) => {
						log::error!("Subscription ID: {:?} failed: {:?}", request_id, e);
						let (_, unsubscribe) =
							active_subscriptions.remove(&request_id).expect("Request is active checked above; qed");
						close_subscription(&mut client, request_id, unsubscribe).await;
					}
				}
			}

			// Request for the server to unsubscribe to us has succeeded.
			Either::Right(Ok(RawClientEvent::Unsubscribed { request_id: _ })) => {}

			Either::Right(Err(RawClientError::Inner(WsConnectError::Ws(SokettoError::UnexpectedOpCode(e))))) => {
				log::error!(
					"Client Error: {:?}, <https://github.com/paritytech/jsonrpsee/issues/154>",
					SokettoError::UnexpectedOpCode(e)
				);
			}
			Either::Right(Err(e)) => {
				// TODO: https://github.com/paritytech/jsonrpsee/issues/67
				log::error!("Client Error: {:?} terminating connection", e);
				break;
			}
		}
	}
}

/// Close subscription in RawClient helper.
/// Logs if the subscription couldn't be found.
async fn close_subscription(client: &mut RawClient, request_id: RawClientRequestId, unsubscribe_method: String) {
	match client.subscription_by_id(request_id).and_then(|s| s.into_active()) {
		Some(mut sub) => {
			if let Err(e) = sub.close(&unsubscribe_method).await {
				log::error!("RequestID : {:?}, unsubscribe to {} failed: {:?}", request_id, unsubscribe_method, e);
			}
		}
		None => log::error!("Request ID: {:?}, not an active subscription", request_id),
	}
}
