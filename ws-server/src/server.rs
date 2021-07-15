// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{net::SocketAddr, sync::Arc};

use crate::types::{
	error::Error,
	v2::error::JsonRpcErrorCode,
	v2::params::Id,
	v2::request::JsonRpcRequest,
	TEN_MB_SIZE_BYTES,
};
use futures_channel::mpsc;
use futures_util::future::{join_all, FutureExt};
use futures_util::stream::StreamExt;
use futures_util::{
	io::{BufReader, BufWriter},
	SinkExt,
};
use soketto::handshake::{server::Response, Server as SokettoServer};
use tokio::{
	net::{TcpListener, TcpStream, ToSocketAddrs},
	sync::RwLock,
};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

use jsonrpsee_utils::server::helpers::{collect_batch_response, prepare_error, send_error};
use jsonrpsee_utils::server::rpc_module::{ConnectionId, Methods};

/// Default maximum connections allowed.
const MAX_CONNECTIONS: u64 = 100;

/// A WebSocket JSON RPC server.
#[derive(Debug)]
pub struct Server {
	methods: Methods,
	listener: TcpListener,
	cfg: Settings,
	/// Pair of channels to stop the server.
	stop_pair: (mpsc::Sender<()>, mpsc::Receiver<()>),
	/// Stop handle that indicates whether server has been stopped.
	stop_handle: Arc<RwLock<()>>,
}

impl Server {
	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.listener.local_addr().map_err(Into::into)
	}

	/// Returns the handle to stop the running server.
	pub fn stop_handle(&self) -> StopHandle {
		StopHandle { stop_sender: self.stop_pair.0.clone(), stop_handle: self.stop_handle.clone() }
	}

	/// Start responding to connections requests. This will block current thread until the server is stopped.
	pub async fn start(self, methods: impl Into<Methods>) {
		// Acquire read access to the lock such that additional reader(s) may share this lock.
		// Write access to this lock will only be possible after the server and all background tasks have stopped.
		let _stop_handle = self.stop_handle.read().await;
		let shutdown = self.stop_pair.0;
		let methods = methods.into();

		let mut id = 0;
		let mut driver = ConnDriver::new(self.listener, self.stop_pair.1);

		loop {
			match Pin::new(&mut driver).await {
				Ok((socket, _addr)) => {
					if let Err(e) = socket.set_nodelay(true) {
						log::error!("Could not set NODELAY on socket: {:?}", e);
						continue;
					}

					if driver.connection_count() >= self.cfg.max_connections as usize {
						log::warn!("Too many connections. Try again in a while.");
						continue;
					}

					let methods = &methods;
					let cfg = &self.cfg;

					driver.add(Box::pin(handshake(socket, id, methods, cfg, &shutdown, &self.stop_handle)));

					id = id.wrapping_add(1);
				}
				Err(DriverError::Io(err)) => {
					log::error!("Error while awaiting a new connection: {:?}", err);
				}
				Err(DriverError::Shutdown) => break,
			}
		}
	}
}

/// This is a glorified select `Future` that will attempt to drive all
/// connection futures `F` to completion on each `poll`, while also
/// handling incoming connections.
struct ConnDriver<F> {
	listener: TcpListener,
	stop_receiver: mpsc::Receiver<()>,
	connections: Vec<F>,
}

impl<F> ConnDriver<F>
where
	F: Future + Unpin,
{
	fn new(listener: TcpListener, stop_receiver: mpsc::Receiver<()>) -> Self {
		ConnDriver { listener, stop_receiver, connections: Vec::new() }
	}

	fn connection_count(&self) -> usize {
		self.connections.len()
	}

	fn add(&mut self, conn: F) {
		self.connections.push(conn);
	}
}

enum DriverError {
	Shutdown,
	Io(std::io::Error),
}

impl<F> Future for ConnDriver<F>
where
	F: Future + Unpin,
{
	type Output = Result<(TcpStream, SocketAddr), DriverError>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let this = Pin::into_inner(self);

		let mut i = 0;

		while i < this.connections.len() {
			if this.connections[i].poll_unpin(cx).is_ready() {
				// Using `swap_remove` since we don't care about ordering
				// but we do care about removing being `O(1)`.
				//
				// We don't increment `i` in this branch, since we now
				// have a shorter length, and potentially a new value at
				// current index
				this.connections.swap_remove(i);
			} else {
				i += 1;
			}
		}

		if let Poll::Ready(Some(())) = this.stop_receiver.next().poll_unpin(cx) {
			return Poll::Ready(Err(DriverError::Shutdown));
		}

		this.listener.poll_accept(cx).map_err(DriverError::Io)
	}
}

async fn handshake(
	socket: tokio::net::TcpStream,
	conn_id: ConnectionId,
	methods: &Methods,
	cfg: &Settings,
	shutdown: &mpsc::Sender<()>,
	stop_handle: &Arc<RwLock<()>>,
) -> Result<(), Error> {
	// For each incoming background_task we perform a handshake.
	let mut server = SokettoServer::new(BufReader::new(BufWriter::new(socket.compat())));

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
		cfg.max_request_body_size,
		shutdown.clone(),
		stop_handle.clone(),
	))
	.await;

	match join_result {
		Err(_) => Err(Error::Custom("Background task was aborted".into())),
		Ok(result) => result,
	}
}

async fn background_task(
	server: SokettoServer<'_, BufReader<BufWriter<Compat<tokio::net::TcpStream>>>>,
	conn_id: ConnectionId,
	methods: Methods,
	max_request_body_size: u32,
	shutdown: mpsc::Sender<()>,
	stop_handle: Arc<RwLock<()>>,
) -> Result<(), Error> {
	let _lock = stop_handle.read().await;
	// And we can finally transition to a websocket background_task.
	let (mut sender, mut receiver) = server.into_builder().finish();
	let (tx, mut rx) = mpsc::unbounded::<String>();

	let shutdown2 = shutdown.clone();
	// Send results back to the client.
	tokio::spawn(async move {
		while !shutdown2.is_closed() {
			match rx.next().await {
				Some(response) => {
					log::debug!("send: {}", response);
					let _ = sender.send_text(response).await;
					let _ = sender.flush().await;
				}
				None => break,
			};
		}
		// terminate connection.
		let _ = sender.close().await;
	});

	// Buffer for incoming data.
	let mut data = Vec::with_capacity(100);

	while !shutdown.is_closed() {
		data.clear();

		receiver.receive_data(&mut data).await?;

		if data.len() > max_request_body_size as usize {
			log::warn!("Request is too big ({} bytes, max is {})", data.len(), max_request_body_size);
			send_error(Id::Null, &tx, JsonRpcErrorCode::OversizedRequest.into());
			continue;
		}

		match data[0] {
			b'{' => {
				if let Ok(req) = serde_json::from_slice::<JsonRpcRequest>(&data) {
					log::debug!("recv: {:?}", req);
					methods.execute(&tx, req, conn_id).await;
				} else {
					let (id, code) = prepare_error(&data);
					send_error(id, &tx, code.into());
				}
			}
			b'[' => {
				if let Ok(batch) = serde_json::from_slice::<Vec<JsonRpcRequest>>(&data) {
					if !batch.is_empty() {
						// Batch responses must be sent back as a single message so we read the results from each request in the
						// batch and read the results off of a new channel, `rx_batch`, and then send the complete batch response
						// back to the client over `tx`.
						let (tx_batch, mut rx_batch) = mpsc::unbounded::<String>();

						join_all(batch.into_iter().map(|req| methods.execute(&tx_batch, req, conn_id))).await;

						// Closes the receiving half of a channel without dropping it. This prevents any further messages from
						// being sent on the channel.
						rx_batch.close();
						let results = collect_batch_response(rx_batch).await;
						if let Err(err) = tx.unbounded_send(results) {
							log::error!("Error sending batch response to the client: {:?}", err)
						}
					} else {
						send_error(Id::Null, &tx, JsonRpcErrorCode::InvalidRequest.into());
					}
				} else {
					let (id, code) = prepare_error(&data);
					send_error(id, &tx, code.into());
				}
			}
			_ => send_error(Id::Null, &tx, JsonRpcErrorCode::ParseError.into()),
		}
	}
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
				log::warn!("{}", error);
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
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			max_request_body_size: TEN_MB_SIZE_BYTES,
			max_connections: MAX_CONNECTIONS,
			allowed_origins: AllowedValue::Any,
			allowed_hosts: AllowedValue::Any,
		}
	}
}

/// Builder to configure and create a JSON-RPC Websocket server
#[derive(Debug)]
pub struct Builder {
	settings: Settings,
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
	/// Will return an error if `list` is empty. Use [`allow_all_origins`](Builder::allow_all_origins) to restore the default.
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
	/// Will return an error if `list` is empty. Use [`allow_all_hosts`](Builder::allow_all_hosts) to restore the default.
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

	/// Finalize the configuration of the server. Consumes the [`Builder`].
	pub async fn build(self, addr: impl ToSocketAddrs) -> Result<Server, Error> {
		let listener = TcpListener::bind(addr).await?;
		let stop_pair = mpsc::channel(1);
		Ok(Server {
			listener,
			methods: Methods::default(),
			cfg: self.settings,
			stop_pair,
			stop_handle: Arc::new(RwLock::new(())),
		})
	}
}

impl Default for Builder {
	fn default() -> Self {
		Self { settings: Settings::default() }
	}
}

/// Handle that is able to stop the running server.
#[derive(Debug, Clone)]
pub struct StopHandle {
	stop_sender: mpsc::Sender<()>,
	stop_handle: Arc<RwLock<()>>,
}

impl StopHandle {
	/// Requests server to stop. Returns an error if server was already stopped.
	pub async fn stop(&mut self) -> Result<(), Error> {
		self.stop_sender.send(()).await.map_err(|_| Error::AlreadyStopped)
	}

	/// Blocks indefinitely until the server is stopped.
	pub async fn wait_for_stop(&self) {
		// blocks until there are no readers left.
		self.stop_handle.write().await;
	}
}
