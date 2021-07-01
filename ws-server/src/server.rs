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
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::{Context, Poll};
use std::{net::SocketAddr, sync::Arc};

use futures_channel::mpsc;
use futures_util::future::{join_all, FutureExt};
use futures_util::io::{BufReader, BufWriter};
use futures_util::stream::StreamExt;
use jsonrpsee_types::TEN_MB_SIZE_BYTES;
use soketto::handshake::{server::Response, Server as SokettoServer};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

use jsonrpsee_types::error::Error;
use jsonrpsee_types::v2::error::JsonRpcErrorCode;
use jsonrpsee_types::v2::params::Id;
use jsonrpsee_types::v2::request::{JsonRpcInvalidRequest, JsonRpcRequest};
use jsonrpsee_utils::server::helpers::{collect_batch_response, send_error};
use jsonrpsee_utils::server::rpc_module::{ConnectionId, Methods, RpcModule};

/// Default maximum connections allowed.
const MAX_CONNECTIONS: u64 = 100;

/// A WebSocket JSON RPC server.
#[derive(Debug)]
pub struct Server {
	methods: Methods,
	listener: TcpListener,
	cfg: Settings,
}

impl Server {
	/// Register all methods from a [`Methods`] of provided [`RpcModule`] on this server.
	/// In case a method already is registered with the same name, no method is added and a [`Error::MethodAlreadyRegistered`]
	/// is returned. Note that the [`RpcModule`] is consumed after this call.
	pub fn register_module<Context>(&mut self, module: RpcModule<Context>) -> Result<(), Error> {
		self.methods.merge(module.into_methods())?;
		Ok(())
	}

	/// Returns a `Vec` with all the method names registered on this server.
	pub fn method_names(&self) -> Vec<&'static str> {
		self.methods.method_names()
	}

	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.listener.local_addr().map_err(Into::into)
	}

	/// Start responding to connections requests. This will block current thread until the server is stopped.
	pub fn start(self) -> StopHandle {
		let shutdown = Arc::new(AtomicBool::new(false));
		let shutdown2 = shutdown.clone();

		let stop_handle = tokio::spawn(async move {
			let methods = self.methods;
			let mut id = 0;

			let mut driver = ConnDriver::new(self.listener, shutdown.clone());

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

						driver.add(Box::pin(handshake(socket, id, methods, cfg, shutdown.clone())));

						id = id.wrapping_add(1);
					}
					Err(DriverError::Io(err)) => {
						log::error!("Error while awaiting a new connection: {:?}", err);
					}
					Err(DriverError::Shutdown) => break,
				}
			}
		});
		StopHandle { stop_handle: Some(stop_handle), shutdown: shutdown2 }
	}
}

/// This is a glorified select `Future` that will attempt to drive all
/// connection futures `F` to completion on each `poll`, while also
/// handling incoming connections.
struct ConnDriver<F> {
	listener: TcpListener,
	shutdown: Arc<AtomicBool>,
	connections: Vec<F>,
}

impl<F> ConnDriver<F>
where
	F: Future + Unpin,
{
	fn new(listener: TcpListener, shutdown: Arc<AtomicBool>) -> Self {
		ConnDriver { listener, shutdown, connections: Vec::new() }
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

		if this.shutdown.load(Ordering::SeqCst) {
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
	shutdown: Arc<AtomicBool>,
) -> Result<(), Error> {
	// For each incoming background_task we perform a handshake.
	let mut server = SokettoServer::new(BufReader::new(BufWriter::new(socket.compat())));

	let key = {
		let req = server.receive_request().await?;
		cfg.allowed_origins.verify(req.headers().origin).map(|()| req.key())
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

	let join_result =
		tokio::spawn(background_task(server, conn_id, methods.clone(), cfg.max_request_body_size, shutdown.clone()))
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
	shutdown: Arc<AtomicBool>,
) -> Result<(), Error> {
	// And we can finally transition to a websocket background_task.
	let (mut sender, mut receiver) = server.into_builder().finish();
	let (tx, mut rx) = mpsc::unbounded::<String>();

	let shutdown2 = shutdown.clone();
	// Send results back to the client.
	tokio::spawn(async move {
		while !shutdown2.load(Ordering::SeqCst) {
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

	while !shutdown.load(Ordering::SeqCst) {
		data.clear();

		receiver.receive_data(&mut data).await?;

		if data.len() > max_request_body_size as usize {
			log::warn!("Request is too big ({} bytes, max is {})", data.len(), max_request_body_size);
			send_error(Id::Null, &tx, JsonRpcErrorCode::OversizedRequest.into());
			continue;
		}

		// For reasons outlined [here](https://github.com/serde-rs/json/issues/497), `RawValue` can't be used with
		// untagged enums at the moment. This means we can't use an `SingleOrBatch` untagged enum here and have to try
		// each case individually: first the single request case, then the batch case and lastly the error. For the
		// worst case – unparseable input – we make three calls to [`serde_json::from_slice`] which is pretty annoying.
		// Our [issue](https://github.com/paritytech/jsonrpsee/issues/296).
		if let Ok(req) = serde_json::from_slice::<JsonRpcRequest>(&data) {
			log::debug!("recv: {:?}", req);
			methods.execute(&tx, req, conn_id).await;
		} else if let Ok(batch) = serde_json::from_slice::<Vec<JsonRpcRequest>>(&data) {
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
			let (id, code) = match serde_json::from_slice::<JsonRpcInvalidRequest>(&data) {
				Ok(req) => (req.id, JsonRpcErrorCode::InvalidRequest),
				Err(_) => (Id::Null, JsonRpcErrorCode::ParseError),
			};

			send_error(id, &tx, code.into());
		}
	}
	Ok(())
}

#[derive(Debug, Clone)]
enum AllowedOrigins {
	Any,
	OneOf(Arc<[String]>),
}

impl AllowedOrigins {
	fn verify(&self, origin: Option<&[u8]>) -> Result<(), Error> {
		if let (AllowedOrigins::OneOf(list), Some(origin)) = (self, origin) {
			if !list.iter().any(|o| o.as_bytes() == origin) {
				let error = format!("Origin denied: {}", String::from_utf8_lossy(origin));
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
	/// Cross-origin policy by which to accept or deny incoming requests.
	allowed_origins: AllowedOrigins,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			max_request_body_size: TEN_MB_SIZE_BYTES,
			max_connections: MAX_CONNECTIONS,
			allowed_origins: AllowedOrigins::Any,
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
	/// Values should include protocol.
	///
	/// ```rust
	/// # let mut builder = jsonrpsee_ws_server::WsServerBuilder::default();
	/// builder.set_allowed_origins(vec!["https://example.com"]);
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
		let list: Arc<_> = list.into_iter().map(Into::into).collect();

		if list.len() == 0 {
			return Err(Error::EmptyAllowedOrigins);
		}

		self.settings.allowed_origins = AllowedOrigins::OneOf(list);

		Ok(self)
	}

	/// Restores the default behavior of allowing connections with `Origin` header
	/// containing any value. This will undo any list set by [`set_allowed_origins`](Builder::set_allowed_origins).
	pub fn allow_all_origins(mut self) -> Self {
		self.settings.allowed_origins = AllowedOrigins::Any;
		self
	}

	/// Finalize the configuration of the server. Consumes the [`Builder`].
	pub async fn build(self, addr: impl ToSocketAddrs) -> Result<Server, Error> {
		let listener = TcpListener::bind(addr).await?;
		Ok(Server { listener, methods: Methods::default(), cfg: self.settings })
	}
}

impl Default for Builder {
	fn default() -> Self {
		Self { settings: Settings::default() }
	}
}

/// Handle that is able to stop the running server.
#[derive(Debug)]
pub struct StopHandle {
	shutdown: Arc<AtomicBool>,
	stop_handle: Option<tokio::task::JoinHandle<()>>,
}

impl StopHandle {
	/// Stop the server.
	pub fn stop(&mut self) {
		self.shutdown.store(true, Ordering::SeqCst);
	}

	/// Blocks indefinitely until the server is stopped.
	pub async fn wait_for_stop(&mut self) {
		if let Some(stop) = self.stop_handle.take() {
			let _ = stop.await;
		}
	}
}
