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
use std::task::Poll;
use std::time::Duration;

use crate::future::{ConnectionGuard, ServerHandle, StopHandle};
use crate::middleware::rpc::{RpcService, RpcServiceBuilder, RpcServiceCfg, RpcServiceT, TransportProtocol};
use crate::transport::http::content_type_is_json;
use crate::transport::ws::BackgroundTaskParams;
use crate::transport::{http, ws};

use futures_util::future::{self, Either, FutureExt};
use futures_util::io::{BufReader, BufWriter};

use hyper::body::HttpBody;
use hyper::Method;

use jsonrpsee_core::http_helpers::read_body;
use jsonrpsee_core::id_providers::RandomIntegerIdProvider;
use jsonrpsee_core::server::helpers::{prepare_error, MethodResponseResult};
use jsonrpsee_core::server::{BatchResponseBuilder, BoundedSubscriptions, MethodResponse, MethodSink, Methods};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::{Error, GenericTransportError, JsonRawValue, TEN_MB_SIZE_BYTES};

use jsonrpsee_types::error::{
	reject_too_big_batch_request, ErrorCode, BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG,
};
use jsonrpsee_types::{ErrorObject, Id, InvalidRequest, Notification, Request};
use soketto::handshake::http::is_upgrade_request;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::sync::{mpsc, watch, OwnedSemaphorePermit};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tower::layer::util::Identity;
use tower::{Layer, Service};
use tracing::{instrument, Instrument};

type Notif<'a> = Notification<'a, Option<&'a JsonRawValue>>;

/// Default maximum connections allowed.
const MAX_CONNECTIONS: u32 = 100;

/// JSON RPC server.
pub struct Server<HttpMiddleware = Identity, RpcMiddleware = Identity> {
	listener: TcpListener,
	cfg: Settings,
	rpc_middleware: RpcServiceBuilder<RpcMiddleware>,
	id_provider: Arc<dyn IdProvider>,
	http_middleware: tower::ServiceBuilder<HttpMiddleware>,
}

impl Server<Identity, Identity> {
	/// Create a builder for the server.
	pub fn builder() -> Builder<Identity, Identity> {
		Builder::new()
	}
}

impl<RpcMiddleware, HttpMiddleware> std::fmt::Debug for Server<RpcMiddleware, HttpMiddleware> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Server")
			.field("listener", &self.listener)
			.field("cfg", &self.cfg)
			.field("id_provider", &self.id_provider)
			.finish()
	}
}

impl<RpcMiddleware, HttpMiddleware> Server<RpcMiddleware, HttpMiddleware> {
	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.listener.local_addr().map_err(Into::into)
	}
}

impl<HttpMiddleware, RpcMiddleware, B> Server<HttpMiddleware, RpcMiddleware>
where
	RpcMiddleware: tower::Layer<RpcService> + Clone + Send + 'static,
	for<'a> <RpcMiddleware as Layer<RpcService>>::Service: RpcServiceT<'a>,
	HttpMiddleware: Layer<TowerService<RpcMiddleware>> + Send + 'static,
	<HttpMiddleware as Layer<TowerService<RpcMiddleware>>>::Service: Send
		+ Service<
			hyper::Request<hyper::Body>,
			Response = hyper::Response<B>,
			Error = Box<(dyn StdError + Send + Sync + 'static)>,
		>,
	<<HttpMiddleware as Layer<TowerService<RpcMiddleware>>>::Service as Service<hyper::Request<hyper::Body>>>::Future:
		Send,
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
		let max_subscriptions_per_connection = self.cfg.max_subscriptions_per_connection;
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
						max_subscriptions_per_connection,
						batch_requests_config,
						id_provider: id_provider.clone(),
						ping_config: self.cfg.ping_config,
						stop_handle: stop_handle.clone(),
						conn_id: id,
						max_connections: self.cfg.max_connections,
						enable_http: self.cfg.enable_http,
						enable_ws: self.cfg.enable_ws,
						message_buffer_capacity: self.cfg.message_buffer_capacity,
					};

					process_connection(
						&self.http_middleware,
						self.rpc_middleware.clone(),
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
pub struct Builder<HttpMiddleware, RpcMiddleware> {
	settings: Settings,
	rpc_middleware: RpcServiceBuilder<RpcMiddleware>,
	id_provider: Arc<dyn IdProvider>,
	http_middleware: tower::ServiceBuilder<HttpMiddleware>,
}

impl Default for Builder<Identity, Identity> {
	fn default() -> Self {
		Builder {
			settings: Settings::default(),
			rpc_middleware: RpcServiceBuilder::new(),
			id_provider: Arc::new(RandomIntegerIdProvider),
			http_middleware: tower::ServiceBuilder::new(),
		}
	}
}

impl Builder<Identity, Identity> {
	/// Create a default server builder.
	pub fn new() -> Self {
		Self::default()
	}
}

impl<HttpMiddleware, RpcMiddleware> Builder<HttpMiddleware, RpcMiddleware> {
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

	/// Enable middleware that is invoked on every JSON-RPC call.
	///
	/// The middleware itself is very similar to the `tower middleware` but
	/// it has a different service trait which takes &self instead &mut self
	/// which means that you can't use built-in middleware from tower.
	///
	/// Another consequence of `&self` is that you must wrap any of the middleware state in
	/// a type which is Send and provides interior mutability such `Arc<Mutex>`.
	///
	/// The builder itself exposes a similar API as the [`tower::ServiceBuilder`]
	/// where it is possible to compose layers to the middleware.
	///
	/// To add a middleware [`crate::middleware::rpc::RpcServiceBuilder`] exposes a few different layer APIs that
	/// is wrapped on top of the [`tower::ServiceBuilder`].
	///
	/// When the server is started these layers are wrapped in the [`crate::middleware::rpc::RpcService`] and
	/// that's why the service APIs is not exposed.
	/// ```
	///
	/// use std::{time::Instant, net::SocketAddr, sync::Arc};
	/// use std::sync::atomic::{Ordering, AtomicUsize};
	///
	/// use jsonrpsee_server::middleware::rpc::{RpcServiceT, RpcService, TransportProtocol, RpcServiceBuilder};
	/// use jsonrpsee_server::{ServerBuilder, MethodResponse};
	/// use jsonrpsee_core::async_trait;
	/// use jsonrpsee_types::Request;
	///
	/// #[derive(Clone)]
	/// struct MyMiddleware<S> {
	///     service: S,
	///     count: Arc<AtomicUsize>,
	/// }
	///
	/// #[async_trait]
	/// impl<'a, S> RpcServiceT<'a> for MyMiddleware<S>
	/// where S: RpcServiceT<'a> + Send + Sync,
	/// {
	///     async fn call(&self, req: Request<'a>, t: TransportProtocol) -> MethodResponse {
	///         tracing::info!("MyMiddleware processed call {}", req.method);
	///         // if one wants to access connection related context
	///         // that can be fetched from `Context`
	///         let rp = self.service.call(req, t).await;
	///         // Modify the state.
	///         self.count.fetch_add(1, Ordering::Relaxed);
	///         rp
	///     }
	/// }
	///
	/// // Create a state per connection
	/// // NOTE: The service type can be omitted once `start` is called on the server.
	/// let m = RpcServiceBuilder::new().layer_fn(move |service: ()| MyMiddleware { service, count: Arc::new(AtomicUsize::new(0)) });
	/// let builder = ServerBuilder::default().set_rpc_middleware(m);
	/// ```
	pub fn set_rpc_middleware<T>(self, rpc_middleware: RpcServiceBuilder<T>) -> Builder<HttpMiddleware, T> {
		Builder {
			settings: self.settings,
			rpc_middleware,
			id_provider: self.id_provider,
			http_middleware: self.http_middleware,
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
	///         .set_http_middleware(builder)
	///         .build("127.0.0.1:0".parse::<SocketAddr>().unwrap())
	///         .await
	///         .unwrap();
	/// }
	/// ```
	pub fn set_http_middleware<T>(self, http_middleware: tower::ServiceBuilder<T>) -> Builder<T, RpcMiddleware> {
		Builder {
			settings: self.settings,
			id_provider: self.id_provider,
			http_middleware,
			rpc_middleware: self.rpc_middleware,
		}
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
	pub async fn build(self, addrs: impl ToSocketAddrs) -> Result<Server<HttpMiddleware, RpcMiddleware>, Error> {
		let listener = TcpListener::bind(addrs).await?;

		Ok(Server {
			listener,
			cfg: self.settings,
			rpc_middleware: self.rpc_middleware,
			id_provider: self.id_provider,
			http_middleware: self.http_middleware,
		})
	}

	/// Finalizes the configuration of the server with customized TCP settings on the socket.
	///
	///
	/// ```rust
	/// use jsonrpsee_server::Server;
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
	///   let server = Server::builder().build_from_tcp(socket).unwrap();
	/// }
	/// ```
	pub fn build_from_tcp(
		self,
		listener: impl Into<StdTcpListener>,
	) -> Result<Server<HttpMiddleware, RpcMiddleware>, Error> {
		let listener = TcpListener::from_std(listener.into())?;

		Ok(Server {
			listener,
			cfg: self.settings,
			rpc_middleware: self.rpc_middleware,
			id_provider: self.id_provider,
			http_middleware: self.http_middleware,
		})
	}
}

/// Data required by the server to handle requests.
#[derive(Debug, Clone)]
pub(crate) struct ServiceData {
	/// Registered server methods.
	pub(crate) methods: Methods,
	/// Max request body size.
	pub(crate) max_request_body_size: u32,
	/// Max response body size.
	pub(crate) max_response_body_size: u32,
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
pub struct TowerService<L> {
	inner: ServiceData,
	rpc_middleware: RpcServiceBuilder<L>,
}

impl<RpcMiddleware> hyper::service::Service<hyper::Request<hyper::Body>> for TowerService<RpcMiddleware>
where
	RpcMiddleware: for<'a> tower::Layer<RpcService>,
	<RpcMiddleware as Layer<RpcService>>::Service: Send + Sync + 'static,
	for<'a> <RpcMiddleware as Layer<RpcService>>::Service: RpcServiceT<'a>,
{
	type Response = hyper::Response<hyper::Body>;

	// The following associated type is required by the `impl<B, U, M: JsonRpcMiddleware> Server<B, L>` bounds.
	// It satisfies the server's bounds when the `tower::ServiceBuilder<B>` is not set (ie `B: Identity`).
	type Error = Box<dyn StdError + Send + Sync + 'static>;

	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

	/// Opens door for back pressure implementation.
	fn poll_ready(&mut self, _: &mut std::task::Context) -> Poll<Result<(), Self::Error>> {
		Poll::Ready(Ok(()))
	}

	fn call(&mut self, request: hyper::Request<hyper::Body>) -> Self::Future {
		tracing::trace!("{:?}", request);

		let is_upgrade_request = is_upgrade_request(&request);

		if self.inner.enable_ws && is_upgrade_request {
			let this = self.inner.clone();

			let mut server = soketto::handshake::http::Server::new();

			let response = match server.receive_request(&request) {
				Ok(response) => {
					let (tx, rx) = mpsc::channel::<String>(this.message_buffer_capacity as usize);
					let sink = MethodSink::new(tx);

					// On each method call the `pending_calls` is cloned
					// then when all pending_calls are dropped
					// a graceful shutdown can occur.
					let (pending_calls, pending_calls_completed) = mpsc::channel::<()>(1);

					let cfg = RpcServiceCfg::CallsAndSubscriptions {
						bounded_subscriptions: BoundedSubscriptions::new(this.max_subscriptions_per_connection),
						id_provider: self.inner.id_provider.clone(),
						sink: sink.clone(),
						_pending_calls: pending_calls,
					};

					let rpc_service = RpcService::new(
						self.inner.methods.clone(),
						this.max_response_body_size as usize,
						this.conn_id as usize,
						cfg,
					);

					let rpc_service = self.rpc_middleware.service(rpc_service);

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
							ws_builder.set_max_message_size(this.max_request_body_size as usize);
							let (sender, receiver) = ws_builder.finish();

							let params = BackgroundTaskParams {
								other: this,
								ws_sender: sender,
								ws_receiver: receiver,
								rpc_service,
								sink,
								rx,
								pending_calls_completed,
							};

							ws::background_task(params).await;
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
			let rpc_service = self.rpc_middleware.service(RpcService::new(
				self.inner.methods.clone(),
				self.inner.max_response_body_size as usize,
				self.inner.conn_id as usize,
				RpcServiceCfg::OnlyCalls,
			));

			let batch_config = self.inner.batch_requests_config;
			let max_request_size = self.inner.max_request_body_size;
			let max_response_size = self.inner.max_response_body_size;

			let fut = async move {
				// Only the `POST` method is allowed.
				match *request.method() {
					Method::POST if content_type_is_json(&request) => {
						let (parts, body) = request.into_parts();

						let (body, is_single) = match read_body(&parts.headers, body, max_request_size).await {
							Ok(r) => r,
							Err(GenericTransportError::TooLarge) => return http::response::too_large(max_request_size),
							Err(GenericTransportError::Malformed) => return http::response::malformed(),
							Err(GenericTransportError::Inner(e)) => {
								tracing::warn!("Internal error reading request body: {}", e);
								return http::response::internal_error();
							}
						};

						let rp = handle_rpc_call(
							&body,
							is_single,
							batch_config,
							max_response_size,
							&rpc_service,
							TransportProtocol::Http,
						)
						.await;

						// If the response is empty it means that it was a notification or empty batch.
						// For HTTP these are just ACK:ed with a empty body.
						http::response::ok_response(rp.map_or(String::new(), |r| r.result))
					}
					// Error scenarios:
					Method::POST => http::response::unsupported_content_type(),
					_ => http::response::method_not_allowed(),
				}
			};

			Box::pin(fut.map(Ok))
		} else {
			Box::pin(async { http::response::denied() }.map(Ok))
		}
	}
}

struct ProcessConnection {
	/// Remote server address.
	remote_addr: SocketAddr,
	/// Registered server methods.
	methods: Methods,
	/// Max request body size.
	max_request_body_size: u32,
	/// Max response body size.
	max_response_body_size: u32,
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
	/// Allow JSON-RPC HTTP requests.
	enable_http: bool,
	/// Allow JSON-RPC WS request and WS upgrade requests.
	enable_ws: bool,
	/// Number of messages that server is allowed `buffer` until backpressure kicks in.
	message_buffer_capacity: u32,
}

#[instrument(name = "connection", skip_all, fields(remote_addr = %cfg.remote_addr, conn_id = %cfg.conn_id), level = "INFO")]
fn process_connection<'a, RpcMiddleware, HttpMiddleware, U>(
	http_middleware_builder: &tower::ServiceBuilder<HttpMiddleware>,
	rpc_middleware_builder: RpcServiceBuilder<RpcMiddleware>,
	connection_guard: &ConnectionGuard,
	cfg: ProcessConnection,
	socket: TcpStream,
	drop_on_completion: mpsc::Sender<()>,
) where
	RpcMiddleware: 'static,
	HttpMiddleware: Layer<TowerService<RpcMiddleware>> + Send + 'static,
	<HttpMiddleware as Layer<TowerService<RpcMiddleware>>>::Service: Send
		+ 'static
		+ Service<
			hyper::Request<hyper::Body>,
			Response = hyper::Response<U>,
			Error = Box<(dyn StdError + Send + Sync + 'static)>,
		>,
	<<HttpMiddleware as Layer<TowerService<RpcMiddleware>>>::Service as Service<hyper::Request<hyper::Body>>>::Future:
		Send + 'static,
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
			methods: cfg.methods,
			max_request_body_size: cfg.max_request_body_size,
			max_response_body_size: cfg.max_response_body_size,
			max_subscriptions_per_connection: cfg.max_subscriptions_per_connection,
			batch_requests_config: cfg.batch_requests_config,
			id_provider: cfg.id_provider,
			ping_config: cfg.ping_config,
			stop_handle: cfg.stop_handle.clone(),
			conn_id: cfg.conn_id,
			conn: Arc::new(conn),
			enable_http: cfg.enable_http,
			enable_ws: cfg.enable_ws,
			message_buffer_capacity: cfg.message_buffer_capacity,
		},
		rpc_middleware: rpc_middleware_builder,
	};

	let service = http_middleware_builder.service(tower_service);

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

pub(crate) async fn handle_rpc_call<S>(
	body: &[u8],
	is_single: bool,
	batch_config: BatchRequestConfig,
	max_response_size: u32,
	rpc_service: &S,
	transport: TransportProtocol,
) -> Option<MethodResponse>
where
	for<'a> S: RpcServiceT<'a> + Send,
{
	// Single request or notification
	if is_single {
		if let Ok(req) = serde_json::from_slice(body) {
			Some(rpc_service.call(req, transport).await)
		} else if let Ok(_notif) = serde_json::from_slice::<Notif>(body) {
			None
		} else {
			let (id, code) = prepare_error(body);
			Some(MethodResponse::error(id, ErrorObject::from(code)))
		}
	}
	// Batch of requests.
	else {
		let max_len = match batch_config {
			BatchRequestConfig::Disabled => {
				let rp = MethodResponse::error(
					Id::Null,
					ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG, None),
				);
				return Some(rp);
			}
			BatchRequestConfig::Limit(limit) => limit as usize,
			BatchRequestConfig::Unlimited => usize::MAX,
		};

		if let Ok(batch) = serde_json::from_slice::<Vec<&JsonRawValue>>(body) {
			if batch.len() > max_len {
				return Some(MethodResponse::error(Id::Null, reject_too_big_batch_request(max_len)));
			}

			let mut got_notif = false;
			let mut batch_response = BatchResponseBuilder::new_with_limit(max_response_size as usize);

			for call in batch {
				if let Ok(req) = serde_json::from_str::<Request>(call.get()) {
					let rp = rpc_service.call(req, transport).await;

					if let Err(too_large) = batch_response.append(&rp) {
						return Some(too_large);
					}
				} else if let Ok(_notif) = serde_json::from_str::<Notif>(call.get()) {
					// notifications should not be answered.
					got_notif = true;
				} else {
					// valid JSON but could be not parsable as `InvalidRequest`
					let id = match serde_json::from_str::<InvalidRequest>(call.get()) {
						Ok(err) => err.id,
						Err(_) => Id::Null,
					};

					if let Err(too_large) =
						batch_response.append(&MethodResponse::error(id, ErrorObject::from(ErrorCode::InvalidRequest)))
					{
						return Some(too_large);
					}
				}
			}

			if got_notif && batch_response.is_empty() {
				None
			} else {
				let result = batch_response.finish();
				Some(MethodResponse { result, success_or_error: MethodResponseResult::Success, is_subscription: false })
			}
		} else {
			Some(MethodResponse::error(Id::Null, ErrorObject::from(ErrorCode::ParseError)))
		}
	}
}
