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
use crate::types::error::Error;
use crate::types::jsonrpc::{self, JsonValue};
// NOTE: this is a sign of a leaky abstraction to expose transport related details
// Should be removed after https://github.com/paritytech/jsonrpsee/issues/154
use crate::client::ws::manager::{RequestManager, RequestStatus};

use futures::{
	channel::{mpsc, oneshot},
	prelude::*,
	sink::SinkExt,
};
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
	id: String,
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
		send_back: oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, String), Error>>,
	},

	/// When a subscription channel is closed, we send this message to the background
	/// task to mark it ready for garbage collection.
	// NOTE: It is not possible to cancel pending subscriptions or pending requests.
	// Such operations will be blocked until a response is received or the background
	// thread has been terminated.
	SubscriptionClosed(String),
}

impl Client {
	/// Initializes a new WebSocket client
	///
	/// Fails when the URL is invalid.
	pub async fn new(target: impl AsRef<str>, config: Config) -> Result<Self, Error> {
		let (sender, receiver) = jsonrpc_transport::websocket_context(target.as_ref()).await.unwrap();

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
		let id = std::mem::take(&mut self.id);
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
			event = next_frontend => {
				match event {
					None => {
						log::trace!("[backend]: frontend channel dropped; terminate client");
						break
					}
					Some(FrontToBack::Notification { method, params }) => {
						log::trace!("[backend]: client send notification");
						let _ = sender.send_notification(method, params).await;
					}
					Some(FrontToBack::StartRequest { method, params, send_back }) => {
						log::trace!("[backend]: client prepare to send request={:?}", method);
						match sender.start_request(method, params).await {
							Ok(id) => {
								manager.insert_pending_request(id, send_back).unwrap();
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
								manager.insert_pending_subscription(id, send_back, unsubscribe_method).unwrap();
							}
							Err(err) => {
								log::warn!("[backend]: client start subscription failed: {:?}", err);
								let _ = send_back.send(Err(Error::TransportError(Box::new(err))));
							}
						}
					}
					Some(FrontToBack::SubscriptionClosed(sub_id)) => {
						log::debug!("Closing in subscription: {}", sub_id);
						// NOTE: The subscription may have been closed earlier if
						// the channel was full or disconnected.
						if let Ok(request_id) = manager.get_request_id(&sub_id) {
							manager.remove_active_subscription(&request_id, &sub_id).expect("subscription ID and request ID valid; checked above qed");
						}
					}
				}
			}
			event = next_backend => {
				match event {
					None => {
						log::trace!("[backend]: backend channel dropped; terminate client");
						break;
					}
					Some(Ok(jsonrpc::Response::Single(response))) => {
						if let Some((unsubscribe, params)) = handle_request(&mut manager, response, config.subscription_channel_capacity) {
							sender.start_request(unsubscribe, params).await.unwrap();
						}
					}
					Some(Ok(jsonrpc::Response::Batch(responses))) => {
						for response in responses {
							if let Some((unsubscribe, params)) = handle_request(&mut manager, response, config.subscription_channel_capacity) {
								sender.start_request(unsubscribe, params).await.unwrap();
							}
						}
					}
					Some(Ok(jsonrpc::Response::Notif(notif))) => {
						// TODO: possible to avoid allocation here if the subscription is a number for example.
						let sub_id = notif.params.subscription.into_string();
						let request_id = match manager.get_request_id(&sub_id) {
							Ok(r) => r,
							Err(_) => {
								log::error!("Subscription ID: {:?} not found", sub_id);
								continue;
							}
						};

						match manager.as_active_subscription(&request_id) {
							Ok(callback) => {
								if let Err(e) = callback.try_send(notif.params.result) {
									log::error!("Dropping subscription {} error: {:?}", sub_id, e);
									manager.remove_active_subscription(&request_id, &sub_id).unwrap();
								}
							}
							Err(_) => (),
						}
					}
					Some(Err(e)) => {
						log::error!("error: {:?}", e);
						return;
					}
				}
			},
		}
	}
}

fn handle_request(
	manager: &mut RequestManager,
	response: jsonrpc::Output,
	subscription_capacity: usize,
) -> Option<(String, jsonrpc::Params)> {
	let response_id = match response.id().as_number() {
		Some(n) => *n,
		None => {
			log::error!("Invalid request ID: {:?}", response.id());
			return None;
		}
	};

	match manager.request_status(&response_id) {
		RequestStatus::PendingMethodCall => {
			let callback = manager.try_complete_method_call(&response_id).unwrap();
			let response: Result<JsonValue, Error> = response.try_into().map_err(Error::Request);
			callback.send(response).unwrap();
			None
		}
		RequestStatus::PendingSubscription => {
			let (callback, unsubscribe_method) = manager.try_complete_pending_subscription(&response_id).unwrap();

			let subscription_id: JsonValue = match response.try_into() {
				Ok(response) => response,
				Err(e) => {
					let _ = callback.send(Err(Error::Request(e)));
					return None;
				}
			};

			// TODO: ugly thing for https://github.com/serde-rs/json/issues/709
			let sub_id = subscription_id.as_str().unwrap().to_owned();
			let (subscribe_tx, subscribe_rx) = mpsc::channel(subscription_capacity);

			// Send receiving end of `subscription channel` to the frontend
			match callback.send(Ok((subscribe_rx, sub_id.clone()))) {
				Ok(_) => {
					manager.insert_active_subscription(response_id, sub_id, subscribe_tx, unsubscribe_method).unwrap();
					None
				}
				Err(_) => {
					let (_, unsubscribe_method) = manager.remove_active_subscription(&response_id, &sub_id).unwrap();
					let params = jsonrpc::Params::Array(vec![subscription_id]);
					Some((unsubscribe_method, params))
				}
			}
		}
		RequestStatus::Subscription => unreachable!(),
		RequestStatus::Invalid => {
			log::error!("Invalid request ID: {:?}", response_id);
			None
		}
	}
}
