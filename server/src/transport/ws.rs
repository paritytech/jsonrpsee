use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::future::{FutureDriver, StopHandle};
use crate::logger::{self, Logger, TransportProtocol};
use crate::server::{MethodResult, ServiceData};

use futures_channel::mpsc;
use futures_util::future::{self, Either};
use futures_util::io::{BufReader, BufWriter};
use futures_util::stream::FuturesOrdered;
use futures_util::{Future, FutureExt, StreamExt};
use hyper::upgrade::Upgraded;
use jsonrpsee_core::server::helpers::{
	prepare_error, BatchResponse, BatchResponseBuilder, BoundedSubscriptions, MethodResponse, MethodSink,
};
use jsonrpsee_core::server::resource_limiting::Resources;
use jsonrpsee_core::server::rpc_module::{ConnState, MethodKind, Methods};
use jsonrpsee_core::tracing::{rx_log_from_json, tx_log_from_str};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::{Error, JsonRawValue};
use jsonrpsee_types::error::{
	reject_too_big_request, reject_too_many_subscriptions, ErrorCode, BATCHES_NOT_SUPPORTED_CODE,
	BATCHES_NOT_SUPPORTED_MSG,
};
use jsonrpsee_types::{ErrorObject, Id, InvalidRequest, Notification, Params, Request};
use soketto::connection::Error as SokettoError;
use soketto::data::ByteSlice125;
use tokio_stream::wrappers::IntervalStream;
use tokio_util::compat::Compat;
use tracing::instrument;

pub(crate) type Sender = soketto::Sender<BufReader<BufWriter<Compat<Upgraded>>>>;
pub(crate) type Receiver = soketto::Receiver<BufReader<BufWriter<Compat<Upgraded>>>>;

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
}

#[derive(Debug, Clone)]
pub(crate) struct CallData<'a, L: Logger> {
	pub(crate) conn_id: usize,
	pub(crate) bounded_subscriptions: BoundedSubscriptions,
	pub(crate) id_provider: &'a dyn IdProvider,
	pub(crate) methods: &'a Methods,
	pub(crate) max_response_body_size: u32,
	pub(crate) max_log_length: u32,
	pub(crate) resources: &'a Resources,
	pub(crate) sink: &'a MethodSink,
	pub(crate) logger: &'a L,
	pub(crate) request_start: L::Instant,
}

/// This is a glorified select listening for new messages, while also checking the `stop_receiver` signal.
struct Monitored<'a, F> {
	future: F,
	stop_monitor: &'a StopHandle,
}

impl<'a, F> Monitored<'a, F> {
	fn new(future: F, stop_monitor: &'a StopHandle) -> Self {
		Monitored { future, stop_monitor }
	}
}

enum MonitoredError<E> {
	Shutdown,
	Selector(E),
}

impl<'a, 'f, F, T, E> Future for Monitored<'a, Pin<&'f mut F>>
where
	F: Future<Output = Result<T, E>>,
{
	type Output = Result<T, MonitoredError<E>>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let this = Pin::into_inner(self);

		if this.stop_monitor.shutdown_requested() {
			return Poll::Ready(Err(MonitoredError::Shutdown));
		}

		this.future.poll_unpin(cx).map_err(MonitoredError::Selector)
	}
}

// Batch responses must be sent back as a single message so we read the results from each
// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
// complete batch response back to the client over `tx`.
#[instrument(name = "batch", skip(b), level = "TRACE")]
pub(crate) async fn process_batch_request<L: Logger>(b: Batch<'_, L>) -> Option<BatchResponse> {
	let Batch { data, call } = b;

	if let Ok(batch) = serde_json::from_slice::<Vec<&JsonRawValue>>(&data) {
		let mut got_notif = false;
		let mut batch_response = BatchResponseBuilder::new_with_limit(call.max_response_body_size as usize);

		let mut pending_calls: FuturesOrdered<_> = batch
			.into_iter()
			.filter_map(|v| {
				if let Ok(req) = serde_json::from_str::<Request>(v.get()) {
					Some(Either::Right(async { execute_call(req, call.clone()).await.into_inner() }))
				} else if let Ok(_notif) = serde_json::from_str::<Notification<&JsonRawValue>>(v.get()) {
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
		Some(BatchResponse::error(Id::Null, ErrorObject::from(ErrorCode::ParseError)))
	}
}

pub(crate) async fn process_single_request<L: Logger>(data: Vec<u8>, call: CallData<'_, L>) -> MethodResult {
	if let Ok(req) = serde_json::from_slice::<Request>(&data) {
		execute_call_with_tracing(req, call).await
	} else {
		let (id, code) = prepare_error(&data);
		MethodResult::SendAndLogger(MethodResponse::error(id, ErrorObject::from(code)))
	}
}

#[instrument(name = "method_call", fields(method = req.method.as_ref()), skip(call, req), level = "TRACE")]
pub(crate) async fn execute_call_with_tracing<'a, L: Logger>(req: Request<'a>, call: CallData<'_, L>) -> MethodResult {
	execute_call(req, call).await
}

/// Execute a call which returns result of the call with a additional sink
/// to fire a signal once the subscription call has been answered.
///
/// Returns `(MethodResponse, None)` on every call that isn't a subscription
/// Otherwise `(MethodResponse, Some(PendingSubscriptionCallTx)`.
pub(crate) async fn execute_call<'a, L: Logger>(req: Request<'a>, call: CallData<'_, L>) -> MethodResult {
	let CallData {
		resources,
		methods,
		max_response_body_size,
		max_log_length,
		conn_id,
		bounded_subscriptions,
		id_provider,
		sink,
		logger,
		request_start,
	} = call;

	rx_log_from_json(&req, call.max_log_length);

	let params = Params::new(req.params.map(|params| params.get()));
	let name = &req.method;
	let id = req.id;

	let response = match methods.method_with_name(name) {
		None => {
			logger.on_call(name, params.clone(), logger::MethodKind::Unknown, TransportProtocol::WebSocket);
			let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::MethodNotFound));
			MethodResult::SendAndLogger(response)
		}
		Some((name, method)) => match &method.inner() {
			MethodKind::Sync(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::MethodCall, TransportProtocol::WebSocket);
				match method.claim(name, resources) {
					Ok(guard) => {
						let r = (callback)(id, params, max_response_body_size as usize);
						drop(guard);
						MethodResult::SendAndLogger(r)
					}
					Err(err) => {
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
						let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy));
						MethodResult::SendAndLogger(response)
					}
				}
			}
			MethodKind::Async(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::MethodCall, TransportProtocol::WebSocket);
				match method.claim(name, resources) {
					Ok(guard) => {
						let id = id.into_owned();
						let params = params.into_owned();

						let response =
							(callback)(id, params, conn_id, max_response_body_size as usize, Some(guard)).await;
						MethodResult::SendAndLogger(response)
					}
					Err(err) => {
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
						let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy));
						MethodResult::SendAndLogger(response)
					}
				}
			}
			MethodKind::Subscription(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::Subscription, TransportProtocol::WebSocket);
				match method.claim(name, resources) {
					Ok(guard) => {
						if let Some(cn) = bounded_subscriptions.acquire() {
							let conn_state = ConnState { conn_id, close_notify: cn, id_provider };
							let response = callback(id.clone(), params, sink.clone(), conn_state, Some(guard)).await;
							MethodResult::JustLogger(response)
						} else {
							let response =
								MethodResponse::error(id, reject_too_many_subscriptions(bounded_subscriptions.max()));
							MethodResult::SendAndLogger(response)
						}
					}
					Err(err) => {
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
						let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy));
						MethodResult::SendAndLogger(response)
					}
				}
			}
			MethodKind::Unsubscription(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::Unsubscription, TransportProtocol::WebSocket);

				// Don't adhere to any resource or subscription limits; always let unsubscribing happen!
				let result = callback(id, params, conn_id, max_response_body_size as usize);
				MethodResult::SendAndLogger(result)
			}
		},
	};

	let r = response.as_inner();

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
		resources,
		max_request_body_size,
		max_response_body_size,
		max_log_length,
		batch_requests_supported,
		stop_handle,
		id_provider,
		ping_interval,
		max_subscriptions_per_connection,
		conn_id,
		logger,
		remote_addr,
		conn,
		..
	} = svc;

	let (tx, rx) = mpsc::unbounded::<String>();
	let bounded_subscriptions = BoundedSubscriptions::new(max_subscriptions_per_connection);
	let sink = MethodSink::new_with_limit(tx, max_response_body_size, max_log_length);

	// Spawn another task that sends out the responses on the Websocket.
	tokio::spawn(send_task(rx, sender, stop_handle.clone(), ping_interval));

	// Buffer for incoming data.
	let mut data = Vec::with_capacity(100);
	let mut method_executors = FutureDriver::default();
	let logger = &logger;

	let result = loop {
		data.clear();

		{
			// Need the extra scope to drop this pinned future and reclaim access to `data`
			let receive = async {
				// Identical loop to `soketto::receive_data` with debug logs for `Pong` frames.
				loop {
					match receiver.receive(&mut data).await? {
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

			if let Err(err) = method_executors.select_with(Monitored::new(receive, &stop_handle)).await {
				match err {
					MonitoredError::Selector(SokettoError::Closed) => {
						tracing::debug!("WS transport: remote peer terminated the connection: {}", conn_id);
						break Ok(());
					}
					MonitoredError::Selector(SokettoError::MessageTooLarge { current, maximum }) => {
						tracing::warn!(
							"WS transport error: request length: {} exceeded max limit: {} bytes",
							current,
							maximum
						);
						sink.send_error(Id::Null, reject_too_big_request(max_request_body_size));
						continue;
					}

					// These errors can not be gracefully handled, so just log them and terminate the connection.
					MonitoredError::Selector(err) => {
						tracing::error!("WS transport error: {}; terminate connection: {}", err, conn_id);
						break Err(err.into());
					}
					MonitoredError::Shutdown => {
						break Ok(());
					}
				};
			};
		};

		let request_start = logger.on_request(TransportProtocol::WebSocket);

		let first_non_whitespace = data.iter().find(|byte| !byte.is_ascii_whitespace());
		match first_non_whitespace {
			Some(b'{') => {
				let data = std::mem::take(&mut data);
				let sink = sink.clone();
				let resources = &resources;
				let methods = &methods;
				let bounded_subscriptions = bounded_subscriptions.clone();
				let id_provider = &*id_provider;

				let fut = async move {
					let call = CallData {
						conn_id: conn_id as usize,
						resources,
						max_response_body_size,
						max_log_length,
						methods,
						bounded_subscriptions,
						sink: &sink,
						id_provider,
						logger,
						request_start,
					};

					match process_single_request(data, call).await {
						MethodResult::JustLogger(r) => {
							logger.on_response(&r.result, request_start, TransportProtocol::WebSocket);
						}
						MethodResult::SendAndLogger(r) => {
							logger.on_response(&r.result, request_start, TransportProtocol::WebSocket);
							let _ = sink.send_raw(r.result);
						}
					};
				}
				.boxed();

				method_executors.add(fut);
			}
			Some(b'[') if !batch_requests_supported => {
				let response = MethodResponse::error(
					Id::Null,
					ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, &BATCHES_NOT_SUPPORTED_MSG, None),
				);
				logger.on_response(&response.result, request_start, TransportProtocol::WebSocket);
				let _ = sink.send_raw(response.result);
			}
			Some(b'[') => {
				// Make sure the following variables are not moved into async closure below.
				let resources = &resources;
				let methods = &methods;
				let bounded_subscriptions = bounded_subscriptions.clone();
				let sink = sink.clone();
				let id_provider = id_provider.clone();
				let data = std::mem::take(&mut data);

				let fut = async move {
					let response = process_batch_request(Batch {
						data,
						call: CallData {
							conn_id: conn_id as usize,
							resources,
							max_response_body_size,
							max_log_length,
							methods,
							bounded_subscriptions,
							sink: &sink,
							id_provider: &*id_provider,
							logger,
							request_start,
						},
					})
					.await;

					if let Some(response) = response {
						tx_log_from_str(&response.result, max_log_length);
						logger.on_response(&response.result, request_start, TransportProtocol::WebSocket);
						let _ = sink.send_raw(response.result);
					}
				};

				method_executors.add(Box::pin(fut));
			}
			_ => {
				sink.send_error(Id::Null, ErrorCode::ParseError.into());
			}
		}
	};

	logger.on_disconnect(remote_addr, TransportProtocol::WebSocket);

	// Drive all running methods to completion.
	// **NOTE** Do not return early in this function. This `await` needs to run to guarantee
	// proper drop behaviour.
	method_executors.await;

	// Notify all listeners and close down associated tasks.
	sink.close();
	bounded_subscriptions.close();

	drop(conn);

	result
}

/// A task that waits for new messages via the `rx channel` and sends them out on the `WebSocket`.
async fn send_task(
	mut rx: mpsc::UnboundedReceiver<String>,
	mut ws_sender: Sender,
	mut stop_handle: StopHandle,
	ping_interval: Duration,
) {
	// Received messages from the WebSocket.
	let mut rx_item = rx.next();

	// Interval to send out continuously `pings`.
	let ping_interval = IntervalStream::new(tokio::time::interval(ping_interval));
	let stopped = stop_handle.shutdown();

	tokio::pin!(ping_interval, stopped);

	let next_ping = ping_interval.next();
	let mut futs = future::select(next_ping, stopped);

	loop {
		// Ensure select is cancel-safe by fetching and storing the `rx_item` that did not finish yet.
		// Note: Although, this is cancel-safe already, avoid using `select!` macro for future proofing.
		match future::select(rx_item, futs).await {
			// Received message.
			Either::Left((Some(response), not_ready)) => {
				// If websocket message send fail then terminate the connection.
				if let Err(err) = send_message(&mut ws_sender, response).await {
					tracing::error!("WS transport error: send failed: {}", err);
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
					tracing::error!("WS transport error: send ping failed: {}", err);
					break;
				}
				rx_item = next_rx;
				futs = future::select(ping_interval.next(), stop);
			}

			// Server is closed
			Either::Right((Either::Right((_, _)), _)) => {
				break;
			}
		}
	}

	// Terminate connection and send close message.
	let _ = ws_sender.close().await;
}
