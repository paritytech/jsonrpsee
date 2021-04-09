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

use crate::manager::{RequestManager, RequestStatus};
use crate::transport::WsTransportClientBuilder;
use crate::{jsonrpc_transport, transport::parse_url};
use async_std::sync::Mutex;
use async_trait::async_trait;
use futures::{
	channel::{mpsc, oneshot},
	future::Either,
	prelude::*,
	sink::SinkExt,
};
use jsonrpsee_types::{
	client::{BatchMessage, FrontToBack, NotificationMessage, RequestMessage, Subscription, SubscriptionMessage},
	error::Error,
	traits::{Client, SubscriptionClient},
	v2::dummy::{JsonRpcMethod, JsonRpcParams, JsonRpcResponse, JsonRpcResponseObject, SubscriptionId},
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::time::Duration;

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
	//
	// NOTE(niklasad1): allow str slices instead of allocated Strings but it's hard to do because putting a lifetime on FrontToBack
	// screws everything up.
	to_back: mpsc::Sender<FrontToBack>,
	/// If the background thread terminates the error is sent to this channel.
	// NOTE(niklasad1): This is a Mutex to circumvent that the async fns takes immutable references.
	error: Mutex<ErrorFromBack>,
	/// Request timeout
	request_timeout: Option<Duration>,
}

/// Configuration.
#[derive(Clone, Debug)]
pub struct WsClientBuilder<'a> {
	max_request_body_size: usize,
	request_timeout: Option<Duration>,
	connection_timeout: Duration,
	origin: Option<Cow<'a, str>>,
	handshake_url: Cow<'a, str>,
	max_concurrent_requests: usize,
	max_notifs_per_subscription: usize,
}

impl<'a> Default for WsClientBuilder<'a> {
	fn default() -> Self {
		Self {
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

impl<'a> WsClientBuilder<'a> {
	/// Set max request body size.
	pub fn max_request_body_size(mut self, size: usize) -> Self {
		self.max_request_body_size = size;
		self
	}

	/// Set request timeout.
	pub fn request_timeout(mut self, timeout: Option<Duration>) -> Self {
		self.request_timeout = timeout;
		self
	}

	/// Set connection timeout for the handshake.
	pub fn connection_timeout(mut self, timeout: Duration) -> Self {
		self.connection_timeout = timeout;
		self
	}

	/// Set origin header to pass during the handshake.
	pub fn origin_header(mut self, origin: Option<Cow<'a, str>>) -> Self {
		self.origin = origin;
		self
	}

	/// Set URL to send during the handshake.
	pub fn handshake_url(mut self, url: Cow<'a, str>) -> Self {
		self.handshake_url = url;
		self
	}

	/// Set max concurrent requests.
	pub fn max_concurrent_requests(mut self, max: usize) -> Self {
		self.max_concurrent_requests = max;
		self
	}

	/// Set max concurrent notification capacity for each subscription; when the capacity is exceeded the subscription will be dropped.
	///
	/// You can also prevent the subscription being dropped by calling [`WsSubscription::next()`](jsonrpsee_types::client::Subscription) frequently enough
	/// such that the buffer capacity doesn't exceeds.
	///
	/// **Note**: The actual capacity is `num_senders + max_subscription_capacity`
	/// because it is passed to [`futures::channel::mpsc::channel`].
	///
	pub fn max_notifs_per_subscription(mut self, max: usize) -> Self {
		self.max_notifs_per_subscription = max;
		self
	}

	/// Build the client with specified URL to connect to.
	/// If the port number is missing from the URL, the default port number is used.
	///
	///
	/// `ws://host` - port 80 is used
	///
	/// `wss://host` - port 443 is used
	pub async fn build(self, url: &'a str) -> Result<WsClient, Error> {
		let max_capacity_per_subscription = self.max_notifs_per_subscription;
		let max_concurrent_requests = self.max_concurrent_requests;
		let request_timeout = self.request_timeout;
		let (to_back, from_front) = mpsc::channel(self.max_concurrent_requests);
		let (err_tx, err_rx) = oneshot::channel();

		let (sockaddrs, host, mode) = parse_url(url).map_err(|e| Error::TransportError(Box::new(e)))?;

		let builder = WsTransportClientBuilder {
			sockaddrs,
			mode,
			host,
			handshake_url: self.handshake_url,
			timeout: self.connection_timeout,
			origin: None,
			max_request_body_size: self.max_request_body_size,
		};

		let (sender, receiver) = builder.build().await.map_err(|e| Error::TransportError(Box::new(e)))?;

		async_std::task::spawn(async move {
			background_task(
				jsonrpc_transport::Sender::new(sender),
				jsonrpc_transport::Receiver::new(receiver),
				from_front,
				err_tx,
				max_capacity_per_subscription,
				max_concurrent_requests,
			)
			.await;
		});
		Ok(WsClient { to_back, request_timeout, error: Mutex::new(ErrorFromBack::Unread(err_rx)) })
	}
}

impl WsClient {
	/// Checks if the client is connected to the target.
	pub fn is_connected(&self) -> bool {
		!self.to_back.is_closed()
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
	async fn notification<'a, T>(&self, method: &'a str, params: JsonRpcParams<'a, T>) -> Result<(), Error>
	where
		T: Serialize + std::fmt::Debug + PartialEq + Send + Sync + Clone,
	{
		todo!();
		/*log::trace!("[frontend]: send notification: method={:?}, params={:?}", method, params);
		match self
			.to_back
			.clone()
			.send(FrontToBack::Notification(NotificationMessage {
				method: JsonRpcMethodOwned(method.to_string()),
				params: params.to_owned(),
			}))
			.await
		{
			Ok(()) => Ok(()),
			Err(_) => Err(self.read_error_from_backend().await),
		}*/
	}

	async fn request<'a, T, R>(&self, method: &'a str, params: JsonRpcParams<'a, T>) -> Result<R, Error>
	where
		T: Serialize + std::fmt::Debug + PartialEq + Send + Sync + Clone,
		R: DeserializeOwned,
	{
		todo!();
		/*log::trace!("[frontend]: send request: method={:?}, params={:?}", method, params);
		let (send_back_tx, send_back_rx) = oneshot::channel();

		if self
			.to_back
			.clone()
			.send(FrontToBack::StartRequest(RequestMessage {
				method: method.to_owned(),
				params: params.to_owned(),
				send_back: Some(send_back_tx),
			}))
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
		serde_json::from_value(json_value).map_err(Error::ParseError)*/
	}

	async fn batch_request<'a, T, R>(&self, batch: Vec<(&'a str, JsonRpcParams<'a, T>)>) -> Result<Vec<R>, Error>
	where
		T: Serialize + std::fmt::Debug + PartialEq + Send + Sync + Clone,
		R: DeserializeOwned + Default + Clone,
	{
		todo!();
		/*
		let (send_back_tx, send_back_rx) = oneshot::channel();
		let requests: Vec<(String, jsonrpc::Params)> = batch.into_iter().map(|(r, p)| (r.into(), p.into())).collect();
		log::trace!("[frontend]: send batch request: {:?}", requests);
		if self
			.to_back
			.clone()
			.send(FrontToBack::Batch(BatchMessage { requests, send_back: send_back_tx }))
			.await
			.is_err()
		{
			return Err(self.read_error_from_backend().await);
		}

		let json_values = match send_back_rx.await {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};

		let values: Result<_, _> =
			json_values.into_iter().map(|val| jsonrpc::from_value(val).map_err(Error::ParseError)).collect();
		Ok(values?)*/
	}
}

#[async_trait]
impl SubscriptionClient for WsClient {
	/// Send a subscription request to the server.
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server. The `unsubscribe_method` is used to close the subscription.
	async fn subscribe<'a, T, N>(
		&self,
		subscribe_method: &'a str,
		params: JsonRpcParams<'a, T>,
		unsubscribe_method: &'a str,
	) -> Result<Subscription<N>, Error>
	where
		N: DeserializeOwned,
		T: Serialize + std::fmt::Debug + PartialEq + Send + Sync + Clone,
	{
		todo!();
		/*log::trace!("[frontend]: subscribe: {:?}, unsubscribe: {:?}", subscribe_method, unsubscribe_method);
		let sub_method = subscribe_method.to_owned();
		let unsub_method = unsubscribe_method.to_owned();

		if subscribe_method == unsubscribe_method {
			return Err(Error::SubscriptionNameConflict(sub_method.0));
		}

		let (send_back_tx, send_back_rx) = oneshot::channel();
		if self
			.to_back
			.clone()
			.send(FrontToBack::Subscribe(SubscriptionMessage {
				subscribe_method: sub_method,
				unsubscribe_method: unsub_method,
				params: params.to_owned(),
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
		Ok(Subscription { to_back: self.to_back.clone(), notifs_rx, marker: PhantomData, id })*/
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
		// TODO: fix JsonValue here.
		let res = receiver.next_response::<JsonValue>().await;
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

			Either::Left((Some(FrontToBack::Batch(batch)), _)) => {
				log::trace!("[backend]: client prepares to send batch request: {:?}", batch);
				if let Err(e) = sender.start_batch_request(batch, &mut manager).await {
					log::warn!("[backend]: client batch request failed: {:?}", e);
				}
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
				if let Some(unsub) = manager
					.get_request_id_by_subscription_id(&sub_id)
					.and_then(|req_id| build_unsubscribe_message(&mut manager, req_id, sub_id))
				{
					stop_subscription(&mut sender, &mut manager, unsub).await;
				}
			}
			Either::Right((Some(Ok(JsonRpcResponse::Single(response))), _)) => {
				match process_response(&mut manager, response, max_notifs_per_subscription) {
					Ok(Some(unsub)) => {
						stop_subscription(&mut sender, &mut manager, unsub).await;
					}
					Ok(None) => (),
					Err(err) => {
						let _ = front_error.send(err);
						return;
					}
				}
			}
			Either::Right((Some(Ok(JsonRpcResponse::Batch(batch))), _)) => {
				let mut digest = Vec::with_capacity(batch.len());
				let mut ordered_responses = vec![JsonValue::Null; batch.len()];
				let mut rps_unordered: Vec<_> = Vec::with_capacity(batch.len());

				for rp in batch {
					digest.push(rp.id);
					rps_unordered.push((rp.id, rp.result));
				}

				digest.sort_unstable();
				let batch_state = match manager.complete_pending_batch(digest) {
					Some(state) => state,
					None => {
						log::warn!("Received unknown batch response");
						continue;
					}
				};

				for (id, rp) in rps_unordered {
					let pos = batch_state
						.order
						.get(&id)
						.copied()
						.expect("All request IDs valid checked by RequestManager above; qed");
					ordered_responses[pos] = rp;
				}
				manager.reclaim_request_id(batch_state.request_id);
				let _ = batch_state.send_back.send(Ok(ordered_responses));
			}
			Either::Right((Some(Ok(JsonRpcResponse::Subscription(notif))), _)) => {
				log::info!("notif: {:?}", notif);
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
							let unsub_req = build_unsubscribe_message(&mut manager, request_id, sub_id)
								.expect("request ID and subscription ID valid checked above; qed");
							stop_subscription(&mut sender, &mut manager, unsub_req).await;
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
	response: JsonRpcResponseObject<JsonValue>,
	max_capacity_per_subscription: usize,
) -> Result<Option<RequestMessage>, Error> {
	match manager.request_status(&response.id) {
		RequestStatus::PendingMethodCall => {
			let send_back_oneshot = match manager.complete_pending_call(response.id) {
				Some(Some(send)) => send,
				Some(None) => return Ok(None),
				None => return Err(Error::InvalidRequestId),
			};

			manager.reclaim_request_id(response.id);
			let _ = send_back_oneshot.send(Ok(response.result));
			Ok(None)
		}
		RequestStatus::PendingSubscription => {
			let (send_back_oneshot, unsubscribe_method) =
				manager.complete_pending_subscription(response.id).ok_or(Error::InvalidRequestId)?;

			let sub_id: SubscriptionId = match serde_json::from_value(response.result) {
				Ok(sub_id) => sub_id,
				Err(_) => {
					let _ = send_back_oneshot.send(Err(Error::InvalidSubscriptionId));
					return Ok(None);
				}
			};

			let response_id = response.id;
			let (subscribe_tx, subscribe_rx) = mpsc::channel(max_capacity_per_subscription);
			if manager.insert_subscription(response_id, sub_id.clone(), subscribe_tx, unsubscribe_method).is_ok() {
				match send_back_oneshot.send(Ok((subscribe_rx, sub_id.clone()))) {
					Ok(_) => Ok(None),
					Err(_) => Ok(build_unsubscribe_message(manager, response_id, sub_id)),
				}
			} else {
				let _ = send_back_oneshot.send(Err(Error::InvalidSubscriptionId));
				Ok(None)
			}
		}
		RequestStatus::Subscription | RequestStatus::Invalid => Err(Error::InvalidRequestId),
	}
}

/// Sends an unsubscribe to request to server to indicate
/// that the client is not interested in the subscription anymore.
async fn stop_subscription(
	sender: &mut jsonrpc_transport::Sender,
	manager: &mut RequestManager,
	unsub: RequestMessage,
) {
	if let Err(e) = sender.start_request(unsub, manager).await {
		log::error!("Send unsubscribe request failed: {:?}", e);
	}
}

/// Builds an unsubscription message, semantically the same as an ordinary request.
fn build_unsubscribe_message(
	manager: &mut RequestManager,
	req_id: u64,
	sub_id: SubscriptionId,
) -> Option<RequestMessage> {
	let (_, unsub, sub_id) = manager.remove_subscription(req_id, sub_id)?;
	manager.reclaim_request_id(req_id);
	// TODO(niklasad): better type for params or maybe a macro?!.
	let params: JsonRpcParams<_> = vec![&sub_id].into();
	todo!();
	//Some(RequestMessage { method: unsub, params, send_back: None })
}
