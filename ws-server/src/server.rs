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
// IN background_task WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::future::{FutureDriver, StopHandle, StopMonitor};
use crate::types::{
	error::Error,
	v2::{ErrorCode, Id, Request},
	TEN_MB_SIZE_BYTES,
};
use futures_channel::mpsc;
use futures_util::future::FutureExt;
use futures_util::io::{BufReader, BufWriter};
use futures_util::stream::{self, StreamExt};
use soketto::handshake::{server::Response, Server as SokettoServer};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

use jsonrpsee_utils::server::helpers::{collect_batch_response, prepare_error, send_error};
use jsonrpsee_utils::server::resource_limiting::Resources;
use jsonrpsee_utils::server::rpc_module::{ConnectionId, Methods};

/// Default maximum connections allowed.
const MAX_CONNECTIONS: u64 = 100;

/// A WebSocket JSON RPC server.
#[derive(Debug)]
pub struct Server {
	listener: TcpListener,
	cfg: Settings,
	stop_monitor: StopMonitor,
	resources: Resources,
}

impl Server {
	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.listener.local_addr().map_err(Into::into)
	}

	/// Returns the handle to stop the running server.
	pub fn stop_handle(&self) -> StopHandle {
		self.stop_monitor.handle()
	}

	/// Start responding to connections requests. This will run on the tokio runtime until the server is stopped.
	pub fn start(mut self, methods: impl Into<Methods>) -> Result<StopHandle, Error> {
		let methods = methods.into().initialize_resources(&self.resources)?;
		let handle = self.stop_handle();

		match self.cfg.tokio_runtime.take() {
			Some(rt) => rt.spawn(self.start_inner(methods)),
			None => tokio::spawn(self.start_inner(methods)),
		};

		Ok(handle)
	}

	async fn start_inner(self, methods: Methods) {
		let stop_monitor = self.stop_monitor;
		let resources = self.resources;

		let mut id = 0;
		let mut connections = FutureDriver::default();
		let mut incoming = Incoming::new(self.listener, &stop_monitor);

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
						},
					)));

					id = id.wrapping_add(1);
				}
				Err(IncomingError::Io(err)) => {
					tracing::error!("Error while awaiting a new connection: {:?}", err);
				}
				Err(IncomingError::Shutdown) => break,
			}
		}

		connections.await
	}
}

/// This is a glorified select listening to new connections, while also checking
/// for `stop_receiver` signal.
struct Incoming<'a> {
	listener: TcpListener,
	stop_monitor: &'a StopMonitor,
}

impl<'a> Incoming<'a> {
	fn new(listener: TcpListener, stop_monitor: &'a StopMonitor) -> Self {
		Incoming { listener, stop_monitor }
	}
}

enum IncomingError {
	Shutdown,
	Io(std::io::Error),
}

impl<'a> Future for Incoming<'a> {
	type Output = Result<(TcpStream, SocketAddr), IncomingError>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let this = Pin::into_inner(self);

		if this.stop_monitor.shutdown_requested() {
			return Poll::Ready(Err(IncomingError::Shutdown));
		}

		this.listener.poll_accept(cx).map_err(IncomingError::Io)
	}
}

enum HandshakeResponse<'a> {
	Reject {
		status_code: u16,
	},
	Accept {
		conn_id: ConnectionId,
		methods: &'a Methods,
		resources: &'a Resources,
		cfg: &'a Settings,
		stop_monitor: &'a StopMonitor,
	},
}

async fn handshake(socket: tokio::net::TcpStream, mode: HandshakeResponse<'_>) -> Result<(), Error> {
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
		HandshakeResponse::Accept { conn_id, methods, resources, cfg, stop_monitor } => {
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
) -> Result<(), Error> {
	// And we can finally transition to a websocket background_task.
	let (mut sender, mut receiver) = server.into_builder().finish();
	let (tx, mut rx) = mpsc::unbounded::<String>();
	let stop_server2 = stop_server.clone();

	// Send results back to the client.
	tokio::spawn(async move {
		while !stop_server2.shutdown_requested() {
			match rx.next().await {
				Some(response) => {
					tracing::debug!("send: {}", response);
					let _ = sender.send_text(response).await;
					let _ = sender.flush().await;
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

	while !stop_server.shutdown_requested() {
		data.clear();

		if let Err(e) = method_executors.select_with(receiver.receive_data(&mut data)).await {
			tracing::error!("Could not receive WS data: {:?}; closing connection", e);
			tx.close_channel();
			return Err(e.into());
		}

		if data.len() > max_request_body_size as usize {
			tracing::warn!("Request is too big ({} bytes, max is {})", data.len(), max_request_body_size);
			send_error(Id::Null, &tx, ErrorCode::OversizedRequest.into());
			continue;
		}

		match data.get(0) {
			Some(b'{') => {
				if let Ok(req) = serde_json::from_slice::<Request>(&data) {
					tracing::debug!("recv: {:?}", req);
					if let Some(fut) = methods.execute_with_resources(&tx, req, conn_id, &resources) {
						method_executors.add(fut);
					}
				} else {
					let (id, code) = prepare_error(&data);
					send_error(id, &tx, code.into());
				}
			}
			Some(b'[') => {
				// Make sure the following variables are not moved into async closure below.
				let d = std::mem::take(&mut data);
				let resources = &resources;
				let methods = &methods;
				let tx2 = tx.clone();

				let fut = async move {
					// Batch responses must be sent back as a single message so we read the results from each
					// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
					// complete batch response back to the client over `tx`.
					let (tx_batch, mut rx_batch) = mpsc::unbounded();
					if let Ok(batch) = serde_json::from_slice::<Vec<Request>>(&d) {
						if !batch.is_empty() {
							let methods_stream =
								stream::iter(batch.into_iter().filter_map(|req| {
									methods.execute_with_resources(&tx_batch, req, conn_id, resources)
								}));

							let results = methods_stream
								.for_each_concurrent(None, |item| item)
								.then(|_| {
									rx_batch.close();
									collect_batch_response(rx_batch)
								})
								.await;

							if let Err(err) = tx2.unbounded_send(results) {
								tracing::error!("Error sending batch response to the client: {:?}", err)
							}
						} else {
							send_error(Id::Null, &tx2, ErrorCode::InvalidRequest.into());
						}
					} else {
						let (id, code) = prepare_error(&d);
						send_error(id, &tx2, code.into());
					}
				};

				method_executors.add(Box::pin(fut));
			}
			_ => send_error(Id::Null, &tx, ErrorCode::ParseError.into()),
		}
	}

	// Drive all running methods to completion
	method_executors.await;

	Ok(())
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
#[derive(Debug, Default)]
pub struct Builder {
	settings: Settings,
	resources: Resources,
}

impl Builder {
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
	pub async fn build(self, addr: impl ToSocketAddrs) -> Result<Server, Error> {
		let listener = TcpListener::bind(addr).await?;
		let stop_monitor = StopMonitor::new();
		let resources = self.resources;
		Ok(Server { listener, cfg: self.settings, stop_monitor, resources })
	}
}
