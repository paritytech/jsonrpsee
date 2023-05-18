use std::sync::Arc;
use std::time::Duration;

use crate::logger::{self, Logger, TransportProtocol};
use crate::server::{BatchRequestConfig, ServiceData};

use futures_util::future::{self, Either};
use futures_util::io::{BufReader, BufWriter};
use futures_util::stream::{FuturesOrdered, FuturesUnordered};
use futures_util::{Future, StreamExt};
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
	pub(crate) bounded_subscriptions: BoundedSubscriptions,
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
	logger.on_result(name, r.success, request_start, TransportProtocol::WebSocket);
	response
}

pub(crate) async fn background_task<L: Logger>(sender: Sender, mut receiver: Receiver, svc: ServiceData<L>) {
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
	let pending_calls = FuturesUnordered::new();

	// Spawn another task that sends out the responses on the Websocket.
	let send_task_handle = tokio::spawn(send_task(rx, sender, ping_interval, conn_rx));

	// Buffer for incoming data.
	let mut data = Vec::with_capacity(100);
	let stopped = stop_handle.clone().shutdown();

	tokio::pin!(stopped);

	let result = loop {
		data.clear();

		// This is a guard to ensure that the underlying socket is only read if there is space in
		// the buffer for messages to be sent back to them.
		//
		// Thus, this check enforces that if the client can't keep up with receiving messages,
		// then no new messages will be read from them.
		//
		// TCP retransmission mechanism will take of the rest and adjust the window size accordingly.
		let Some(stop) = wait_until_connection_buffer_has_capacity(&sink, stopped).await else {
			break Ok(Shutdown::ConnectionClosed)
		};

		stopped = stop;

		match try_recv(&mut receiver, &mut data, stopped).await {
			Receive::Shutdown => break Ok(Shutdown::Stopped),
			Receive::Ok(stop) => {
				stopped = stop;
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

		let params = ExecuteCallParams {
			batch_requests_config,
			bounded_subscriptions: bounded_subscriptions.clone(),
			conn_id,
			methods: methods.clone(),
			max_log_length,
			max_response_body_size,
			sink: sink.clone(),
			id_provider: id_provider.clone(),
			logger: logger.clone(),
			data: std::mem::take(&mut data),
		};

		pending_calls.push(tokio::spawn(execute_unchecked_call(params)));
	};

	// Drive all running methods to completion.
	// **NOTE** Do not return early in this function. This `await` needs to run to guarantee
	// proper drop behaviour.
	graceful_shutdown(result, pending_calls, receiver, data, conn_tx, send_task_handle).await;

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
	Shutdown,
	Err(SokettoError, S),
	Ok(S),
}

// Wait until there is capacity in connection buffer to send one message.
//
// Fails if the server was stopped.
async fn wait_until_connection_buffer_has_capacity<S>(sink: &MethodSink, stopped: S) -> Option<S>
where
	S: Future<Output = ()> + Unpin,
{
	let reserve = sink.has_capacity();
	tokio::pin!(reserve);

	match futures_util::future::select(reserve, stopped).await {
		Either::Left((Ok(_), s)) => Some(s),
		_ => None,
	}
}

/// Attempts to read data from WebSocket fails if the server was stopped.
async fn try_recv<S>(receiver: &mut Receiver, data: &mut Vec<u8>, stopped: S) -> Receive<S>
where
	S: Future<Output = ()> + Unpin,
{
	let receive = async {
		// Identical loop to `soketto::receive_data` with debug logs for `Pong` frames.
		loop {
			match receiver.receive(data).await? {
				soketto::Incoming::Data(d) => break Ok(d),
				soketto::Incoming::Pong(_) => tracing::debug!("Received pong"),
				soketto::Incoming::Closed(_) => {
					// The closing reason is already logged by `soketto` trace log level.
					// Return the `Closed` error to avoid logging unnecessary warnings on clean shutdown.
					break Err(SokettoError::Closed);
				}
			}
		}
	};

	tokio::pin!(receive);

	match futures_util::future::select(receive, stopped).await {
		Either::Left((Ok(_), s)) => Receive::Ok(s),
		Either::Left((Err(e), s)) => Receive::Err(e, s),
		Either::Right(_) => Receive::Shutdown,
	}
}

struct ExecuteCallParams<L: Logger> {
	batch_requests_config: BatchRequestConfig,
	bounded_subscriptions: BoundedSubscriptions,
	conn_id: u32,
	data: Vec<u8>,
	id_provider: Arc<dyn IdProvider>,
	methods: Methods,
	max_response_body_size: u32,
	max_log_length: u32,
	sink: MethodSink,
	logger: L,
}

async fn execute_unchecked_call<L: Logger>(params: ExecuteCallParams<L>) {
	let ExecuteCallParams {
		batch_requests_config,
		conn_id,
		data,
		sink,
		max_response_body_size,
		max_log_length,
		methods,
		id_provider,
		bounded_subscriptions,
		logger,
	} = params;

	let request_start = logger.on_request(TransportProtocol::WebSocket);
	let first_non_whitespace = data.iter().enumerate().take(128).find(|(_, byte)| !byte.is_ascii_whitespace());

	match first_non_whitespace {
		Some((start, b'{')) => {
			let call_data = CallData {
				conn_id: conn_id as usize,
				bounded_subscriptions,
				max_response_body_size,
				max_log_length,
				methods: &methods,
				sink: &sink,
				id_provider: &*id_provider,
				logger: &logger,
				request_start,
			};

			if let Some(rp) = process_single_request(&data[start..], call_data).await {
				match rp {
					CallOrSubscription::Subscription(r) => {
						logger.on_response(&r.result, request_start, TransportProtocol::WebSocket);
					}

					CallOrSubscription::Call(r) => {
						logger.on_response(&r.result, request_start, TransportProtocol::WebSocket);
						_ = sink.send(r.result).await;
					}
				}
			}
		}
		Some((start, b'[')) => {
			let limit = match batch_requests_config {
				BatchRequestConfig::Disabled => {
					let response = MethodResponse::error(
						Id::Null,
						ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, &BATCHES_NOT_SUPPORTED_MSG, None),
					);
					logger.on_response(&response.result, request_start, TransportProtocol::WebSocket);
					_ = sink.send(response.result).await;
					return;
				}
				BatchRequestConfig::Limit(limit) => limit as usize,
				BatchRequestConfig::Unlimited => usize::MAX,
			};

			let call_data = CallData {
				conn_id: conn_id as usize,
				bounded_subscriptions,
				max_response_body_size,
				max_log_length,
				methods: &methods,
				sink: &sink,
				id_provider: &*id_provider,
				logger: &logger,
				request_start,
			};

			let response = process_batch_request(Batch { data: &data[start..], call: call_data, max_len: limit }).await;

			if let Some(response) = response {
				tx_log_from_str(&response, max_log_length);
				logger.on_response(&response, request_start, TransportProtocol::WebSocket);
				_ = sink.send(response).await;
			}
		}
		_ => {
			_ = sink.send_error(Id::Null, ErrorCode::ParseError.into()).await;
		}
	};
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Shutdown {
	Stopped,
	ConnectionClosed,
}

/// Enforce a graceful shutdown.
///
/// This will return once the connection has been terminated or all pending calls have been executed.
async fn graceful_shutdown<F: Future>(
	result: Result<Shutdown, SokettoError>,
	pending_calls: FuturesUnordered<F>,
	receiver: Receiver,
	data: Vec<u8>,
	mut conn_tx: oneshot::Sender<()>,
	send_task_handle: tokio::task::JoinHandle<()>,
) {
	match result {
		Ok(Shutdown::ConnectionClosed) | Err(SokettoError::Closed) => (),
		Ok(Shutdown::Stopped) | Err(_) => {
			// Soketto doesn't have a way to signal when the connection is closed
			// thus just throw away the data and terminate the stream once the connection has
			// been terminated.
			//
			// The receiver is not cancel-safe such that it's used in a stream to enforce that.
			let disconnect_stream = futures_util::stream::unfold((receiver, data), |(mut receiver, mut data)| async {
				if let Err(SokettoError::Closed) = receiver.receive(&mut data).await {
					None
				} else {
					Some(((), (receiver, data)))
				}
			});

			let graceful_shutdown = pending_calls.for_each(|_| async {});
			let disconnect = disconnect_stream.for_each(|_| async {});

			// All pending calls has been finished or the connection closed.
			// Fine to terminate
			tokio::select! {
				_ = graceful_shutdown => {}
				_ = disconnect => {}
				_ = conn_tx.closed() => {}
			}
		}
	};

	// Send a message to close down the "send task".
	_ = conn_tx.send(());
	// Ensure that send task has been closed.
	_ = send_task_handle.await;
}
