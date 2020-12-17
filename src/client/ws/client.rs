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

use crate::client::ws::jsonrpc_transport;
use crate::client::ws::manager::{RequestManager, RequestStatus};
use crate::types::error::Error;
use crate::types::jsonrpc::{self, JsonValue};

use futures::{
	channel::{mpsc, oneshot},
	prelude::*,
	sink::SinkExt,
};
use jsonrpc::SubscriptionId;
use std::convert::TryInto;
use std::{io, marker::PhantomData};

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
	/// Initializes a new WebSocket client
	///
	/// Fails when the URL is invalid.
	pub async fn new(remote_addr: impl AsRef<str>, config: Config) -> Result<Self, Error> {
		let (sender, receiver) = jsonrpc_transport::websocket_connection(remote_addr.as_ref())
			.await
			.map_err(|e| Error::TransportError(Box::new(e)))?;

		let (to_back, from_front) = mpsc::channel(config.request_channel_capacity);

		async_std::task::spawn(async move {
			background_task(sender, receiver, from_front, config).await;
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
		let id = std::mem::replace(&mut self.id, SubscriptionId::Num(0));
		let _ = self.to_back.send(FrontToBack::SubscriptionClosed(id)).now_or_never();
	}
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task(
	mut sender: jsonrpc_transport::Sender,
	receiver: jsonrpc_transport::Receiver,
	mut frontend: mpsc::Receiver<FrontToBack>,
	config: Config,
) {
	let mut manager = RequestManager::new();

	let backend_event = futures::stream::unfold(receiver, |mut receiver| async {
		let res = receiver.next_response().await;
		Some((res, receiver))
	});

	futures::pin_mut!(backend_event);

	loop {
		let next_frontend = frontend.next();
		let next_backend = backend_event.next();
		futures::pin_mut!(next_frontend, next_backend);

		futures::select! {
			event = next_frontend => match event {
				// User dropped its sender side of the channel.
				None => {
					log::trace!("[backend]: frontend channel dropped; terminate client");
					break
				}
				// User called `notification` on the front-end
				Some(FrontToBack::Notification { method, params }) => {
					log::trace!("[backend]: client send notification");
					let _ = sender.send_notification(method, params).await;
				}
				// User called `request` on the front-end
				Some(FrontToBack::StartRequest { method, params, send_back }) => {
					log::trace!("[backend]: client prepare to send request={:?}", method);
					match sender.start_request(method, params).await {
						Ok(id) => {
							if let Err(send_back) = manager.insert_pending_call(id, send_back) {
								let _ = send_back.send(Err(Error::DuplicateRequestId));
							}
						}
						Err(err) => {
							log::warn!("[backend]: client send request failed: {:?}", err);
							let _ = send_back.send(Err(Error::TransportError(Box::new(err))));
						}
					}
				}
				// User called `subscribe` on the front-end.
				Some(FrontToBack::Subscribe { subscribe_method, unsubscribe_method, params, send_back }) => {
					log::trace!(
						"[backend]: client prepare to start subscription, subscribe_method={:?} unsubscribe_method:{:?}",
						subscribe_method,
						unsubscribe_method
					);
					match sender.start_subscription(subscribe_method, params).await {
						Ok(id) => {
							if let Err(send_back) = manager.insert_pending_subscription(id, send_back, unsubscribe_method) {
								let _ = send_back.send(Err(Error::DuplicateRequestId));
							}
						}
						Err(err) => {
							log::warn!("[backend]: client start subscription failed: {:?}", err);
							let _ = send_back.send(Err(Error::TransportError(Box::new(err))));
						}
					}
				}
				// User dropped a subscription.
				Some(FrontToBack::SubscriptionClosed(sub_id)) => {
					log::trace!("Closing in subscription: {:?}", sub_id);
					// NOTE: The subscription may have been closed earlier if
					// the channel was full or disconnected.
					if let Some(request_id) = manager.get_request_id_by_subscription_id(&sub_id) {
						manager.remove_subscription(request_id, sub_id.clone());
					}
				}
			},
			event = next_backend => match event {
				None => {
					log::trace!("[backend]: backend channel dropped; terminate client");
					break;
				}
				Some(Ok(jsonrpc::Response::Single(response))) => {
					match process_response(&mut manager, response, config.subscription_channel_capacity) {
						Ok(Some((unsubscribe, params))) => {
							if let Err(e) = sender.start_request(unsubscribe, params).await {
								log::error!("Failed to send unsubscription response: {:?}", e);
							}
						}
						Ok(None) => (),
						Err(e) => {
							log::error!("Error: {:?} terminating client", e);
							return;
						}
					}
				}
				Some(Ok(jsonrpc::Response::Batch(responses))) => {
					// if any request fails, throw away entire batch.
					for response in responses {
						match process_response(&mut manager, response, config.subscription_channel_capacity) {
							Ok(Some((unsubscribe, params))) => {
								if let Err(e) = sender.start_request(unsubscribe, params).await {
									log::error!("Failed to send unsubscription response: {:?}", e);
								}
							}
							Ok(None) => (),
							Err(e) => {
								log::error!("Error: {:?} terminating client", e);
								return;
							}
						}
					}
				}
				Some(Ok(jsonrpc::Response::Notif(notif))) => {
					let sub_id = notif.params.subscription;
					let request_id = match manager.get_request_id_by_subscription_id(&sub_id) {
						Some(r) => r,
						None => {
							log::error!("Subscription ID: {:?} not found", sub_id);
							continue;
						}
					};

					match manager.as_subscription_mut(&request_id) {
						Some(send_back_sink) => {
							if let Err(e) = send_back_sink.try_send(notif.params.result) {
								log::error!("Dropping subscription {:?} error: {:?}", sub_id, e);
								manager.remove_subscription(request_id, sub_id).expect("subscription is active; checked above");
							}
						}
						None => {
							log::error!("Subscription ID: {:?} not an active subscription", sub_id);
						},
					}
				}
				Some(Err(e)) => {
					log::error!("Error: {:?} terminating client", e);
					return;
				}
			},
		}
	}
}

/// Process a response from the server.
///
/// Returns `Ok(_)` if the response was successful or if the error could be handled.
/// Returns `Err(_)` if the response couldn't be handled.
fn process_response(
	manager: &mut RequestManager,
	response: jsonrpc::Output,
	subscription_capacity: usize,
) -> Result<Option<(String, jsonrpc::Params)>, Error> {
	let response_id = match response.id().as_number() {
		Some(n) => *n,
		None => return Err(Error::InvalidRequestId),
	};

	match manager.request_status(&response_id) {
		RequestStatus::PendingMethodCall => {
			let send_back_oneshot = match manager.complete_pending_call(response_id) {
				Some(send_back) => send_back,
				None => return Err(Error::InvalidRequestId),
			};
			let response: Result<JsonValue, Error> = response.try_into().map_err(Error::Request);
			match send_back_oneshot.send(response) {
				Err(Err(e)) => Err(e),
				_ => Ok(None),
			}
		}
		RequestStatus::PendingSubscription => {
			let (send_back_oneshot, unsubscribe_method) = match manager.complete_pending_subscription(response_id) {
				Some(pending) => pending,
				None => return Err(Error::InvalidRequestId),
			};

			let json_sub_id: JsonValue = match response.try_into() {
				Ok(response) => response,
				Err(e) => {
					return match send_back_oneshot.send(Err(Error::Request(e))) {
						Err(Err(e)) => Err(e),
						_ => Ok(None),
					};
				}
			};

			let sub_id: SubscriptionId = match jsonrpc::from_value(json_sub_id.clone()) {
				Ok(sub_id) => sub_id,
				Err(_) => {
					return match send_back_oneshot.send(Err(Error::InvalidSubscriptionId)) {
						Err(Err(e)) => Err(e),
						_ => Ok(None),
					}
				}
			};

			let (subscribe_tx, subscribe_rx) = mpsc::channel(subscription_capacity);
			if manager.insert_subscription(response_id, sub_id.clone(), subscribe_tx, unsubscribe_method).is_ok() {
				match send_back_oneshot.send(Ok((subscribe_rx, sub_id))) {
					Ok(_) => Ok(None),
					Err(Ok((_val, sub_id))) => {
						let (_, unsubscribe_method) =
							manager.remove_subscription(response_id, sub_id).expect("Subscription inserted above; qed");
						let params = jsonrpc::Params::Array(vec![json_sub_id]);
						Ok(Some((unsubscribe_method, params)))
					}
					Err(Err(e)) => Err(e),
				}
			} else {
				match send_back_oneshot.send(Err(Error::InvalidSubscriptionId)) {
					Err(Err(e)) => Err(e),
					_ => Ok(None),
				}
			}
		}
		RequestStatus::Subscription | RequestStatus::Invalid => Err(Error::InvalidRequestId),
	}
}
