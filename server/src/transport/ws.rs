use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::middleware::{RpcService, RpcServiceT};
use crate::server::{BatchRequestConfig, ServiceData};
use crate::PingConfig;

use futures_util::future::{self, Either, Fuse};
use futures_util::io::{BufReader, BufWriter};
use futures_util::{Future, FutureExt, StreamExt};
use hyper::upgrade::Upgraded;
use jsonrpsee_core::server::helpers::{
	batch_response_error, prepare_error, BatchResponseBuilder, MethodResponse, MethodSink,
};
use jsonrpsee_core::server::BoundedSubscriptions;
use jsonrpsee_core::{Error, JsonRawValue};
use jsonrpsee_types::error::{
	reject_too_big_batch_request, reject_too_big_request, ErrorCode, BATCHES_NOT_SUPPORTED_CODE,
	BATCHES_NOT_SUPPORTED_MSG,
};
use jsonrpsee_types::{ErrorObject, Id, InvalidRequest, Notification, Request};
use soketto::connection::Error as SokettoError;
use soketto::data::ByteSlice125;

use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::{IntervalStream, ReceiverStream};
use tokio_util::compat::Compat;
use tower::Layer;
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

pub(crate) async fn background_task<RpcMiddleware>(
	sender: Sender,
	mut receiver: Receiver,
	svc: ServiceData,
	rpc_middleware: RpcMiddleware,
) where
	RpcMiddleware: for<'a> tower::Layer<RpcService> + Send + Sync + Clone + 'static,
	<RpcMiddleware as Layer<RpcService>>::Service: Send + Sync + 'static,
	for<'a> <RpcMiddleware as Layer<RpcService>>::Service: RpcServiceT<'a>,
{
	let ServiceData {
		methods,
		max_request_body_size,
		max_response_body_size,
		max_log_length,
		max_subscriptions_per_connection,
		batch_requests_config,
		stop_handle,
		id_provider,
		ping_config,
		conn_id,
		message_buffer_capacity,
		conn,
		..
	} = svc;

	let (tx, rx) = mpsc::channel::<String>(message_buffer_capacity as usize);
	let (conn_tx, conn_rx) = oneshot::channel();
	let sink = MethodSink::new_with_limit(tx, max_response_body_size, max_log_length);
	let bounded_subscriptions = BoundedSubscriptions::new(max_subscriptions_per_connection);

	// Spawn another task that sends out the responses on the Websocket.
	let send_task_handle = tokio::spawn(send_task(rx, sender, ping_config.ping_interval(), conn_rx));

	// Buffer for incoming data.
	let mut data = Vec::with_capacity(100);
	let stopped = stop_handle.clone().shutdown();

	// On each method call the `pending_calls` is cloned
	// then when all pending_calls are dropped
	// a graceful shutdown can has occur.
	let (pending_calls, pending_calls_completed) = mpsc::channel::<()>(1);

	let rpc_service = RpcService::full(
		methods,
		max_response_body_size as usize,
		bounded_subscriptions,
		sink.clone(),
		id_provider.clone(),
		conn_id as usize,
		pending_calls,
	);

	let rpc_service = Arc::new(rpc_middleware.layer(rpc_service));

	tokio::pin!(stopped);

	let result = loop {
		data.clear();

		match try_recv(&mut receiver, &mut data, stopped, ping_config).await {
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

		let rpc_service = rpc_service.clone();
		let data = std::mem::take(&mut data);

		let sink = sink.clone();
		tokio::spawn(async move {
			process_request(data, batch_requests_config, max_response_body_size, sink, rpc_service).await;
		});
	};

	drop(rpc_service);

	// Drive all running methods to completion.
	// **NOTE** Do not return early in this function. This `await` needs to run to guarantee
	// proper drop behaviour.
	graceful_shutdown(result, pending_calls_completed, receiver, data, conn_tx, send_task_handle).await;

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
			Either::Right((Either::Left((_instant, _stopped)), next_rx)) => {
				stop = _stopped;
				if let Err(err) = send_ping(&mut ws_sender).await {
					tracing::debug!("WS transport error: send ping failed: {}", err);
					break;
				}

				rx_item = next_rx;
				futs = future::select(ping_interval.next(), stop);
			}
			Either::Right((Either::Right((_stopped, _)), _)) => {
				// server has stopped
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

/// Attempts to read data from WebSocket fails if the server was stopped.
async fn try_recv<S>(receiver: &mut Receiver, data: &mut Vec<u8>, mut stopped: S, ping_config: PingConfig) -> Receive<S>
where
	S: Future<Output = ()> + Unpin,
{
	let mut last_active = Instant::now();

	let receive = futures_util::stream::unfold((receiver, data), |(receiver, data)| async {
		match receiver.receive(data).await {
			Ok(soketto::Incoming::Data(_)) => None,
			Ok(soketto::Incoming::Pong(_)) => Some((Ok(()), (receiver, data))),
			Ok(soketto::Incoming::Closed(_)) => Some((Err(SokettoError::Closed), (receiver, data))),
			// The closing reason is already logged by `soketto` trace log level.
			// Return the `Closed` error to avoid logging unnecessary warnings on clean shutdown.
			Err(e) => Some((Err(e), (receiver, data))),
		}
	});

	tokio::pin!(receive);

	let inactivity_check =
		Box::pin(ping_config.inactive_limit().map(|d| tokio::time::sleep(d).fuse()).unwrap_or_else(Fuse::terminated));
	let mut futs = futures_util::future::select(receive.next(), inactivity_check);

	loop {
		match futures_util::future::select(futs, stopped).await {
			// The message has been received, we are done
			Either::Left((Either::Left((None, _)), s)) => break Receive::Ok(s),
			// Got a pong response, update our "last seen" timestamp.
			Either::Left((Either::Left((Some(Ok(())), inactive)), s)) => {
				last_active = Instant::now();
				stopped = s;
				futs = futures_util::future::select(receive.next(), inactive);
			}
			// Received an error, terminate the connection.
			Either::Left((Either::Left((Some(Err(e)), _)), s)) => break Receive::Err(e, s),
			// Max inactivity timeout fired, check if the connection has been idle too long.
			Either::Left((Either::Right((_instant, rcv)), s)) => {
				let inactive_limit_exceeded =
					ping_config.inactive_limit().map_or(false, |duration| last_active.elapsed() > duration);

				if inactive_limit_exceeded {
					break Receive::Err(SokettoError::Closed, s);
				}

				stopped = s;
				// use really large duration instead of Duration::MAX to
				// solve the panic issue with interval initialization
				let inactivity_check = Box::pin(
					ping_config.inactive_limit().map(|d| tokio::time::sleep(d).fuse()).unwrap_or_else(Fuse::terminated),
				);
				futs = futures_util::future::select(rcv, inactivity_check);
			}
			// Server has been stopped.
			Either::Right(_) => break Receive::Shutdown,
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Shutdown {
	Stopped,
	ConnectionClosed,
}

/// Enforce a graceful shutdown.
///
/// This will return once the connection has been terminated or all pending calls have been executed.
async fn graceful_shutdown(
	result: Result<Shutdown, SokettoError>,
	pending_calls: mpsc::Receiver<()>,
	receiver: Receiver,
	data: Vec<u8>,
	mut conn_tx: oneshot::Sender<()>,
	send_task_handle: tokio::task::JoinHandle<()>,
) {
	let pending_calls = ReceiverStream::new(pending_calls);

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

pub(crate) async fn process_request<S>(
	data: Vec<u8>,
	batch_config: BatchRequestConfig,
	max_response_size: u32,
	sink: MethodSink,
	rpc_service: Arc<S>,
) where
	for<'a> S: RpcServiceT<'a>,
{
	let first_non_whitespace = data.iter().enumerate().take(128).find(|(_, byte)| !byte.is_ascii_whitespace());

	match first_non_whitespace {
		Some((start, b'{')) => {
			if let Ok(req) = serde_json::from_slice::<Request>(&data[start..]) {
				let rp = rpc_service.call(req).await;
				if !rp.is_subscription {
					let _ = sink.send(rp.result).await;
				}
			} else if serde_json::from_slice::<Notif>(&data[start..]).is_ok() {
				// just ignore.
			} else {
				let (id, code) = prepare_error(&data[start..]);
				let rp = MethodResponse::error(id, ErrorObject::from(code));
				if !rp.is_subscription {
					let _ = sink.send(rp.result).await;
				}
			}
		}
		Some((start, b'[')) => {
			let max_len = match batch_config {
				BatchRequestConfig::Disabled => {
					let response = MethodResponse::error(
						Id::Null,
						ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG, None),
					);
					_ = sink.send(response.result).await;
					return;
				}
				BatchRequestConfig::Limit(limit) => limit as usize,
				BatchRequestConfig::Unlimited => usize::MAX,
			};

			if let Ok(batch) = serde_json::from_slice::<Vec<&JsonRawValue>>(&data[start..]) {
				if batch.len() > max_len {
					_ = sink.send(batch_response_error(Id::Null, reject_too_big_batch_request(max_len))).await;
				}

				let mut got_notif = false;
				let mut batch_response = BatchResponseBuilder::new_with_limit(max_response_size as usize);

				for call in batch {
					if let Ok(req) = serde_json::from_str::<Request>(call.get()) {
						let rp = rpc_service.call(req).await;

						if let Err(too_large) = batch_response.append(&rp) {
							let _ = sink.send(too_large).await;
							return;
						}
					} else if let Ok(_notif) = serde_json::from_str::<Notif>(call.get()) {
						// notifications should not be answered.
						got_notif = true;
					} else {
						// valid JSON but could be not parsable as `InvalidRequest`
						let id = match serde_json::from_str::<InvalidRequest>(call.get()) {
							Ok(err) => err.id,
							Err(_) => Id::Null,
						};

						let rp = MethodResponse::error(id, ErrorObject::from(ErrorCode::InvalidRequest));

						if let Err(too_large) = batch_response.append(&rp) {
							let _ = sink.send(too_large).await;
							return;
						}
					}
				}

				if got_notif && batch_response.is_empty() {
					_ = sink.send(String::new()).await;
				} else {
					_ = sink.send(batch_response.finish()).await;
				}
			} else {
				_ = sink.send(batch_response_error(Id::Null, ErrorObject::from(ErrorCode::ParseError))).await;
			}
		}
		_ => {
			_ = sink.send_error(Id::Null, ErrorCode::ParseError.into()).await;
		}
	};
}
