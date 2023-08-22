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

use std::error::Error as StdError;
use std::future::Future;
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::future::{ConnectionGuard, ServerHandle, StopHandle};
use crate::logger::{Logger, TransportProtocol};
use crate::transport::{http, ws};

use futures_util::future::{self, Either, FutureExt};
use futures_util::io::{BufReader, BufWriter};

use hyper::body::HttpBody;
use jsonrpsee_core::id_providers::RandomIntegerIdProvider;

use jsonrpsee_core::server::Methods;
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::{Error, TEN_MB_SIZE_BYTES};

use soketto::handshake::http::is_upgrade_request;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::sync::{mpsc, watch, OwnedSemaphorePermit};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tower::layer::util::Identity;
use tower::{Layer, Service};
use tracing::{instrument, Instrument};

/// Default maximum connections allowed.
const MAX_CONNECTIONS: u32 = 100;

/// JSON RPC server.
pub struct Server<B = Identity, L = ()> {
	listener: TcpListener,
	cfg: Settings,
	logger: L,
	id_provider: Arc<dyn IdProvider>,
	service_builder: tower::ServiceBuilder<B>,
}

impl Server<Identity, ()> {
	/// Create a builder for the server.
	pub fn builder() -> Builder {
		Builder::new()
	}
}

impl<L> std::fmt::Debug for Server<L> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Server")
			.field("listener", &self.listener)
			.field("cfg", &self.cfg)
			.field("id_provider", &self.id_provider)
			.finish()
	}
}

impl<B, L> Server<B, L> {
	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.listener.local_addr().map_err(Into::into)
	}
}

impl<S, B, L> Server<S, L>
where
	L: Logger,
	S: Layer<TowerService<L>> + Send + 'static,
	<S as Layer<TowerService<L>>>::Service: Send
		+ Service<
			hyper::Request<hyper::Body>,
			Response = hyper::Response<B>,
			Error = Box<(dyn StdError + Send + Sync + 'static)>,
		>,
	<<S as Layer<TowerService<L>>>::Service as Service<hyper::Request<hyper::Body>>>::Future: Send,
	B: HttpBody + Send + 'static,
	<B as HttpBody>::Error: Send + Sync + StdError,
	<B as HttpBody>::Data: Send,
{
	/// Start responding to connections requests.
	///
	/// This will run on the tokio runtime until the server is stopped or the `ServerHandle` is dropped.
	pub fn start(mut self, methods: impl Into<Methods>) -> ServerHandle {
		let methods = methods.into();
		let (stop_tx, stop_rx) = watch::channel(());

		let stop_handle = StopHandle::new(stop_rx);

		match self.cfg.tokio_runtime.take() {
			Some(rt) => rt.spawn(self.start_inner(methods, stop_handle)),
			None => tokio::spawn(self.start_inner(methods, stop_handle)),
		};

		ServerHandle::new(stop_tx)
	}

	async fn start_inner(self, methods: Methods, stop_handle: StopHandle) {
		let max_request_body_size = self.cfg.max_request_body_size;
		let max_response_body_size = self.cfg.max_response_body_size;
		let max_log_length = self.cfg.max_log_length;
		let max_subscriptions_per_connection = self.cfg.max_subscriptions_per_connection;
		let logger = self.logger;
		let batch_requests_config = self.cfg.batch_requests_config;
		let id_provider = self.id_provider;

		let mut id: u32 = 0;
		let connection_guard = ConnectionGuard::new(self.cfg.max_connections as usize);
		let listener = self.listener;

		let stopped = stop_handle.clone().shutdown();
		tokio::pin!(stopped);

		let (drop_on_completion, mut process_connection_awaiter) = mpsc::channel::<()>(1);

		loop {
			match try_accept_conn(&listener, stopped).await {
				AcceptConnection::Established { socket, remote_addr, stop } => {
					let data = ProcessConnection {
						remote_addr,
						methods: methods.clone(),
						max_request_body_size,
						max_response_body_size,
						max_log_length,
						max_subscriptions_per_connection,
						batch_requests_config,
						id_provider: id_provider.clone(),
						ping_config: self.cfg.ping_config,
						stop_handle: stop_handle.clone(),
						conn_id: id,
						logger: logger.clone(),
						max_connections: self.cfg.max_connections,
						enable_http: self.cfg.enable_http,
						enable_ws: self.cfg.enable_ws,
						message_buffer_capacity: self.cfg.message_buffer_capacity,
					};

					process_connection(
						&self.service_builder,
						&connection_guard,
						data,
						socket,
						drop_on_completion.clone(),
					);
					id = id.wrapping_add(1);
					stopped = stop;
				}
				AcceptConnection::Err((e, stop)) => {
					tracing::debug!("Error while awaiting a new connection: {:?}", e);
					stopped = stop;
				}
				AcceptConnection::Shutdown => break,
			}
		}

		// Drop the last Sender
		drop(drop_on_completion);

		// Once this channel is closed it is safe to assume that all connections have been gracefully shutdown
		while process_connection_awaiter.recv().await.is_some() {
			// Generally, messages should not be sent across this channel,
			// but we'll loop here to wait for `None` just to be on the safe side
		}
	}
}

/// JSON-RPC Websocket server settings.
#[derive(Debug, Clone)]
struct Settings {
	/// Maximum size in bytes of a request.
	max_request_body_size: u32,
	/// Maximum size in bytes of a response.
	max_response_body_size: u32,
	/// Maximum number of incoming connections allowed.
	max_connections: u32,
	/// Max length for logging for requests and responses
	///
	/// Logs bigger than this limit will be truncated.
	max_log_length: u32,
	/// Maximum number of subscriptions per connection.
	max_subscriptions_per_connection: u32,
	/// Whether batch requests are supported by this server or not.
	batch_requests_config: BatchRequestConfig,
	/// Custom tokio runtime to run the server on.
	tokio_runtime: Option<tokio::runtime::Handle>,
	/// Enable HTTP.
	enable_http: bool,
	/// Enable WS.
	enable_ws: bool,
	/// Number of messages that server is allowed to `buffer` until backpressure kicks in.
	message_buffer_capacity: u32,
	/// Ping settings.
	ping_config: PingConfig,
}

/// Configuration for batch request handling.
#[derive(Debug, Copy, Clone)]
pub enum BatchRequestConfig {
	/// Batch requests are disabled.
	Disabled,
	/// Each batch request is limited to `len` and any batch request bigger than `len` will not be processed.
	Limit(u32),
	/// The batch request is unlimited.
	Unlimited,
}

/// Configuration for WebSocket ping's.
///
/// If the server sends out a ping then remote peer must reply with a corresponding pong message.
///
/// It's possible to just send out pings then don't care about response
/// or terminate the connection if the ping isn't replied to the configured `max_inactivity` limit.
///
/// NOTE: It's possible that a `ping` may be backpressured and if you expect a connection
/// to be reassumed after interruption it's not recommended to enable the activity check.
#[derive(Debug, Copy, Clone)]
pub enum PingConfig {
	/// The server pings the connected clients continuously at the configured interval but
	/// doesn't disconnect them if no pongs are received from the client.
	WithoutInactivityCheck(Duration),
	/// The server pings the connected clients continuously at the configured interval
	/// and terminates the connection if no websocket messages received from client
	/// after the max limit is exceeded.
	WithInactivityCheck {
		/// Time interval between consequent pings from server
		ping_interval: Duration,
		/// Max allowed time for connection to stay idle
		inactive_limit: Duration,
	},
}

impl PingConfig {
	pub(crate) fn ping_interval(&self) -> Duration {
		match self {
			Self::WithoutInactivityCheck(ping_interval) => *ping_interval,
			Self::WithInactivityCheck { ping_interval, .. } => *ping_interval,
		}
	}

	pub(crate) fn inactive_limit(&self) -> Option<Duration> {
		if let Self::WithInactivityCheck { inactive_limit, .. } = self {
			Some(*inactive_limit)
		} else {
			None
		}
	}
}

impl Default for PingConfig {
	fn default() -> Self {
		Self::WithoutInactivityCheck(Duration::from_secs(60))
	}
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			max_request_body_size: TEN_MB_SIZE_BYTES,
			max_response_body_size: TEN_MB_SIZE_BYTES,
			max_log_length: 4096,
			max_connections: MAX_CONNECTIONS,
			max_subscriptions_per_connection: 1024,
			batch_requests_config: BatchRequestConfig::Unlimited,
			tokio_runtime: None,
			enable_http: true,
			enable_ws: true,
			message_buffer_capacity: 1024,
			ping_config: PingConfig::WithoutInactivityCheck(Duration::from_secs(60)),
		}
	}
}

/// Builder to configure and create a JSON-RPC server
#[derive(Debug)]
pub struct Builder<B = Identity, L = ()> {
	settings: Settings,
	logger: L,
	id_provider: Arc<dyn IdProvider>,
	service_builder: tower::ServiceBuilder<B>,
}

impl Default for Builder {
	fn default() -> Self {
		Builder {
			settings: Settings::default(),
			logger: (),
			id_provider: Arc::new(RandomIntegerIdProvider),
			service_builder: tower::ServiceBuilder::new(),
		}
	}
}

impl Builder {
	/// Create a default server builder.
	pub fn new() -> Self {
		Self::default()
	}
}

impl<B, L> Builder<B, L> {
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
	pub fn max_connections(mut self, max: u32) -> Self {
		self.settings.max_connections = max;
		self
	}

	/// Configure how [batch requests](https://www.jsonrpc.org/specification#batch) shall be handled
	/// by the server.
	///
	/// Default: batch requests are allowed and can be arbitrary big but the maximum payload size is limited.
	pub fn set_batch_request_config(mut self, cfg: BatchRequestConfig) -> Self {
		self.settings.batch_requests_config = cfg;
		self
	}

	/// Set the maximum number of connections allowed. Default is 1024.
	pub fn max_subscriptions_per_connection(mut self, max: u32) -> Self {
		self.settings.max_subscriptions_per_connection = max;
		self
	}

	/// Add a logger to the builder [`Logger`](../jsonrpsee_core/logger/trait.Logger.html).
	///
	/// ```
	/// use std::{time::Instant, net::SocketAddr};
	///
	/// use jsonrpsee_server::logger::{Logger, HttpRequest, MethodKind, Params, TransportProtocol, SuccessOrError};
	/// use jsonrpsee_server::ServerBuilder;
	///
	/// #[derive(Clone)]
	/// struct MyLogger;
	///
	/// impl Logger for MyLogger {
	///     type Instant = Instant;
	///
	///     fn on_connect(&self, remote_addr: SocketAddr, request: &HttpRequest, transport: TransportProtocol) {
	///          println!("[MyLogger::on_call] remote_addr: {:?}, headers: {:?}, transport: {}", remote_addr, request, transport);
	///     }
	///
	///     fn on_request(&self, transport: TransportProtocol) -> Self::Instant {
	///          Instant::now()
	///     }
	///
	///     fn on_call(&self, method_name: &str, params: Params, kind: MethodKind, transport: TransportProtocol) {
	///          println!("[MyLogger::on_call] method: '{}' params: {:?}, kind: {:?}, transport: {}", method_name, params, kind, transport);
	///     }
	///
	///     fn on_result(&self, method_name: &str, success_or_error: SuccessOrError, started_at: Self::Instant, transport: TransportProtocol) {
	///          println!("[MyLogger::on_result] '{}', worked? {}, time elapsed {:?}, transport: {}", method_name, success_or_error.is_success(), started_at.elapsed(), transport);
	///     }
	///
	///     fn on_response(&self, result: &str, started_at: Self::Instant, transport: TransportProtocol) {
	///          println!("[MyLogger::on_response] result: {}, time elapsed {:?}, transport: {}", result, started_at.elapsed(), transport);
	///     }
	///
	///     fn on_disconnect(&self, remote_addr: SocketAddr, transport: TransportProtocol) {
	///          println!("[MyLogger::on_disconnect] remote_addr: {:?}, transport: {}", remote_addr, transport);
	///     }
	/// }
	///
	/// let builder = ServerBuilder::new().set_logger(MyLogger);
	/// ```
	pub fn set_logger<T: Logger>(self, logger: T) -> Builder<B, T> {
		Builder {
			settings: self.settings,
			logger,
			id_provider: self.id_provider,
			service_builder: self.service_builder,
		}
	}

	/// Configure a custom [`tokio::runtime::Handle`] to run the server on.
	///
	/// Default: [`tokio::spawn`]
	pub fn custom_tokio_runtime(mut self, rt: tokio::runtime::Handle) -> Self {
		self.settings.tokio_runtime = Some(rt);
		self
	}

	/// Configure the interval at which pings are submitted,
	/// and optionally enable connection inactivity check
	///
	/// This option is used to keep the connection alive, and can be configured to just submit `Ping` frames or with extra parameter, configuring max interval when a `Pong` frame should be received
	///
	/// Default: ping interval is set to 60 seconds and the inactivity check is disabled
	///
	/// # Examples
	///
	/// ```rust
	/// use std::time::Duration;
	/// use jsonrpsee_server::{ServerBuilder, PingConfig};
	///
	/// // Set the ping interval to 10 seconds but terminate the connection if a client is inactive for more than 2 minutes
	/// let builder = ServerBuilder::default().ping_interval(PingConfig::WithInactivityCheck { ping_interval: Duration::from_secs(10), inactive_limit: Duration::from_secs(2 * 60) }).unwrap();
	/// ```
	pub fn ping_interval(mut self, config: PingConfig) -> Result<Self, Error> {
		if let PingConfig::WithInactivityCheck { ping_interval, inactive_limit } = config {
			if ping_interval >= inactive_limit {
				return Err(Error::Custom("`inactive_limit` must be bigger than `ping_interval` to work".into()));
			}
		}

		self.settings.ping_config = config;
		Ok(self)
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
	/// use jsonrpsee_server::{ServerBuilder, RandomStringIdProvider, IdProvider};
	///
	/// // static dispatch
	/// let builder1 = ServerBuilder::default().set_id_provider(RandomStringIdProvider::new(16));
	///
	/// // or dynamic dispatch
	/// let builder2 = ServerBuilder::default().set_id_provider(Box::new(RandomStringIdProvider::new(16)));
	/// ```
	///
	pub fn set_id_provider<I: IdProvider + 'static>(mut self, id_provider: I) -> Self {
		self.id_provider = Arc::new(id_provider);
		self
	}

	/// Configure a custom [`tower::ServiceBuilder`] middleware for composing layers to be applied to the RPC service.
	///
	/// Default: No tower layers are applied to the RPC service.
	///
	/// # Examples
	///
	/// ```rust
	///
	/// use std::time::Duration;
	/// use std::net::SocketAddr;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let builder = tower::ServiceBuilder::new().timeout(Duration::from_secs(2));
	///
	///     let server = jsonrpsee_server::ServerBuilder::new()
	///         .set_middleware(builder)
	///         .build("127.0.0.1:0".parse::<SocketAddr>().unwrap())
	///         .await
	///         .unwrap();
	/// }
	/// ```
	pub fn set_middleware<T>(self, service_builder: tower::ServiceBuilder<T>) -> Builder<T, L> {
		Builder { settings: self.settings, logger: self.logger, id_provider: self.id_provider, service_builder }
	}

	/// Configure the server to only serve JSON-RPC HTTP requests.
	///
	/// Default: both http and ws are enabled.
	pub fn http_only(mut self) -> Self {
		self.settings.enable_http = true;
		self.settings.enable_ws = false;
		self
	}

	/// Configure the server to only serve JSON-RPC WebSocket requests.
	///
	/// That implies that server just denies HTTP requests which isn't a WebSocket upgrade request
	///
	/// Default: both http and ws are enabled.
	pub fn ws_only(mut self) -> Self {
		self.settings.enable_http = false;
		self.settings.enable_ws = true;
		self
	}

	/// The server enforces backpressure which means that
	/// `n` messages can be buffered and if the client
	/// can't keep with up the server.
	///
	/// This `capacity` is applied per connection and
	/// applies globally on the connection which implies
	/// all JSON-RPC messages.
	///
	/// For example if a subscription produces plenty of new items
	/// and the client can't keep up then no new messages are handled.
	///
	/// If this limit is exceeded then the server will "back-off"
	/// and only accept new messages once the client reads pending messages.
	///
	/// # Panics
	///
	/// Panics if the buffer capacity is 0.
	///
	pub fn set_message_buffer_capacity(mut self, c: u32) -> Self {
		self.settings.message_buffer_capacity = c;
		self
	}

	/// Set maximum length for logging calls and responses.
	///
	/// Logs bigger than this limit will be truncated.
	pub fn set_max_logging_length(mut self, max: u32) -> Self {
		self.settings.max_log_length = max;
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
	///   assert!(jsonrpsee_server::ServerBuilder::default().build(occupied_addr).await.is_err());
	///   assert!(jsonrpsee_server::ServerBuilder::default().build(addrs).await.is_ok());
	/// }
	/// ```
	///
	pub async fn build(self, addrs: impl ToSocketAddrs) -> Result<Server<B, L>, Error> {
		let listener = TcpListener::bind(addrs).await?;

		Ok(Server {
			listener,
			cfg: self.settings,
			logger: self.logger,
			id_provider: self.id_provider,
			service_builder: self.service_builder,
		})
	}

	/// Finalizes the configuration of the server with customized TCP settings on the socket.
	///
	///
	/// ```rust
	/// use jsonrpsee_server::ServerBuilder;
	/// use socket2::{Domain, Socket, Type};
	/// use std::time::Duration;
	///
	/// #[tokio::main]
	/// async fn main() {
	///   let addr = "127.0.0.1:0".parse().unwrap();
	///   let domain = Domain::for_address(addr);
	///   let socket = Socket::new(domain, Type::STREAM, None).unwrap();
	///   socket.set_nonblocking(true).unwrap();
	///
	///   let address = addr.into();
	///   socket.bind(&address).unwrap();
	///
	///   socket.listen(4096).unwrap();
	///
	///   let server = ServerBuilder::new().build_from_tcp(socket).unwrap();
	/// }
	/// ```
	pub fn build_from_tcp(self, listener: impl Into<StdTcpListener>) -> Result<Server<B, L>, Error> {
		let listener = TcpListener::from_std(listener.into())?;

		Ok(Server {
			listener,
			cfg: self.settings,
			logger: self.logger,
			id_provider: self.id_provider,
			service_builder: self.service_builder,
		})
	}
}

/// Data required by the server to handle requests.
#[derive(Debug, Clone)]
pub(crate) struct ServiceData<L: Logger> {
	/// Remote server address.
	pub(crate) remote_addr: SocketAddr,
	/// Registered server methods.
	pub(crate) methods: Methods,
	/// Max request body size.
	pub(crate) max_request_body_size: u32,
	/// Max response body size.
	pub(crate) max_response_body_size: u32,
	/// Max length for logging for request and response
	///
	/// Logs bigger than this limit will be truncated.
	pub(crate) max_log_length: u32,
	/// Maximum number of subscriptions per connection.
	pub(crate) max_subscriptions_per_connection: u32,
	/// Whether batch requests are supported by this server or not.
	pub(crate) batch_requests_config: BatchRequestConfig,
	/// Subscription ID provider.
	pub(crate) id_provider: Arc<dyn IdProvider>,
	/// Ping configuration.
	pub(crate) ping_config: PingConfig,
	/// Stop handle.
	pub(crate) stop_handle: StopHandle,
	/// Connection ID
	pub(crate) conn_id: u32,
	/// Logger.
	pub(crate) logger: L,
	/// Handle to hold a `connection permit`.
	pub(crate) conn: Arc<OwnedSemaphorePermit>,
	/// Enable HTTP.
	pub(crate) enable_http: bool,
	/// Enable WS.
	pub(crate) enable_ws: bool,
	/// Number of messages that server is allowed `buffer` until backpressure kicks in.
	pub(crate) message_buffer_capacity: u32,
}

/// JsonRPSee service compatible with `tower`.
///
/// # Note
/// This is similar to [`hyper::service::service_fn`].
#[derive(Debug, Clone)]
pub struct TowerService<L: Logger> {
	inner: ServiceData<L>,
}

impl<L: Logger> hyper::service::Service<hyper::Request<hyper::Body>> for TowerService<L> {
	type Response = hyper::Response<hyper::Body>;

	// The following associated type is required by the `impl<B, U, L: Logger> Server<B, L>` bounds.
	// It satisfies the server's bounds when the `tower::ServiceBuilder<B>` is not set (ie `B: Identity`).
	type Error = Box<dyn StdError + Send + Sync + 'static>;

	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

	/// Opens door for back pressure implementation.
	fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
		Poll::Ready(Ok(()))
	}

	fn call(&mut self, request: hyper::Request<hyper::Body>) -> Self::Future {
		tracing::trace!("{:?}", request);

		let is_upgrade_request = is_upgrade_request(&request);

		if self.inner.enable_ws && is_upgrade_request {
			let mut server = soketto::handshake::http::Server::new();

			let response = match server.receive_request(&request) {
				Ok(response) => {
					self.inner.logger.on_connect(self.inner.remote_addr, &request, TransportProtocol::WebSocket);
					let data = self.inner.clone();

					tokio::spawn(
						async move {
							let upgraded = match hyper::upgrade::on(request).await {
								Ok(u) => u,
								Err(e) => {
									tracing::debug!("Could not upgrade connection: {}", e);
									return;
								}
							};

							let stream = BufReader::new(BufWriter::new(upgraded.compat()));
							let mut ws_builder = server.into_builder(stream);
							ws_builder.set_max_message_size(data.max_request_body_size as usize);
							let (sender, receiver) = ws_builder.finish();

							ws::background_task::<L>(sender, receiver, data).await;
						}
						.in_current_span(),
					);

					response.map(|()| hyper::Body::empty())
				}
				Err(e) => {
					tracing::debug!("Could not upgrade connection: {}", e);
					hyper::Response::new(hyper::Body::from(format!("Could not upgrade connection: {e}")))
				}
			};

			async { Ok(response) }.boxed()
		} else if self.inner.enable_http && !is_upgrade_request {
			// The request wasn't an upgrade request; let's treat it as a standard HTTP request:
			let data = http::HandleRequest {
				methods: self.inner.methods.clone(),
				max_request_body_size: self.inner.max_request_body_size,
				max_response_body_size: self.inner.max_response_body_size,
				max_log_length: self.inner.max_log_length,
				batch_requests_config: self.inner.batch_requests_config,
				logger: self.inner.logger.clone(),
				conn: self.inner.conn.clone(),
				remote_addr: self.inner.remote_addr,
			};

			self.inner.logger.on_connect(self.inner.remote_addr, &request, TransportProtocol::Http);

			Box::pin(http::handle_request(request, data).map(Ok))
		} else {
			Box::pin(async { http::response::denied() }.map(Ok))
		}
	}
}

struct ProcessConnection<L> {
	/// Remote server address.
	remote_addr: SocketAddr,
	/// Registered server methods.
	methods: Methods,
	/// Max request body size.
	max_request_body_size: u32,
	/// Max response body size.
	max_response_body_size: u32,
	/// Max length for logging for request and response
	///
	/// Logs bigger than this limit will be truncated.
	max_log_length: u32,
	/// Maximum number of subscriptions per connection.
	max_subscriptions_per_connection: u32,
	/// Whether batch requests are supported by this server or not.
	batch_requests_config: BatchRequestConfig,
	/// Subscription ID provider.
	id_provider: Arc<dyn IdProvider>,
	/// Ping config.
	ping_config: PingConfig,
	/// Stop handle.
	stop_handle: StopHandle,
	/// Max connections,
	max_connections: u32,
	/// Connection ID
	conn_id: u32,
	/// Logger.
	logger: L,
	/// Allow JSON-RPC HTTP requests.
	enable_http: bool,
	/// Allow JSON-RPC WS request and WS upgrade requests.
	enable_ws: bool,
	/// Number of messages that server is allowed `buffer` until backpressure kicks in.
	message_buffer_capacity: u32,
}

#[instrument(name = "connection", skip_all, fields(remote_addr = %cfg.remote_addr, conn_id = %cfg.conn_id), level = "INFO")]
fn process_connection<'a, L: Logger, B, U>(
	service_builder: &tower::ServiceBuilder<B>,
	connection_guard: &ConnectionGuard,
	cfg: ProcessConnection<L>,
	socket: TcpStream,
	drop_on_completion: mpsc::Sender<()>,
) where
	B: Layer<TowerService<L>> + Send + 'static,
	<B as Layer<TowerService<L>>>::Service: Send
		+ Service<
			hyper::Request<hyper::Body>,
			Response = hyper::Response<U>,
			Error = Box<(dyn StdError + Send + Sync + 'static)>,
		>,
	<<B as Layer<TowerService<L>>>::Service as Service<hyper::Request<hyper::Body>>>::Future: Send,
	U: HttpBody + Send + 'static,
	<U as HttpBody>::Error: Send + Sync + StdError,
	<U as HttpBody>::Data: Send,
{
	if let Err(e) = socket.set_nodelay(true) {
		tracing::warn!("Could not set NODELAY on socket: {:?}", e);
		return;
	}

	let conn = match connection_guard.try_acquire() {
		Some(conn) => conn,
		None => {
			tracing::debug!("Too many connections. Please try again later.");
			tokio::spawn(async {
				http::reject_connection(socket).in_current_span().await;
				drop(drop_on_completion);
			});
			return;
		}
	};

	let max_conns = cfg.max_connections as usize;
	let curr_conns = max_conns - connection_guard.available_connections();
	tracing::debug!("Accepting new connection {}/{}", curr_conns, max_conns);

	let tower_service = TowerService {
		inner: ServiceData {
			remote_addr: cfg.remote_addr,
			methods: cfg.methods,
			max_request_body_size: cfg.max_request_body_size,
			max_response_body_size: cfg.max_response_body_size,
			max_log_length: cfg.max_log_length,
			max_subscriptions_per_connection: cfg.max_subscriptions_per_connection,
			batch_requests_config: cfg.batch_requests_config,
			id_provider: cfg.id_provider,
			ping_config: cfg.ping_config,
			stop_handle: cfg.stop_handle.clone(),
			conn_id: cfg.conn_id,
			logger: cfg.logger,
			conn: Arc::new(conn),
			enable_http: cfg.enable_http,
			enable_ws: cfg.enable_ws,
			message_buffer_capacity: cfg.message_buffer_capacity,
		},
	};

	let service = service_builder.service(tower_service);

	tokio::spawn(async {
		to_http_service(socket, service, cfg.stop_handle).in_current_span().await;
		drop(drop_on_completion)
	});
}

// Attempts to create a HTTP connection from a socket.
async fn to_http_service<S, B>(socket: TcpStream, service: S, stop_handle: StopHandle)
where
	S: Service<hyper::Request<hyper::Body>, Response = hyper::Response<B>> + Send + 'static,
	S::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
	S::Future: Send,
	B: HttpBody + Send + 'static,
	<B as HttpBody>::Error: Send + Sync + StdError,
	<B as HttpBody>::Data: Send,
{
	let conn = hyper::server::conn::Http::new().serve_connection(socket, service).with_upgrades();
	let stopped = stop_handle.shutdown();

	tokio::pin!(stopped);

	let res = match future::select(conn, stopped).await {
		Either::Left((conn, _)) => conn,
		Either::Right((_, mut conn)) => {
			// NOTE: the connection should continue to be polled until shutdown can finish.
			// Thus, both lines below are needed and not a nit.
			Pin::new(&mut conn).graceful_shutdown();
			conn.await
		}
	};

	if let Err(e) = res {
		tracing::debug!("HTTP serve connection failed {:?}", e);
	}
}

enum AcceptConnection<S> {
	Shutdown,
	Established { socket: TcpStream, remote_addr: SocketAddr, stop: S },
	Err((std::io::Error, S)),
}

async fn try_accept_conn<S>(listener: &TcpListener, stopped: S) -> AcceptConnection<S>
where
	S: Future + Unpin,
{
	let accept = listener.accept();
	tokio::pin!(accept);

	match futures_util::future::select(accept, stopped).await {
		Either::Left((res, stop)) => match res {
			Ok((socket, remote_addr)) => AcceptConnection::Established { socket, remote_addr, stop },
			Err(e) => AcceptConnection::Err((e, stop)),
		},
		Either::Right(_) => AcceptConnection::Shutdown,
	}
}
