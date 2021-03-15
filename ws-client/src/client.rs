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
use async_std::sync::Mutex;
use async_trait::async_trait;
use futures::{
	channel::{mpsc, oneshot},
	future::Either,
	prelude::*,
	sink::SinkExt,
};
use jsonrpc::DeserializeOwned;
use jsonrpsee_types::{
	client::{FrontToBack, NotificationMessage, RequestMessage, Subscription, SubscriptionMessage},
	error::Error,
	jsonrpc::{self, JsonValue, SubscriptionId},
	traits::{Client, SubscriptionClient},
};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{borrow::Cow, convert::TryInto};

/// Wrapper over a [`oneshot::Receiver`](futures::channel::oneshot::Receiver) that reads
/// the underlying channel once and then stores the result in String.
/// It is possible that the error is read more than once if several calls are made
/// when the background thread has been terminated.
#[derive(Debug)]
enum ErrorFromBack {
	/// Error message is already read.
	Read(String),
	/// Error message is unread.
	Unread(oneshot::Receiver<Error>),
}

impl ErrorFromBack {
	async fn read_error(self) -> (Self, Error) {
		match self {
			Self::Unread(rx) => {
				let msg = match rx.await {
					Ok(msg) => msg.to_string(),
					// This should never happen because the receiving end is still alive.
					// Would be a bug in the logic of the background task.
					Err(_) => "Error reason could not be found. This is a bug. Please open an issue.".to_string(),
				};
				let err = Error::RestartNeeded(msg.clone());
				(Self::Read(msg), err)
			}
			Self::Read(msg) => (Self::Read(msg.clone()), Error::RestartNeeded(msg)),
		}
	}
}

/// WebSocket client that works by maintaining a background task running in parallel.
///
/// It's possible that the background thread is terminated and this makes the client unusable.
/// An error [`Error::RestartNeeded`] is returned if this happens and users has to manually
/// handle dropping and restarting a new client.
#[derive(Debug)]
pub struct WsClient {
	/// Channel to send requests to the background task.
	to_back: mpsc::Sender<FrontToBack>,
	/// If the background thread terminates the error is sent to this channel.
	// NOTE(niklasad1): This is a Mutex to circumvent that the async fns takes immutable references.
	error: Mutex<ErrorFromBack>,
	/// Request timeout
	request_timeout: Option<Duration>,
	/// True if the background thread has shutdown
	has_shutdown: Arc<AtomicBool>,
}

/// Configuration.
#[derive(Clone, Debug)]
pub struct WsConfig<'a> {
	/// URL to connect to.
	///
	/// If the port number is missing from the URL, the default port number is used.
	///
	///
	/// `ws://host` - port 80 is used
	///
	/// `wss://host` - port 443 is used
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
	/// Max concurrent request.
	pub max_concurrent_requests: usize,
	/// Max concurrent notification capacity for each subscription; when the capacity is exceeded the subscription will be dropped.
	///
	/// You can also prevent the subscription being dropped by calling [`WsSubscription::next()`](jsonrpsee_types::client::Subscription) frequently enough
	/// such that the buffer capacity doesn't exceeds.
	///
	/// **Note**: The actual capacity is `num_senders + max_subscription_capacity`
	/// because it is passed to [`futures::channel::mpsc::channel`].
	pub max_notifs_per_subscription: usize,
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
			max_concurrent_requests: 256,
			max_notifs_per_subscription: 4,
		}
	}
}

impl WsClient {
	/// Initializes a new WebSocket client
	///
	/// Fails when the URL is invalid.
	pub async fn new(config: WsConfig<'_>) -> Result<WsClient, Error> {
		let max_capacity_per_subscription = config.max_notifs_per_subscription;
		let max_concurrent_requests = config.max_concurrent_requests;
		let request_timeout = config.request_timeout;
		let (to_back, from_front) = mpsc::channel(config.max_concurrent_requests);
		let (err_tx, err_rx) = oneshot::channel();

		let (sender, receiver) = jsonrpc_transport::websocket_connection(config.clone())
			.await
			.map_err(|e| Error::TransportError(Box::new(e)))?;

		let has_shutdown = Arc::new(AtomicBool::new(false));
		// clone this to move into background task
		let has_shutdown_sender = has_shutdown.clone();

		async_std::task::spawn(async move {
			background_task(
				sender,
				receiver,
				from_front,
				err_tx,
				max_capacity_per_subscription,
				max_concurrent_requests,
			)
			.await;
			has_shutdown_sender.store(true, Ordering::Relaxed);
		});
		Ok(Self { to_back, request_timeout, error: Mutex::new(ErrorFromBack::Unread(err_rx)), has_shutdown })
	}

	/// Checks if the client is connected to the target.
	pub fn is_connected(&self) -> bool {
		!self.has_shutdown.load(Ordering::Relaxed)
	}

	// Reads the error message from the backend thread.
	async fn read_error_from_backend(&self) -> Error {
		let mut err_lock = self.error.lock().await;
		let from_back = std::mem::replace(&mut *err_lock, ErrorFromBack::Read(String::new()));
		let (next_state, err) = from_back.read_error().await;
		*err_lock = next_state;
		err
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
		match self.to_back.clone().send(FrontToBack::Notification(NotificationMessage { method, params })).await {
			Ok(()) => Ok(()),
			Err(_) => Err(self.read_error_from_backend().await),
		}
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

		if self
			.to_back
			.clone()
			.send(FrontToBack::StartRequest(RequestMessage { method, params, send_back: Some(send_back_tx) }))
			.await
			.is_err()
		{
			return Err(self.read_error_from_backend().await);
		}

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
			Err(_) => return Err(self.read_error_from_backend().await),
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
		if self
			.to_back
			.clone()
			.send(FrontToBack::Subscribe(SubscriptionMessage {
				subscribe_method,
				unsubscribe_method,
				params,
				send_back: send_back_tx,
			}))
			.await
			.is_err()
		{
			return Err(self.read_error_from_backend().await);
		}

		let (notifs_rx, id) = match send_back_rx.await {
			Ok(Ok(val)) => val,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};
		Ok(Subscription { to_back: self.to_back.clone(), notifs_rx, marker: PhantomData, id })
	}
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task(
	mut sender: jsonrpc_transport::Sender,
	receiver: jsonrpc_transport::Receiver,
	mut frontend: mpsc::Receiver<FrontToBack>,
	front_error: oneshot::Sender<Error>,
	max_notifs_per_subscription: usize,
	max_concurrent_requests: usize,
) {
	let mut manager = RequestManager::new(max_concurrent_requests);

	let backend_event = futures::stream::unfold(receiver, |mut receiver| async {
		let res = receiver.next_response().await;
		Some((res, receiver))
	});

	futures::pin_mut!(backend_event);

	loop {
		let next_frontend = frontend.next();
		let next_backend = backend_event.next();
		futures::pin_mut!(next_frontend, next_backend);

		match future::select(next_frontend, next_backend).await {
			// User dropped the sender side of the channel.
			// There is nothing to do just terminate.
			Either::Left((None, _)) => {
				log::trace!("[backend]: frontend dropped; terminate client");
				return;
			}

			// User called `notification` on the front-end
			Either::Left((Some(FrontToBack::Notification(notif)), _)) => {
				log::trace!("[backend]: client prepares to send notification: {:?}", notif);
				if let Err(e) = sender.send_notification(notif).await {
					log::warn!("[backend]: client notif failed: {:?}", e);
				}
			}

			// User called `request` on the front-end
			Either::Left((Some(FrontToBack::StartRequest(request)), _)) => {
				log::trace!("[backend]: client prepares to send request={:?}", request);
				if let Err(e) = sender.start_request(request, &mut manager).await {
					log::warn!("[backend]: client request failed: {:?}", e);
				}
			}
			// User called `subscribe` on the front-end.
			Either::Left((Some(FrontToBack::Subscribe(subscribe)), _)) => {
				log::trace!("[backend]: client prepares to start subscription: {:?}", subscribe);
				if let Err(e) = sender.start_subscription(subscribe, &mut manager).await {
					log::warn!("[backend]: client subscription failed: {:?}", e);
				}
			}
			// User dropped a subscription.
			Either::Left((Some(FrontToBack::SubscriptionClosed(sub_id)), _)) => {
				log::trace!("Closing subscription: {:?}", sub_id);
				// NOTE: The subscription may have been closed earlier if
				// the channel was full or disconnected.
				if let Some((_, unsub_method)) = manager
					.get_request_id_by_subscription_id(&sub_id)
					.and_then(|req_id| manager.remove_subscription(req_id, sub_id.clone()))
				{
					let json_sub_id = jsonrpc::to_value(sub_id).expect("SubscriptionID to JSON is infallible; qed");
					let request = RequestMessage {
						method: unsub_method,
						params: jsonrpc::Params::Array(vec![json_sub_id]),
						send_back: None,
					};
					send_unsubscribe_request(&mut sender, &mut manager, request).await;
				}
			}
			Either::Right((Some(Ok(jsonrpc::Response::Single(response))), _)) => {
				match process_response(&mut manager, response, max_notifs_per_subscription) {
					Ok(Some(unsub_request)) => {
						send_unsubscribe_request(&mut sender, &mut manager, unsub_request).await;
					}
					Ok(None) => (),
					Err(err) => {
						let _ = front_error.send(err);
						return;
					}
				}
			}
			Either::Right((Some(Ok(jsonrpc::Response::Batch(_responses))), _)) => {
				log::warn!("Ignore batch response not supported, #103");
			}
			Either::Right((Some(Ok(jsonrpc::Response::Notif(notif))), _)) => {
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
							manager
								.remove_subscription(request_id, sub_id)
								.expect("subscription is active; checked above");
							manager.reclaim_request_id(request_id);
						}
					}
					None => {
						log::error!("Subscription ID: {:?} not an active subscription", sub_id);
					}
				}
			}
			Either::Right((Some(Err(e)), _)) => {
				log::error!("Error: {:?} terminating client", e);
				let _ = front_error.send(Error::TransportError(Box::new(e)));
				return;
			}
			Either::Right((None, _)) => {
				log::error!("[backend]: WebSocket receiver dropped; terminate client");
				let _ = front_error.send(Error::Custom("WebSocket receiver dropped".into()));
				return;
			}
		}
	}
}

/// Process a response from the server.
///
/// Returns `Ok(None)` if the response was successful
/// Returns `Ok(Some(_))` if the response got an error but could be handled.
/// Returns `Err(_)` if the response couldn't be handled.
fn process_response(
	manager: &mut RequestManager,
	response: jsonrpc::Output,
	max_capacity_per_subscription: usize,
) -> Result<Option<RequestMessage>, Error> {
	let response_id: u64 = *response.id().as_number().ok_or(Error::InvalidRequestId)?;

	match manager.request_status(&response_id) {
		RequestStatus::PendingMethodCall => {
			let send_back_oneshot = match manager.complete_pending_call(response_id) {
				Some(Some(send)) => send,
				Some(None) => return Ok(None),
				None => return Err(Error::InvalidRequestId),
			};

			manager.reclaim_request_id(response_id);
			let response = response.try_into().map_err(Error::Request);
			let _ = send_back_oneshot.send(response);
			Ok(None)
		}
		RequestStatus::PendingSubscription => {
			let (send_back_oneshot, unsubscribe_method) =
				manager.complete_pending_subscription(response_id).ok_or(Error::InvalidRequestId)?;
			let json_sub_id: JsonValue = match response.try_into() {
				Ok(response) => response,
				Err(e) => {
					let _ = send_back_oneshot.send(Err(Error::Request(e)));
					return Ok(None);
				}
			};

			let sub_id: SubscriptionId = match jsonrpc::from_value(json_sub_id.clone()) {
				Ok(sub_id) => sub_id,
				Err(_) => {
					let _ = send_back_oneshot.send(Err(Error::InvalidSubscriptionId));
					return Ok(None);
				}
			};

			let (subscribe_tx, subscribe_rx) = mpsc::channel(max_capacity_per_subscription);
			if manager.insert_subscription(response_id, sub_id.clone(), subscribe_tx, unsubscribe_method).is_ok() {
				match send_back_oneshot.send(Ok((subscribe_rx, sub_id.clone()))) {
					Ok(_) => Ok(None),
					Err(_) => {
						let (_, unsubscribe_method) =
							manager.remove_subscription(response_id, sub_id).expect("Subscription inserted above; qed");
						manager.reclaim_request_id(response_id);
						let params = jsonrpc::Params::Array(vec![json_sub_id]);
						Ok(Some(RequestMessage { method: unsubscribe_method, params, send_back: None }))
					}
				}
			} else {
				let _ = send_back_oneshot.send(Err(Error::InvalidSubscriptionId));
				Ok(None)
			}
		}
		RequestStatus::Subscription | RequestStatus::Invalid => Err(Error::InvalidRequestId),
	}
}

async fn send_unsubscribe_request(
	sender: &mut jsonrpc_transport::Sender,
	manager: &mut RequestManager,
	request: RequestMessage,
) {
	if let Err(e) = sender.start_request(request, manager).await {
		log::error!("send unsubscribe request failed: {:?}", e);
	}
}
