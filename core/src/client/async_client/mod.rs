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

//! Abstract async client.

mod helpers;
mod manager;
mod utils;

use crate::client::async_client::helpers::{process_subscription_close_response, InnerBatchResponse};
use crate::client::async_client::utils::MaybePendingFutures;
use crate::client::{
	BatchMessage, BatchResponse, ClientT, Error, ReceivedMessage, RegisterNotificationMessage, RequestMessage,
	Subscription, SubscriptionClientT, SubscriptionKind, SubscriptionMessage, TransportReceiverT, TransportSenderT,
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
use std::sync::Arc;

use async_trait::async_trait;
use futures_timer::Delay;
use futures_util::future::{self, Either};
use futures_util::stream::StreamExt;
use futures_util::Stream;
use jsonrpsee_types::response::{ResponsePayload, SubscriptionError};
use jsonrpsee_types::{Notification, NotificationSer, RequestSer, Response, SubscriptionResponse};
use serde::de::DeserializeOwned;
use tokio::sync::{mpsc, oneshot};
use tracing::instrument;

use self::utils::{InactivityCheck, IntervalStream};

use super::{generate_batch_id_range, FrontToBack, IdKind, RequestIdManager};

const LOG_TARGET: &str = "jsonrpsee-client";
const NOT_POISONED: &str = "Not poisoned; qed";

/// Configuration for WebSocket ping/pong mechanism and it may be used to disconnect
/// an inactive connection.
///
/// jsonrpsee doesn't associate the ping/pong frames just that if
/// a pong frame isn't received within the `inactive_limit` then it's regarded
/// as missed.
///
/// Such that the `inactive_limit` should be configured to longer than a single
/// WebSocket ping takes or it might be missed and may end up
/// terminating the connection.
///
/// Default: ping_interval: 30 seconds, max failures: 1 and inactive limit: 40 seconds.
#[derive(Debug, Copy, Clone)]
pub struct PingConfig {
	/// Interval that the pings are sent.
	pub(crate) ping_interval: Duration,
	/// Max allowed time for a connection to stay idle.
	pub(crate) inactive_limit: Duration,
	/// Max failures.
	pub(crate) max_failures: usize,
}

impl Default for PingConfig {
	fn default() -> Self {
		Self { ping_interval: Duration::from_secs(30), max_failures: 1, inactive_limit: Duration::from_secs(40) }
	}
}

impl PingConfig {
	/// Create a new PingConfig.
	pub fn new() -> Self {
		Self::default()
	}

	/// Configure the interval when the WebSocket pings are sent out.
	pub fn ping_interval(mut self, ping_interval: Duration) -> Self {
		self.ping_interval = ping_interval;
		self
	}

	/// Configure how long to wait for the WebSocket pong.
	/// When this limit is expired it's regarded as inresponsive.
	///
	/// You may configure how many times the connection is allowed to
	/// be inactive by [`PingConfig::max_failures`].
	pub fn inactive_limit(mut self, inactivity_limit: Duration) -> Self {
		self.inactive_limit = inactivity_limit;
		self
	}

	/// Configure how many times the connection is allowed be
	/// inactive until the connection is closed.
	///
	/// # Panics
	///
	/// This method panics if `max` == 0.
	pub fn max_failures(mut self, max: usize) -> Self {
		assert!(max > 0);
		self.max_failures = max;
		self
	}
}

#[derive(Debug, Default, Clone)]
pub(crate) struct ThreadSafeRequestManager(Arc<std::sync::Mutex<RequestManager>>);

impl ThreadSafeRequestManager {
	pub(crate) fn new() -> Self {
		Self::default()
	}

	pub(crate) fn lock(&self) -> std::sync::MutexGuard<RequestManager> {
		self.0.lock().expect(NOT_POISONED)
	}
}

pub(crate) type SharedDisconnectReason = Arc<std::sync::RwLock<Option<Arc<Error>>>>;

/// If the background thread is terminated, this type
/// can be used to read the error cause.
///
// NOTE: This is an AsyncRwLock to be &self.
#[derive(Debug)]
struct ErrorFromBack {
	conn: mpsc::Sender<FrontToBack>,
	disconnect_reason: SharedDisconnectReason,
}

impl ErrorFromBack {
	fn new(conn: mpsc::Sender<FrontToBack>, disconnect_reason: SharedDisconnectReason) -> Self {
		Self { conn, disconnect_reason }
	}

	async fn read_error(&self) -> Error {
		// When the background task is closed the error is written to `disconnect_reason`.
		self.conn.closed().await;

		if let Some(err) = self.disconnect_reason.read().expect(NOT_POISONED).as_ref() {
			Error::RestartNeeded(err.clone())
		} else {
			Error::Custom("Error reason could not be found. This is a bug. Please open an issue.".to_string())
		}
	}
}

/// Builder for [`Client`].
#[derive(Debug, Copy, Clone)]
pub struct ClientBuilder {
	request_timeout: Duration,
	max_concurrent_requests: usize,
	max_buffer_capacity_per_subscription: usize,
	id_kind: IdKind,
	max_log_length: u32,
	ping_config: Option<PingConfig>,
	tcp_no_delay: bool,
}

impl Default for ClientBuilder {
	fn default() -> Self {
		Self {
			request_timeout: Duration::from_secs(60),
			max_concurrent_requests: 256,
			max_buffer_capacity_per_subscription: 1024,
			id_kind: IdKind::Number,
			max_log_length: 4096,
			ping_config: None,
			tcp_no_delay: true,
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
		assert!(max > 0);
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

	/// Enable WebSocket ping/pong on the client.
	///
	/// This only works if the transport supports WebSocket pings.
	///
	/// Default: pings are disabled.
	pub fn enable_ws_ping(mut self, cfg: PingConfig) -> Self {
		self.ping_config = Some(cfg);
		self
	}

	/// Disable WebSocket ping/pong on the server.
	///
	/// Default: pings are disabled.
	pub fn disable_ws_ping(mut self) -> Self {
		self.ping_config = None;
		self
	}

	/// Configure `TCP_NODELAY` on the socket to the supplied value `nodelay`.
	///
	/// On some transports this may have no effect.
	///
	/// Default is `true`.
	pub fn set_tcp_no_delay(mut self, no_delay: bool) -> Self {
		self.tcp_no_delay = no_delay;
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
		let disconnect_reason = SharedDisconnectReason::default();
		let max_buffer_capacity_per_subscription = self.max_buffer_capacity_per_subscription;
		let (client_dropped_tx, client_dropped_rx) = oneshot::channel();
		let (send_receive_task_sync_tx, send_receive_task_sync_rx) = mpsc::channel(1);
		let manager = ThreadSafeRequestManager::new();

		let (ping_interval, inactivity_stream, inactivity_check) = match self.ping_config {
			None => (IntervalStream::pending(), IntervalStream::pending(), InactivityCheck::Disabled),
			Some(p) => {
				// NOTE: This emits a tick immediately to sync how the `inactive_interval` works
				// because it starts measuring when the client start-ups.
				let ping_interval = IntervalStream::new(tokio_stream::wrappers::IntervalStream::new(
					tokio::time::interval(p.ping_interval),
				));

				let inactive_interval = {
					let start = tokio::time::Instant::now() + p.inactive_limit;
					IntervalStream::new(tokio_stream::wrappers::IntervalStream::new(tokio::time::interval_at(
						start,
						p.inactive_limit,
					)))
				};

				let inactivity_check = InactivityCheck::new(p.inactive_limit, p.max_failures);

				(ping_interval, inactive_interval, inactivity_check)
			}
		};

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
			inactivity_check,
			inactivity_stream,
		}));

		tokio::spawn(wait_for_shutdown(send_receive_task_sync_rx, client_dropped_rx, disconnect_reason.clone()));

		Client {
			to_back: to_back.clone(),
			request_timeout: self.request_timeout,
			error: ErrorFromBack::new(to_back, disconnect_reason),
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
		use futures_util::stream::Pending;

		type PendingIntervalStream = IntervalStream<Pending<()>>;

		let (to_back, from_front) = mpsc::channel(self.max_concurrent_requests);
		let disconnect_reason = SharedDisconnectReason::default();
		let max_buffer_capacity_per_subscription = self.max_buffer_capacity_per_subscription;
		let (client_dropped_tx, client_dropped_rx) = oneshot::channel();
		let (send_receive_task_sync_tx, send_receive_task_sync_rx) = mpsc::channel(1);
		let manager = ThreadSafeRequestManager::new();

		let ping_interval = PendingIntervalStream::pending();
		let inactivity_stream = PendingIntervalStream::pending();
		let inactivity_check = InactivityCheck::Disabled;

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
			inactivity_check,
			inactivity_stream,
		}));

		wasm_bindgen_futures::spawn_local(wait_for_shutdown(
			send_receive_task_sync_rx,
			client_dropped_rx,
			disconnect_reason.clone(),
		));

		Client {
			to_back: to_back.clone(),
			request_timeout: self.request_timeout,
			error: ErrorFromBack::new(to_back, disconnect_reason),
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
	/// This method is cancel-safe
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

		rx_log_from_json(&Response::new(ResponsePayload::success_borrowed(&json_value), id), self.max_log_length);

		serde_json::from_value(json_value).map_err(Error::ParseError)
	}

	#[instrument(name = "batch", skip(self, batch), level = "trace")]
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
			return Err(RegisterMethodError::SubscriptionNameConflict(unsubscribe_method.to_owned()).into());
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

		rx_log_from_json(&Response::new(ResponsePayload::success_borrowed(&sub_id), id_unsub), self.max_log_length);

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
) -> Result<Vec<FrontToBack>, Error> {
	// Handle raw messages of form `ReceivedMessage::Bytes` (Vec<u8>) or ReceivedMessage::Data` (String).
	fn handle_recv_message(
		raw: &[u8],
		manager: &ThreadSafeRequestManager,
		max_buffer_capacity_per_subscription: usize,
	) -> Result<Vec<FrontToBack>, Error> {
		let first_non_whitespace = raw.iter().find(|byte| !byte.is_ascii_whitespace());
		let mut messages = Vec::new();

		match first_non_whitespace {
			Some(b'{') => {
				// Single response to a request.
				if let Ok(single) = serde_json::from_slice::<Response<_>>(raw) {
					let maybe_unsub =
						process_single_response(&mut manager.lock(), single, max_buffer_capacity_per_subscription)?;

					if let Some(unsub) = maybe_unsub {
						return Ok(vec![FrontToBack::Request(unsub)]);
					}
				}
				// Subscription response.
				else if let Ok(response) = serde_json::from_slice::<SubscriptionResponse<_>>(raw) {
					if let Err(Some(sub_id)) = process_subscription_response(&mut manager.lock(), response) {
						return Ok(vec![FrontToBack::SubscriptionClosed(sub_id)]);
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
					let mut got_notif = false;

					for r in raw_responses {
						if let Ok(response) = serde_json::from_str::<Response<_>>(r.get()) {
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
						} else if let Ok(response) = serde_json::from_str::<SubscriptionResponse<_>>(r.get()) {
							got_notif = true;
							if let Err(Some(sub_id)) = process_subscription_response(&mut manager.lock(), response) {
								messages.push(FrontToBack::SubscriptionClosed(sub_id));
							}
						} else if let Ok(response) = serde_json::from_slice::<SubscriptionError<_>>(raw) {
							got_notif = true;
							process_subscription_close_response(&mut manager.lock(), response);
						} else if let Ok(notif) = serde_json::from_str::<Notification<_>>(r.get()) {
							got_notif = true;
							process_notification(&mut manager.lock(), notif);
						} else {
							return Err(unparse_error(raw));
						};
					}

					if let Some(mut range) = range {
						// the range is exclusive so need to add one.
						range.end += 1;
						process_batch_response(&mut manager.lock(), batch, range)?;
					} else if !got_notif {
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

		Ok(messages)
	}

	match message {
		Some(Ok(ReceivedMessage::Pong)) => {
			tracing::debug!(target: LOG_TARGET, "Received pong");
			Ok(vec![])
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
				tracing::debug!(target: LOG_TARGET, "Batch request already pending: {:?}", batch.ids);
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
				tracing::debug!(target: LOG_TARGET, "Denied duplicate method call");

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
				tracing::debug!(target: LOG_TARGET, "Denied duplicate subscription");

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
				let _ = reg.send_back.send(Err(RegisterMethodError::AlreadyRegistered(reg.method).into()));
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

struct SendTaskParams<T: TransportSenderT, S> {
	sender: T,
	from_frontend: mpsc::Receiver<FrontToBack>,
	close_tx: mpsc::Sender<Result<(), Error>>,
	manager: ThreadSafeRequestManager,
	max_buffer_capacity_per_subscription: usize,
	ping_interval: IntervalStream<S>,
}

async fn send_task<T, S>(params: SendTaskParams<T, S>)
where
	T: TransportSenderT,
	S: Stream + Unpin,
{
	let SendTaskParams {
		mut sender,
		mut from_frontend,
		close_tx,
		manager,
		max_buffer_capacity_per_subscription,
		mut ping_interval,
	} = params;

	// This is safe because `tokio::time::Interval`, `tokio::mpsc::Sender` and `tokio::mpsc::Receiver`
	// are cancel-safe.
	let res = loop {
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
					tracing::debug!(target: LOG_TARGET, "ws send failed: {e}");
					break Err(Error::Transport(e.into()));
				}
			}
			_ = ping_interval.next() => {
				if let Err(err) = sender.send_ping().await {
					tracing::debug!(target: LOG_TARGET, "Send ws ping failed: {err}");
					break Err(Error::Transport(err.into()));
				}
			}
		}
	};

	from_frontend.close();
	let _ = sender.close().await;
	let _ = close_tx.send(res).await;
}

struct ReadTaskParams<R: TransportReceiverT, S> {
	receiver: R,
	close_tx: mpsc::Sender<Result<(), Error>>,
	to_send_task: mpsc::Sender<FrontToBack>,
	manager: ThreadSafeRequestManager,
	max_buffer_capacity_per_subscription: usize,
	inactivity_check: InactivityCheck,
	inactivity_stream: IntervalStream<S>,
}

async fn read_task<R, S>(params: ReadTaskParams<R, S>)
where
	R: TransportReceiverT,
	S: Stream + Unpin,
{
	let ReadTaskParams {
		receiver,
		close_tx,
		to_send_task,
		manager,
		max_buffer_capacity_per_subscription,
		mut inactivity_check,
		mut inactivity_stream,
	} = params;

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
				inactivity_check.mark_as_active();
				let Some(msg) = maybe_msg else { break Ok(()) };

				match handle_backend_messages::<R>(Some(msg), &manager, max_buffer_capacity_per_subscription) {
					Ok(messages) => {
						for msg in messages {
							pending_unsubscribes.push(to_send_task.send(msg));
						}
					}
					Err(e) => {
						tracing::debug!(target: LOG_TARGET, "Failed to read message: {e}");
						break Err(e);
					}
				}
			}
			_ = inactivity_stream.next() => {
				if inactivity_check.is_inactive() {
					break Err(Error::Transport(anyhow::anyhow!("WebSocket ping/pong inactive")));
				}
			}
		}
	};

	let _ = close_tx.send(res).await;
}

async fn wait_for_shutdown(
	mut close_rx: mpsc::Receiver<Result<(), Error>>,
	client_dropped: oneshot::Receiver<()>,
	err_to_front: SharedDisconnectReason,
) {
	let rx_item = close_rx.recv();

	tokio::pin!(rx_item);

	// Send an error to the frontend if the send or receive task completed with an error.
	if let Either::Left((Some(Err(err)), _)) = future::select(rx_item, client_dropped).await {
		*err_to_front.write().expect(NOT_POISONED) = Some(Arc::new(err));
	}
}
