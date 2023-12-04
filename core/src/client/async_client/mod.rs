//! Abstract async client.

mod helpers;
mod manager;

use crate::client::async_client::helpers::{process_subscription_close_response, InnerBatchResponse};
use crate::client::{
	BatchMessage, BatchResponse, ClientT, ReceivedMessage, RegisterNotificationMessage, RequestMessage,
	Subscription, SubscriptionClientT, SubscriptionKind, SubscriptionMessage, TransportReceiverT, TransportSenderT, Error
};
use crate::error::RegisterMethodError;
use crate::params::{BatchRequestBuilder, EmptyBatchRequest};
use crate::tracing::client::{rx_log_from_json, tx_log_from_str};
use crate::traits::ToRpcParams;
use crate::JsonRawValue;
use std::borrow::Cow as StdCow;

use core::time::Duration;
use helpers::{
	build_unsubscribe_message, call_with_timeout, process_batch_response, process_notification,
	process_single_response, process_subscription_response, stop_subscription,
};
use jsonrpsee_types::{InvalidRequestId, ResponseSuccess, TwoPointZero};
use manager::RequestManager;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

use async_lock::RwLock as AsyncRwLock;
use async_trait::async_trait;
use futures_timer::Delay;
use futures_util::future::{self, Either};
use futures_util::stream::{FuturesUnordered, StreamExt};
use futures_util::{Future, Stream};
use jsonrpsee_types::response::{ResponsePayload, SubscriptionError};
use jsonrpsee_types::{Notification, NotificationSer, RequestSer, Response, SubscriptionResponse};
use serde::de::DeserializeOwned;
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

use super::{generate_batch_id_range, FrontToBack, IdKind, RequestIdManager};

const LOG_TARGET: &str = "jsonrpsee-client";

#[derive(Debug, Default, Clone)]
pub(crate) struct ThreadSafeRequestManager(Arc<std::sync::Mutex<RequestManager>>);

impl ThreadSafeRequestManager {
	pub(crate) fn new() -> Self {
		Self::default()
	}

	pub(crate) fn lock(&self) -> std::sync::MutexGuard<RequestManager> {
		self.0.lock().expect("Not poisoned; qed")
	}
}
/// If the background thread is terminated, this type
/// can be used to read the error cause.
///
// NOTE: This is an AsyncRwLock to be &self.
#[derive(Debug)]
struct ErrorFromBack(AsyncRwLock<Option<ReadErrorOnce>>);

impl ErrorFromBack {
	fn new(unread: oneshot::Receiver<Error>) -> Self {
		Self(AsyncRwLock::new(Some(ReadErrorOnce::Unread(unread))))
	}

	async fn read_error(&self) -> Error {
		const PROOF: &str = "Option is only is used to workaround ownership issue and is always Some; qed";

		if let ReadErrorOnce::Read(ref err) = self.0.read().await.as_ref().expect(PROOF) {
			return Error::RestartNeeded(err.clone());
		};

		let mut write = self.0.write().await;
		let state = write.take();

		let err = match state.expect(PROOF) {
			ReadErrorOnce::Unread(rx) => {
				let arc_err = Arc::new(match rx.await {
					Ok(err) => err,
					// This should never happen because the receiving end is still alive.
					// Before shutting down the background task a error message should
					// be emitted.
					Err(_) => Error::Custom("Error reason could not be found. This is a bug. Please open an issue.".to_string()),
				});
				*write = Some(ReadErrorOnce::Read(arc_err.clone()));
				arc_err
			}
			ReadErrorOnce::Read(arc_err) => {
				*write = Some(ReadErrorOnce::Read(arc_err.clone()));
				arc_err
			}
		};

		Error::RestartNeeded(err)
	}
}

/// Wrapper over a [`oneshot::Receiver`] that reads
/// the underlying channel once and then stores the result in String.
/// It is possible that the error is read more than once if several calls are made
/// when the background thread has been terminated.
#[derive(Debug)]
enum ReadErrorOnce {
	/// Error message is already read.
	Read(Arc<Error>),
	/// Error message is unread.
	Unread(oneshot::Receiver<Error>),
}

/// Builder for [`Client`].
#[derive(Debug, Copy, Clone)]
pub struct ClientBuilder {
	request_timeout: Duration,
	max_concurrent_requests: usize,
	max_buffer_capacity_per_subscription: usize,
	id_kind: IdKind,
	max_log_length: u32,
	ping_interval: Option<Duration>,
}

impl Default for ClientBuilder {
	fn default() -> Self {
		Self {
			request_timeout: Duration::from_secs(60),
			max_concurrent_requests: 256,
			max_buffer_capacity_per_subscription: 1024,
			id_kind: IdKind::Number,
			max_log_length: 4096,
			ping_interval: None,
		}
	}
}

impl ClientBuilder {
	/// Create a builder for the client.
	pub fn new() -> ClientBuilder {
		ClientBuilder::default()
	}

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

	/// Set max buffer capacity for each subscription; when the capacity is exceeded the subscription
	/// will be dropped (default is 1024).
	///
	/// You may prevent the subscription from being dropped by polling often enough
	/// [`Subscription::next()`](../../jsonrpsee_core/client/struct.Subscription.html#method.next) such that
	/// it can keep with the rate as server produces new items on the subscription.
	///
	///
	/// # Panics
	///
	/// This function panics if `max` is 0.
	pub fn max_buffer_capacity_per_subscription(mut self, max: usize) -> Self {
		self.max_buffer_capacity_per_subscription = max;
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
		let (err_to_front, err_from_back) = oneshot::channel::<Error>();
		let max_buffer_capacity_per_subscription = self.max_buffer_capacity_per_subscription;
		let ping_interval = self.ping_interval;
		let (client_dropped_tx, client_dropped_rx) = oneshot::channel();
		let (send_receive_task_sync_tx, send_receive_task_sync_rx) = mpsc::channel(1);
		let manager = ThreadSafeRequestManager::new();

		tokio::spawn(send_task(SendTaskParams {
			sender,
			from_frontend: from_front,
			close_tx: send_receive_task_sync_tx.clone(),
			manager: manager.clone(),
			max_buffer_capacity_per_subscription,
			ping_interval,
		}));

		tokio::spawn(read_task(ReadTaskParams {
			receiver,
			close_tx: send_receive_task_sync_tx,
			to_send_task: to_back.clone(),
			manager,
			max_buffer_capacity_per_subscription: self.max_buffer_capacity_per_subscription,
		}));

		tokio::spawn(wait_for_shutdown(send_receive_task_sync_rx, client_dropped_rx, err_to_front));

		Client {
			to_back,
			request_timeout: self.request_timeout,
			error: ErrorFromBack::new(err_from_back),
			id_manager: RequestIdManager::new(self.max_concurrent_requests, self.id_kind),
			max_log_length: self.max_log_length,
			on_exit: Some(client_dropped_tx),
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
		let (err_to_front, err_from_back) = oneshot::channel::<Error>();
		let max_buffer_capacity_per_subscription = self.max_buffer_capacity_per_subscription;
		let ping_interval = self.ping_interval;
		let (client_dropped_tx, client_dropped_rx) = oneshot::channel();
		let (send_receive_task_sync_tx, send_receive_task_sync_rx) = mpsc::channel(1);
		let manager = ThreadSafeRequestManager::new();

		wasm_bindgen_futures::spawn_local(send_task(SendTaskParams {
			sender,
			from_frontend: from_front,
			close_tx: send_receive_task_sync_tx.clone(),
			manager: manager.clone(),
			max_buffer_capacity_per_subscription,
			ping_interval,
		}));

		wasm_bindgen_futures::spawn_local(read_task(ReadTaskParams {
			receiver,
			close_tx: send_receive_task_sync_tx,
			to_send_task: to_back.clone(),
			manager,
			max_buffer_capacity_per_subscription: self.max_buffer_capacity_per_subscription,
		}));

		wasm_bindgen_futures::spawn_local(wait_for_shutdown(
			send_receive_task_sync_rx,
			client_dropped_rx,
			err_to_front,
		));

		Client {
			to_back,
			request_timeout: self.request_timeout,
			error: ErrorFromBack::new(err_from_back),
			id_manager: RequestIdManager::new(self.max_concurrent_requests, self.id_kind),
			max_log_length: self.max_log_length,
			on_exit: Some(client_dropped_tx),
		}
	}
}

/// Generic asynchronous client.
#[derive(Debug)]
pub struct Client {
	/// Channel to send requests to the background task.
	to_back: mpsc::Sender<FrontToBack>,
	error: ErrorFromBack,
	/// Request timeout. Defaults to 60sec.
	request_timeout: Duration,
	/// Request ID manager.
	id_manager: RequestIdManager,
	/// Max length for logging for requests and responses.
	///
	/// Entries bigger than this limit will be truncated.
	max_log_length: u32,
	/// When the client is dropped a message is sent to the background thread.
	on_exit: Option<oneshot::Sender<()>>,
}

impl Client {
	/// Create a builder for the server.
	pub fn builder() -> ClientBuilder {
		ClientBuilder::new()
	}

	/// Checks if the client is connected to the target.
	pub fn is_connected(&self) -> bool {
		!self.to_back.is_closed()
	}

	/// This is similar to [`Client::on_disconnect`] but it can be used to get
	/// the reason why the client was disconnected but it's not cancel-safe.
	/// 
	/// The typical use-case is that this method will be called after
	/// [`Client::on_disconnect`] has returned in a "select loop".
	/// 
	/// # Cancel-safety
	/// 
	/// This method is not cancel-safe
	pub async fn disconnect_reason(&self) -> Error {
		self.error.read_error().await
	}

	/// Completes when the client is disconnected or the client's background task encountered an error.
	/// If the client is already disconnected, the future produced by this method will complete immediately.
	///
	/// # Cancel safety
	///
	/// This method is cancel safe.
	pub async fn on_disconnect(&self) {
		self.to_back.closed().await;
	}
}

impl Drop for Client {
	fn drop(&mut self) {
		if let Some(e) = self.on_exit.take() {
			let _ = e.send(());
		}
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

		let sender = self.to_back.clone();
		let fut = sender.send(FrontToBack::Notification(raw));

		tokio::pin!(fut);

		match future::select(fut, Delay::new(self.request_timeout)).await {
			Either::Left((Ok(()), _)) => Ok(()),
			Either::Left((Err(_), _)) => Err(self.disconnect_reason().await),
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
			return Err(self.disconnect_reason().await);
		}

		let json_value = match call_with_timeout(self.request_timeout, send_back_rx).await {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.disconnect_reason().await),
		};

		rx_log_from_json(&Response::new(ResponsePayload::result_borrowed(&json_value), id), self.max_log_length);

		serde_json::from_value(json_value).map_err(Error::ParseError)
	}

	#[instrument(name = "batch", skip(self, batch), level = "trace")]
	async fn batch_request<'a, R>(&self, batch: BatchRequestBuilder<'a>) -> Result<BatchResponse<'a, R>, Error>
	where
		R: DeserializeOwned,
	{
		// TODO: remove unwrap
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
			return Err(self.disconnect_reason().await);
		}

		let res = call_with_timeout(self.request_timeout, send_back_rx).await;
		let json_values = match res {
			Ok(Ok(v)) => v,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.disconnect_reason().await),
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
			return Err(Error::RegisterMethod(RegisterMethodError::SubscriptionNameConflict(
				unsubscribe_method.to_owned(),
			)));
		}

		let guard = self.id_manager.next_request_two_ids()?;
		let (id_sub, id_unsub) = guard.inner();
		let params = params.to_rpc_params()?;

		let raw = serde_json::to_string(&RequestSer::borrowed(&id_sub, &subscribe_method, params.as_deref()))
			.map_err(Error::ParseError)?;

		tx_log_from_str(&raw, self.max_log_length);

		let (send_back_tx, send_back_rx) = tokio::sync::oneshot::channel();
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
			return Err(self.disconnect_reason().await);
		}

		let (notifs_rx, sub_id) = match call_with_timeout(self.request_timeout, send_back_rx).await {
			Ok(Ok(val)) => val,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.disconnect_reason().await),
		};

		rx_log_from_json(&Response::new(ResponsePayload::result_borrowed(&sub_id), id_unsub), self.max_log_length);

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
			return Err(self.disconnect_reason().await);
		}

		let res = call_with_timeout(self.request_timeout, send_back_rx).await;

		let (notifs_rx, method) = match res {
			Ok(Ok(val)) => val,
			Ok(Err(err)) => return Err(err),
			Err(_) => return Err(self.disconnect_reason().await),
		};

		Ok(Subscription::new(self.to_back.clone(), notifs_rx, SubscriptionKind::Method(method)))
	}
}

/// Handle backend messages.
///
/// Returns an error if the main background loop should be terminated.
fn handle_backend_messages<R: TransportReceiverT>(
	message: Option<Result<ReceivedMessage, R::Error>>,
	manager: &ThreadSafeRequestManager,
	max_buffer_capacity_per_subscription: usize,
) -> Result<Option<FrontToBack>, Error> {
	// Handle raw messages of form `ReceivedMessage::Bytes` (Vec<u8>) or ReceivedMessage::Data` (String).
	fn handle_recv_message(
		raw: &[u8],
		manager: &ThreadSafeRequestManager,
		max_buffer_capacity_per_subscription: usize,
	) -> Result<Option<FrontToBack>, Error> {
		let first_non_whitespace = raw.iter().find(|byte| !byte.is_ascii_whitespace());

		match first_non_whitespace {
			Some(b'{') => {
				// Single response to a request.
				if let Ok(single) = serde_json::from_slice::<Response<_>>(raw) {
					let maybe_unsub =
						process_single_response(&mut manager.lock(), single, max_buffer_capacity_per_subscription)?;

					if let Some(unsub) = maybe_unsub {
						return Ok(Some(FrontToBack::Request(unsub)));
					}
				}
				// Subscription response.
				else if let Ok(response) = serde_json::from_slice::<SubscriptionResponse<_>>(raw) {
					if let Err(Some(sub_id)) = process_subscription_response(&mut manager.lock(), response) {
						return Ok(Some(FrontToBack::SubscriptionClosed(sub_id)));
					}
				}
				// Subscription error response.
				else if let Ok(response) = serde_json::from_slice::<SubscriptionError<_>>(raw) {
					process_subscription_close_response(&mut manager.lock(), response);
				}
				// Incoming Notification
				else if let Ok(notif) = serde_json::from_slice::<Notification<_>>(raw) {
					process_notification(&mut manager.lock(), notif);
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
						let Ok(response) = serde_json::from_str::<Response<_>>(r.get()) else {
							return Err(unparse_error(raw));
						};

						let id = response.id.try_parse_inner_as_number()?;
						let result = ResponseSuccess::try_from(response).map(|s| s.result);
						batch.push(InnerBatchResponse { id, result });

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
						process_batch_response(&mut manager.lock(), batch, range)?;
					} else {
						return Err(EmptyBatchRequest.into());
					}
				} else {
					return Err(unparse_error(raw));
				}
			}
			_ => {
				return Err(unparse_error(raw));
			}
		};

		Ok(None)
	}

	match message {
		Some(Ok(ReceivedMessage::Pong)) => {
			tracing::debug!(target: LOG_TARGET, "Received pong");
			Ok(None)
		}
		Some(Ok(ReceivedMessage::Bytes(raw))) => {
			handle_recv_message(raw.as_ref(), manager, max_buffer_capacity_per_subscription)
		}
		Some(Ok(ReceivedMessage::Text(raw))) => {
			handle_recv_message(raw.as_ref(), manager, max_buffer_capacity_per_subscription)
		}
		Some(Err(e)) => Err(Error::Transport(e.into())),
		None => Err(Error::Custom("TransportReceiver dropped".into())),
	}
}

/// Handle frontend messages.
async fn handle_frontend_messages<S: TransportSenderT>(
	message: FrontToBack,
	manager: &ThreadSafeRequestManager,
	sender: &mut S,
	max_buffer_capacity_per_subscription: usize,
) -> Result<(), S::Error> {
	match message {
		FrontToBack::Batch(batch) => {
			if let Err(send_back) = manager.lock().insert_pending_batch(batch.ids.clone(), batch.send_back) {
				tracing::warn!(target: LOG_TARGET, "Batch request already pending: {:?}", batch.ids);
				let _ = send_back.send(Err(InvalidRequestId::Occupied(format!("{:?}", batch.ids)).into()));
				return Ok(());
			}

			sender.send(batch.raw).await?;
		}
		// User called `notification` on the front-end
		FrontToBack::Notification(notif) => {
			sender.send(notif).await?;
		}
		// User called `request` on the front-end
		FrontToBack::Request(request) => {
			if let Err(send_back) = manager.lock().insert_pending_call(request.id.clone(), request.send_back) {
				tracing::warn!(target: LOG_TARGET, "Denied duplicate method call");

				if let Some(s) = send_back {
					let _ = s.send(Err(InvalidRequestId::Occupied(request.id.to_string()).into()));
				}
				return Ok(());
			}

			sender.send(request.raw).await?;
		}
		// User called `subscribe` on the front-end.
		FrontToBack::Subscribe(sub) => {
			if let Err(send_back) = manager.lock().insert_pending_subscription(
				sub.subscribe_id.clone(),
				sub.unsubscribe_id.clone(),
				sub.send_back,
				sub.unsubscribe_method,
			) {
				tracing::warn!(target: LOG_TARGET, "Denied duplicate subscription");

				let _ = send_back.send(Err(InvalidRequestId::Occupied(format!(
					"sub_id={}:req_id={}",
					sub.subscribe_id, sub.unsubscribe_id
				))
				.into()));
				return Ok(());
			}

			sender.send(sub.raw).await?;
		}
		// User dropped a subscription.
		FrontToBack::SubscriptionClosed(sub_id) => {
			tracing::trace!(target: LOG_TARGET, "Closing subscription: {:?}", sub_id);
			// NOTE: The subscription may have been closed earlier if
			// the channel was full or disconnected.

			let maybe_unsub = {
				let m = &mut *manager.lock();

				m.get_request_id_by_subscription_id(&sub_id)
					.and_then(|req_id| build_unsubscribe_message(m, req_id, sub_id))
			};

			if let Some(unsub) = maybe_unsub {
				stop_subscription::<S>(sender, unsub).await?;
			}
		}
		// User called `register_notification` on the front-end.
		FrontToBack::RegisterNotification(reg) => {
			let (subscribe_tx, subscribe_rx) = mpsc::channel(max_buffer_capacity_per_subscription);

			if manager.lock().insert_notification_handler(&reg.method, subscribe_tx).is_ok() {
				let _ = reg.send_back.send(Ok((subscribe_rx, reg.method)));
			} else {
				let _ =
					reg.send_back.send(Err(Error::RegisterMethod(RegisterMethodError::AlreadyRegistered(reg.method))));
			}
		}
		// User dropped the NotificationHandler for this method
		FrontToBack::UnregisterNotification(method) => {
			let _ = manager.lock().remove_notification_handler(&method);
		}
	};

	Ok(())
}

fn unparse_error(raw: &[u8]) -> Error {
	let json = serde_json::from_slice::<serde_json::Value>(raw);

	let json_str = match json {
		Ok(json) => serde_json::to_string(&json).expect("valid JSON; qed"),
		Err(e) => e.to_string(),
	};

	Error::Custom(format!("Unparseable message: {json_str}"))
}

struct SendTaskParams<S: TransportSenderT> {
	sender: S,
	from_frontend: mpsc::Receiver<FrontToBack>,
	close_tx: mpsc::Sender<Result<(), Error>>,
	manager: ThreadSafeRequestManager,
	max_buffer_capacity_per_subscription: usize,
	ping_interval: Option<Duration>,
}

async fn send_task<S>(params: SendTaskParams<S>)
where
	S: TransportSenderT,
{
	let SendTaskParams {
		mut sender,
		mut from_frontend,
		close_tx,
		manager,
		max_buffer_capacity_per_subscription,
		ping_interval,
	} = params;

	// This is safe because `tokio::time::Interval`, `tokio::mpsc::Sender` and `tokio::mpsc::Receiver`
	// are cancel-safe.
	let res = if let Some(ping_interval) = ping_interval {
		let mut ping = tokio::time::interval_at(tokio::time::Instant::now() + ping_interval, ping_interval);

		loop {
			tokio::select! {
				biased;
				_ = close_tx.closed() => break Ok(()),
				maybe_msg = from_frontend.recv() => {
					let Some(msg) = maybe_msg else {
						break Ok(());
					};

					if let Err(e) =
						handle_frontend_messages(msg, &manager, &mut sender, max_buffer_capacity_per_subscription).await
					{
						tracing::error!(target: LOG_TARGET, "Could not send message: {e}");
						break Err(Error::Transport(e.into()));
					}
				}
				_ = ping.tick() => {
					if let Err(err) = sender.send_ping().await {
						tracing::error!(target: LOG_TARGET, "Could not send ping frame: {err}");
						break Err(Error::Custom("Could not send ping frame".into()));
					}
				}
			}
		}
	} else {
		loop {
			tokio::select! {
				biased;
				_ = close_tx.closed() => break Ok(()),
				maybe_msg = from_frontend.recv() => {
					let Some(msg) = maybe_msg else {
						break Ok(());
					};

					if let Err(e) =
						handle_frontend_messages(msg, &manager, &mut sender, max_buffer_capacity_per_subscription).await
					{
						tracing::error!(target: LOG_TARGET, "Could not send message: {e}");
						break Err(Error::Transport(e.into()));
					}
				}
			}
		}
	};

	from_frontend.close();
	let _ = sender.close().await;
	let _ = close_tx.send(res).await;
}

struct ReadTaskParams<R: TransportReceiverT> {
	receiver: R,
	close_tx: mpsc::Sender<Result<(), Error>>,
	to_send_task: mpsc::Sender<FrontToBack>,
	manager: ThreadSafeRequestManager,
	max_buffer_capacity_per_subscription: usize,
}

async fn read_task<R>(params: ReadTaskParams<R>)
where
	R: TransportReceiverT,
{
	let ReadTaskParams { receiver, close_tx, to_send_task, manager, max_buffer_capacity_per_subscription } = params;

	let backend_event = futures_util::stream::unfold(receiver, |mut receiver| async {
		let res = receiver.receive().await;
		Some((res, receiver))
	});

	// These "unsubscription" occurs if a subscription gets dropped by frontend before ack:ed or that if
	// a subscription couldn't keep with the server.
	//
	// Thus, these needs to be sent to the server inorder to tell the server to not bother
	// with those messages anymore.
	let pending_unsubscribes = MaybePendingFutures::new();

	tokio::pin!(backend_event, pending_unsubscribes);

	// This is safe because futures::Stream and tokio::mpsc::Sender are cancel-safe.
	let res = loop {
		tokio::select! {
			// Closed.
			biased;
			_ = close_tx.closed() => break Ok(()),
			// Unsubscribe completed.
			_ = pending_unsubscribes.next() => (),
			// New message received.
			maybe_msg = backend_event.next() => {
				let Some(msg) = maybe_msg else { break Ok(()) };

				match handle_backend_messages::<R>(Some(msg), &manager, max_buffer_capacity_per_subscription) {
					Ok(Some(msg)) => {
						pending_unsubscribes.push(to_send_task.send(msg));
					}
					Err(e) => {
						tracing::error!(target: LOG_TARGET, "Failed to read message: {e}");
						break Err(e);
					}
					Ok(None) => (),
				}

			}
		}
	};

	let _ = close_tx.send(res).await;
}

async fn wait_for_shutdown(
	mut close_rx: mpsc::Receiver<Result<(), Error>>,
	client_dropped: oneshot::Receiver<()>,
	err_to_front: oneshot::Sender<Error>,
) {
	let rx_item = close_rx.recv();

	tokio::pin!(rx_item);

	// Send an error to the frontend if the send or receive task completed with an error.
	if let Either::Left((Some(Err(err)), _)) = future::select(rx_item, client_dropped).await {
		let _ = err_to_front.send(err);
	}
}

/// A wrapper around `FuturesUnordered` that doesn't return `None` when it's empty.
struct MaybePendingFutures<Fut> {
	futs: FuturesUnordered<Fut>,
	waker: Option<Waker>,
}

impl<Fut> MaybePendingFutures<Fut> {
	fn new() -> Self {
		Self { futs: FuturesUnordered::new(), waker: None }
	}

	fn push(&mut self, fut: Fut) {
		self.futs.push(fut);

		if let Some(w) = self.waker.take() {
			w.wake();
		}
	}
}

impl<Fut: Future> Stream for MaybePendingFutures<Fut> {
	type Item = Fut::Output;

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
		if self.futs.is_empty() {
			self.waker = Some(cx.waker().clone());
			return Poll::Pending;
		}

		self.futs.poll_next_unpin(cx)
	}
}
