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
use crate::client::ws::transport::WsConnectError;
use crate::types::error::Error;
use crate::types::jsonrpc::{self, JsonValue};
// NOTE: this is a sign of a leaky abstraction to expose transport related details
// Should be removed after https://github.com/paritytech/jsonrpsee/issues/154
use crate::client::ws::manager::{RequestManager, RequestStatus};
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
		send_back: oneshot::Sender<Result<mpsc::Receiver<JsonValue>, Error>>,
	},

	/// When a request or subscription channel is closed, we send this message to the background
	/// task in order for it to garbage collect closed requests and subscriptions.
	///
	/// While this means that closing a request or a subscription is a `O(n)` operation, it is
	/// expected that the volume of requests and subscriptions is low enough that this isn't
	/// a problem in practice.
	ChannelClosed,
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

		let notifs_rx = match send_back_rx.await {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => {
				let err = io::Error::new(io::ErrorKind::Other, "background task closed");
				return Err(Error::TransportError(Box::new(err)));
			}
		};
		Ok(Subscription { to_back: self.to_back.clone(), notifs_rx, marker: PhantomData })
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
		let _ = self.to_back.try_send(FrontToBack::ChannelClosed);
	}
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task(
	mut sender: jsonrpc_transport::Sender,
	mut receiver: jsonrpc_transport::Receiver,
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
						log::trace!("[backend]: client terminated");
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
					todo!();
					/*match sender.start_subscription(subscribe_method, params).await {
						Ok(id) => {
							manager.insert_pending_subscription(id, send_back, unsubscribe_method);
						}
						Err(err) => {
							log::warn!("[backend]: client start subscription failed: {:?}", err);
							let _ = send_back.send(Err(Error::TransportError(Box::new(err))));
						}
					}*/
				}
			Some(FrontToBack::ChannelClosed) => {
				todo!()
			}

					_ => (),
				}
			}
			event = next_backend => {
				match event {
					Some(Ok(responses)) => {
						log::trace!("[backend] client received response {:?}", responses);
						for (response_id, response) in responses {
							handle_response(&mut manager, response_id, response, config.subscription_channel_capacity);
						}
					}
					_ => (),
				}

			},
		}
	}
}

fn handle_response(
	manager: &mut RequestManager,
	response_id: u64,
	response: jsonrpc::Output,
	subscription_capacity: usize,
) {
	match manager.request_status(&response_id) {
		RequestStatus::PendingMethodCall => {
			let callback = manager.try_complete_method_call(&response_id).unwrap();
			callback.send(Result::from(response).map_err(Error::Request)).unwrap();
		}
		RequestStatus::PendingSubscription => {
			let (callback, unsubscribe_method) = manager.try_complete_pending_subscription(&response_id).unwrap();
			let (subscribe_tx, subscribe_rx) = mpsc::channel(subscription_capacity);

			// Send receiving end of `subscription channel` to the frontend
			match callback.send(Ok((subscribe_rx, response_id))) {
				Ok(_) => manager.insert_active_subscription(response_id, subscribe_tx, unsubscribe_method).unwrap(),
				//TODO: send unsubscribe request.
				Err(_) => (),
			};
		}

		RequestStatus::Subscription => {
			let callback = manager.as_subscription(&response_id).unwrap();
			let json_val = Result::from(response).unwrap();
			callback.try_send(json_val).unwrap();
		}

		RequestStatus::Invalid => {}
	}
}
