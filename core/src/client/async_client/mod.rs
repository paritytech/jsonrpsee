//! Abstract async client.

mod helpers;
mod manager;

use core::time::Duration;

use crate::client::{
	async_client::helpers::process_subscription_close_response, BatchMessage, ClientT, RegisterNotificationMessage,
	RequestMessage, Subscription, SubscriptionClientT, SubscriptionKind, SubscriptionMessage, TransportReceiverT,
	TransportSenderT,
};
use crate::tracing::{rx_log_from_json, tx_log_from_str, RpcTracing};

use helpers::{
	build_unsubscribe_message, call_with_timeout, process_batch_response, process_error_response, process_notification,
	process_single_response, process_subscription_response, stop_subscription,
};
use manager::RequestManager;

use crate::error::Error;
use async_lock::Mutex;
use async_trait::async_trait;
use futures_channel::{mpsc, oneshot};
use futures_timer::Delay;
use futures_util::future::{self, Either};
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use jsonrpsee_types::{
	response::SubscriptionError, ErrorResponse, Id, Notification, NotificationSer, ParamsSer, RequestSer, Response,
	SubscriptionResponse,
};
use serde::de::DeserializeOwned;
use tracing_futures::Instrument;

use super::{FrontToBack, IdKind, RequestIdManager};

/// Wrapper over a [`oneshot::Receiver`](futures_channel::oneshot::Receiver) that reads
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

/// Builder for [`Client`].
#[derive(Clone, Debug)]
pub struct ClientBuilder {
	request_timeout: Duration,
	max_concurrent_requests: usize,
	max_notifs_per_subscription: usize,
	id_kind: IdKind,
	max_log_length: u32,
}

impl Default for ClientBuilder {
	fn default() -> Self {
		Self {
			request_timeout: Duration::from_secs(60),
			max_concurrent_requests: 256,
			max_notifs_per_subscription: 1024,
			id_kind: IdKind::Number,
			max_log_length: 4096,
		}
	}
}

impl ClientBuilder {
	/// Set request timeout (default is 60 seconds).
	pub fn request_timeout(mut self, timeout: Duration) -> Self {
		self.request_timeout = timeout;
		self
	}

	/// Set max concurrent requests (default is 256).
	pub fn max_concurrent_requests(mut self, max: usize) -> Self {
		self.max_concurrent_requests = max;
		self
	}

	/// Set max concurrent notification capacity for each subscription; when the capacity is exceeded the subscription
	/// will be dropped (default is 1024).
	///
	/// You may prevent the subscription from being dropped by polling often enough
	/// [`Subscription::next()`](../../jsonrpsee_core/client/struct.Subscription.html#method.next) such that
	/// it can keep with the rate as server produces new items on the subscription.
	///
	/// **Note**: The actual capacity is `num_senders + max_subscription_capacity`
	/// because it is passed to [`futures_channel::mpsc::channel`].
	pub fn max_notifs_per_subscription(mut self, max: usize) -> Self {
		self.max_notifs_per_subscription = max;
		self
	}

	/// Configure the data type of the request object ID (default is number).
	pub fn id_format(mut self, id_kind: IdKind) -> Self {
		self.id_kind = id_kind;
		self
	}

	/// Max length for logging for requests and responses in number characters.
	///
	/// Logs bigger than this limit will be truncated.
	pub fn set_max_logging_length(mut self, max: u32) -> Self {
		self.max_log_length = max;
		self
	}

	/// Build the client with given transport.
	///
	/// ## Panics
	///
	/// Panics if called outside of `tokio` runtime context.
	#[cfg(feature = "async-client")]
	#[cfg_attr(docsrs, doc(cfg(feature = "async-client")))]
	pub fn build_with_tokio<S, R>(self, sender: S, receiver: R) -> Client
	where
		S: TransportSenderT + Send,
		R: TransportReceiverT + Send,
	{
		let (to_back, from_front) = mpsc::channel(self.max_concurrent_requests);
		let (err_tx, err_rx) = oneshot::channel();
		let max_notifs_per_subscription = self.max_notifs_per_subscription;

		tokio::spawn(async move {
			background_task(sender, receiver, from_front, err_tx, max_notifs_per_subscription).await;
		});
		Client {
			to_back,
			request_timeout: self.request_timeout,
			error: Mutex::new(ErrorFromBack::Unread(err_rx)),
			id_manager: RequestIdManager::new(self.max_concurrent_requests, self.id_kind),
			max_log_length: self.max_log_length,
		}
	}

	/// Build the client with given transport.
	#[cfg(all(feature = "async-wasm-client", target_arch = "wasm32"))]
	#[cfg_attr(docsrs, doc(cfg(feature = "async-wasm-client")))]
	pub fn build_with_wasm<S, R>(self, sender: S, receiver: R) -> Client
	where
		S: TransportSenderT,
		R: TransportReceiverT,
	{
		let (to_back, from_front) = mpsc::channel(self.max_concurrent_requests);
		let (err_tx, err_rx) = oneshot::channel();
		let max_notifs_per_subscription = self.max_notifs_per_subscription;

		wasm_bindgen_futures::spawn_local(async move {
			background_task(sender, receiver, from_front, err_tx, max_notifs_per_subscription).await;
		});
		Client {
			to_back,
			request_timeout: self.request_timeout,
			error: Mutex::new(ErrorFromBack::Unread(err_rx)),
			id_manager: RequestIdManager::new(self.max_concurrent_requests, self.id_kind),
			max_log_length: self.max_log_length,
		}
	}
}

/// Generic asynchronous client.
#[derive(Debug)]
pub struct Client {
	/// Channel to send requests to the background task.
	to_back: mpsc::Sender<FrontToBack>,
	/// If the background thread terminates the error is sent to this channel.
	// NOTE(niklasad1): This is a Mutex to circumvent that the async fns takes immutable references.
	error: Mutex<ErrorFromBack>,
	/// Request timeout. Defaults to 60sec.
	request_timeout: Duration,
	/// Request ID manager.
	id_manager: RequestIdManager,
	/// Max length for logging for requests and responses.
	///
	/// Entires bigger than this limit will be truncated.
	max_log_length: u32,
}

impl Client {
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

impl Drop for Client {
	fn drop(&mut self) {
		self.to_back.close_channel();
	}
}

#[async_trait]
impl ClientT for Client {
	async fn notification<'a>(&self, method: &'a str, params: Option<ParamsSer<'a>>) -> Result<(), Error> {
		// NOTE: we use this to guard against max number of concurrent requests.
		let _req_id = self.id_manager.next_request_id()?;
		let notif = NotificationSer::new(method, params);
		let trace = RpcTracing::batch();
		let _enter = trace.span().enter();

		let raw = serde_json::to_string(&notif).map_err(Error::ParseError)?;
		tx_log_from_str(&raw, self.max_log_length);

		let mut sender = self.to_back.clone();
		let fut = sender.send(FrontToBack::Notification(raw)).in_current_span();

		match future::select(fut, Delay::new(self.request_timeout)).await {
			Either::Left((Ok(()), _)) => Ok(()),
			Either::Left((Err(_), _)) => Err(self.read_error_from_backend().await),
			Either::Right((_, _)) => Err(Error::RequestTimeout),
		}
	}

	async fn request<'a, R>(&self, method: &'a str, params: Option<ParamsSer<'a>>) -> Result<R, Error>
	where
		R: DeserializeOwned,
	{
		let (send_back_tx, send_back_rx) = oneshot::channel();
		let guard = self.id_manager.next_request_id()?;
		let id = guard.inner();
		let trace = RpcTracing::method_call(method);
		let _enter = trace.span().enter();

		let raw = serde_json::to_string(&RequestSer::new(&id, method, params)).map_err(Error::ParseError)?;
		tx_log_from_str(&raw, self.max_log_length);

		if self
			.to_back
			.clone()
			.send(FrontToBack::Request(RequestMessage { raw, id: id.clone(), send_back: Some(send_back_tx) }))
			.await
			.is_err()
		{
			return Err(self.read_error_from_backend().await);
		}

		let res = call_with_timeout(self.request_timeout, send_back_rx).in_current_span().await;
		let json_value = match res {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};

		rx_log_from_json(&Response::new(&json_value, id), self.max_log_length);

		serde_json::from_value(json_value).map_err(Error::ParseError)
	}

	async fn batch_request<'a, R>(&self, batch: Vec<(&'a str, Option<ParamsSer<'a>>)>) -> Result<Vec<R>, Error>
	where
		R: DeserializeOwned + Default + Clone,
	{
		let guard = self.id_manager.next_request_ids(batch.len())?;
		let batch_ids: Vec<Id> = guard.inner();
		let mut batches = Vec::with_capacity(batch.len());
		let log = RpcTracing::batch();
		let _enter = log.span().enter();

		for (idx, (method, params)) in batch.into_iter().enumerate() {
			batches.push(RequestSer::new(&batch_ids[idx], method, params));
		}

		let (send_back_tx, send_back_rx) = oneshot::channel();

		let raw = serde_json::to_string(&batches).map_err(Error::ParseError)?;

		tx_log_from_str(&raw, self.max_log_length);

		if self
			.to_back
			.clone()
			.send(FrontToBack::Batch(BatchMessage { raw, ids: batch_ids, send_back: send_back_tx }))
			.await
			.is_err()
		{
			return Err(self.read_error_from_backend().await);
		}

		let res = call_with_timeout(self.request_timeout, send_back_rx).in_current_span().await;
		let json_values = match res {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};

		rx_log_from_json(&json_values, self.max_log_length);

		let values: Result<_, _> =
			json_values.into_iter().map(|val| serde_json::from_value(val).map_err(Error::ParseError)).collect();
		Ok(values?)
	}
}

#[async_trait]
impl SubscriptionClientT for Client {
	/// Send a subscription request to the server.
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server. The `unsubscribe_method` is used to close the subscription.
	async fn subscribe<'a, N>(
		&self,
		subscribe_method: &'a str,
		params: Option<ParamsSer<'a>>,
		unsubscribe_method: &'a str,
	) -> Result<Subscription<N>, Error>
	where
		N: DeserializeOwned,
	{
		if subscribe_method == unsubscribe_method {
			return Err(Error::SubscriptionNameConflict(unsubscribe_method.to_owned()));
		}

		let guard = self.id_manager.next_request_ids(2)?;
		let mut ids: Vec<Id> = guard.inner();
		let trace = RpcTracing::method_call(subscribe_method);
		let _enter = trace.span().enter();

		let id = ids[0].clone();

		let raw = serde_json::to_string(&RequestSer::new(&id, subscribe_method, params)).map_err(Error::ParseError)?;

		tx_log_from_str(&raw, self.max_log_length);

		let (send_back_tx, send_back_rx) = oneshot::channel();
		if self
			.to_back
			.clone()
			.send(FrontToBack::Subscribe(SubscriptionMessage {
				raw,
				subscribe_id: ids.swap_remove(0),
				unsubscribe_id: ids.swap_remove(0),
				unsubscribe_method: unsubscribe_method.to_owned(),
				send_back: send_back_tx,
			}))
			.await
			.is_err()
		{
			return Err(self.read_error_from_backend().await);
		}

		let res = call_with_timeout(self.request_timeout, send_back_rx).in_current_span().await;

		let (notifs_rx, sub_id) = match res {
			Ok(Ok(val)) => val,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};

		rx_log_from_json(&Response::new(&sub_id, id), self.max_log_length);

		Ok(Subscription::new(self.to_back.clone(), notifs_rx, SubscriptionKind::Subscription(sub_id)))
	}

	/// Subscribe to a specific method.
	async fn subscribe_to_method<'a, N>(&self, method: &'a str) -> Result<Subscription<N>, Error>
	where
		N: DeserializeOwned,
	{
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
async fn background_task<S, R>(
	mut sender: S,
	receiver: R,
	mut frontend: mpsc::Receiver<FrontToBack>,
	front_error: oneshot::Sender<Error>,
	max_notifs_per_subscription: usize,
) where
	S: TransportSenderT,
	R: TransportReceiverT,
{
	let mut manager = RequestManager::new();

	let backend_event = futures_util::stream::unfold(receiver, |mut receiver| async {
		let res = receiver.receive().await;
		Some((res, receiver))
	});

	futures_util::pin_mut!(backend_event);

	loop {
		let next_frontend = frontend.next();
		let next_backend = backend_event.next();
		futures_util::pin_mut!(next_frontend, next_backend);

		match futures_util::future::select(next_frontend, next_backend).await {
			// User dropped the sender side of the channel.
			// There is nothing to do just terminate.
			Either::Left((None, _)) => {
				tracing::warn!("[backend]: background task terminated");
				break;
			}

			Either::Left((Some(FrontToBack::Batch(batch)), _)) => {
				tracing::trace!("[backend]: client prepares to send batch request: {:?}", batch.raw);
				if let Err(send_back) = manager.insert_pending_batch(batch.ids.clone(), batch.send_back) {
					tracing::warn!("[backend]: batch request: {:?} already pending", batch.ids);
					let _ = send_back.send(Err(Error::InvalidRequestId));
					continue;
				}

				if let Err(e) = sender.send(batch.raw).await {
					tracing::warn!("[backend]: client batch request failed: {:?}", e);
					manager.complete_pending_batch(batch.ids);
				}
			}
			// User called `notification` on the front-end
			Either::Left((Some(FrontToBack::Notification(notif)), _)) => {
				if let Err(e) = sender.send(notif).await {
					tracing::warn!("[backend]: client notif failed: {:?}", e);
				}
			}

			// User called `request` on the front-end
			Either::Left((Some(FrontToBack::Request(request)), _)) => match sender.send(request.raw).await {
				Ok(_) => {
					manager.insert_pending_call(request.id, request.send_back).expect("ID unused checked above; qed")
				}
				Err(e) => {
					tracing::warn!("[backend]: client request failed: {:?}", e);
					let _ = request.send_back.map(|s| s.send(Err(Error::Transport(e.into()))));
				}
			},

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
					tracing::warn!("[backend]: client subscription failed: {:?}", e);
					let _ = sub.send_back.send(Err(Error::Transport(e.into())));
				}
			},
			// User dropped a subscription.
			Either::Left((Some(FrontToBack::SubscriptionClosed(sub_id)), _)) => {
				tracing::trace!("Closing subscription: {:?}", sub_id);
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
				let (subscribe_tx, subscribe_rx) = mpsc::channel(max_notifs_per_subscription);

				if manager.insert_notification_handler(&reg.method, subscribe_tx).is_ok() {
					let _ = reg.send_back.send(Ok((subscribe_rx, reg.method)));
				} else {
					let _ = reg.send_back.send(Err(Error::MethodAlreadyRegistered(reg.method)));
				}
			}

			// User dropped the notificationHandler for this method
			Either::Left((Some(FrontToBack::UnregisterNotification(method)), _)) => {
				let _ = manager.remove_notification_handler(method);
			}
			Either::Right((Some(Ok(raw)), _)) => {
				// Single response to a request.
				if let Ok(single) = serde_json::from_str::<Response<_>>(&raw) {
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
				else if let Ok(response) = serde_json::from_str::<SubscriptionResponse<_>>(&raw) {
					if let Err(Some(unsub)) = process_subscription_response(&mut manager, response) {
						let _ = stop_subscription(&mut sender, &mut manager, unsub).await;
					}
				}
				// Subscription error response.
				else if let Ok(response) = serde_json::from_str::<SubscriptionError<_>>(&raw) {
					let _ = process_subscription_close_response(&mut manager, response);
				}
				// Incoming Notification
				else if let Ok(notif) = serde_json::from_str::<Notification<_>>(&raw) {
					let _ = process_notification(&mut manager, notif);
				}
				// Batch response.
				else if let Ok(batch) = serde_json::from_str::<Vec<Response<_>>>(&raw) {
					if let Err(e) = process_batch_response(&mut manager, batch) {
						let _ = front_error.send(e);
						break;
					}
				}
				// Error response
				else if let Ok(err) = serde_json::from_str::<ErrorResponse>(&raw) {
					if let Err(e) = process_error_response(&mut manager, err) {
						let _ = front_error.send(e);
						break;
					}
				}
				// Unparsable response
				else {
					let json = serde_json::from_str::<serde_json::Value>(&raw);
					let _ = front_error.send(Error::Custom(format!("Unparsable response: {:?}", json)));
					break;
				}
			}
			Either::Right((Some(Err(e)), _)) => {
				let _ = front_error.send(Error::Transport(e.into()));
				break;
			}
			Either::Right((None, _)) => {
				let _ = front_error.send(Error::Custom("Receiver dropped".into()));
				break;
			}
		}
	}
	// Send close message to the server.
	let _ = sender.close().await;
}
