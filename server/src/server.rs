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
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use crate::future::{ConnectionGuard, FutureDriver, ServerHandle, StopMonitor};
use crate::logger::Logger;
use crate::transport::{http, ws};

use futures_util::future::FutureExt;
use futures_util::io::{BufReader, BufWriter};

use hyper::body::HttpBody;
use jsonrpsee_core::id_providers::RandomIntegerIdProvider;

use jsonrpsee_core::server::helpers::MethodResponse;
use jsonrpsee_core::server::host_filtering::AllowHosts;
use jsonrpsee_core::server::resource_limiting::Resources;
use jsonrpsee_core::server::rpc_module::Methods;
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::{http_helpers, Error, TEN_MB_SIZE_BYTES};

use soketto::handshake::http::is_upgrade_request;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::sync::OwnedSemaphorePermit;
use tokio_util::compat::TokioAsyncReadCompatExt;
use tower::layer::util::Identity;
use tower::{Layer, Service};

/// Default maximum connections allowed.
const MAX_CONNECTIONS: u32 = 100;

/// JSON RPC server.
pub struct Server<B = Identity, L = ()> {
	listener: TcpListener,
	cfg: Settings,
	stop_monitor: StopMonitor,
	resources: Resources,
	logger: L,
	id_provider: Arc<dyn IdProvider>,
	service_builder: tower::ServiceBuilder<B>,
}

impl<L> std::fmt::Debug for Server<L> {
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

impl<B, L> Server<B, L> {
	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.listener.local_addr().map_err(Into::into)
	}

	/// Returns the handle to stop the running server.
	pub fn server_handle(&self) -> ServerHandle {
		self.stop_monitor.handle()
	}
}

impl<B, U, L> Server<B, L>
where
	L: Logger,
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
		let max_request_body_size = self.cfg.max_request_body_size;
		let max_response_body_size = self.cfg.max_response_body_size;
		let max_log_length = self.cfg.max_log_length;
		let allow_hosts = self.cfg.allow_hosts;
		let resources = self.resources;
		let listener = self.listener;
		let logger = self.logger;
		let batch_requests_supported = self.cfg.batch_requests_supported;
		let id_provider = self.id_provider;
		let stop_monitor = self.stop_monitor;
		let max_subscriptions_per_connection = self.cfg.max_subscriptions_per_connection;

		let mut id: u32 = 0;
		let mut connections = FutureDriver::default();
		let mut incoming = Monitored::new(Incoming(listener), &stop_monitor);
		let connection_guard = ConnectionGuard::new(self.cfg.max_connections as usize);

		loop {
			match connections.select_with(&mut incoming).await {
				Ok((socket, _addr)) => {
					if let Err(e) = socket.set_nodelay(true) {
						tracing::warn!("Could not set NODELAY on socket: {:?}", e);
						continue;
					}

					let conn = match connection_guard.try_acquire() {
						Some(conn) => conn,
						None => {
							tracing::warn!("Too many connections. Please try again later.");
							connections.add(http::reject_connection(socket).boxed());
							continue;
						}
					};

					let shutdown_requested = stop_monitor.shutdown_requested();

					let tower_service = TowerService {
						inner: ServiceData {
							remote_addr: socket.peer_addr().ok(),
							methods: methods.clone(),
							allow_hosts: allow_hosts.clone(),
							resources: resources.clone(),
							max_request_body_size,
							max_response_body_size,
							max_log_length,
							batch_requests_supported,
							id_provider: id_provider.clone(),
							ping_interval: self.cfg.ping_interval,
							stop_monitor: stop_monitor.clone(),
							max_subscriptions_per_connection,
							conn_id: id,
							logger: logger.clone(),
							conn: Arc::new(conn),
						},
					};

					let service = self.service_builder.service(tower_service);

					connections.add(
						async move {
							let conn =
								hyper::server::conn::Http::new().serve_connection(socket, service).with_upgrades();

							tokio::pin!(conn);

							tokio::select! {
								res = &mut conn => {
									tracing::info!("conn: {} finished res: {:?}", id, res);
									if let Err(e) = res {
										tracing::error!("Error when processing connection: {:?}", e);
									}
								},
								_ = shutdown_requested => {
									tracing::info!("starting graceful conn");
									conn.graceful_shutdown();
								}
							}
						}
						.boxed(),
					);

					tracing::info!(
						"Accepting new connection {}/{}",
						self.cfg.max_connections as usize - connection_guard.available_connections(),
						self.cfg.max_connections
					);

					id = id.wrapping_add(1);
				}
				Err(MonitoredError::Selector(err)) => {
					tracing::error!("Error while awaiting a new connection: {:?}", err);
				}
				Err(MonitoredError::Shutdown) => {
					tracing::info!("stopping server");
					break;
				}
			}
		}

		connections.await
	}
}

/// This is a glorified select listening for new messages, while also checking the `stop_receiver` signal.
pub(crate) struct Monitored<'a, F> {
	future: F,
	stop_monitor: &'a StopMonitor,
}

impl<'a, F> Monitored<'a, F> {
	pub(crate) fn new(future: F, stop_monitor: &'a StopMonitor) -> Self {
		Monitored { future, stop_monitor }
	}
}

pub(crate) enum MonitoredError<E> {
	Shutdown,
	Selector(E),
}

struct Incoming(TcpListener);

impl<'a> Future for Monitored<'a, Incoming> {
	type Output = Result<(TcpStream, SocketAddr), MonitoredError<std::io::Error>>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		let this = Pin::into_inner(self);

		if this.stop_monitor.is_shutdown_requested() {
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

		if this.stop_monitor.is_shutdown_requested() {
			return Poll::Ready(Err(MonitoredError::Shutdown));
		}

		this.future.poll_unpin(cx).map_err(MonitoredError::Selector)
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
	/// Maximum number of subscriptions per connection.
	max_subscriptions_per_connection: u32,
	/// Max length for logging for requests and responses
	///
	/// Logs bigger than this limit will be truncated.
	max_log_length: u32,
	/// Host filtering.
	allow_hosts: AllowHosts,
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
			allow_hosts: AllowHosts::Any,
			tokio_runtime: None,
			ping_interval: Duration::from_secs(60),
		}
	}
}

/// Builder to configure and create a JSON-RPC server
#[derive(Debug)]
pub struct Builder<B = Identity, L = ()> {
	settings: Settings,
	resources: Resources,
	logger: L,
	id_provider: Arc<dyn IdProvider>,
	service_builder: tower::ServiceBuilder<B>,
}

impl Default for Builder {
	fn default() -> Self {
		Builder {
			settings: Settings::default(),
			resources: Resources::default(),
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

	/// Add a logger to the builder [`Logger`](../jsonrpsee_core/logger/trait.Logger.html).
	///
	/// ```
	/// use std::{time::Instant, net::SocketAddr};
	///
	/// use jsonrpsee_server::logger::{Logger, HttpRequest, MethodKind, Params};
	/// use jsonrpsee_server::ServerBuilder;
	///
	/// #[derive(Clone)]
	/// struct MyLogger;
	///
	/// impl Logger for MyLogger {
	///     type Instant = Instant;
	///
	///     fn on_connect(&self, remote_addr: Option<SocketAddr>, request: &HttpRequest) {
	///          println!("[MyLogger::on_call] remote_addr: {:?}, headers: {:?}", remote_addr, request);
	///     }
	///
	///     fn on_request(&self) -> Self::Instant {
	///          Instant::now()
	///     }
	///
	///     fn on_call(&self, method_name: &str, params: Params, kind: MethodKind) {
	///          println!("[MyLogger::on_call] method: '{}' params: {:?}, kind: {:?}", method_name, params, kind);
	///     }
	///
	///     fn on_result(&self, method_name: &str, success: bool, started_at: Self::Instant) {
	///          println!("[MyLogger::on_result] '{}', worked? {}, time elapsed {:?}", method_name, success, started_at.elapsed());
	///     }
	///
	///     fn on_response(&self, result: &str, started_at: Self::Instant) {
	///          println!("[MyLogger::on_response] result: {}, time elapsed {:?}", result, started_at.elapsed());
	///     }
	///
	///     fn on_disconnect(&self, remote_addr: Option<SocketAddr>) {
	///          println!("[MyLogger::on_disconnect] remote_addr: {:?}", remote_addr);
	///     }
	/// }
	///
	/// let builder = ServerBuilder::new().set_logger(MyLogger);
	/// ```
	pub fn set_logger<T: Logger>(self, logger: T) -> Builder<B, T> {
		Builder {
			settings: self.settings,
			resources: self.resources,
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
	/// use jsonrpsee_server::ServerBuilder;
	///
	/// // Set the ping interval to 10 seconds.
	/// let builder = ServerBuilder::default().ping_interval(Duration::from_secs(10));
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

	/// Sets host filtering.
	pub fn set_host_filtering(mut self, allow: AllowHosts) -> Self {
		self.settings.allow_hosts = allow;
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
		Builder {
			settings: self.settings,
			resources: self.resources,
			logger: self.logger,
			id_provider: self.id_provider,
			service_builder,
		}
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
		let stop_monitor = StopMonitor::new();
		let resources = self.resources;
		Ok(Server {
			listener,
			cfg: self.settings,
			stop_monitor,
			resources,
			logger: self.logger,
			id_provider: self.id_provider,
			service_builder: self.service_builder,
		})
	}
}

pub(crate) enum MethodResult {
	JustLogger(MethodResponse),
	SendAndLogger(MethodResponse),
}

impl MethodResult {
	pub(crate) fn as_inner(&self) -> &MethodResponse {
		match &self {
			Self::JustLogger(r) => r,
			Self::SendAndLogger(r) => r,
		}
	}
}

/// Data required by the server to handle requests.
#[derive(Debug, Clone)]
pub(crate) struct ServiceData<L: Logger> {
	/// Remote server address.
	pub(crate) remote_addr: Option<SocketAddr>,
	/// Registered server methods.
	pub(crate) methods: Methods,
	/// Access control.
	pub(crate) allow_hosts: AllowHosts,
	/// Tracker for currently used resources on the server.
	pub(crate) resources: Resources,
	/// Max request body size.
	pub(crate) max_request_body_size: u32,
	/// Max response body size.
	pub(crate) max_response_body_size: u32,
	/// Max length for logging for request and response
	///
	/// Logs bigger than this limit will be truncated.
	pub(crate) max_log_length: u32,
	/// Whether batch requests are supported by this server or not.
	pub(crate) batch_requests_supported: bool,
	/// Subscription ID provider.
	pub(crate) id_provider: Arc<dyn IdProvider>,
	/// Ping interval
	pub(crate) ping_interval: Duration,
	/// Stop handle.
	pub(crate) stop_monitor: StopMonitor,
	/// Max subscriptions per connection.
	pub(crate) max_subscriptions_per_connection: u32,
	/// Connection ID
	pub(crate) conn_id: u32,
	/// Logger.
	pub(crate) logger: L,
	/// Handle to hold a `connection permit`.
	pub(crate) conn: Arc<OwnedSemaphorePermit>,
}

/// JsonRPSee service compatible with `tower`.
///
/// # Note
/// This is similar to [`hyper::service::service_fn`].
#[derive(Debug)]
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

		let host = match http_helpers::read_header_value(request.headers(), hyper::header::HOST) {
			Some(host) => host,
			None if request.version() == hyper::Version::HTTP_2 => match request.uri().host() {
				Some(host) => host,
				None => return async move { Ok(http::response::malformed()) }.boxed(),
			},
			None => return async move { Ok(http::response::malformed()) }.boxed(),
		};

		if let Err(e) = self.inner.allow_hosts.verify(host) {
			tracing::warn!("Denied request: {}", e);
			return async { Ok(http::response::host_not_allowed()) }.boxed();
		}

		if is_upgrade_request(&request) {
			let mut server = soketto::handshake::http::Server::new();

			let response = match server.receive_request(&request) {
				Ok(response) => {
					self.inner.logger.on_connect(self.inner.remote_addr, &request);

					let data = self.inner.clone();
					tokio::spawn(async move {
						let upgraded = match hyper::upgrade::on(request).await {
							Ok(u) => u,
							Err(e) => {
								tracing::warn!("Could not upgrade connection: {}", e);
								return;
							}
						};

						let stream = BufReader::new(BufWriter::new(upgraded.compat()));
						let mut ws_builder = server.into_builder(stream);
						ws_builder.set_max_message_size(data.max_request_body_size as usize);
						let (sender, receiver) = ws_builder.finish();

						let _ = ws::background_task::<L>(sender, receiver, data).await;
					});

					response.map(|()| hyper::Body::empty())
				}
				Err(e) => {
					tracing::error!("Could not upgrade connection: {}", e);
					hyper::Response::new(hyper::Body::from(format!("Could not upgrade connection: {}", e)))
				}
			};

			async { Ok(response) }.boxed()
		} else {
			// The request wasn't an upgrade request; let's treat it as a standard HTTP request:
			let data = http::HandleRequest {
				methods: self.inner.methods.clone(),
				resources: self.inner.resources.clone(),
				max_request_body_size: self.inner.max_request_body_size,
				max_response_body_size: self.inner.max_response_body_size,
				max_log_length: self.inner.max_log_length,
				batch_requests_supported: self.inner.batch_requests_supported,
				logger: self.inner.logger.clone(),
				conn: self.inner.conn.clone(),
			};

			Box::pin(http::handle_request(request, data).map(Ok))
		}
	}
}
