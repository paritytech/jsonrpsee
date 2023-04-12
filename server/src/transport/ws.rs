use std::time::Duration;

use crate::future::{FutureDriver, StopHandle};
use crate::logger::{self, Logger, TransportProtocol};
use crate::server::{BatchRequestConfig, ServiceData};

use futures_util::future::{self, Either};
use futures_util::io::{BufReader, BufWriter};
use futures_util::stream::FuturesOrdered;
use futures_util::{Future, FutureExt, StreamExt};
use hyper::upgrade::Upgraded;
use jsonrpsee_core::server::helpers::{
	batch_response_error, prepare_error, BatchResponseBuilder, MethodResponse, MethodSink,
};
use jsonrpsee_core::server::{
	BoundedSubscriptions, CallOrSubscription, MethodCallback, MethodSinkPermit, Methods, SubscriptionState,
};
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
	pub(crate) data: Vec<u8>,
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

	if let Ok(batch) = serde_json::from_slice::<Vec<&JsonRawValue>>(&data) {
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
	data: Vec<u8>,
	call: CallData<'_, L>,
) -> Option<CallOrSubscription> {
	if let Ok(req) = serde_json::from_slice::<Request>(&data) {
		Some(execute_call_with_tracing(req, call).await)
	} else if serde_json::from_slice::<Notif>(&data).is_ok() {
		None
	} else {
		let (id, code) = prepare_error(&data);
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

pub(crate) async fn background_task<L: Logger>(
	sender: Sender,
	mut receiver: Receiver,
	svc: ServiceData<L>,
) -> Result<(), Error> {
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

	// Spawn another task that sends out the responses on the Websocket.
	tokio::spawn(send_task(rx, sender, stop_handle.clone(), ping_interval, conn_rx));

	// Buffer for incoming data.
	let mut data = Vec::with_capacity(100);
	let mut method_executor = FutureDriver::default();
	let logger = &logger;
	let stopped = stop_handle.shutdown();

	tokio::pin!(stopped);

	let result = loop {
		data.clear();

		let sink_permit = match wait_for_permit(&sink, &mut method_executor, stopped).await {
			Some((p, s)) => {
				stopped = s;
				p
			}
			None => break Ok(()),
		};

		match try_recv(&mut receiver, &mut data, &mut method_executor, stopped).await {
			Receive::Shutdown => break Ok(()),
			Receive::Ok(stop) => {
				stopped = stop;
			}
			Receive::Err(err, stop) => {
				stopped = stop;

				match err {
					SokettoError::Closed => {
						tracing::debug!("WS transport: remote peer terminated the connection: {}", conn_id);
						break Ok(());
					}
					SokettoError::MessageTooLarge { current, maximum } => {
						tracing::debug!(
							"WS transport error: request length: {} exceeded max limit: {} bytes",
							current,
							maximum
						);
						sink_permit.send_error(Id::Null, reject_too_big_request(max_request_body_size));

						continue;
					}
					err => {
						tracing::debug!("WS transport error: {}; terminate connection: {}", err, conn_id);
						break Err(err.into());
					}
				};
			}
		}

		let request_start = logger.on_request(TransportProtocol::WebSocket);
		let first_non_whitespace = data.iter().find(|byte| !byte.is_ascii_whitespace());

		match first_non_whitespace {
			Some(b'{') => {
				let data = std::mem::take(&mut data);
				let sink = sink.clone();
				let methods = &methods;
				let id_provider = &*id_provider;
				let bounded_subscriptions = bounded_subscriptions.clone();

				let fut = async move {
					let call = CallData {
						conn_id: conn_id as usize,
						bounded_subscriptions,
						max_response_body_size,
						max_log_length,
						methods,
						sink: &sink,
						id_provider,
						logger,
						request_start,
					};

					if let Some(rp) = process_single_request(data, call).await {
						match rp {
							CallOrSubscription::Subscription(r) => {
								logger.on_response(&r.result, request_start, TransportProtocol::WebSocket);
							}

							CallOrSubscription::Call(r) => {
								logger.on_response(&r.result, request_start, TransportProtocol::WebSocket);
								sink_permit.send_raw(r.result);
							}
						}
					};
				}
				.boxed();

				method_executor.add(fut);
			}
			Some(b'[') => {
				let limit = match batch_requests_config {
					BatchRequestConfig::Disabled => {
						let response = MethodResponse::error(
							Id::Null,
							ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, &BATCHES_NOT_SUPPORTED_MSG, None),
						);
						logger.on_response(&response.result, request_start, TransportProtocol::WebSocket);
						sink_permit.send_raw(response.result);
						continue;
					}
					BatchRequestConfig::Limit(limit) => limit as usize,
					BatchRequestConfig::Unlimited => usize::MAX,
				};

				// Make sure the following variables are not moved into async closure below.
				let methods = &methods;
				let sink = sink.clone();
				let id_provider = id_provider.clone();
				let data = std::mem::take(&mut data);
				let bounded_subscriptions = bounded_subscriptions.clone();

				let fut = async move {
					let response = process_batch_request(Batch {
						data,
						call: CallData {
							conn_id: conn_id as usize,
							bounded_subscriptions,
							max_response_body_size,
							max_log_length,
							methods,
							sink: &sink,
							id_provider: &*id_provider,
							logger,
							request_start,
						},
						max_len: limit,
					})
					.await;

					if let Some(response) = response {
						tx_log_from_str(&response, max_log_length);
						logger.on_response(&response, request_start, TransportProtocol::WebSocket);
						sink_permit.send_raw(response);
					}
				};

				method_executor.add(Box::pin(fut));
			}
			_ => {
				sink_permit.send_error(Id::Null, ErrorCode::ParseError.into());
			}
		}
	};

	logger.on_disconnect(remote_addr, TransportProtocol::WebSocket);

	// Drive all running methods to completion.
	// **NOTE** Do not return early in this function. This `await` needs to run to guarantee
	// proper drop behaviour.
	method_executor.await;

	let _ = conn_tx.send(());
	drop(conn);

	result
}

/// A task that waits for new messages via the `rx channel` and sends them out on the `WebSocket`.
async fn send_task(
	rx: mpsc::Receiver<String>,
	mut ws_sender: Sender,
	stop_handle: StopHandle,
	ping_interval: Duration,
	conn_closed: oneshot::Receiver<()>,
) {
	// Interval to send out continuously `pings`.
	let ping_interval = IntervalStream::new(tokio::time::interval(ping_interval));
	let stopped = stop_handle.shutdown();
	let rx = ReceiverStream::new(rx);

	tokio::pin!(ping_interval, stopped, rx, conn_closed);

	// Received messages from the WebSocket.
	let mut rx_item = rx.next();
	let next_ping = ping_interval.next();
	let mut futs = future::select(next_ping, future::select(stopped, conn_closed));

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
			Either::Right((Either::Left((_, stop)), next_rx)) => {
				if let Err(err) = send_ping(&mut ws_sender).await {
					tracing::debug!("WS transport error: send ping failed: {}", err);
					break;
				}
				rx_item = next_rx;
				futs = future::select(ping_interval.next(), stop);
			}

			// Server is stopped or closed
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

async fn try_recv<F, S>(
	receiver: &mut Receiver,
	data: &mut Vec<u8>,
	method_executor: &mut FutureDriver<F>,
	stopped: S,
) -> Receive<S>
where
	F: Future + Unpin,
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

	let receive = method_executor.select_with(receive);
	tokio::pin!(receive);

	match futures_util::future::select(receive, stopped).await {
		Either::Left((Ok(_), s)) => Receive::Ok(s),
		Either::Left((Err(e), s)) => Receive::Err(e, s),
		Either::Right(_) => Receive::Shutdown,
	}
}

// Wait until there is a slot in the bounded channel which means that
// the underlying TCP socket won't be read.
//
// This will force the client to read socket on the other side
// otherwise the socket will not be read again.
//
// Fails if the connection was closed or if the server was stopped,
async fn wait_for_permit<'a, F, S>(
	sink: &'a MethodSink,
	method_executor: &mut FutureDriver<F>,
	stopped: S,
) -> Option<(MethodSinkPermit<'a>, S)>
where
	F: Future + Unpin,
	S: Future<Output = ()> + Unpin,
{
	let sink_permit_fut = method_executor.select_with(sink.reserve());
	tokio::pin!(sink_permit_fut);

	match futures_util::future::select(sink_permit_fut, stopped).await {
		Either::Left((Ok(permit), s)) => Some((permit, s)),
		// The sink or stopped were triggered, just terminate.
		_ => None,
	}
}
