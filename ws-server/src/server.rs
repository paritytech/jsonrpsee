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

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::future::{FutureDriver, ServerHandle, StopMonitor};
use crate::types::error::ErrorCode;
use crate::types::{Id, Request, TEN_MB_SIZE_BYTES};
use futures_channel::mpsc;
use futures_util::future::{join_all, FutureExt};
use futures_util::io::{BufReader, BufWriter};
use futures_util::stream::StreamExt;
use jsonrpsee_core::middleware::Middleware;
use jsonrpsee_core::server::helpers::{collect_batch_response, prepare_error, MethodSink};
use jsonrpsee_core::server::resource_limiting::Resources;
use jsonrpsee_core::server::rpc_module::{ConnectionId, MethodResult, Methods};
use jsonrpsee_core::Error;
use soketto::connection::Error as SokettoError;
use soketto::handshake::{server::Response, Server as SokettoServer};
use soketto::Sender;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

/// Default maximum connections allowed.
const MAX_CONNECTIONS: u64 = 100;

/// A WebSocket JSON RPC server.
#[derive(Debug)]
pub struct Server<M> {
	listener: TcpListener,
	cfg: Settings,
	stop_monitor: StopMonitor,
	resources: Resources,
	middleware: M,
}

impl<M: Middleware> Server<M> {
	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.listener.local_addr().map_err(Into::into)
	}

	/// Returns the handle to stop the running server.
	pub fn server_handle(&self) -> ServerHandle {
		self.stop_monitor.handle()
	}

	/// Start responding to connections requests. This will run on the tokio runtime until the server is stopped.
	pub fn start(mut self, methods: impl Into<Methods>) -> Result<ServerHandle, Error> {
		let methods = methods.into().initialize_resources(&self.resources)?;
		let handle = self.server_handle();

		match self.cfg.tokio_runtime.take() {
			Some(rt) => rt.spawn(self.start_inner(methods)),
			None => tokio::spawn(self.start_inner(methods)),
		};

		Ok(handle)
	}

	async fn start_inner(self, methods: Methods) {
		let stop_monitor = self.stop_monitor;
		let resources = self.resources;
		let middleware = self.middleware;

		let mut id = 0;
		let mut connections = FutureDriver::default();
		let mut incoming = Monitored::new(Incoming(self.listener), &stop_monitor);

		loop {
			match connections.select_with(&mut incoming).await {
				Ok((socket, _addr)) => {
					if let Err(e) = socket.set_nodelay(true) {
						tracing::error!("Could not set NODELAY on socket: {:?}", e);
						continue;
					}

					if connections.count() >= self.cfg.max_connections as usize {
						tracing::warn!("Too many connections. Try again in a while.");
						connections.add(Box::pin(handshake(socket, HandshakeResponse::Reject { status_code: 429 })));
						continue;
					}

					let methods = &methods;
					let cfg = &self.cfg;

					connections.add(Box::pin(handshake(
						socket,
						HandshakeResponse::Accept {
							conn_id: id,
							methods,
							resources: &resources,
							cfg,
							stop_monitor: &stop_monitor,
							middleware: middleware.clone(),
						},
					)));

					id = id.wrapping_add(1);
				}
				Err(MonitoredError::Selector(err)) => {
					tracing::error!("Error while awaiting a new connection: {:?}", err);
				}
				Err(MonitoredError::Shutdown) => break,
			}
		}

		connections.await
	}
}

/// This is a glorified select listening for new messages, while also checking the `stop_receiver` signal.
struct Monitored<'a, F> {
	future: F,
	stop_monitor: &'a StopMonitor,
}

impl<'a, F> Monitored<'a, F> {
	fn new(future: F, stop_monitor: &'a StopMonitor) -> Self {
		Monitored { future, stop_monitor }
	}
}

enum MonitoredError<E> {
	Shutdown,
	Selector(E),
}

struct Incoming(TcpListener);

impl<'a> Future for Monitored<'a, Incoming> {
	type Output = Result<(TcpStream, SocketAddr), MonitoredError<std::io::Error>>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let this = Pin::into_inner(self);

		if this.stop_monitor.shutdown_requested() {
			return Poll::Ready(Err(MonitoredError::Shutdown));
		}

		this.future.0.poll_accept(cx).map_err(MonitoredError::Selector)
	}
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

enum HandshakeResponse<'a, M> {
	Reject {
		status_code: u16,
	},
	Accept {
		conn_id: ConnectionId,
		methods: &'a Methods,
		resources: &'a Resources,
		cfg: &'a Settings,
		stop_monitor: &'a StopMonitor,
		middleware: M,
	},
}

async fn handshake<M>(socket: tokio::net::TcpStream, mode: HandshakeResponse<'_, M>) -> Result<(), Error>
where
	M: Middleware,
{
	// For each incoming background_task we perform a handshake.
	let mut server = SokettoServer::new(BufReader::new(BufWriter::new(socket.compat())));

	match mode {
		HandshakeResponse::Reject { status_code } => {
			// Forced rejection, don't need to read anything from the socket
			let reject = Response::Reject { status_code };
			server.send_response(&reject).await?;

			let (mut sender, _) = server.into_builder().finish();

			// Gracefully shut down the connection
			sender.close().await?;

			Ok(())
		}
		HandshakeResponse::Accept { conn_id, methods, resources, cfg, stop_monitor, middleware } => {
			tracing::debug!("Accepting new connection: {}", conn_id);
			let key = {
				let req = server.receive_request().await?;
				let host_check = cfg.allowed_hosts.verify("Host", Some(req.headers().host));
				let origin_check = cfg.allowed_origins.verify("Origin", req.headers().origin);

				host_check.and(origin_check).map(|()| req.key())
			};

			match key {
				Ok(key) => {
					let accept = Response::Accept { key, protocol: None };
					server.send_response(&accept).await?;
				}
				Err(error) => {
					let reject = Response::Reject { status_code: 403 };
					server.send_response(&reject).await?;

					return Err(error);
				}
			}

			let join_result = tokio::spawn(background_task(
				server,
				conn_id,
				methods.clone(),
				resources.clone(),
				cfg.max_request_body_size,
				stop_monitor.clone(),
				middleware,
			))
			.await;

			match join_result {
				Err(_) => Err(Error::Custom("Background task was aborted".into())),
				Ok(result) => result,
			}
		}
	}
}

async fn background_task(
	server: SokettoServer<'_, BufReader<BufWriter<Compat<tokio::net::TcpStream>>>>,
	conn_id: ConnectionId,
	methods: Methods,
	resources: Resources,
	max_request_body_size: u32,
	stop_server: StopMonitor,
	middleware: impl Middleware,
) -> Result<(), Error> {
	// And we can finally transition to a websocket background_task.
	let mut builder = server.into_builder();
	builder.set_max_message_size(max_request_body_size as usize);
	let (mut sender, mut receiver) = builder.finish();
	let (tx, mut rx) = mpsc::unbounded::<String>();
	let stop_server2 = stop_server.clone();
	let sink = MethodSink::new_with_limit(tx, max_request_body_size);

	middleware.on_connect();

	// Send results back to the client.
	tokio::spawn(async move {
		while !stop_server2.shutdown_requested() {
			match rx.next().await {
				Some(response) => {
					// If websocket message send fail then terminate the connection.
					if let Err(err) = send_ws_message(&mut sender, response).await {
						tracing::error!("WS transport error: {:?}; terminate connection", err);
						break;
					}
				}
				None => break,
			};
		}
		// terminate connection.
		let _ = sender.close().await;
		// NOTE(niklasad1): when the receiver is dropped no further requests or subscriptions
		// will be possible.
	});

	// Buffer for incoming data.
	let mut data = Vec::with_capacity(100);
	let mut method_executors = FutureDriver::default();
	let middleware = &middleware;

	let result = loop {
		data.clear();

		{
			// Need the extra scope to drop this pinned future and reclaim access to `data`
			let receive = receiver.receive_data(&mut data);

			tokio::pin!(receive);

			if let Err(err) = method_executors.select_with(Monitored::new(receive, &stop_server)).await {
				match err {
					MonitoredError::Selector(SokettoError::Closed) => {
						tracing::debug!("WS transport error: remote peer terminated the connection: {}", conn_id);
						sink.close();
						break Ok(());
					}
					MonitoredError::Selector(SokettoError::MessageTooLarge { current, maximum }) => {
						tracing::warn!(
							"WS transport error: outgoing message is too big error ({} bytes, max is {})",
							current,
							maximum
						);
						sink.send_error(Id::Null, ErrorCode::OversizedRequest.into());
						continue;
					}
					// These errors can not be gracefully handled, so just log them and terminate the connection.
					MonitoredError::Selector(err) => {
						tracing::error!("WS transport error: {:?} => terminating connection {}", err, conn_id);
						sink.close();
						break Err(err.into());
					}
					MonitoredError::Shutdown => break Ok(()),
				};
			};
		};

		tracing::debug!("recv {} bytes", data.len());

		let request_start = middleware.on_request();

		match data.get(0) {
			Some(b'{') => {
				if let Ok(req) = serde_json::from_slice::<Request>(&data) {
					middleware.on_call(req.method.as_ref());

					tracing::debug!("recv method call={}", req.method);
					tracing::trace!("recv: req={:?}", req);
					match methods.execute_with_resources(&sink, req, conn_id, &resources) {
						Ok((name, MethodResult::Sync(success))) => {
							middleware.on_result(name, success, request_start);
							middleware.on_response(request_start);
						}
						Ok((name, MethodResult::Async(fut))) => {
							let request_start = request_start;

							let fut = async move {
								let success = fut.await;
								middleware.on_result(name, success, request_start);
								middleware.on_response(request_start);
							};

							method_executors.add(fut.boxed());
						}
						Err(name) => {
							middleware.on_result(name.as_ref(), false, request_start);
							middleware.on_response(request_start);
						}
					}
				} else {
					let (id, code) = prepare_error(&data);
					sink.send_error(id, code.into());
					middleware.on_response(request_start);
				}
			}
			Some(b'[') => {
				// Make sure the following variables are not moved into async closure below.
				let d = std::mem::take(&mut data);
				let resources = &resources;
				let methods = &methods;
				let sink = sink.clone();

				let fut = async move {
					// Batch responses must be sent back as a single message so we read the results from each
					// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
					// complete batch response back to the client over `tx`.
					let (tx_batch, mut rx_batch) = mpsc::unbounded();
					let sink_batch = MethodSink::new_with_limit(tx_batch, max_request_body_size);
					if let Ok(batch) = serde_json::from_slice::<Vec<Request>>(&d) {
						tracing::debug!("recv batch len={}", batch.len());
						tracing::trace!("recv: batch={:?}", batch);
						if !batch.is_empty() {
							join_all(batch.into_iter().filter_map(move |req| {
								match methods.execute_with_resources(&sink_batch, req, conn_id, resources) {
									Ok((name, MethodResult::Sync(success))) => {
										middleware.on_result(name, success, request_start);
										None
									}
									Ok((name, MethodResult::Async(fut))) => Some(async move {
										let success = fut.await;
										middleware.on_result(name, success, request_start);
									}),
									Err(name) => {
										middleware.on_result(name.as_ref(), false, request_start);
										None
									}
								}
							}))
							.await;

							rx_batch.close();
							let results = collect_batch_response(rx_batch).await;

							if let Err(err) = sink.send_raw(results) {
								tracing::error!("Error sending batch response to the client: {:?}", err)
							} else {
								middleware.on_response(request_start);
							}
						} else {
							sink.send_error(Id::Null, ErrorCode::InvalidRequest.into());
							middleware.on_response(request_start);
						}
					} else {
						let (id, code) = prepare_error(&d);
						sink.send_error(id, code.into());
						middleware.on_response(request_start);
					}
				};

				method_executors.add(Box::pin(fut));
			}
			_ => {
				sink.send_error(Id::Null, ErrorCode::ParseError.into());
			}
		}
	};

	middleware.on_disconnect();

	// Drive all running methods to completion.
	// **NOTE** Do not return early in this function. This `await` needs to run to guarantee
	// proper drop behaviour.
	method_executors.await;

	result
}

#[derive(Debug, Clone)]
enum AllowedValue {
	Any,
	OneOf(Box<[String]>),
}

impl AllowedValue {
	fn verify(&self, header: &str, value: Option<&[u8]>) -> Result<(), Error> {
		if let (AllowedValue::OneOf(list), Some(value)) = (self, value) {
			if !list.iter().any(|o| o.as_bytes() == value) {
				let error = format!("{} denied: {}", header, String::from_utf8_lossy(value));
				tracing::warn!("{}", error);
				return Err(Error::Request(error));
			}
		}

		Ok(())
	}
}

/// JSON-RPC Websocket server settings.
#[derive(Debug, Clone)]
struct Settings {
	/// Maximum size in bytes of a request.
	max_request_body_size: u32,
	/// Maximum number of incoming connections allowed.
	max_connections: u64,
	/// Policy by which to accept or deny incoming requests based on the `Origin` header.
	allowed_origins: AllowedValue,
	/// Policy by which to accept or deny incoming requests based on the `Host` header.
	allowed_hosts: AllowedValue,
	/// Custom tokio runtime to run the server on.
	tokio_runtime: Option<tokio::runtime::Handle>,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			max_request_body_size: TEN_MB_SIZE_BYTES,
			max_connections: MAX_CONNECTIONS,
			allowed_origins: AllowedValue::Any,
			allowed_hosts: AllowedValue::Any,
			tokio_runtime: None,
		}
	}
}

/// Builder to configure and create a JSON-RPC Websocket server
#[derive(Debug)]
pub struct Builder<M = ()> {
	settings: Settings,
	resources: Resources,
	middleware: M,
}

impl Default for Builder {
	fn default() -> Self {
		Builder { settings: Settings::default(), resources: Resources::default(), middleware: () }
	}
}

impl Builder {
	/// Create a default server builder.
	pub fn new() -> Self {
		Self::default()
	}
}

impl<M> Builder<M> {
	/// Set the maximum size of a request body in bytes. Default is 10 MiB.
	pub fn max_request_body_size(mut self, size: u32) -> Self {
		self.settings.max_request_body_size = size;
		self
	}

	/// Set the maximum number of connections allowed. Default is 100.
	pub fn max_connections(mut self, max: u64) -> Self {
		self.settings.max_connections = max;
		self
	}

	/// Register a new resource kind. Errors if `label` is already registered, or if the number of
	/// registered resources on this server instance would exceed 8.
	///
	/// See the module documentation for [`resurce_limiting`](../jsonrpsee_utils/server/resource_limiting/index.html#resource-limiting)
	/// for details.
	pub fn register_resource(mut self, label: &'static str, capacity: u16, default: u16) -> Result<Self, Error> {
		self.resources.register(label, capacity, default)?;
		Ok(self)
	}

	/// Set a list of allowed origins. During the handshake, the `Origin` header will be
	/// checked against the list, connections without a matching origin will be denied.
	/// Values should be hostnames with protocol.
	///
	/// ```rust
	/// # let mut builder = jsonrpsee_ws_server::WsServerBuilder::default();
	/// builder.set_allowed_origins(["https://example.com"]);
	/// ```
	///
	/// By default allows any `Origin`.
	///
	/// Will return an error if `list` is empty. Use [`allow_all_origins`](Builder::allow_all_origins) to restore the
	/// default.
	pub fn set_allowed_origins<Origin, List>(mut self, list: List) -> Result<Self, Error>
	where
		List: IntoIterator<Item = Origin>,
		Origin: Into<String>,
	{
		let list: Box<_> = list.into_iter().map(Into::into).collect();

		if list.len() == 0 {
			return Err(Error::EmptyAllowList("Origin"));
		}

		self.settings.allowed_origins = AllowedValue::OneOf(list);

		Ok(self)
	}

	/// Add a middleware to the builder [`Middleware`](../jsonrpsee_core/middleware/trait.Middleware.html).
	///
	/// ```
	/// use std::time::Instant;
	///
	/// use jsonrpsee_core::middleware::Middleware;
	/// use jsonrpsee_ws_server::WsServerBuilder;
	///
	/// #[derive(Clone)]
	/// struct MyMiddleware;
	///
	/// impl Middleware for MyMiddleware {
	///     type Instant = Instant;
	///
	///     fn on_request(&self) -> Instant {
	///         Instant::now()
	///     }
	///
	///     fn on_result(&self, name: &str, success: bool, started_at: Instant) {
	///         println!("Call to '{}' took {:?}", name, started_at.elapsed());
	///     }
	/// }
	///
	/// let builder = WsServerBuilder::new().set_middleware(MyMiddleware);
	/// ```
	pub fn set_middleware<T: Middleware>(self, middleware: T) -> Builder<T> {
		Builder { settings: self.settings, resources: self.resources, middleware }
	}

	/// Restores the default behavior of allowing connections with `Origin` header
	/// containing any value. This will undo any list set by [`set_allowed_origins`](Builder::set_allowed_origins).
	pub fn allow_all_origins(mut self) -> Self {
		self.settings.allowed_origins = AllowedValue::Any;
		self
	}

	/// Set a list of allowed hosts. During the handshake, the `Host` header will be
	/// checked against the list. Connections without a matching host will be denied.
	/// Values should be hostnames without protocol.
	///
	/// ```rust
	/// # let mut builder = jsonrpsee_ws_server::WsServerBuilder::default();
	/// builder.set_allowed_hosts(["example.com"]);
	/// ```
	///
	/// By default allows any `Host`.
	///
	/// Will return an error if `list` is empty. Use [`allow_all_hosts`](Builder::allow_all_hosts) to restore the
	/// default.
	pub fn set_allowed_hosts<Host, List>(mut self, list: List) -> Result<Self, Error>
	where
		List: IntoIterator<Item = Host>,
		Host: Into<String>,
	{
		let list: Box<_> = list.into_iter().map(Into::into).collect();

		if list.len() == 0 {
			return Err(Error::EmptyAllowList("Host"));
		}

		self.settings.allowed_hosts = AllowedValue::OneOf(list);

		Ok(self)
	}

	/// Restores the default behavior of allowing connections with `Host` header
	/// containing any value. This will undo any list set by [`set_allowed_hosts`](Builder::set_allowed_hosts).
	pub fn allow_all_hosts(mut self) -> Self {
		self.settings.allowed_hosts = AllowedValue::Any;
		self
	}

	/// Configure a custom [`tokio::runtime::Handle`] to run the server on.
	///
	/// Default: [`tokio::spawn`]
	pub fn custom_tokio_runtime(mut self, rt: tokio::runtime::Handle) -> Self {
		self.settings.tokio_runtime = Some(rt);
		self
	}

	/// Finalize the configuration of the server. Consumes the [`Builder`].
	///
	/// ```rust
	/// #[tokio::main]
	/// async fn main() {
	///   let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
	///   let occupied_addr = listener.local_addr().unwrap();
	///   let addrs: &[std::net::SocketAddr] = &[
	///       occupied_addr,
	///       "127.0.0.1:0".parse().unwrap(),
	///   ];
	///   assert!(jsonrpsee_ws_server::WsServerBuilder::default().build(occupied_addr).await.is_err());
	///   assert!(jsonrpsee_ws_server::WsServerBuilder::default().build(addrs).await.is_ok());
	/// }
	/// ```
	///
	pub async fn build(self, addrs: impl ToSocketAddrs) -> Result<Server<M>, Error> {
		let listener = TcpListener::bind(addrs).await?;
		let stop_monitor = StopMonitor::new();
		let resources = self.resources;
		Ok(Server { listener, cfg: self.settings, stop_monitor, resources, middleware: self.middleware })
	}
}

async fn send_ws_message(
	sender: &mut Sender<BufReader<BufWriter<Compat<TcpStream>>>>,
	response: String,
) -> Result<(), Error> {
	tracing::debug!("send {} bytes", response.len());
	tracing::trace!("send: {}", response);
	sender.send_text_owned(response).await?;
	sender.flush().await.map_err(Into::into)
}
