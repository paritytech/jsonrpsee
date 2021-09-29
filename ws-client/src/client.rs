// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

use crate::transport::{Receiver as WsReceiver, Sender as WsSender, Target, WsTransportClientBuilder};
use crate::types::{
	RequestIdGuard,
	traits::{Client, SubscriptionClient},
	v2::{Id, Notification, NotificationSer, ParamsSer, RequestSer, Response, RpcError, SubscriptionResponse},
	BatchMessage, Error, FrontToBack, RegisterNotificationMessage, RequestMessage, Subscription, SubscriptionKind,
	SubscriptionMessage, TEN_MB_SIZE_BYTES,
};
use crate::{
	helpers::{
		build_unsubscribe_message, call_with_timeout, process_batch_response, process_error_response,
		process_notification, process_single_response, process_subscription_response, stop_subscription,
	},
	manager::RequestManager,
	transport::CertificateStore,
};
use async_trait::async_trait;
use futures::{
	channel::{mpsc, oneshot},
	future::Either,
	prelude::*,
	sink::SinkExt,
};
use tokio::sync::Mutex;

use serde::de::DeserializeOwned;
use std::{
	borrow::Cow,
	time::Duration,
};

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
	/// Request timeout. Defaults to 60sec.
	request_timeout: Duration,
	/// Request ID manager.
	id_guard: RequestIdGuard,
}

/// Configuration.
#[derive(Clone, Debug)]
pub struct WsClientBuilder<'a> {
	certificate_store: CertificateStore,
	max_request_body_size: u32,
	request_timeout: Duration,
	connection_timeout: Duration,
	origin_header: Option<Cow<'a, str>>,
	max_concurrent_requests: usize,
	max_notifs_per_subscription: usize,
}

impl<'a> Default for WsClientBuilder<'a> {
	fn default() -> Self {
		Self {
			certificate_store: CertificateStore::Native,
			max_request_body_size: TEN_MB_SIZE_BYTES,
			request_timeout: Duration::from_secs(60),
			connection_timeout: Duration::from_secs(10),
			origin_header: None,
			max_concurrent_requests: 256,
			max_notifs_per_subscription: 1024,
		}
	}
}

impl<'a> WsClientBuilder<'a> {
	/// Set whether to use system certificates
	pub fn certificate_store(mut self, certificate_store: CertificateStore) -> Self {
		self.certificate_store = certificate_store;
		self
	}

	/// Set max request body size.
	pub fn max_request_body_size(mut self, size: u32) -> Self {
		self.max_request_body_size = size;
		self
	}

	/// Set request timeout (default is 60 seconds).
	pub fn request_timeout(mut self, timeout: Duration) -> Self {
		self.request_timeout = timeout;
		self
	}

	/// Set connection timeout for the handshake.
	pub fn connection_timeout(mut self, timeout: Duration) -> Self {
		self.connection_timeout = timeout;
		self
	}

	/// Set origin header to pass during the handshake.
	pub fn origin_header(mut self, origin: &'a str) -> Self {
		self.origin_header = Some(Cow::Borrowed(origin));
		self
	}

	/// Set max concurrent requests.
	pub fn max_concurrent_requests(mut self, max: usize) -> Self {
		self.max_concurrent_requests = max;
		self
	}

	/// Set max concurrent notification capacity for each subscription; when the capacity is exceeded the subscription
	/// will be dropped.
	///
	/// You can also prevent the subscription being dropped by calling
	/// [`Subscription::next()`](crate::types::Subscription) frequently enough such that the buffer capacity doesn't
	/// exceeds.
	///
	/// **Note**: The actual capacity is `num_senders + max_subscription_capacity`
	/// because it is passed to [`futures::channel::mpsc::channel`].
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
	///
	/// ## Panics
	///
	/// Panics if being called outside of `tokio` runtime context.
	pub async fn build(self, url: &'a str) -> Result<WsClient, Error> {
		let certificate_store = self.certificate_store;
		let max_capacity_per_subscription = self.max_notifs_per_subscription;
		let max_concurrent_requests = self.max_concurrent_requests;
		let request_timeout = self.request_timeout;
		let (to_back, from_front) = mpsc::channel(self.max_concurrent_requests);
		let (err_tx, err_rx) = oneshot::channel();

		let builder = WsTransportClientBuilder {
			certificate_store,
			target: Target::parse(url).map_err(|e| Error::Transport(e.into()))?,
			timeout: self.connection_timeout,
			origin_header: self.origin_header,
			max_request_body_size: self.max_request_body_size,
		};

		let (sender, receiver) = builder.build().await.map_err(|e| Error::Transport(e.into()))?;

		tokio::spawn(async move {
			background_task(sender, receiver, from_front, err_tx, max_capacity_per_subscription).await;
		});
		Ok(WsClient {
			to_back,
			request_timeout,
			error: Mutex::new(ErrorFromBack::Unread(err_rx)),
			id_guard: RequestIdGuard::new(max_concurrent_requests),
		})
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

impl Drop for WsClient {
	fn drop(&mut self) {
		self.to_back.close_channel();
	}
}

#[async_trait]
impl Client for WsClient {
	async fn notification<'a>(&self, method: &'a str, params: ParamsSer<'a>) -> Result<(), Error> {
		// NOTE: we use this to guard against max number of concurrent requests.
		let _req_id = self.id_guard.next_request_id()?;
		let notif = NotificationSer::new(method, params);
		let raw = serde_json::to_string(&notif).map_err(|e| {
			self.id_guard.reclaim_request_id();
			Error::ParseError(e)
		})?;
		log::trace!("[frontend]: send notification: {:?}", raw);

		let mut sender = self.to_back.clone();
		let fut = sender.send(FrontToBack::Notification(raw));

		let timeout = tokio::time::sleep(self.request_timeout);

		let res = tokio::select! {
			x = fut => x,
			_ = timeout => return Err(Error::RequestTimeout)
		};

		self.id_guard.reclaim_request_id();
		match res {
			Ok(()) => Ok(()),
			Err(_) => Err(self.read_error_from_backend().await),
		}
	}

	async fn request<'a, R>(&self, method: &'a str, params: ParamsSer<'a>) -> Result<R, Error>
	where
		R: DeserializeOwned,
	{
		let (send_back_tx, send_back_rx) = oneshot::channel();
		let req_id = self.id_guard.next_request_id()?;
		let raw = serde_json::to_string(&RequestSer::new(Id::Number(req_id), method, params)).map_err(|e| {
			self.id_guard.reclaim_request_id();
			Error::ParseError(e)
		})?;
		log::trace!("[frontend]: send request: {:?}", raw);

		if self
			.to_back
			.clone()
			.send(FrontToBack::Request(RequestMessage { raw, id: req_id, send_back: Some(send_back_tx) }))
			.await
			.is_err()
		{
			self.id_guard.reclaim_request_id();
			return Err(self.read_error_from_backend().await);
		}

		let res = call_with_timeout(self.request_timeout, send_back_rx).await;

		self.id_guard.reclaim_request_id();
		let json_value = match res {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};
		serde_json::from_value(json_value).map_err(Error::ParseError)
	}

	async fn batch_request<'a, R>(&self, batch: Vec<(&'a str, ParamsSer<'a>)>) -> Result<Vec<R>, Error>
	where
		R: DeserializeOwned + Default + Clone,
	{
		let batch_ids = self.id_guard.next_request_ids(batch.len())?;
		let mut batches = Vec::with_capacity(batch.len());

		for (idx, (method, params)) in batch.into_iter().enumerate() {
			batches.push(RequestSer::new(Id::Number(batch_ids[idx]), method, params));
		}

		let (send_back_tx, send_back_rx) = oneshot::channel();

		let raw = serde_json::to_string(&batches).map_err(|e| {
			self.id_guard.reclaim_request_id();
			Error::ParseError(e)
		})?;
		log::trace!("[frontend]: send batch request: {:?}", raw);
		if self
			.to_back
			.clone()
			.send(FrontToBack::Batch(BatchMessage { raw, ids: batch_ids, send_back: send_back_tx }))
			.await
			.is_err()
		{
			self.id_guard.reclaim_request_id();
			return Err(self.read_error_from_backend().await);
		}

		let res = call_with_timeout(self.request_timeout, send_back_rx).await;

		self.id_guard.reclaim_request_id();
		let json_values = match res {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};

		let values: Result<_, _> =
			json_values.into_iter().map(|val| serde_json::from_value(val).map_err(Error::ParseError)).collect();
		Ok(values?)
	}
}

#[async_trait]
impl SubscriptionClient for WsClient {
	/// Send a subscription request to the server.
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server. The `unsubscribe_method` is used to close the subscription.
	async fn subscribe<'a, N>(
		&self,
		subscribe_method: &'a str,
		params: ParamsSer<'a>,
		unsubscribe_method: &'a str,
	) -> Result<Subscription<N>, Error>
	where
		N: DeserializeOwned,
	{
		log::trace!("[frontend]: subscribe: {:?}, unsubscribe: {:?}", subscribe_method, unsubscribe_method);

		if subscribe_method == unsubscribe_method {
			return Err(Error::SubscriptionNameConflict(unsubscribe_method.to_owned()));
		}

		let ids = self.id_guard.next_request_ids(2)?;
		let raw =
			serde_json::to_string(&RequestSer::new(Id::Number(ids[0]), subscribe_method, params)).map_err(|e| {
				self.id_guard.reclaim_request_id();
				Error::ParseError(e)
			})?;

		let (send_back_tx, send_back_rx) = oneshot::channel();
		if self
			.to_back
			.clone()
			.send(FrontToBack::Subscribe(SubscriptionMessage {
				raw,
				subscribe_id: ids[0],
				unsubscribe_id: ids[1],
				unsubscribe_method: unsubscribe_method.to_owned(),
				send_back: send_back_tx,
			}))
			.await
			.is_err()
		{
			self.id_guard.reclaim_request_id();
			return Err(self.read_error_from_backend().await);
		}

		let res = call_with_timeout(self.request_timeout, send_back_rx).await;

		self.id_guard.reclaim_request_id();
		let (notifs_rx, id) = match res {
			Ok(Ok(val)) => val,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};
		Ok(Subscription::new(self.to_back.clone(), notifs_rx, SubscriptionKind::Subscription(id)))
	}

	/// Subscribe to a specific method.
	async fn subscribe_to_method<'a, N>(&self, method: &'a str) -> Result<Subscription<N>, Error>
	where
		N: DeserializeOwned,
	{
		log::trace!("[frontend]: register_notification: {:?}", method);

		let (send_back_tx, send_back_rx) = oneshot::channel();
		if self
			.to_back
			.clone()
			.send(FrontToBack::RegisterNotification(RegisterNotificationMessage {
				send_back: send_back_tx,
				method: method.to_owned(),
			}))
			.await
			.is_err()
		{
			return Err(self.read_error_from_backend().await);
		}

		let res = call_with_timeout(self.request_timeout, send_back_rx).await;

		let (notifs_rx, method) = match res {
			Ok(Ok(val)) => val,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};

		Ok(Subscription::new(self.to_back.clone(), notifs_rx, SubscriptionKind::Method(method)))
	}
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task(
	mut sender: WsSender,
	receiver: WsReceiver,
	mut frontend: mpsc::Receiver<FrontToBack>,
	front_error: oneshot::Sender<Error>,
	max_notifs_per_subscription: usize,
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

		match future::select(next_frontend, next_backend).await {
			// User dropped the sender side of the channel.
			// There is nothing to do just terminate.
			Either::Left((None, _)) => {
				log::trace!("[backend]: frontend dropped; terminate client");
				break;
			}

			Either::Left((Some(FrontToBack::Batch(batch)), _)) => {
				log::trace!("[backend]: client prepares to send batch request: {:?}", batch.raw);
				// NOTE(niklasad1): annoying allocation.
				if let Err(send_back) = manager.insert_pending_batch(batch.ids.clone(), batch.send_back) {
					log::warn!("[backend]: batch request: {:?} already pending", batch.ids);
					let _ = send_back.send(Err(Error::InvalidRequestId));
					continue;
				}

				if let Err(e) = sender.send(batch.raw).await {
					log::warn!("[backend]: client batch request failed: {:?}", e);
					manager.complete_pending_batch(batch.ids);
				}
			}
			// User called `notification` on the front-end
			Either::Left((Some(FrontToBack::Notification(notif)), _)) => {
				log::trace!("[backend]: client prepares to send notification: {:?}", notif);
				if let Err(e) = sender.send(notif).await {
					log::warn!("[backend]: client notif failed: {:?}", e);
				}
			}

			// User called `request` on the front-end
			Either::Left((Some(FrontToBack::Request(request)), _)) => {
				log::trace!("[backend]: client prepares to send request={:?}", request);
				match sender.send(request.raw).await {
					Ok(_) => manager
						.insert_pending_call(request.id, request.send_back)
						.expect("ID unused checked above; qed"),
					Err(e) => {
						log::warn!("[backend]: client request failed: {:?}", e);
						let _ = request.send_back.map(|s| s.send(Err(Error::Transport(e.into()))));
					}
				}
			}

			// User called `subscribe` on the front-end.
			Either::Left((Some(FrontToBack::Subscribe(sub)), _)) => match sender.send(sub.raw).await {
				Ok(_) => manager
					.insert_pending_subscription(
						sub.subscribe_id,
						sub.unsubscribe_id,
						sub.send_back,
						sub.unsubscribe_method,
					)
					.expect("Request ID unused checked above; qed"),
				Err(e) => {
					log::warn!("[backend]: client subscription failed: {:?}", e);
					let _ = sub.send_back.send(Err(Error::Transport(e.into())));
				}
			},
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

			// User called `register_notification` on the front-end.
			Either::Left((Some(FrontToBack::RegisterNotification(reg)), _)) => {
				log::trace!("[backend] registering notification handler: {:?}", reg.method);
				let (subscribe_tx, subscribe_rx) = mpsc::channel(max_notifs_per_subscription);

				if manager.insert_notification_handler(&reg.method, subscribe_tx).is_ok() {
					let _ = reg.send_back.send(Ok((subscribe_rx, reg.method)));
				} else {
					let _ = reg.send_back.send(Err(Error::MethodAlreadyRegistered(reg.method)));
				}
			}

			// User dropped the notificationHandler for this method
			Either::Left((Some(FrontToBack::UnregisterNotification(method)), _)) => {
				log::trace!("[backend] unregistering notification handler: {:?}", method);
				let _ = manager.remove_notification_handler(method);
			}
			Either::Right((Some(Ok(raw)), _)) => {
				// Single response to a request.
				if let Ok(single) = serde_json::from_slice::<Response<_>>(&raw) {
					log::debug!("[backend]: recv method_call {:?}", single);
					match process_single_response(&mut manager, single, max_notifs_per_subscription) {
						Ok(Some(unsub)) => {
							stop_subscription(&mut sender, &mut manager, unsub).await;
						}
						Ok(None) => (),
						Err(err) => {
							let _ = front_error.send(err);
							break;
						}
					}
				}
				// Subscription response.
				else if let Ok(response) = serde_json::from_slice::<SubscriptionResponse<_>>(&raw) {
					log::debug!("[backend]: recv subscription {:?}", response);
					if let Err(Some(unsub)) = process_subscription_response(&mut manager, response) {
						let _ = stop_subscription(&mut sender, &mut manager, unsub).await;
					}
				}
				// Incoming Notification
				else if let Ok(notif) = serde_json::from_slice::<Notification<_>>(&raw) {
					log::debug!("[backend]: recv notification {:?}", notif);
					let _ = process_notification(&mut manager, notif);
				}
				// Batch response.
				else if let Ok(batch) = serde_json::from_slice::<Vec<Response<_>>>(&raw) {
					log::debug!("[backend]: recv batch {:?}", batch);
					if let Err(e) = process_batch_response(&mut manager, batch) {
						let _ = front_error.send(e);
						break;
					}
				}
				// Error response
				else if let Ok(err) = serde_json::from_slice::<RpcError>(&raw) {
					log::debug!("[backend]: recv error response {:?}", err);
					if let Err(e) = process_error_response(&mut manager, err) {
						let _ = front_error.send(e);
						break;
					}
				}
				// Unparsable response
				else {
					log::debug!(
						"[backend]: recv unparseable message: {:?}",
						serde_json::from_slice::<serde_json::Value>(&raw)
					);
					let _ = front_error.send(Error::Custom("Unparsable response".into()));
					break;
				}
			}
			Either::Right((Some(Err(e)), _)) => {
				log::error!("Error: {:?} terminating client", e);
				let _ = front_error.send(Error::Transport(e.into()));
				break;
			}
			Either::Right((None, _)) => {
				log::error!("[backend]: WebSocket receiver dropped; terminate client");
				let _ = front_error.send(Error::Custom("WebSocket receiver dropped".into()));
				break;
			}
		}
	}

	// Send close message to the server.
	let _ = sender.close().await;
}
