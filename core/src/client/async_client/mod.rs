//! Abstract async client.

mod helpers;
mod manager;

use crate::client::async_client::helpers::InnerBatchResponse;
use crate::client::{
	async_client::helpers::process_subscription_close_response, BatchMessage, BatchResponse, ClientT, ReceivedMessage,
	RegisterNotificationMessage, RequestMessage, Subscription, SubscriptionClientT, SubscriptionKind,
	SubscriptionMessage, TransportReceiverT, TransportSenderT,
};
use crate::error::Error;
use crate::params::BatchRequestBuilder;
use crate::tracing::{rx_log_from_json, tx_log_from_str};
use crate::traits::ToRpcParams;
use crate::JsonRawValue;
use std::borrow::Cow as StdCow;

use core::time::Duration;
use helpers::{
	build_unsubscribe_message, call_with_timeout, process_batch_response, process_error_response, process_notification,
	process_single_response, process_subscription_response, stop_subscription,
};
use jsonrpsee_types::TwoPointZero;
use manager::RequestManager;

use async_lock::Mutex;
use async_trait::async_trait;
use futures_channel::{mpsc, oneshot};
use futures_timer::Delay;
use futures_util::future::{self, Either, Fuse};
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use futures_util::FutureExt;
use jsonrpsee_types::{
	response::SubscriptionError, ErrorResponse, Notification, NotificationSer, RequestSer, Response,
	SubscriptionResponse,
};
use serde::de::DeserializeOwned;
use tracing::instrument;

use super::{generate_batch_id_range, FrontToBack, IdKind, RequestIdManager};

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
#[derive(Debug, Copy, Clone)]
pub struct ClientBuilder {
	request_timeout: Duration,
	max_concurrent_requests: usize,
	max_notifs_per_subscription: usize,
	id_kind: IdKind,
	max_log_length: u32,
	ping_interval: Option<Duration>,
}

impl Default for ClientBuilder {
	fn default() -> Self {
		Self {
			request_timeout: Duration::from_secs(60),
			max_concurrent_requests: 256,
			max_notifs_per_subscription: 1024,
			id_kind: IdKind::Number,
			max_log_length: 4096,
			ping_interval: None,
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

	/// Set maximum length for logging calls and responses.
	///
	/// Logs bigger than this limit will be truncated.
	pub fn set_max_logging_length(mut self, max: u32) -> Self {
		self.max_log_length = max;
		self
	}
	/// Set the interval at which pings frames are submitted (disabled by default).
	///
	/// Periodically submitting pings at a defined interval has mainly two benefits:
	///  - Directly, it acts as a "keep-alive" alternative in the WebSocket world.
	///  - Indirectly by inspecting debug logs, it ensures that the endpoint is still responding to messages.
	///
	/// The underlying implementation does not make any assumptions about at which intervals pongs are received.
	///
	/// Note: The interval duration is restarted when
	///  - a frontend command is submitted
	///  - a reply is received from the backend
	///  - the interval duration expires
	pub fn ping_interval(mut self, interval: Duration) -> Self {
		self.ping_interval = Some(interval);
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
		let ping_interval = self.ping_interval;
		let (on_close_tx, on_close_rx) = oneshot::channel();

		tokio::spawn(async move {
			background_task(
				sender,
				receiver,
				from_front,
				err_tx,
				max_notifs_per_subscription,
				ping_interval,
				on_close_tx,
			)
			.await;
		});
		Client {
			to_back,
			request_timeout: self.request_timeout,
			error: Mutex::new(ErrorFromBack::Unread(err_rx)),
			id_manager: RequestIdManager::new(self.max_concurrent_requests, self.id_kind),
			max_log_length: self.max_log_length,
			notify: Mutex::new(Some(on_close_rx)),
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
		let (on_close_tx, on_close_rx) = oneshot::channel();

		wasm_bindgen_futures::spawn_local(async move {
			background_task(sender, receiver, from_front, err_tx, max_notifs_per_subscription, None, on_close_tx).await;
		});
		Client {
			to_back,
			request_timeout: self.request_timeout,
			error: Mutex::new(ErrorFromBack::Unread(err_rx)),
			id_manager: RequestIdManager::new(self.max_concurrent_requests, self.id_kind),
			max_log_length: self.max_log_length,
			notify: Mutex::new(Some(on_close_rx)),
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
	/// Entries bigger than this limit will be truncated.
	max_log_length: u32,
	/// Notify when the client is disconnected or encountered an error.
	// NOTE: Similar to error, the async fns use immutable references. The `Receiver` is wrapped
	// into `Option` to ensure the `on_disconnect` awaits only once.
	notify: Mutex<Option<oneshot::Receiver<()>>>,
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

	/// Completes when the client is disconnected or the client's background task encountered an error.
	/// If the client is already disconnected, the future produced by this method will complete immediately.
	///
	/// # Cancel safety
	///
	/// This method is cancel safe.
	pub async fn on_disconnect(&self) {
		// Wait until the `background_task` exits.
		let mut notify_lock = self.notify.lock().await;
		if let Some(notify) = notify_lock.take() {
			let _ = notify.await;
		}
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		self.to_back.close_channel();
	}
}

#[async_trait]
impl ClientT for Client {
	#[instrument(name = "notification", skip(self, params), level = "trace")]
	async fn notification<Params>(&self, method: &str, params: Params) -> Result<(), Error>
	where
		Params: ToRpcParams + Send,
	{
		// NOTE: we use this to guard against max number of concurrent requests.
		let _req_id = self.id_manager.next_request_id()?;
		let params = params.to_rpc_params()?;
		let notif = NotificationSer::borrowed(&method, params.as_deref());

		let raw = serde_json::to_string(&notif).map_err(Error::ParseError)?;
		tx_log_from_str(&raw, self.max_log_length);

		let mut sender = self.to_back.clone();
		let fut = sender.send(FrontToBack::Notification(raw));

		match future::select(fut, Delay::new(self.request_timeout)).await {
			Either::Left((Ok(()), _)) => Ok(()),
			Either::Left((Err(_), _)) => Err(self.read_error_from_backend().await),
			Either::Right((_, _)) => Err(Error::RequestTimeout),
		}
	}

	#[instrument(name = "method_call", skip(self, params), level = "trace")]
	async fn request<R, Params>(&self, method: &str, params: Params) -> Result<R, Error>
	where
		R: DeserializeOwned,
		Params: ToRpcParams + Send,
	{
		let (send_back_tx, send_back_rx) = oneshot::channel();
		let guard = self.id_manager.next_request_id()?;
		let id = guard.inner();

		let params = params.to_rpc_params()?;
		let raw =
			serde_json::to_string(&RequestSer::borrowed(&id, &method, params.as_deref())).map_err(Error::ParseError)?;
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

		let json_value = match call_with_timeout(self.request_timeout, send_back_rx).await {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};

		rx_log_from_json(&Response::new(&json_value, id), self.max_log_length);

		serde_json::from_value(json_value).map_err(Error::ParseError)
	}

	async fn batch_request<'a, R>(&self, batch: BatchRequestBuilder<'a>) -> Result<BatchResponse<'a, R>, Error>
	where
		R: DeserializeOwned,
	{
		let batch = batch.build()?;
		let guard = self.id_manager.next_request_id()?;
		let id_range = generate_batch_id_range(&guard, batch.len() as u64)?;

		let mut batches = Vec::with_capacity(batch.len());
		for ((method, params), id) in batch.into_iter().zip(id_range.clone()) {
			let id = self.id_manager.as_id_kind().into_id(id);
			batches.push(RequestSer {
				jsonrpc: TwoPointZero,
				id,
				method: method.into(),
				params: params.map(StdCow::Owned),
			});
		}

		let (send_back_tx, send_back_rx) = oneshot::channel();

		let raw = serde_json::to_string(&batches).map_err(Error::ParseError)?;

		tx_log_from_str(&raw, self.max_log_length);

		if self
			.to_back
			.clone()
			.send(FrontToBack::Batch(BatchMessage { raw, ids: id_range, send_back: send_back_tx }))
			.await
			.is_err()
		{
			return Err(self.read_error_from_backend().await);
		}

		let res = call_with_timeout(self.request_timeout, send_back_rx).await;
		let json_values = match res {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};

		rx_log_from_json(&json_values, self.max_log_length);

		let mut responses = Vec::with_capacity(json_values.len());
		let mut successful_calls = 0;
		let mut failed_calls = 0;

		for json_val in json_values {
			match json_val {
				Ok(val) => {
					let result: R = serde_json::from_value(val).map_err(Error::ParseError)?;
					responses.push(Ok(result));
					successful_calls += 1;
				}
				Err(err) => {
					responses.push(Err(err));
					failed_calls += 1;
				}
			}
		}
		Ok(BatchResponse { successful_calls, failed_calls, responses })
	}
}

#[async_trait]
impl SubscriptionClientT for Client {
	/// Send a subscription request to the server.
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server. The `unsubscribe_method` is used to close the subscription.
	#[instrument(name = "subscription", fields(method = subscribe_method), skip(self, params, subscribe_method, unsubscribe_method), level = "trace")]
	async fn subscribe<'a, Notif, Params>(
		&self,
		subscribe_method: &'a str,
		params: Params,
		unsubscribe_method: &'a str,
	) -> Result<Subscription<Notif>, Error>
	where
		Params: ToRpcParams + Send,
		Notif: DeserializeOwned,
	{
		if subscribe_method == unsubscribe_method {
			return Err(Error::SubscriptionNameConflict(unsubscribe_method.to_owned()));
		}

		let guard = self.id_manager.next_request_two_ids()?;
		let (id_sub, id_unsub) = guard.inner();
		let params = params.to_rpc_params()?;

		let raw = serde_json::to_string(&RequestSer::borrowed(&id_sub, &subscribe_method, params.as_deref()))
			.map_err(Error::ParseError)?;

		tx_log_from_str(&raw, self.max_log_length);

		let (send_back_tx, send_back_rx) = oneshot::channel();
		if self
			.to_back
			.clone()
			.send(FrontToBack::Subscribe(SubscriptionMessage {
				raw,
				subscribe_id: id_sub,
				unsubscribe_id: id_unsub.clone(),
				unsubscribe_method: unsubscribe_method.to_owned(),
				send_back: send_back_tx,
			}))
			.await
			.is_err()
		{
			return Err(self.read_error_from_backend().await);
		}

		let (notifs_rx, sub_id) = match call_with_timeout(self.request_timeout, send_back_rx).await {
			Ok(Ok(val)) => val,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.read_error_from_backend().await),
		};

		rx_log_from_json(&Response::new(&sub_id, id_unsub), self.max_log_length);

		Ok(Subscription::new(self.to_back.clone(), notifs_rx, SubscriptionKind::Subscription(sub_id)))
	}

	/// Subscribe to a specific method.
	#[instrument(name = "subscribe_method", skip(self), level = "trace")]
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

/// Handle backend messages.
///
/// Returns an error if the main background loop should be terminated.
async fn handle_backend_messages<S: TransportSenderT, R: TransportReceiverT>(
	message: Option<Result<ReceivedMessage, R::Error>>,
	manager: &mut RequestManager,
	sender: &mut S,
	max_notifs_per_subscription: usize,
) -> Result<(), Error> {
	// Handle raw messages of form `ReceivedMessage::Bytes` (Vec<u8>) or ReceivedMessage::Data` (String).
	async fn handle_recv_message<S: TransportSenderT>(
		raw: &[u8],
		manager: &mut RequestManager,
		sender: &mut S,
		max_notifs_per_subscription: usize,
	) -> Result<(), Error> {
		let first_non_whitespace = raw.iter().find(|byte| !byte.is_ascii_whitespace());

		match first_non_whitespace {
			Some(b'{') => {
				// Single response to a request.
				if let Ok(single) = serde_json::from_slice::<Response<_>>(raw) {
					match process_single_response(manager, single, max_notifs_per_subscription) {
						Ok(Some(unsub)) => {
							stop_subscription(sender, manager, unsub).await;
						}
						Ok(None) => (),
						Err(err) => return Err(err),
					}
				}
				// Subscription response.
				else if let Ok(response) = serde_json::from_slice::<SubscriptionResponse<_>>(raw) {
					if let Err(Some(unsub)) = process_subscription_response(manager, response) {
						stop_subscription(sender, manager, unsub).await;
					}
				}
				// Subscription error response.
				else if let Ok(response) = serde_json::from_slice::<SubscriptionError<_>>(raw) {
					let _ = process_subscription_close_response(manager, response);
				}
				// Incoming Notification
				else if let Ok(notif) = serde_json::from_slice::<Notification<_>>(raw) {
					let _ = process_notification(manager, notif);
				}
				// Error response
				else if let Ok(err) = serde_json::from_slice::<ErrorResponse>(raw) {
					process_error_response(manager, err)?;
				} else {
					return Err(unparse_error(raw));
				}
			}
			Some(b'[') => {
				// Batch response.
				if let Ok(raw_responses) = serde_json::from_slice::<Vec<&JsonRawValue>>(raw) {
					let mut batch = Vec::with_capacity(raw_responses.len());

					let mut range = None;

					for r in raw_responses {
						let id = if let Ok(response) = serde_json::from_str::<Response<_>>(r.get()) {
							let id = response.id.try_parse_inner_as_number().ok_or(Error::InvalidRequestId)?;
							batch.push(InnerBatchResponse { id, result: Ok(response.result) });
							id
						} else if let Ok(err) = serde_json::from_str::<ErrorResponse>(r.get()) {
							let id = err.id().try_parse_inner_as_number().ok_or(Error::InvalidRequestId)?;
							batch.push(InnerBatchResponse { id, result: Err(err.error_object().clone().into_owned()) });
							id
						} else {
							return Err(unparse_error(raw));
						};

						let r = range.get_or_insert(id..id);

						if id < r.start {
							r.start = id;
						}

						if id > r.end {
							r.end = id;
						}
					}

					if let Some(mut range) = range {
						// the range is exclusive so need to add one.
						range.end += 1;
						process_batch_response(manager, batch, range)?;
					} else {
						return Err(Error::EmptyBatchRequest);
					}
				} else {
					return Err(unparse_error(raw));
				}
			}
			_ => {
				return Err(unparse_error(raw));
			}
		};

		Ok(())
	}

	match message {
		Some(Ok(ReceivedMessage::Pong)) => {
			tracing::debug!("Received pong");
		}
		Some(Ok(ReceivedMessage::Bytes(raw))) => {
			handle_recv_message(raw.as_ref(), manager, sender, max_notifs_per_subscription).await?;
		}
		Some(Ok(ReceivedMessage::Text(raw))) => {
			handle_recv_message(raw.as_ref(), manager, sender, max_notifs_per_subscription).await?;
		}
		Some(Err(e)) => {
			return Err(Error::Transport(e.into()));
		}
		None => {
			return Err(Error::Custom("TransportReceiver dropped".into()));
		}
	}

	Ok(())
}

/// Handle frontend messages.
async fn handle_frontend_messages<S: TransportSenderT>(
	message: FrontToBack,
	manager: &mut RequestManager,
	sender: &mut S,
	max_notifs_per_subscription: usize,
) {
	match message {
		FrontToBack::Batch(batch) => {
			if let Err(send_back) = manager.insert_pending_batch(batch.ids.clone(), batch.send_back) {
				tracing::warn!("[backend]: Batch request already pending: {:?}", batch.ids);
				let _ = send_back.send(Err(Error::InvalidRequestId));
				return;
			}

			if let Err(e) = sender.send(batch.raw).await {
				tracing::warn!("[backend]: Batch request failed: {:?}", e);
				manager.complete_pending_batch(batch.ids);
			}
		}
		// User called `notification` on the front-end
		FrontToBack::Notification(notif) => {
			if let Err(e) = sender.send(notif).await {
				tracing::warn!("[backend]: Notification failed: {:?}", e);
			}
		}
		// User called `request` on the front-end
		FrontToBack::Request(request) => match sender.send(request.raw).await {
			Ok(_) => manager.insert_pending_call(request.id, request.send_back).expect("ID unused checked above; qed"),
			Err(e) => {
				tracing::warn!("[backend]: Request failed: {:?}", e);
				let _ = request.send_back.map(|s| s.send(Err(Error::Transport(e.into()))));
			}
		},
		// User called `subscribe` on the front-end.
		FrontToBack::Subscribe(sub) => match sender.send(sub.raw).await {
			Ok(_) => manager
				.insert_pending_subscription(
					sub.subscribe_id,
					sub.unsubscribe_id,
					sub.send_back,
					sub.unsubscribe_method,
				)
				.expect("Request ID unused checked above; qed"),
			Err(e) => {
				tracing::warn!("[backend]: Subscription failed: {:?}", e);
				let _ = sub.send_back.send(Err(Error::Transport(e.into())));
			}
		},
		// User dropped a subscription.
		FrontToBack::SubscriptionClosed(sub_id) => {
			tracing::trace!("[backend]: Closing subscription: {:?}", sub_id);
			// NOTE: The subscription may have been closed earlier if
			// the channel was full or disconnected.
			if let Some(unsub) = manager
				.get_request_id_by_subscription_id(&sub_id)
				.and_then(|req_id| build_unsubscribe_message(manager, req_id, sub_id))
			{
				stop_subscription(sender, manager, unsub).await;
			}
		}
		// User called `register_notification` on the front-end.
		FrontToBack::RegisterNotification(reg) => {
			let (subscribe_tx, subscribe_rx) = mpsc::channel(max_notifs_per_subscription);

			if manager.insert_notification_handler(&reg.method, subscribe_tx).is_ok() {
				let _ = reg.send_back.send(Ok((subscribe_rx, reg.method)));
			} else {
				let _ = reg.send_back.send(Err(Error::MethodAlreadyRegistered(reg.method)));
			}
		}
		// User dropped the NotificationHandler for this method
		FrontToBack::UnregisterNotification(method) => {
			let _ = manager.remove_notification_handler(method);
		}
	}
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task<S, R>(
	mut sender: S,
	receiver: R,
	mut frontend: mpsc::Receiver<FrontToBack>,
	front_error: oneshot::Sender<Error>,
	max_notifs_per_subscription: usize,
	ping_interval: Option<Duration>,
	on_close: oneshot::Sender<()>,
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

	// Place frontend and backend messages into their own select.
	// This implies that either messages are received (both front or backend),
	// or the submitted ping timer expires (if provided).
	let next_frontend = frontend.next();
	let next_backend = backend_event.next();
	let mut message_fut = future::select(next_frontend, next_backend);

	loop {
		// Create either a valid delay fuse triggered every provided `duration`,
		// or create a terminated fuse that's never selected if the provided `duration` is None.
		let submit_ping = if let Some(duration) = ping_interval {
			Delay::new(duration).fuse()
		} else {
			// The select macro bypasses terminated futures, and the `submit_ping` branch is never selected.
			Fuse::<Delay>::terminated()
		};

		match future::select(message_fut, submit_ping).await {
			// Message received from the frontend.
			Either::Left((Either::Left((frontend_value, backend)), _)) => {
				let frontend_value = if let Some(value) = frontend_value {
					value
				} else {
					// User dropped the sender side of the channel.
					// There is nothing to do just terminate.
					tracing::debug!("[backend]: Client dropped");
					break;
				};

				handle_frontend_messages(frontend_value, &mut manager, &mut sender, max_notifs_per_subscription).await;

				// Advance frontend, save backend.
				message_fut = future::select(frontend.next(), backend);
			}
			// Message received from the backend.
			Either::Left((Either::Right((backend_value, frontend)), _)) => {
				if let Err(err) = handle_backend_messages::<S, R>(
					backend_value,
					&mut manager,
					&mut sender,
					max_notifs_per_subscription,
				)
				.await
				{
					tracing::error!("[backend]: {}", err);
					let _ = front_error.send(err);
					break;
				}
				// Advance backend, save frontend.
				message_fut = future::select(frontend, backend_event.next());
			}
			// Submit ping interval was triggered if enabled.
			Either::Right((_, next_message_fut)) => {
				if let Err(err) = sender.send_ping().await {
					tracing::error!("[backend]: Could not send ping frame: {}", err);
					let _ = front_error.send(Error::Custom("Could not send ping frame".into()));
					break;
				}
				message_fut = next_message_fut;
			}
		};
	}

	// Wake the `on_disconnect` method.
	let _ = on_close.send(());
	// Send close message to the server.
	let _ = sender.close().await;
}

fn unparse_error(raw: &[u8]) -> Error {
	let json = serde_json::from_slice::<serde_json::Value>(raw);

	let json_str = match json {
		Ok(json) => serde_json::to_string(&json).expect("valid JSON; qed"),
		Err(e) => e.to_string(),
	};

	Error::Custom(format!("Unparseable message: {}", json_str))
}
