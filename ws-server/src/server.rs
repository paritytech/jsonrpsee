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
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::future::{FutureDriver, ServerHandle, StopMonitor};
use crate::types::error::{ErrorCode, ErrorObject, BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG};
use crate::types::{Id, Request};
use futures_channel::{mpsc, oneshot};
use futures_util::future::{Either, FutureExt};
use futures_util::io::{BufReader, BufWriter};
use futures_util::stream::StreamExt;
use http::header::{HOST, ORIGIN};
use http::{HeaderMap, HeaderValue};
use jsonrpsee_core::id_providers::RandomIntegerIdProvider;
use jsonrpsee_core::middleware::Middleware;
use jsonrpsee_core::server::access_control::AccessControl;
use jsonrpsee_core::server::helpers::{
	prepare_error, BatchResponse, BatchResponseBuilder, BoundedSubscriptions, MethodResponse, MethodSink,
};
use jsonrpsee_core::server::resource_limiting::Resources;
use jsonrpsee_core::server::rpc_module::{ConnState, ConnectionId, MethodKind, Methods};
use jsonrpsee_core::tracing::{rx_log_from_json, tx_log_from_str, RpcTracing};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::{Error, TEN_MB_SIZE_BYTES};
use jsonrpsee_types::error::{reject_too_big_request, reject_too_many_subscriptions};
use jsonrpsee_types::Params;
use soketto::connection::Error as SokettoError;
use soketto::data::ByteSlice125;
use soketto::handshake::{server::Response, Server as SokettoServer};
use soketto::Sender;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_stream::wrappers::IntervalStream;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};
use tracing_futures::Instrument;

/// Default maximum connections allowed.
const MAX_CONNECTIONS: u64 = 100;

/// A WebSocket JSON RPC server.
pub struct Server<M> {
	listener: TcpListener,
	cfg: Settings,
	stop_monitor: StopMonitor,
	resources: Resources,
	middleware: M,
	id_provider: Arc<dyn IdProvider>,
}

impl<M> std::fmt::Debug for Server<M> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Server")
			.field("listener", &self.listener)
			.field("cfg", &self.cfg)
			.field("stop_monitor", &self.stop_monitor)
			.field("id_provider", &self.id_provider)
			.field("resources", &self.resources)
			.finish()
	}
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
					let id_provider = self.id_provider.clone();

					connections.add(Box::pin(handshake(
						socket,
						HandshakeResponse::Accept {
							conn_id: id,
							methods,
							resources: &resources,
							cfg,
							stop_monitor: &stop_monitor,
							middleware: middleware.clone(),
							id_provider,
						},
					)));

					tracing::info!("Accepting new connection {}/{}", connections.count(), self.cfg.max_connections);

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
		id_provider: Arc<dyn IdProvider>,
	},
}

async fn handshake<M: Middleware>(socket: tokio::net::TcpStream, mode: HandshakeResponse<'_, M>) -> Result<(), Error> {
	let remote_addr = socket.peer_addr()?;

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
		HandshakeResponse::Accept { conn_id, methods, resources, cfg, stop_monitor, middleware, id_provider } => {
			tracing::debug!("Accepting new connection: {}", conn_id);
			let key_and_headers = {
				let req = server.receive_request().await?;

				let host = std::str::from_utf8(req.headers().host)
					.map_err(|_e| Error::HttpHeaderRejected("Host", "Invalid UTF-8".to_string()))?;
				let origin = req.headers().origin.and_then(|h| {
					let res = std::str::from_utf8(h).ok();
					if res.is_none() {
						tracing::warn!("Origin header invalid UTF-8; treated as no Origin header");
					}
					res
				});

				let host_check = cfg.access_control.verify_host(host);
				let origin_check = cfg.access_control.verify_origin(origin, host);

				host_check.and(origin_check).map(|()| {
					let key = req.key();

					let mut headers = HeaderMap::new();

					if let Ok(val) = HeaderValue::from_str(host) {
						headers.insert(HOST, val);
					}

					if let Some(Ok(val)) = origin.map(HeaderValue::from_str) {
						headers.insert(ORIGIN, val);
					}

					(key, headers)
				})
			};

			let headers = match key_and_headers {
				Ok((key, headers)) => {
					let accept = Response::Accept { key, protocol: None };
					server.send_response(&accept).await?;
					headers
				}
				Err(err) => {
					tracing::warn!("Rejected connection: {:?}", err);
					let reject = Response::Reject { status_code: 403 };
					server.send_response(&reject).await?;

					return Err(err);
				}
			};

			let join_result = tokio::spawn(background_task(BackgroundTask {
				server,
				conn_id,
				methods: methods.clone(),
				resources: resources.clone(),
				max_request_body_size: cfg.max_request_body_size,
				max_response_body_size: cfg.max_response_body_size,
				max_log_length: cfg.max_log_length,
				batch_requests_supported: cfg.batch_requests_supported,
				bounded_subscriptions: BoundedSubscriptions::new(cfg.max_subscriptions_per_connection),
				stop_server: stop_monitor.clone(),
				middleware,
				id_provider,
				ping_interval: cfg.ping_interval,
				remote_addr,
				headers,
			}))
			.await;

			match join_result {
				Err(_) => Err(Error::Custom("Background task was aborted".into())),
				Ok(result) => result,
			}
		}
	}
}

struct BackgroundTask<'a, M> {
	server: SokettoServer<'a, BufReader<BufWriter<Compat<tokio::net::TcpStream>>>>,
	conn_id: ConnectionId,
	methods: Methods,
	resources: Resources,
	max_request_body_size: u32,
	max_response_body_size: u32,
	max_log_length: u32,
	batch_requests_supported: bool,
	bounded_subscriptions: BoundedSubscriptions,
	stop_server: StopMonitor,
	middleware: M,
	id_provider: Arc<dyn IdProvider>,
	ping_interval: Duration,
	remote_addr: SocketAddr,
	headers: HeaderMap,
}

async fn background_task<M: Middleware>(input: BackgroundTask<'_, M>) -> Result<(), Error> {
	let BackgroundTask {
		server,
		conn_id,
		methods,
		resources,
		max_request_body_size,
		max_response_body_size,
		max_log_length,
		batch_requests_supported,
		bounded_subscriptions,
		stop_server,
		middleware,
		id_provider,
		ping_interval,
		remote_addr,
		headers,
	} = input;

	// And we can finally transition to a websocket background_task.
	let mut builder = server.into_builder();
	builder.set_max_message_size(max_request_body_size as usize);
	let (mut sender, mut receiver) = builder.finish();
	let (tx, mut rx) = mpsc::unbounded::<String>();
	let bounded_subscriptions2 = bounded_subscriptions.clone();

	let stop_server2 = stop_server.clone();
	let sink = MethodSink::new_with_limit(tx, max_response_body_size, max_log_length);

	middleware.on_connect();

	// Send results back to the client.
	tokio::spawn(async move {
		// Received messages from the WebSocket.
		let mut rx_item = rx.next();

		// Interval to send out continuously `pings`.
		let ping_interval = IntervalStream::new(tokio::time::interval(ping_interval));
		tokio::pin!(ping_interval);
		let mut next_ping = ping_interval.next();

		while !stop_server2.shutdown_requested() {
			// Ensure select is cancel-safe by fetching and storing the `rx_item` that did not finish yet.
			// Note: Although, this is cancel-safe already, avoid using `select!` macro for future proofing.
			match futures_util::future::select(rx_item, next_ping).await {
				Either::Left((Some(response), ping)) => {
					// If websocket message send fail then terminate the connection.
					if let Err(err) = send_ws_message(&mut sender, response).await {
						tracing::warn!("WS send error: {}; terminate connection", err);
						break;
					}
					rx_item = rx.next();
					next_ping = ping;
				}
				// Nothing else to receive.
				Either::Left((None, _)) => break,

				// Handle timer intervals.
				Either::Right((_, next_rx)) => {
					if let Err(err) = send_ws_ping(&mut sender).await {
						tracing::warn!("WS send ping error: {}; terminate connection", err);
						break;
					}
					rx_item = next_rx;
					next_ping = ping_interval.next();
				}
			}
		}

		// Terminate connection and send close message.
		let _ = sender.close().await;

		// Notify all listeners and close down associated tasks.
		bounded_subscriptions2.close();
	});

	// Buffer for incoming data.
	let mut data = Vec::with_capacity(100);
	let mut method_executors = FutureDriver::default();
	let middleware = &middleware;

	let result = loop {
		data.clear();

		{
			// Need the extra scope to drop this pinned future and reclaim access to `data`
			let receive = async {
				// Identical loop to `soketto::receive_data` with debug logs for `Pong` frames.
				loop {
					match receiver.receive(&mut data).await? {
						soketto::Incoming::Data(d) => break Ok(d),
						soketto::Incoming::Pong(_) => tracing::debug!("recv pong"),
						_ => continue,
					}
				}
			};

			tokio::pin!(receive);

			if let Err(err) = method_executors.select_with(Monitored::new(receive, &stop_server)).await {
				match err {
					MonitoredError::Selector(SokettoError::Closed) => {
						tracing::debug!("WS transport: remote peer terminated the connection: {}", conn_id);
						sink.close();
						break Ok(());
					}
					MonitoredError::Selector(SokettoError::MessageTooLarge { current, maximum }) => {
						tracing::warn!(
							"WS transport error: outgoing message is too big error ({} bytes, max is {})",
							current,
							maximum
						);
						sink.send_error(Id::Null, reject_too_big_request(max_request_body_size));
						continue;
					}
					// These errors can not be gracefully handled, so just log them and terminate the connection.
					MonitoredError::Selector(err) => {
						tracing::warn!("WS error: {}; terminate connection {}", err, conn_id);
						sink.close();
						break Err(err.into());
					}
					MonitoredError::Shutdown => break Ok(()),
				};
			};
		};

		let request_start = middleware.on_request(remote_addr, &headers);

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
						conn_id,
						resources: &resources,
						max_response_body_size,
						methods: &methods,
						bounded_subscriptions,
						sink: &sink,
						id_provider: &*id_provider,
						middleware,
						request_start,
					};

					let (response, maybe_sub) = process_single_request(data, call).await;
					middleware.on_response(&response.result, request_start);
					let _ = sink.send_raw(response.result);

					if let Some(sub) = maybe_sub {
						let _ = sub.send(());
					}
				}
				.boxed();

				method_executors.add(fut);
			}
			Some(b'[') if !batch_requests_supported => {
				let response = MethodResponse::error(
					Id::Null,
					ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, &BATCHES_NOT_SUPPORTED_MSG, None),
				);
				middleware.on_response(&response.result, request_start);
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
							conn_id,
							resources,
							max_response_body_size,
							methods,
							bounded_subscriptions,
							sink: &sink,
							id_provider: &*id_provider,
							middleware,
							request_start,
						},
					})
					.await;

					tx_log_from_str(&response.result, max_log_length);
					middleware.on_response(&response.result, request_start);
					let _ = sink.send_raw(response.result);
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

/// JSON-RPC Websocket server settings.
#[derive(Debug, Clone)]
struct Settings {
	/// Maximum size in bytes of a request.
	max_request_body_size: u32,
	/// Maximum size in bytes of a response.
	max_response_body_size: u32,
	/// Maximum number of incoming connections allowed.
	max_connections: u64,
	/// Maximum number of subscriptions per connection.
	max_subscriptions_per_connection: u32,
	/// Max length for logging for requests and responses
	///
	/// Logs bigger than this limit will be truncated.
	max_log_length: u32,
	/// Access control based on HTTP headers
	access_control: AccessControl,
	/// Whether batch requests are supported by this server or not.
	batch_requests_supported: bool,
	/// Custom tokio runtime to run the server on.
	tokio_runtime: Option<tokio::runtime::Handle>,
	/// The interval at which `Ping` frames are submitted.
	ping_interval: Duration,
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			max_request_body_size: TEN_MB_SIZE_BYTES,
			max_response_body_size: TEN_MB_SIZE_BYTES,
			max_log_length: 4096,
			max_subscriptions_per_connection: 1024,
			max_connections: MAX_CONNECTIONS,
			batch_requests_supported: true,
			access_control: AccessControl::default(),
			tokio_runtime: None,
			ping_interval: Duration::from_secs(60),
		}
	}
}

/// Builder to configure and create a JSON-RPC Websocket server
#[derive(Debug)]
pub struct Builder<M = ()> {
	settings: Settings,
	resources: Resources,
	middleware: M,
	id_provider: Arc<dyn IdProvider>,
}

impl Default for Builder {
	fn default() -> Self {
		Builder {
			settings: Settings::default(),
			resources: Resources::default(),
			middleware: (),
			id_provider: Arc::new(RandomIntegerIdProvider),
		}
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

	/// Set the maximum size of a response body in bytes. Default is 10 MiB.
	pub fn max_response_body_size(mut self, size: u32) -> Self {
		self.settings.max_response_body_size = size;
		self
	}

	/// Set the maximum number of connections allowed. Default is 100.
	pub fn max_connections(mut self, max: u64) -> Self {
		self.settings.max_connections = max;
		self
	}

	/// Enables or disables support of [batch requests](https://www.jsonrpc.org/specification#batch).
	/// By default, support is enabled.
	pub fn batch_requests_supported(mut self, supported: bool) -> Self {
		self.settings.batch_requests_supported = supported;
		self
	}

	/// Set the maximum number of connections allowed. Default is 1024.
	pub fn max_subscriptions_per_connection(mut self, max: u32) -> Self {
		self.settings.max_subscriptions_per_connection = max;
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

	/// Add a middleware to the builder [`Middleware`](../jsonrpsee_core/middleware/trait.Middleware.html).
	///
	/// ```
	/// use std::{time::Instant, net::SocketAddr};
	///
	/// use jsonrpsee_core::middleware::Middleware;
	/// use jsonrpsee_core::HeaderMap;
	/// use jsonrpsee_ws_server::WsServerBuilder;
	///
	/// #[derive(Clone)]
	/// struct MyMiddleware;
	///
	/// impl Middleware for MyMiddleware {
	///     type Instant = Instant;
	///
	///     fn on_request(&self, _remote_addr: SocketAddr, _headers: &HeaderMap) -> Instant {
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
		Builder { settings: self.settings, resources: self.resources, middleware, id_provider: self.id_provider }
	}

	/// Configure a custom [`tokio::runtime::Handle`] to run the server on.
	///
	/// Default: [`tokio::spawn`]
	pub fn custom_tokio_runtime(mut self, rt: tokio::runtime::Handle) -> Self {
		self.settings.tokio_runtime = Some(rt);
		self
	}

	/// Configure the interval at which pings are submitted.
	///
	/// This option is used to keep the connection alive, and is just submitting `Ping` frames,
	/// without making any assumptions about when a `Pong` frame should be received.
	///
	/// Default: 60 seconds.
	///
	/// # Examples
	///
	/// ```rust
	/// use std::time::Duration;
	/// use jsonrpsee_ws_server::WsServerBuilder;
	///
	/// // Set the ping interval to 10 seconds.
	/// let builder = WsServerBuilder::default().ping_interval(Duration::from_secs(10));
	/// ```
	pub fn ping_interval(mut self, interval: Duration) -> Self {
		self.settings.ping_interval = interval;
		self
	}

	/// Configure custom `subscription ID` provider for the server to use
	/// to when getting new subscription calls.
	///
	/// You may choose static dispatch or dynamic dispatch because
	/// `IdProvider` is implemented for `Box<T>`.
	///
	/// Default: [`RandomIntegerIdProvider`].
	///
	/// # Examples
	///
	/// ```rust
	/// use jsonrpsee_ws_server::{WsServerBuilder, RandomStringIdProvider, IdProvider};
	///
	/// // static dispatch
	/// let builder1 = WsServerBuilder::default().set_id_provider(RandomStringIdProvider::new(16));
	///
	/// // or dynamic dispatch
	/// let builder2 = WsServerBuilder::default().set_id_provider(Box::new(RandomStringIdProvider::new(16)));
	/// ```
	///
	pub fn set_id_provider<I: IdProvider + 'static>(mut self, id_provider: I) -> Self {
		self.id_provider = Arc::new(id_provider);
		self
	}

	/// Sets access control settings.
	pub fn set_access_control(mut self, acl: AccessControl) -> Self {
		self.settings.access_control = acl;
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
		Ok(Server {
			listener,
			cfg: self.settings,
			stop_monitor,
			resources,
			middleware: self.middleware,
			id_provider: self.id_provider,
		})
	}
}

async fn send_ws_message(
	sender: &mut Sender<BufReader<BufWriter<Compat<TcpStream>>>>,
	response: String,
) -> Result<(), Error> {
	sender.send_text_owned(response).await?;
	sender.flush().await.map_err(Into::into)
}

async fn send_ws_ping(sender: &mut Sender<BufReader<BufWriter<Compat<TcpStream>>>>) -> Result<(), Error> {
	tracing::debug!("send ping");
	// Submit empty slice as "optional" parameter.
	let slice: &[u8] = &[];
	// Byte slice fails if the provided slice is larger than 125 bytes.
	let byte_slice = ByteSlice125::try_from(slice).expect("Empty slice should fit into ByteSlice125");
	sender.send_ping(byte_slice).await?;
	sender.flush().await.map_err(Into::into)
}

#[derive(Debug, Clone)]
struct Batch<'a, M: Middleware> {
	data: Vec<u8>,
	call: CallData<'a, M>,
}

#[derive(Debug, Clone)]
struct CallData<'a, M: Middleware> {
	conn_id: usize,
	bounded_subscriptions: BoundedSubscriptions,
	id_provider: &'a dyn IdProvider,
	middleware: &'a M,
	methods: &'a Methods,
	max_response_body_size: u32,
	resources: &'a Resources,
	sink: &'a MethodSink,
	request_start: M::Instant,
}

#[derive(Debug, Clone)]
struct Call<'a, M: Middleware> {
	params: Params<'a>,
	name: &'a str,
	call: CallData<'a, M>,
	id: Id<'a>,
}

// Batch responses must be sent back as a single message so we read the results from each
// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
// complete batch response back to the client over `tx`.
async fn process_batch_request<M>(b: Batch<'_, M>) -> BatchResponse
where
	M: Middleware,
{
	let Batch { data, call } = b;

	if let Ok(batch) = serde_json::from_slice::<Vec<Request>>(&data) {
		tracing::debug!("recv batch len={}", batch.len());
		tracing::trace!("recv: batch={:?}", batch);
		return if !batch.is_empty() {
			let batch = batch.into_iter().map(|req| (req, call.clone()));

			let batch_stream = futures_util::stream::iter(batch);

			let batch_response = batch_stream
				.fold(BatchResponseBuilder::new(), |mut batch_response, (req, call)| async move {
					let params = Params::new(req.params.map(|params| params.get()));

					let (response, maybe_sub) =
						execute_call(Call { name: &req.method, params, id: req.id, call }).await;

					if let Some(sub) = maybe_sub {
						let _ = sub.send(());
					}

					batch_response.append(&response);

					batch_response
				})
				.await;

			return batch_response.finish();
		} else {
			BatchResponse::error(Id::Null, ErrorObject::from(ErrorCode::InvalidRequest))
		};
	}

	let (id, code) = prepare_error(&data);
	BatchResponse::error(id, ErrorObject::from(code))
}

async fn process_single_request<M: Middleware>(
	data: Vec<u8>,
	call: CallData<'_, M>,
) -> (MethodResponse, Option<oneshot::Sender<()>>) {
	if let Ok(req) = serde_json::from_slice::<Request>(&data) {
		tracing::debug!("recv method call={}", req.method);
		tracing::trace!("recv: req={:?}", req);
		let params = Params::new(req.params.map(|params| params.get()));
		let name = &req.method;
		let id = req.id;

		execute_call(Call { name, params, id, call }).await
	} else {
		let (id, code) = prepare_error(&data);
		(MethodResponse::error(id, ErrorObject::from(code)), None)
	}
}

/// This a workaround the see whether it works
///
/// Essentially, the sender is used to indicate to the other side that call has been answered
/// such that the subscription notifications are not allowed to start until `the sender` has ACK:ed
/// that.
///
/// Otherwise it's possible that the subscription notifications could start before that the actual subscription
/// response has been sent.
async fn execute_call<M: Middleware>(c: Call<'_, M>) -> (MethodResponse, Option<oneshot::Sender<()>>) {
	let Call { name, id, params, call } = c;
	let CallData {
		resources,
		methods,
		middleware,
		max_response_body_size,
		conn_id,
		bounded_subscriptions,
		id_provider,
		sink,
		request_start,
	} = call;

	middleware.on_call(name, params.clone());

	let response = match methods.method_with_name(name) {
		None => (MethodResponse::error(id, ErrorObject::from(ErrorCode::MethodNotFound)), None),
		Some((name, method)) => match &method.inner() {
			MethodKind::Sync(callback) => match method.claim(name, resources) {
				Ok(guard) => {
					let r = (callback)(id, params, max_response_body_size as usize);
					drop(guard);
					(r, None)
				}
				Err(err) => {
					tracing::error!("[Methods::execute_with_resources] failed to lock resources: {:?}", err);
					(MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy)), None)
				}
			},
			MethodKind::Async(callback) => match method.claim(name, resources) {
				Ok(guard) => {
					let id = id.into_owned();
					let params = params.into_owned();

					((callback)(id, params, conn_id, max_response_body_size as usize, Some(guard)).await, None)
				}
				Err(err) => {
					tracing::error!("[Methods::execute_with_resources] failed to lock resources: {:?}", err);
					(MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy)), None)
				}
			},
			MethodKind::Subscription(callback) => match method.claim(name, resources) {
				Ok(guard) => {
					if let Some(cn) = bounded_subscriptions.acquire() {
						let conn_state = ConnState { conn_id, close_notify: cn, id_provider };
						let (subscribe_tx, subscribe_rx) = oneshot::channel();
						let result =
							callback(id.clone(), params, sink.clone(), conn_state, subscribe_rx, Some(guard)).await;
						(result, Some(subscribe_tx))
					} else {
						(MethodResponse::error(id, reject_too_many_subscriptions(bounded_subscriptions.max())), None)
					}
				}
				Err(err) => {
					tracing::error!("[Methods::execute_with_resources] failed to lock resources: {:?}", err);
					(MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy)), None)
				}
			},
			MethodKind::Unsubscription(callback) => {
				// Don't adhere to any resource or subscription limits; always let unsubscribing happen!
				(callback(id, params, conn_id, max_response_body_size as usize), None)
			}
		},
	};

	middleware.on_result(name, response.0.success, request_start);
	response
}
