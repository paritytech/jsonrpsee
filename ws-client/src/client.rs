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

use crate::jsonrpc_transport;
use crate::manager::{RequestManager, RequestStatus};
use async_trait::async_trait;
use futures::{
	channel::{mpsc, oneshot},
	prelude::*,
	sink::SinkExt,
};
use jsonrpc::DeserializeOwned;
use jsonrpsee_types::{
	client::{FrontToBack, Subscription},
	error::Error,
	jsonrpc::{self, JsonValue, SubscriptionId},
	traits::{Client, SubscriptionClient},
};
use std::time::Duration;
use std::{borrow::Cow, convert::TryInto};
use std::{io, marker::PhantomData};

/// Client that can be cloned.
///
/// > **Note**: This struct is designed to be easy to use, but it works by maintaining a background task running in parallel.
#[derive(Clone)]
pub struct WsClient {
	/// Channel to send requests to the background task.
	to_back: mpsc::Sender<FrontToBack>,
	/// Request timeout
	request_timeout: Option<Duration>,
}

/// Configuration.
#[derive(Clone, Debug)]
pub struct WsConfig<'a> {
	/// URL to connect to.
	pub url: &'a str,
	/// Max request body size
	pub max_request_body_size: usize,
	/// Request timeout
	pub request_timeout: Option<Duration>,
	/// Connection timeout
	pub connection_timeout: Duration,
	/// `Origin` header to pass during the HTTP handshake. If `None`, no
	/// `Origin` header was passed.
	pub origin: Option<Cow<'a, str>>,
	/// Url to send during the HTTP handshake.
	pub handshake_url: Cow<'a, str>,
	/// Max concurrent request capacity.
	///
	/// **Note**: The actual capacity is `num_senders + max_concurrent_requests_capacity`
	/// because it is passed to [`futures::channel::mpsc::channel`]
	/// and the capacity may increase because the sender is cloned when new
	/// requests, notifications and subscriptions are created.
	pub max_concurrent_requests_capacity: usize,
	/// Max concurrent capacity for each subscription; when the capacity is exceeded the subscription will be dropped.
	///
	/// You can also prevent the subscription being dropped by calling [`WsSubscription::next()`](jsonrpsee_types::client::Subscription) frequently enough
	/// such that the buffer capacity doesn't exceeds.
	///
	/// **Note**: The actual capacity is `num_senders + max_subscription_capacity`
	/// because it is passed to [`futures::channel::mpsc::channel`].
	pub max_subscription_capacity: usize,
}

impl<'a> WsConfig<'a> {
	/// Default WebSocket configuration with a specified URL to connect to.
	pub fn with_url(url: &'a str) -> Self {
		Self {
			url,
			max_request_body_size: 10 * 1024 * 1024,
			request_timeout: None,
			connection_timeout: Duration::from_secs(10),
			origin: None,
			handshake_url: From::from("/"),
			max_concurrent_requests_capacity: 256,
			max_subscription_capacity: 4,
		}
	}
}

impl WsClient {
	/// Initializes a new WebSocket client
	///
	/// Fails when the URL is invalid.
	pub async fn new(config: WsConfig<'_>) -> Result<WsClient, Error> {
		let max_capacity_per_subscription = config.max_subscription_capacity;
		let request_timeout = config.request_timeout;
		let (to_back, from_front) = mpsc::channel(config.max_concurrent_requests_capacity);

		let (sender, receiver) =
			jsonrpc_transport::websocket_connection(config).await.map_err(|e| Error::TransportError(Box::new(e)))?;

		async_std::task::spawn(async move {
			background_task(sender, receiver, from_front, max_capacity_per_subscription).await;
		});
		Ok(Self { to_back, request_timeout })
	}
}

#[async_trait]
impl Client for WsClient {
	/// Send a notification to the server.
	async fn notification<M, P>(&self, method: M, params: P) -> Result<(), Error>
	where
		M: Into<String> + Send,
		P: Into<jsonrpc::Params> + Send,
	{
		let method = method.into();
		let params = params.into();
		log::trace!("[frontend]: send notification: method={:?}, params={:?}", method, params);
		self.to_back.clone().send(FrontToBack::Notification { method, params }).await.map_err(Error::Internal)
	}

	/// Perform a request towards the server.
	async fn request<T, M, P>(&self, method: M, params: P) -> Result<T, Error>
	where
		T: DeserializeOwned,
		M: Into<String> + Send,
		P: Into<jsonrpc::Params> + Send,
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

		let send_back_rx_out = if let Some(duration) = self.request_timeout {
			let timeout = async_std::task::sleep(duration);
			futures::pin_mut!(send_back_rx, timeout);
			match future::select(send_back_rx, timeout).await {
				future::Either::Left((send_back_rx_out, _)) => send_back_rx_out,
				future::Either::Right((_, _)) => return Err(Error::WsRequestTimeout),
			}
		} else {
			send_back_rx.await
		};

		let json_value = match send_back_rx_out {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => {
				let err = io::Error::new(io::ErrorKind::Other, "background task closed");
				return Err(Error::TransportError(Box::new(err)));
			}
		};
		jsonrpc::from_value(json_value).map_err(Error::ParseError)
	}
}

#[async_trait]
impl SubscriptionClient for WsClient {
	/// Send a subscription request to the server.
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server. The `unsubscribe_method` is used to close the subscription.
	async fn subscribe<SM, UM, P, N>(
		&self,
		subscribe_method: SM,
		params: P,
		unsubscribe_method: UM,
	) -> Result<Subscription<N>, Error>
	where
		SM: Into<String> + Send,
		UM: Into<String> + Send,
		P: Into<jsonrpc::Params> + Send,
		N: DeserializeOwned,
	{
		let subscribe_method = subscribe_method.into();
		let unsubscribe_method = unsubscribe_method.into();
		let params = params.into();

		if subscribe_method == unsubscribe_method {
			return Err(Error::Subscription(subscribe_method, unsubscribe_method));
		}

		log::trace!("[frontend]: subscribe: {:?}, unsubscribe: {:?}", subscribe_method, unsubscribe_method);
		let (send_back_tx, send_back_rx) = oneshot::channel();
		self.to_back
			.clone()
			.send(FrontToBack::Subscribe { subscribe_method, unsubscribe_method, params, send_back: send_back_tx })
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

/// Function being run in the background that processes messages from the frontend.
async fn background_task(
	mut sender: jsonrpc_transport::Sender,
	receiver: jsonrpc_transport::Receiver,
	mut frontend: mpsc::Receiver<FrontToBack>,
	max_capacity_per_subscription: usize,
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
				// User dropped the sender side of the channel.
				None => {
					log::trace!("[backend]: frontend channel dropped; terminate client");
					break
				}
				// User called `notification` on the front-end
				Some(FrontToBack::Notification { method, params }) => {
					log::trace!("[backend]: client prepares to send notification");
					let _ = sender.send_notification(method, params).await;
				}
				// User called `request` on the front-end
				Some(FrontToBack::StartRequest { method, params, send_back }) => {
					log::trace!("[backend]: client prepares to send request={:?}", method);
					match sender.start_request(method, params).await {
						Ok(id) => {
							if let Err(send_back) = manager.insert_pending_call(id, send_back) {
								let _ = send_back.send(Err(Error::DuplicateRequestId));
							}
						}
						Err(err) => {
							log::warn!("[backend]: client request failed: {:?}", err);
							let _ = send_back.send(Err(Error::TransportError(Box::new(err))));
						}
					}
				}
				// User called `subscribe` on the front-end.
				Some(FrontToBack::Subscribe { subscribe_method, unsubscribe_method, params, send_back }) => {
					log::trace!(
						"[backend]: client prepares to start subscription, subscribe_method={:?} unsubscribe_method:{:?}",
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
							log::warn!("[backend]: client subscription failed: {:?}", err);
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
						if let Some((_sink, unsubscribe_method)) = manager.remove_subscription(request_id, sub_id.clone()) {
							if let Ok(json_sub_id) = jsonrpc::to_value(sub_id) {
								let params = jsonrpc::Params::Array(vec![json_sub_id]);
								let _ = sender.start_request(unsubscribe_method, params).await;
							}
						}
					}
				}
			},
			event = next_backend => match event {
				None => {
					log::trace!("[backend]: backend channel dropped; terminate client");
					break;
				}
				Some(Ok(jsonrpc::Response::Single(response))) => {
					match process_response(&mut manager, response, max_capacity_per_subscription) {
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
						match process_response(&mut manager, response, max_capacity_per_subscription) {
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
	max_capacity_per_subscription: usize,
) -> Result<Option<(String, jsonrpc::Params)>, Error> {
	let response_id = *response.id().as_number().ok_or(Error::InvalidRequestId)?;

	match manager.request_status(&response_id) {
		RequestStatus::PendingMethodCall => {
			let send_back_oneshot = manager.complete_pending_call(response_id).ok_or(Error::InvalidRequestId)?;
			let response = response.try_into().map_err(Error::Request);
			match send_back_oneshot.send(response) {
				Err(Err(e)) => Err(e),
				Err(Ok(_)) => Err(Error::Custom("Frontend channel closed".into())),
				Ok(_) => Ok(None),
			}
		}
		RequestStatus::PendingSubscription => {
			let (send_back_oneshot, unsubscribe_method) =
				manager.complete_pending_subscription(response_id).ok_or(Error::InvalidRequestId)?;
			let json_sub_id: JsonValue = match response.try_into() {
				Ok(response) => response,
				Err(e) => {
					return match send_back_oneshot.send(Err(Error::Request(e))) {
						Err(Err(e)) => Err(e),
						Err(Ok(_)) => unreachable!("Error sent above; qed"),
						_ => Ok(None),
					};
				}
			};

			let sub_id: SubscriptionId = match jsonrpc::from_value(json_sub_id.clone()) {
				Ok(sub_id) => sub_id,
				Err(_) => {
					return match send_back_oneshot.send(Err(Error::InvalidSubscriptionId)) {
						Err(Err(e)) => Err(e),
						Err(Ok(_)) => unreachable!("Error sent above; qed"),
						_ => Ok(None),
					}
				}
			};

			let (subscribe_tx, subscribe_rx) = mpsc::channel(max_capacity_per_subscription);
			if manager.insert_subscription(response_id, sub_id.clone(), subscribe_tx, unsubscribe_method).is_ok() {
				match send_back_oneshot.send(Ok((subscribe_rx, sub_id.clone()))) {
					Ok(_) => Ok(None),
					Err(_) => {
						let (_, unsubscribe_method) =
							manager.remove_subscription(response_id, sub_id).expect("Subscription inserted above; qed");
						let params = jsonrpc::Params::Array(vec![json_sub_id]);
						Ok(Some((unsubscribe_method, params)))
					}
				}
			} else {
				match send_back_oneshot.send(Err(Error::InvalidSubscriptionId)) {
					Err(Err(e)) => Err(e),
					Err(Ok(_)) => unreachable!("Error sent above; qed"),
					_ => Ok(None),
				}
			}
		}
		RequestStatus::Subscription | RequestStatus::Invalid => Err(Error::InvalidRequestId),
	}
}
