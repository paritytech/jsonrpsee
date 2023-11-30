use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::middleware::rpc::{RpcService, RpcServiceBuilder, RpcServiceCfg, RpcServiceT};
use crate::server::{handle_rpc_call, ConnectionState, ServerConfig};
use crate::{PingConfig, LOG_TARGET};

use futures_util::future::{self, Either, Fuse};
use futures_util::io::{BufReader, BufWriter};
use futures_util::{Future, FutureExt, StreamExt, TryStreamExt};
use hyper::upgrade::Upgraded;
use jsonrpsee_core::server::helpers::MethodSink;
use jsonrpsee_core::server::{BoundedSubscriptions, Methods};
use jsonrpsee_core::Error;
use jsonrpsee_types::error::{reject_too_big_request, ErrorCode};
use jsonrpsee_types::Id;
use soketto::connection::Error as SokettoError;
use soketto::data::ByteSlice125;

use tokio::sync::{mpsc, oneshot};
use tokio_stream::wrappers::{IntervalStream, ReceiverStream};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

pub(crate) type Sender = soketto::Sender<BufReader<BufWriter<Compat<Upgraded>>>>;
pub(crate) type Receiver = soketto::Receiver<BufReader<BufWriter<Compat<Upgraded>>>>;

pub use soketto::handshake::http::is_upgrade_request;

enum Incoming {
	Data(Vec<u8>),
	Pong,
}

pub(crate) async fn send_message(sender: &mut Sender, response: String) -> Result<(), Error> {
	sender.send_text_owned(response).await?;
	sender.flush().await.map_err(Into::into)
}

pub(crate) async fn send_ping(sender: &mut Sender) -> Result<(), Error> {
	tracing::debug!(target: LOG_TARGET, "Send ping");
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
	} = params;
	let ServerConfig { ping_config, batch_requests_config, max_request_body_size, max_response_body_size, .. } =
		server_cfg;

	let (conn_tx, conn_rx) = oneshot::channel();

	// Spawn another task that sends out the responses on the Websocket.
	let send_task_handle = tokio::spawn(send_task(rx, ws_sender, ping_config.ping_interval(), conn_rx));

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
		let data = match try_recv(&mut ws_stream, stopped, ping_config).await {
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
						tracing::debug!(target: LOG_TARGET, "WS error: {}; terminate connection: {}", err, conn.conn_id);
						break Err(err);
					}
				};
			}
		};

		let rpc_service = rpc_service.clone();
		let sink = sink.clone();

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

			if let Some(rp) =
				handle_rpc_call(&data[idx..], is_single, batch_requests_config, max_response_body_size, &*rpc_service)
					.await
			{
				if !rp.is_subscription {
					_ = sink.send(rp.result).await;
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
	Err(SokettoError, S),
	Ok(Vec<u8>, S),
}

/// Attempts to read data from WebSocket fails if the server was stopped.
async fn try_recv<T, S>(ws_stream: &mut T, mut stopped: S, ping_config: PingConfig) -> Receive<S>
where
	S: Future<Output = ()> + Unpin,
	T: StreamExt<Item = Result<Incoming, SokettoError>> + Unpin,
{
	let mut last_active = Instant::now();

	let inactivity_check =
		Box::pin(ping_config.inactive_limit().map(|d| tokio::time::sleep(d).fuse()).unwrap_or_else(Fuse::terminated));
	let mut futs = futures_util::future::select(ws_stream.next(), inactivity_check);

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

/// Low-level API that attempts to establish WebSocket connection
///
/// Returns Ok((http_response, fut)) if websocket connection was successfully established
/// otherwise Err(http_response).
///
/// `fut` is a future that drives the WebSocket connection
/// and if it's dropped the connection will be closed.
///
/// If you calling this from the `hyper::service_fn` the HTTP response
/// must be sent back and the websocket connection will held in another task.
///
/// ```no_run
/// use jsonrpsee_server::{ws, ServerConfig, Methods, ConnectionState};
/// use jsonrpsee_server::middleware::rpc::{RpcServiceBuilder, RpcServiceT, RpcService};
///
/// async fn handle_websocket_conn<L>(
///     req: hyper::Request<hyper::Body>,
///     server_cfg: ServerConfig,
///     methods: impl Into<Methods> + 'static,
///     conn: ConnectionState,
///     rpc_middleware: RpcServiceBuilder<L>,
///     mut disconnect: tokio::sync::mpsc::Receiver<()>
/// ) -> hyper::Response<hyper::Body>
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
pub async fn connect<L>(
	req: hyper::Request<hyper::Body>,
	server_cfg: ServerConfig,
	methods: impl Into<Methods>,
	conn: ConnectionState,
	rpc_middleware: RpcServiceBuilder<L>,
) -> Result<(hyper::Response<hyper::Body>, impl Future<Output = ()>), hyper::Response<hyper::Body>>
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
				conn.conn_id as usize,
				rpc_service_cfg,
			);

			let rpc_service = rpc_middleware.service(rpc_service);

			let fut = async move {
				let upgraded = match hyper::upgrade::on(req).await {
					Ok(u) => u,
					Err(e) => {
						tracing::debug!(target: LOG_TARGET, "WS upgrade handshake failed: {}", e);
						return;
					}
				};

				let stream = BufReader::new(BufWriter::new(upgraded.compat()));
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
				};

				background_task(params).await;
			};

			Ok((response.map(|()| hyper::Body::empty()), fut))
		}
		Err(e) => {
			tracing::debug!(target: LOG_TARGET, "WS upgrade handshake failed: {}", e);
			Err(hyper::Response::new(hyper::Body::from(format!("Could not upgrade connection: {e}"))))
		}
	}
}
