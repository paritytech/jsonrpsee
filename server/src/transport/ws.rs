use std::sync::Arc;
use std::time::Instant;

use crate::future::{IntervalStream, SessionClose};
use crate::middleware::rpc::{RpcService, RpcServiceCfg};
use crate::server::{ConnectionState, ServerConfig, handle_rpc_call};
use crate::{HttpBody, HttpRequest, HttpResponse, LOG_TARGET, PingConfig};

use futures_util::future::{self, Either};
use futures_util::{Future, SinkExt, StreamExt};
use jsonrpsee_core::middleware::{RpcServiceBuilder, RpcServiceT};
use jsonrpsee_core::server::{BoundedSubscriptions, MethodResponse, MethodSink, Methods};
use jsonrpsee_types::Id;
use jsonrpsee_types::error::ErrorCode;
use serde_json::value::RawValue;
use tokio::sync::{mpsc, oneshot};
use tokio::time::{interval, interval_at};
use tokio_stream::wrappers::ReceiverStream;
use yawc::frame::{Frame, OpCode};
use yawc::{HttpStream, WebSocket};

pub(crate) type Sender = futures_util::stream::SplitSink<WebSocket<HttpStream>, Frame>;
pub(crate) type Receiver = futures_util::stream::SplitStream<WebSocket<HttpStream>>;

/// Checks whether the incoming request is a WebSocket upgrade request.
pub fn is_upgrade_request<B>(req: &http::Request<B>) -> bool {
	let dominated_upgrade = req
		.headers()
		.get(http::header::UPGRADE)
		.and_then(|v| v.to_str().ok())
		.map(|v| v.eq_ignore_ascii_case("websocket"))
		.unwrap_or(false);
	let has_connection_upgrade = req
		.headers()
		.get(http::header::CONNECTION)
		.and_then(|v| v.to_str().ok())
		.map(|v| v.to_lowercase().contains("upgrade"))
		.unwrap_or(false);
	dominated_upgrade && has_connection_upgrade
}

enum Incoming {
	Data(Vec<u8>),
	Pong,
}

pub(crate) async fn send_message(sender: &mut Sender, response: Box<RawValue>) -> Result<(), yawc::WebSocketError> {
	sender.send(Frame::text(String::from(Box::<str>::from(response)))).await
}

pub(crate) async fn send_ping(sender: &mut Sender) -> Result<(), yawc::WebSocketError> {
	tracing::debug!(target: LOG_TARGET, "Send ping");
	sender.send(Frame::ping(b"" as &[u8])).await
}

pub(crate) struct BackgroundTaskParams<S> {
	pub(crate) server_cfg: ServerConfig,
	pub(crate) conn: ConnectionState,
	pub(crate) ws_sender: Sender,
	pub(crate) ws_receiver: Receiver,
	pub(crate) rpc_service: S,
	pub(crate) sink: MethodSink,
	pub(crate) rx: mpsc::Receiver<Box<RawValue>>,
	pub(crate) pending_calls_completed: mpsc::Receiver<()>,
	pub(crate) on_session_close: Option<SessionClose>,
	pub(crate) extensions: http::Extensions,
}

pub(crate) async fn background_task<S>(params: BackgroundTaskParams<S>)
where
	S: RpcServiceT<
			MethodResponse = MethodResponse,
			BatchResponse = MethodResponse,
			NotificationResponse = MethodResponse,
		> + Send
		+ Sync
		+ 'static,
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
	let ServerConfig { ping_config, batch_requests_config, .. } = server_cfg;

	let (conn_tx, conn_rx) = oneshot::channel();

	// Spawn another task that sends out the responses on the Websocket.
	let send_task_handle = tokio::spawn(send_task(rx, ws_sender, ping_config, conn_rx));

	let stopped = conn.stop_handle.clone().shutdown();
	let rpc_service = Arc::new(rpc_service);
	let mut missed_pings = 0;

	tokio::pin!(stopped);

	// Convert the yawc Stream<Item = Frame> into Stream<Item = Result<Incoming, ()>>
	let ws_stream = ws_receiver
		.filter_map(|frame| async move {
			match frame.opcode() {
				OpCode::Text | OpCode::Binary => Some(Ok(Incoming::Data(frame.into_payload().to_vec()))),
				OpCode::Pong => Some(Ok(Incoming::Pong)),
				OpCode::Close => None,
				OpCode::Ping => None, // auto-ponged by yawc
				_ => None,
			}
		})
		.fuse();

	tokio::pin!(ws_stream);

	let result = loop {
		let data = match try_recv(&mut ws_stream, stopped, ping_config, &mut missed_pings).await {
			Receive::ConnectionClosed => break Ok(Shutdown::ConnectionClosed),
			Receive::Stopped => break Ok(Shutdown::Stopped),
			Receive::Ok(data, stop) => {
				stopped = stop;
				data
			}
			Receive::Err(_stop) => {
				break Ok(Shutdown::ConnectionClosed);
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

			let rp = handle_rpc_call(&data[idx..], is_single, batch_requests_config, &*rpc_service, extensions).await;

			// Subscriptions are handled by the subscription callback and
			// "ordinary notifications" should not be sent back to the client.
			if rp.is_method_call() || rp.is_batch() {
				let is_success = rp.is_success();
				let (json, mut on_close, _) = rp.into_parts();

				// The connection is closed, just quit.
				if sink.send(json).await.is_err() {
					return;
				}

				// Notify that the message has been sent out to the internal
				// WebSocket buffer.
				if let Some(n) = on_close.take() {
					n.notify(is_success);
				}
			}
		});
	};

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
	rx: mpsc::Receiver<Box<RawValue>>,
	mut ws_sender: Sender,
	ping_config: Option<PingConfig>,
	stop: oneshot::Receiver<()>,
) {
	let ping_interval = match ping_config {
		None => IntervalStream::pending(),
		// NOTE: we are emitted a tick here immediately to sync
		// with how the receive task work because it starts measuring the pong
		// when it starts up.
		Some(p) => IntervalStream::new(interval(p.ping_interval)),
	};
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
			Either::Right((Either::Left((_instant, _stopped)), next_rx)) => {
				stop = _stopped;
				if let Err(err) = send_ping(&mut ws_sender).await {
					tracing::debug!(target: LOG_TARGET, "WS send ping error: {}", err);
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
	ConnectionClosed,
	Stopped,
	Err(S),
	Ok(Vec<u8>, S),
}

/// Attempts to read data from WebSocket fails if the server was stopped.
async fn try_recv<T, S>(
	ws_stream: &mut T,
	mut stopped: S,
	ping_config: Option<PingConfig>,
	missed_pings: &mut usize,
) -> Receive<S>
where
	S: Future<Output = ()> + Unpin,
	T: StreamExt<Item = Result<Incoming, ()>> + Unpin,
{
	let mut last_active = Instant::now();
	let inactivity_check = match ping_config {
		Some(p) => IntervalStream::new(interval_at(tokio::time::Instant::now() + p.ping_interval, p.ping_interval)),
		None => IntervalStream::pending(),
	};

	tokio::pin!(inactivity_check);

	let mut futs = futures_util::future::select(ws_stream.next(), inactivity_check.next());

	loop {
		match futures_util::future::select(futs, stopped).await {
			// The connection is closed.
			Either::Left((Either::Left((None, _)), _)) => break Receive::ConnectionClosed,
			// The message has been received, we are done
			Either::Left((Either::Left((Some(Ok(Incoming::Data(d))), _)), s)) => break Receive::Ok(d, s),
			// Got a pong response, update our "last seen" timestamp.
			Either::Left((Either::Left((Some(Ok(Incoming::Pong)), inactive)), s)) => {
				last_active = Instant::now();
				stopped = s;
				futs = futures_util::future::select(ws_stream.next(), inactive);
			}
			// Received an error, terminate the connection.
			Either::Left((Either::Left((Some(Err(_)), _)), s)) => break Receive::Err(s),
			// Max inactivity timeout fired, check if the connection has been idle too long.
			Either::Left((Either::Right((_instant, rcv)), s)) => {
				if let Some(p) = ping_config {
					if last_active.elapsed() > p.inactive_limit {
						*missed_pings += 1;

						if *missed_pings >= p.max_failures {
							tracing::debug!(
								target: LOG_TARGET,
								"WS ping/pong inactivity limit `{}` exceeded; closing connection",
								p.max_failures,
							);
							break Receive::ConnectionClosed;
						}
					}
				}

				stopped = s;
				futs = futures_util::future::select(rcv, inactivity_check.next());
			}
			// Server has been stopped.
			Either::Right(_) => break Receive::Stopped,
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
async fn graceful_shutdown<S>(
	result: Result<Shutdown, ()>,
	pending_calls: mpsc::Receiver<()>,
	ws_stream: S,
	mut conn_tx: oneshot::Sender<()>,
	send_task_handle: tokio::task::JoinHandle<()>,
) where
	S: StreamExt + Unpin,
{
	let pending_calls = ReceiverStream::new(pending_calls);

	if let Ok(Shutdown::Stopped) = result {
		let graceful_shutdown = pending_calls.for_each(|_| async {});
		let disconnect = ws_stream.for_each(|_| async {});

		tokio::select! {
			_ = graceful_shutdown => {}
			_ = disconnect => {}
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
/// use jsonrpsee_server::middleware::rpc::{RpcServiceBuilder, RpcServiceT, RpcService, MethodResponse};
/// use std::convert::Infallible;
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
///     L: tower::Layer<RpcService> + 'static,
///     <L as tower::Layer<RpcService>>::Service: RpcServiceT<MethodResponse = MethodResponse, BatchResponse = MethodResponse, NotificationResponse = MethodResponse> + Send + Sync + 'static,
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
	L: tower::Layer<RpcService>,
	<L as tower::Layer<RpcService>>::Service: RpcServiceT<
			MethodResponse = MethodResponse,
			BatchResponse = MethodResponse,
			NotificationResponse = MethodResponse,
		> + Send
		+ Sync
		+ 'static,
{
	let mut request = req.map(|_| http_body_util::Empty::<hyper::body::Bytes>::new());

	let options = server_cfg
		.ws_options
		.clone()
		.unwrap_or_else(|| yawc::Options::default().with_balanced_compression())
		.with_max_payload_read(server_cfg.max_request_body_size as usize);

	match WebSocket::upgrade_with_options(&mut request, options) {
		Ok((response, upgrade_fut)) => {
			let (tx, rx) = mpsc::channel(server_cfg.message_buffer_capacity as usize);
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
				let extensions = request.extensions().clone();

				let ws = match upgrade_fut.await {
					Ok(ws) => ws,
					Err(e) => {
						tracing::debug!(target: LOG_TARGET, "WS upgrade handshake failed: {}", e);
						return;
					}
				};

				let (sender, receiver) = futures_util::StreamExt::split(ws);

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

			Ok((response.map(|_| HttpBody::default()), fut))
		}
		Err(e) => {
			tracing::debug!(target: LOG_TARGET, "WS upgrade handshake failed: {}", e);
			Err(HttpResponse::new(HttpBody::from(format!("WS upgrade handshake failed: {e}"))))
		}
	}
}
