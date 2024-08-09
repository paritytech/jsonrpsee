use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::future::{IntervalStream, SessionClose};
use crate::middleware::rpc::{RpcService, RpcServiceBuilder, RpcServiceCfg, RpcServiceT};
use crate::server::{handle_rpc_call, ConnectionState, ServerConfig};
use crate::utils::PendingPings;
use crate::{HttpBody, HttpRequest, HttpResponse, PingConfig, LOG_TARGET};

use futures_util::future::{self, Either, Fuse};
use futures_util::io::{BufReader, BufWriter};
use futures_util::{Future, FutureExt, StreamExt, TryStreamExt};
use hyper::upgrade::Upgraded;
use hyper_util::rt::TokioIo;
use jsonrpsee_core::server::{BoundedSubscriptions, MethodSink, Methods};
use jsonrpsee_types::error::{reject_too_big_request, ErrorCode};
use jsonrpsee_types::Id;
use soketto::connection::Error as SokettoError;
use soketto::data::ByteSlice125;

use tokio::sync::{mpsc, oneshot};
use tokio::time::interval;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

pub(crate) type Sender = soketto::Sender<BufReader<BufWriter<Compat<TokioIo<Upgraded>>>>>;
pub(crate) type Receiver = soketto::Receiver<BufReader<BufWriter<Compat<TokioIo<Upgraded>>>>>;

pub use soketto::handshake::http::is_upgrade_request;

enum Incoming {
	Data(Vec<u8>),
	Pong,
}

pub(crate) async fn send_message(sender: &mut Sender, response: String) -> Result<(), SokettoError> {
	sender.send_text_owned(response).await?;
	sender.flush().await.map_err(Into::into)
}

pub(crate) async fn send_ping(sender: &mut Sender) -> Result<(), SokettoError> {
	// Submit empty slice as "optional" parameter.
	let slice: &[u8] = &[];
	// Byte slice fails if the provided slice is larger than 125 bytes.
	let byte_slice = ByteSlice125::try_from(slice).expect("Empty slice should fit into ByteSlice125");
	sender.send_ping(byte_slice).await?;
	sender.flush().await.map_err(Into::into)
}

pub(crate) struct BackgroundTaskParams<S> {
	pub(crate) server_cfg: ServerConfig,
	pub(crate) conn: ConnectionState,
	pub(crate) ws_sender: Sender,
	pub(crate) ws_receiver: Receiver,
	pub(crate) rpc_service: S,
	pub(crate) sink: MethodSink,
	pub(crate) rx: mpsc::Receiver<String>,
	pub(crate) pending_calls_completed: mpsc::Receiver<()>,
	pub(crate) on_session_close: Option<SessionClose>,
	pub(crate) extensions: http::Extensions,
}

pub(crate) async fn background_task<S>(params: BackgroundTaskParams<S>)
where
	for<'a> S: RpcServiceT<'a> + Send + Sync + 'static,
{
	let BackgroundTaskParams {
		server_cfg,
		conn,
		ws_sender,
		ws_receiver,
		rpc_service,
		sink,
		rx,
		pending_calls_completed,
		mut on_session_close,
		extensions,
	} = params;

	let conn_id = conn.conn_id;
	let ServerConfig { ping_config, batch_requests_config, max_request_body_size, max_response_body_size, .. } =
		server_cfg;
	let (conn_tx, conn_rx) = oneshot::channel();

	// Spawn ping/pong task if ping config is provided.
	let ping_config = if let Some(ping_config) = ping_config {
		let (ping_tx, ping_rx) = mpsc::channel::<KeepAlive>(4);
		tokio::spawn(ping_pong_task(ping_rx, ping_config.inactive_limit, ping_config.max_failures, conn_id));
		Some((ping_config, ping_tx))
	} else {
		None
	};

	let ping_tx = ping_config.as_ref().map(|(_, tx)| tx.clone());

	// Spawn another task that sends out the responses on the Websocket.
	let send_task_handle = tokio::spawn(send_task(rx, ws_sender, ping_config, conn_rx));

	let stopped = conn.stop_handle.clone().shutdown();
	let rpc_service = Arc::new(rpc_service);

	tokio::pin!(stopped);

	let ws_stream = futures_util::stream::unfold(ws_receiver, |mut receiver| async {
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
		let data = match try_recv(&mut ws_stream, stopped, ping_tx.as_ref()).await {
			Receive::ConnectionClosed => break Ok(Shutdown::ConnectionClosed),
			Receive::KeepAliveExpired => break Ok(Shutdown::KeepAliveExpired),
			Receive::Stopped => break Ok(Shutdown::Stopped),
			Receive::Ok(data, stop) => {
				stopped = stop;
				data
			}
			Receive::Err(err, stop) => {
				stopped = stop;

				match err {
					SokettoError::Closed => {
						break Ok(Shutdown::ConnectionClosed);
					}
					SokettoError::MessageTooLarge { current, maximum } => {
						tracing::debug!(
							target: LOG_TARGET,
							"WS recv error: message too large current={}/max={}",
							current,
							maximum
						);
						if sink.send_error(Id::Null, reject_too_big_request(max_request_body_size)).await.is_err() {
							break Ok(Shutdown::ConnectionClosed);
						}

						continue;
					}
					err => {
						break Err(err);
					}
				};
			}
		};

		let rpc_service = rpc_service.clone();
		let sink = sink.clone();
		let extensions = extensions.clone();

		tokio::spawn(async move {
			let first_non_whitespace = data.iter().enumerate().take(128).find(|(_, byte)| !byte.is_ascii_whitespace());

			let (idx, is_single) = match first_non_whitespace {
				Some((start, b'{')) => (start, true),
				Some((start, b'[')) => (start, false),
				_ => {
					_ = sink.send_error(Id::Null, ErrorCode::ParseError.into()).await;
					return;
				}
			};

			if let Some(rp) = handle_rpc_call(
				&data[idx..],
				is_single,
				batch_requests_config,
				max_response_body_size,
				&*rpc_service,
				extensions,
			)
			.await
			{
				if !rp.is_subscription() {
					let is_success = rp.is_success();
					let (serialized_rp, mut on_close) = rp.into_parts();

					// The connection is closed, just quit.
					if sink.send(serialized_rp).await.is_err() {
						return;
					}

					// Notify that the message has been sent out to the internal
					// WebSocket buffer.
					if let Some(n) = on_close.take() {
						n.notify(is_success);
					}
				}
			}
		});
	};

	tracing::debug!(target: LOG_TARGET, "Connection closed for conn_id={conn_id}, reason={:?}", result);

	// Drive all running methods to completion.
	// **NOTE** Do not return early in this function. This `await` needs to run to guarantee
	// proper drop behaviour.
	drop(rpc_service);
	graceful_shutdown(result, pending_calls_completed, ws_stream, conn_tx, send_task_handle).await;

	drop(conn);

	if let Some(c) = on_session_close.take() {
		c.close();
	}
}

/// A task that waits for new messages via the `rx channel` and sends them out on the `WebSocket`.
async fn send_task(
	rx: mpsc::Receiver<String>,
	mut ws_sender: Sender,
	ping_config: Option<(PingConfig, mpsc::Sender<KeepAlive>)>,
	stop: oneshot::Receiver<()>,
) {
	// Ping task is only spawned if ping config is provided.
	let ping = match ping_config {
		None => Either::Left(IntervalStream::pending().map(|_| None)),
		Some((p, ping_tx)) => {
			Either::Right(IntervalStream::new(interval(p.ping_interval)).map(move |_| Some(ping_tx.clone())))
		}
	};
	let rx = ReceiverStream::new(rx);

	tokio::pin!(ping, rx, stop);

	// Received messages from the WebSocket.
	let mut rx_item = rx.next();
	let next_ping = ping.next();
	let mut futs = future::select(next_ping, stop);

	loop {
		// Ensure select is cancel-safe by fetching and storing the `rx_item` that did not finish yet.
		// Note: Although, this is cancel-safe already, avoid using `select!` macro for future proofing.
		match future::select(rx_item, futs).await {
			// Received message.
			Either::Left((Some(response), not_ready)) => {
				// If websocket message send fail then terminate the connection.
				if let Err(err) = send_message(&mut ws_sender, response).await {
					tracing::debug!(target: LOG_TARGET, "WS send error: {}", err);
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
			Either::Right((Either::Left((Some(ping_tx), _stopped)), next_rx)) => {
				stop = _stopped;
				if let Err(err) = send_ping(&mut ws_sender).await {
					tracing::debug!(target: LOG_TARGET, "WS send ping error: {}", err);
					break;
				}

				rx_item = next_rx;

				let ping_tx = ping_tx.expect("ping tx is only `None` if ping_config is `None` checked above; qed");
				tokio::spawn(async move {
					ping_tx.send(KeepAlive::Ping(Instant::now())).await.ok();
				});

				futs = future::select(ping.next(), stop);
			}

			// The interval stream has been closed.
			// This should be unreachable because the interval stream never ends.
			Either::Right((Either::Left((None, _stopped)), _)) => {
				break;
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
	ConnectionClosed,
	KeepAliveExpired,
	Stopped,
	Err(SokettoError, S),
	Ok(Vec<u8>, S),
}

/// Attempts to read data from WebSocket fails if the server was stopped.
async fn try_recv<T, S>(ws_stream: &mut T, stopped: S, ping_tx: Option<&mpsc::Sender<KeepAlive>>) -> Receive<S>
where
	S: Future<Output = ()> + Unpin,
	T: StreamExt<Item = Result<Incoming, SokettoError>> + Unpin,
{
	let mut futs = future::select(ws_stream.next(), stopped);
	let closed = match ping_tx {
		Some(ping_tx) => ping_tx.closed().fuse(),
		None => Fuse::terminated(),
	};

	tokio::pin!(closed);

	loop {
		match future::select(futs, closed).await {
			// The connection is closed.
			Either::Left((Either::Left((None, _)), _)) => break Receive::ConnectionClosed,
			// The message has been received, we are done
			Either::Left((Either::Left((Some(Ok(Incoming::Data(d))), s)), _)) => {
				if let Some(ping_tx) = ping_tx {
					let ping_tx = ping_tx.clone();
					tokio::spawn(async move {
						_ = ping_tx.send(KeepAlive::Data(Instant::now())).await;
					});
				}

				break Receive::Ok(d, s);
			}
			// Got a pong response send status to the ping_pong_task.
			Either::Left((Either::Left((Some(Ok(Incoming::Pong)), s)), c)) => {
				if let Some(ping_tx) = ping_tx {
					let ping_tx = ping_tx.clone();
					tokio::spawn(async move {
						_ = ping_tx.send(KeepAlive::Pong(Instant::now())).await;
					});
				}
				futs = futures_util::future::select(ws_stream.next(), s);
				closed = c;
			}
			// Received an error, terminate the connection.
			Either::Left((Either::Left((Some(Err(e)), s)), _)) => break Receive::Err(e, s),

			// Server has been stopped or closed by inactive peer.
			Either::Left((Either::Right((_, _)), _)) => break Receive::Stopped,
			// Ping task has been stopped.
			Either::Right((_, _)) => break Receive::KeepAliveExpired,
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum KeepAlive {
	Ping(Instant),
	Data(Instant),
	Pong(Instant),
}

async fn ping_pong_task(
	mut rx: mpsc::Receiver<KeepAlive>,
	max_inactivity_dur: Duration,
	max_missed_pings: usize,
	conn_id: u32,
) {
	let mut polling_interval = IntervalStream::new(interval(max_inactivity_dur));
	let mut pending_pings = PendingPings::new(max_missed_pings, max_inactivity_dur, conn_id);

	loop {
		tokio::select! {
			// If the ping is never answered, we use this timer as a fallback.
			_ = polling_interval.next() => {
				if !pending_pings.check_alive() {
					break;
				}
			}
			// Data on the connection.
			msg = rx.recv() => {
				match msg {
					Some(KeepAlive::Ping(start)) => {
						pending_pings.push(start);
					}
					Some(KeepAlive::Pong(end)) | Some(KeepAlive::Data(end)) => {
						if !pending_pings.alive_response(end) {
							break;
						}
					}
					None => break,
				}
			}
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum Shutdown {
	Stopped,
	ConnectionClosed,
	KeepAliveExpired,
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
					tracing::warn!(target: LOG_TARGET, "Graceful shutdown terminated because of error: `{err}`");
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

/// Low-level API that tries to upgrade the HTTP connection to a WebSocket connection.
///
/// Returns `Ok((http_response, conn_fut))` if the WebSocket connection was successfully established
/// otherwise `Err(http_response)`.
///
/// `conn_fut` is a future that drives the WebSocket connection
/// and if it's dropped the connection will be closed.
///
/// Because this API depends on [`hyper`] the response needs to be sent
/// to complete the HTTP request.
///
/// ```no_run
/// use jsonrpsee_server::{ws, ServerConfig, Methods, ConnectionState, HttpRequest, HttpResponse};
/// use jsonrpsee_server::middleware::rpc::{RpcServiceBuilder, RpcServiceT, RpcService};
///
/// async fn handle_websocket_conn<L>(
///     req: HttpRequest,
///     server_cfg: ServerConfig,
///     methods: impl Into<Methods> + 'static,
///     conn: ConnectionState,
///     rpc_middleware: RpcServiceBuilder<L>,
///     mut disconnect: tokio::sync::mpsc::Receiver<()>
/// ) -> HttpResponse
/// where
///     L: for<'a> tower::Layer<RpcService> + 'static,
///     <L as tower::Layer<RpcService>>::Service: Send + Sync + 'static,
///     for<'a> <L as tower::Layer<RpcService>>::Service: RpcServiceT<'a> + 'static,
/// {
///   match ws::connect(req, server_cfg, methods, conn, rpc_middleware).await {
///     Ok((rp, conn_fut)) => {
///         tokio::spawn(async move {
///             // Keep the connection alive until
///             // a close signal is sent.
///             tokio::select! {
///                 _ = conn_fut => (),
///                 _ = disconnect.recv() => (),
///             }
///         });
///         rp
///     }
///     Err(rp) => rp,
///   }
/// }
/// ```
pub async fn connect<L, B>(
	req: HttpRequest<B>,
	server_cfg: ServerConfig,
	methods: impl Into<Methods>,
	conn: ConnectionState,
	rpc_middleware: RpcServiceBuilder<L>,
) -> Result<(HttpResponse, impl Future<Output = ()>), HttpResponse>
where
	L: for<'a> tower::Layer<RpcService>,
	<L as tower::Layer<RpcService>>::Service: Send + Sync + 'static,
	for<'a> <L as tower::Layer<RpcService>>::Service: RpcServiceT<'a>,
{
	let mut server = soketto::handshake::http::Server::new();

	match server.receive_request(&req) {
		Ok(response) => {
			let (tx, rx) = mpsc::channel::<String>(server_cfg.message_buffer_capacity as usize);
			let sink = MethodSink::new(tx);

			// On each method call the `pending_calls` is cloned
			// then when all pending_calls are dropped
			// a graceful shutdown can has occur.
			let (pending_calls, pending_calls_completed) = mpsc::channel::<()>(1);

			let rpc_service_cfg = RpcServiceCfg::CallsAndSubscriptions {
				bounded_subscriptions: BoundedSubscriptions::new(server_cfg.max_subscriptions_per_connection),
				id_provider: server_cfg.id_provider.clone(),
				sink: sink.clone(),
				_pending_calls: pending_calls,
			};

			let rpc_service = RpcService::new(
				methods.into(),
				server_cfg.max_response_body_size as usize,
				conn.conn_id.into(),
				rpc_service_cfg,
			);

			let rpc_service = rpc_middleware.service(rpc_service);

			// Note: This can't possibly be fulfilled until the HTTP response
			// is returned below, so that's why it's a separate async block
			let fut = async move {
				let extensions = req.extensions().clone();

				let upgraded = match hyper::upgrade::on(req).await {
					Ok(upgraded) => upgraded,
					Err(e) => {
						tracing::debug!(target: LOG_TARGET, "WS upgrade handshake failed: {}", e);
						return;
					}
				};

				let io = TokioIo::new(upgraded);

				let stream = BufReader::new(BufWriter::new(io.compat()));
				let mut ws_builder = server.into_builder(stream);
				ws_builder.set_max_message_size(server_cfg.max_response_body_size as usize);
				let (sender, receiver) = ws_builder.finish();

				let params = BackgroundTaskParams {
					server_cfg,
					conn,
					ws_sender: sender,
					ws_receiver: receiver,
					rpc_service,
					sink,
					rx,
					pending_calls_completed,
					on_session_close: None,
					extensions,
				};

				background_task(params).await;
			};

			Ok((response.map(|()| HttpBody::default()), fut))
		}
		Err(e) => {
			tracing::debug!(target: LOG_TARGET, "WS upgrade handshake failed: {}", e);
			Err(HttpResponse::new(HttpBody::from(format!("WS upgrade handshake failed: {e}"))))
		}
	}
}
