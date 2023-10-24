use std::sync::Arc;
use std::time::Duration;

use crate::logger::{self, Logger, TransportProtocol};
use crate::server::{BatchRequestConfig, ServiceData};

use futures_util::future::{self, Either};
use futures_util::io::{BufReader, BufWriter};
use futures_util::stream::FuturesOrdered;
use futures_util::{Future, StreamExt, TryStreamExt};
use hyper::upgrade::Upgraded;
use jsonrpsee_core::server::helpers::{
	batch_response_error, prepare_error, BatchResponseBuilder, MethodResponse, MethodSink,
};
use jsonrpsee_core::server::{BoundedSubscriptions, CallOrSubscription, MethodCallback, Methods, SubscriptionState};
use jsonrpsee_core::tracing::{rx_log_from_json, tx_log_from_str};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::{Error, JsonRawValue};
use jsonrpsee_types::error::{
	reject_too_big_batch_request, reject_too_big_request, reject_too_many_subscriptions, ErrorCode,
	BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG,
};
use jsonrpsee_types::{ErrorObject, Id, InvalidRequest, Notification, Params, Request};
use soketto::connection::Error as SokettoError;
use soketto::data::ByteSlice125;

use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::{IntervalStream, ReceiverStream};
use tokio_util::compat::Compat;
use tracing::instrument;

pub(crate) type Sender = soketto::Sender<BufReader<BufWriter<Compat<Upgraded>>>>;
pub(crate) type Receiver = soketto::Receiver<BufReader<BufWriter<Compat<Upgraded>>>>;

type Notif<'a> = Notification<'a, Option<&'a JsonRawValue>>;

enum Incoming {
	Data(Vec<u8>),
	Pong,
}

pub(crate) async fn send_message(sender: &mut Sender, response: String) -> Result<(), Error> {
	sender.send_text_owned(response).await?;
	sender.flush().await.map_err(Into::into)
}

pub(crate) async fn send_ping(sender: &mut Sender) -> Result<(), Error> {
	tracing::debug!("Send ping");
	// Submit empty slice as "optional" parameter.
	let slice: &[u8] = &[];
	// Byte slice fails if the provided slice is larger than 125 bytes.
	let byte_slice = ByteSlice125::try_from(slice).expect("Empty slice should fit into ByteSlice125");
	sender.send_ping(byte_slice).await?;
	sender.flush().await.map_err(Into::into)
}

#[derive(Debug, Clone)]
pub(crate) struct Batch<'a, L: Logger> {
	pub(crate) data: &'a [u8],
	pub(crate) call: CallData<'a, L>,
	pub(crate) max_len: usize,
}

#[derive(Debug, Clone)]
pub(crate) struct CallData<'a, L: Logger> {
	pub(crate) conn_id: usize,
	pub(crate) bounded_subscriptions: &'a BoundedSubscriptions,
	pub(crate) id_provider: &'a dyn IdProvider,
	pub(crate) methods: &'a Methods,
	pub(crate) max_response_body_size: u32,
	pub(crate) max_log_length: u32,
	pub(crate) sink: &'a MethodSink,
	pub(crate) logger: &'a L,
	pub(crate) request_start: L::Instant,
}

// Batch responses must be sent back as a single message so we read the results from each
// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
// complete batch response back to the client over `tx`.
#[instrument(name = "batch", skip(b), level = "TRACE")]
pub(crate) async fn process_batch_request<L: Logger>(b: Batch<'_, L>) -> Option<String> {
	let Batch { data, call, max_len } = b;

	if let Ok(batch) = serde_json::from_slice::<Vec<&JsonRawValue>>(data) {
		if batch.len() > max_len {
			return Some(batch_response_error(Id::Null, reject_too_big_batch_request(max_len)));
		}

		let mut got_notif = false;
		let mut batch_response = BatchResponseBuilder::new_with_limit(call.max_response_body_size as usize);

		let mut pending_calls: FuturesOrdered<_> = batch
			.into_iter()
			.filter_map(|v| {
				if let Ok(req) = serde_json::from_str::<Request>(v.get()) {
					Some(Either::Right(async { execute_call(req, call.clone()).await.into_response() }))
				} else if let Ok(_notif) = serde_json::from_str::<Notif>(v.get()) {
					// notifications should not be answered.
					got_notif = true;
					None
				} else {
					// valid JSON but could be not parsable as `InvalidRequest`
					let id = match serde_json::from_str::<InvalidRequest>(v.get()) {
						Ok(err) => err.id,
						Err(_) => Id::Null,
					};

					Some(Either::Left(async {
						MethodResponse::error(id, ErrorObject::from(ErrorCode::InvalidRequest))
					}))
				}
			})
			.collect();

		while let Some(response) = pending_calls.next().await {
			if let Err(too_large) = batch_response.append(&response) {
				return Some(too_large);
			}
		}

		if got_notif && batch_response.is_empty() {
			None
		} else {
			Some(batch_response.finish())
		}
	} else {
		Some(batch_response_error(Id::Null, ErrorObject::from(ErrorCode::ParseError)))
	}
}

pub(crate) async fn process_single_request<L: Logger>(
	data: &[u8],
	call: CallData<'_, L>,
) -> Option<CallOrSubscription> {
	if let Ok(req) = serde_json::from_slice::<Request>(data) {
		Some(execute_call_with_tracing(req, call).await)
	} else if serde_json::from_slice::<Notif>(data).is_ok() {
		None
	} else {
		let (id, code) = prepare_error(data);
		Some(CallOrSubscription::Call(MethodResponse::error(id, ErrorObject::from(code))))
	}
}

#[instrument(name = "method_call", fields(method = req.method.as_ref()), skip(call, req), level = "TRACE")]
pub(crate) async fn execute_call_with_tracing<'a, L: Logger>(
	req: Request<'a>,
	call: CallData<'_, L>,
) -> CallOrSubscription {
	execute_call(req, call).await
}

/// Execute a call which returns result of the call with a additional sink
/// to fire a signal once the subscription call has been answered.
///
/// Returns `(MethodResponse, None)` on every call that isn't a subscription
/// Otherwise `(MethodResponse, Some(PendingSubscriptionCallTx)`.
pub(crate) async fn execute_call<'a, L: Logger>(req: Request<'a>, call: CallData<'_, L>) -> CallOrSubscription {
	let CallData {
		methods,
		max_response_body_size,
		max_log_length,
		conn_id,
		id_provider,
		sink,
		logger,
		request_start,
		bounded_subscriptions,
	} = call;

	rx_log_from_json(&req, call.max_log_length);

	let params = Params::new(req.params.map(|params| params.get()));
	let name = &req.method;
	let id = req.id;

	let response = match methods.method_with_name(name) {
		None => {
			logger.on_call(name, params.clone(), logger::MethodKind::Unknown, TransportProtocol::WebSocket);
			let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::MethodNotFound));
			CallOrSubscription::Call(response)
		}
		Some((name, method)) => match method {
			MethodCallback::Sync(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::MethodCall, TransportProtocol::WebSocket);
				CallOrSubscription::Call((callback)(id, params, max_response_body_size as usize))
			}
			MethodCallback::Async(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::MethodCall, TransportProtocol::WebSocket);

				let id = id.into_owned();
				let params = params.into_owned();

				let response = (callback)(id, params, conn_id, max_response_body_size as usize).await;
				CallOrSubscription::Call(response)
			}
			MethodCallback::Subscription(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::Subscription, TransportProtocol::WebSocket);

				if let Some(p) = bounded_subscriptions.acquire() {
					let conn_state = SubscriptionState { conn_id, id_provider, subscription_permit: p };
					match callback(id, params, sink.clone(), conn_state).await {
						Ok(r) => CallOrSubscription::Subscription(r),
						Err(id) => {
							let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError));
							CallOrSubscription::Call(response)
						}
					}
				} else {
					let response =
						MethodResponse::error(id, reject_too_many_subscriptions(bounded_subscriptions.max()));
					CallOrSubscription::Call(response)
				}
			}
			MethodCallback::Unsubscription(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::Unsubscription, TransportProtocol::WebSocket);

				// Don't adhere to any resource or subscription limits; always let unsubscribing happen!
				let result = callback(id, params, conn_id, max_response_body_size as usize);
				CallOrSubscription::Call(result)
			}
		},
	};

	let r = response.as_response();

	tx_log_from_str(&r.result, max_log_length);
	logger.on_result(name, r.success_or_error, request_start, TransportProtocol::WebSocket);
	response
}

pub(crate) async fn background_task<L: Logger>(sender: Sender, receiver: Receiver, svc: ServiceData<L>) {
	let ServiceData {
		methods,
		max_request_body_size,
		max_response_body_size,
		max_log_length,
		max_subscriptions_per_connection,
		batch_requests_config,
		stop_handle,
		id_provider,
		ping_interval,
		conn_id,
		logger,
		remote_addr,
		message_buffer_capacity,
		conn,
		..
	} = svc;

	let (tx, rx) = mpsc::channel::<String>(message_buffer_capacity as usize);
	let (conn_tx, conn_rx) = oneshot::channel();
	let sink = MethodSink::new_with_limit(tx, max_response_body_size, max_log_length);
	let bounded_subscriptions = BoundedSubscriptions::new(max_subscriptions_per_connection);

	// On each method call the `pending_calls` is cloned
	// then when all pending_calls are dropped
	// a graceful shutdown can has occur.
	let (pending_calls, pending_calls_completed) = mpsc::channel::<()>(1);

	// Spawn another task that sends out the responses on the Websocket.
	let send_task_handle = tokio::spawn(send_task(rx, sender, ping_interval, conn_rx));

	let stopped = stop_handle.clone().shutdown();

	let params = Arc::new(ExecuteCallParams {
		batch_requests_config,
		conn_id,
		methods,
		max_log_length,
		max_response_body_size,
		sink: sink.clone(),
		id_provider,
		logger: logger.clone(),
		bounded_subscriptions,
	});

	tokio::pin!(stopped);

	let ws_stream = futures_util::stream::unfold(receiver, |mut receiver| async {
		let mut data = Vec::new();
		match receiver.receive(&mut data).await {
			Ok(soketto::Incoming::Data(_)) => Some((Ok(Incoming::Data(data)), receiver)),
			Ok(soketto::Incoming::Pong(_)) => Some((Ok(Incoming::Pong), receiver)),
			Ok(soketto::Incoming::Closed(_)) | Err(SokettoError::Closed) => None,
			// The closing reason is already logged by `soketto` trace log level.
			// Return the `Closed` error to avoid logging unnecessary warnings on clean shutdown.
			Err(e) => Some((Err(e), receiver)),
		}
	})
	.fuse();

	tokio::pin!(ws_stream);

	let result = loop {
		let data = match try_recv(&mut ws_stream, stopped).await {
			Receive::ConnectionClosed => break Ok(Shutdown::ConnectionClosed),
			Receive::Stopped => break Ok(Shutdown::Stopped),
			Receive::Ok(data, stop) => {
				stopped = stop;
				data
			}
			Receive::Err(err, stop) => {
				stopped = stop;

				match err {
					SokettoError::Closed => {
						tracing::debug!("WS transport: remote peer terminated the connection: {}", conn_id);
						break Ok(Shutdown::ConnectionClosed);
					}
					SokettoError::MessageTooLarge { current, maximum } => {
						tracing::debug!(
							"WS transport error: request length: {} exceeded max limit: {} bytes",
							current,
							maximum
						);
						if sink.send_error(Id::Null, reject_too_big_request(max_request_body_size)).await.is_err() {
							break Ok(Shutdown::ConnectionClosed);
						}

						continue;
					}
					err => {
						tracing::debug!("WS transport error: {}; terminate connection: {}", err, conn_id);
						break Err(err);
					}
				};
			}
		};

		tokio::spawn(execute_unchecked_call(params.clone(), data, pending_calls.clone()));
	};

	// Drive all running methods to completion.
	// **NOTE** Do not return early in this function. This `await` needs to run to guarantee
	// proper drop behaviour.
	drop(pending_calls);
	graceful_shutdown(result, pending_calls_completed, ws_stream, conn_tx, send_task_handle).await;

	logger.on_disconnect(remote_addr, TransportProtocol::WebSocket);
	drop(conn);
	drop(stop_handle);
}

/// A task that waits for new messages via the `rx channel` and sends them out on the `WebSocket`.
async fn send_task(
	rx: mpsc::Receiver<String>,
	mut ws_sender: Sender,
	ping_interval: Duration,
	stop: oneshot::Receiver<()>,
) {
	// Interval to send out continuously `pings`.
	let mut ping_interval = tokio::time::interval(ping_interval);
	// This returns immediately so make sure it doesn't resolve before the ping_interval has been elapsed.
	ping_interval.tick().await;

	let ping_interval = IntervalStream::new(ping_interval);
	let rx = ReceiverStream::new(rx);

	tokio::pin!(ping_interval, rx, stop);

	// Received messages from the WebSocket.
	let mut rx_item = rx.next();
	let next_ping = ping_interval.next();
	let mut futs = future::select(next_ping, stop);

	loop {
		// Ensure select is cancel-safe by fetching and storing the `rx_item` that did not finish yet.
		// Note: Although, this is cancel-safe already, avoid using `select!` macro for future proofing.
		match future::select(rx_item, futs).await {
			// Received message.
			Either::Left((Some(response), not_ready)) => {
				// If websocket message send fail then terminate the connection.
				if let Err(err) = send_message(&mut ws_sender, response).await {
					tracing::debug!("WS transport error: send failed: {}", err);
					break;
				}

				rx_item = rx.next();
				futs = not_ready;
			}

			// Nothing else to receive.
			Either::Left((None, _)) => {
				break;
			}

			// Handle timer intervals.
			Either::Right((Either::Left((Some(_instant), stop)), next_rx)) => {
				if let Err(err) = send_ping(&mut ws_sender).await {
					tracing::debug!("WS transport error: send ping failed: {}", err);
					break;
				}

				rx_item = next_rx;
				futs = future::select(ping_interval.next(), stop);
			}

			Either::Right((Either::Left((None, _)), _)) => unreachable!("IntervalStream never terminates"),

			// Server is stopped.
			Either::Right((Either::Right(_), _)) => {
				break;
			}
		}
	}

	// Terminate connection and send close message.
	let _ = ws_sender.close().await;
	rx.close();
}

enum Receive<S> {
	ConnectionClosed,
	Stopped,
	Err(SokettoError, S),
	Ok(Vec<u8>, S),
}

/// Attempts to read data from WebSocket fails if the server was stopped.
async fn try_recv<T, S>(ws_stream: &mut T, mut stopped: S) -> Receive<S>
where
	S: Future<Output = ()> + Unpin,
	T: StreamExt<Item = Result<Incoming, SokettoError>> + Unpin,
{
	loop {
		match futures_util::future::select(ws_stream.next(), stopped).await {
			// The connection is closed.
			Either::Left((None, _)) => break Receive::ConnectionClosed,
			// The message has been received, we are done
			Either::Left((Some(Ok(Incoming::Data(d))), s)) => break Receive::Ok(d, s),
			// Got a pong response, update our "last seen" timestamp.
			Either::Left((Some(Ok(Incoming::Pong)), s)) => {
				tracing::debug!("Received pong");
				stopped = s;
			}
			// Received an error, terminate the connection.
			Either::Left((Some(Err(e)), s)) => break Receive::Err(e, s),
			// Server has been stopped.
			Either::Right(_) => break Receive::Stopped,
		}
	}
}

struct ExecuteCallParams<L: Logger> {
	batch_requests_config: BatchRequestConfig,
	conn_id: u32,
	id_provider: Arc<dyn IdProvider>,
	methods: Methods,
	max_response_body_size: u32,
	max_log_length: u32,
	sink: MethodSink,
	logger: L,
	bounded_subscriptions: BoundedSubscriptions,
}

async fn execute_unchecked_call<L: Logger>(
	params: Arc<ExecuteCallParams<L>>,
	data: Vec<u8>,
	drop_on_completion: mpsc::Sender<()>,
) {
	let request_start = params.logger.on_request(TransportProtocol::WebSocket);
	let first_non_whitespace = data.iter().enumerate().take(128).find(|(_, byte)| !byte.is_ascii_whitespace());

	let call_data = CallData {
		bounded_subscriptions: &params.bounded_subscriptions,
		conn_id: params.conn_id as usize,
		max_response_body_size: params.max_response_body_size,
		max_log_length: params.max_log_length,
		methods: &params.methods,
		sink: &params.sink,
		id_provider: &*params.id_provider,
		logger: &params.logger,
		request_start,
	};

	match first_non_whitespace {
		Some((start, b'{')) => {
			if let Some(rp) = process_single_request(&data[start..], call_data).await {
				match rp {
					CallOrSubscription::Subscription(r) => {
						params.logger.on_response(&r.result, request_start, TransportProtocol::WebSocket);
					}

					CallOrSubscription::Call(r) => {
						params.logger.on_response(&r.result, request_start, TransportProtocol::WebSocket);
						_ = params.sink.send(r.result).await;
					}
				}
			}
		}
		Some((start, b'[')) => {
			let limit = match params.batch_requests_config {
				BatchRequestConfig::Disabled => {
					let response = MethodResponse::error(
						Id::Null,
						ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG, None),
					);
					params.logger.on_response(&response.result, request_start, TransportProtocol::WebSocket);
					_ = params.sink.send(response.result).await;
					return;
				}
				BatchRequestConfig::Limit(limit) => limit as usize,
				BatchRequestConfig::Unlimited => usize::MAX,
			};

			let response = process_batch_request(Batch { data: &data[start..], call: call_data, max_len: limit }).await;

			if let Some(response) = response {
				tx_log_from_str(&response, params.max_log_length);
				params.logger.on_response(&response, request_start, TransportProtocol::WebSocket);
				_ = params.sink.send(response).await;
			}
		}
		_ => {
			_ = params.sink.send_error(Id::Null, ErrorCode::ParseError.into()).await;
		}
	};

	// NOTE: This channel is only used to indicate that a method call was completed
	// thus the drop here tells the main task that method call was completed.
	drop(drop_on_completion);
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Shutdown {
	Stopped,
	ConnectionClosed,
}

/// Enforce a graceful shutdown.
///
/// This will return once the connection has been terminated or all pending calls have been executed.
async fn graceful_shutdown<S>(
	result: Result<Shutdown, SokettoError>,
	pending_calls: mpsc::Receiver<()>,
	ws_stream: S,
	mut conn_tx: oneshot::Sender<()>,
	send_task_handle: tokio::task::JoinHandle<()>,
) where
	S: StreamExt<Item = Result<Incoming, SokettoError>> + Unpin,
{
	let pending_calls = ReceiverStream::new(pending_calls);

	if let Ok(Shutdown::Stopped) = result {
		let graceful_shutdown = pending_calls.for_each(|_| async {});
		let disconnect = ws_stream.try_for_each(|_| async { Ok(()) });

		tokio::select! {
			_ = graceful_shutdown => {}
			res = disconnect => {
				if let Err(err) = res {
					tracing::warn!("Graceful shutdown terminated because of error: `{err}`");
				}
			}
			_ = conn_tx.closed() => {}
		}
	}

	// Send a message to close down the "send task".
	_ = conn_tx.send(());
	// Ensure that send task has been closed.
	_ = send_task_handle.await;
}
