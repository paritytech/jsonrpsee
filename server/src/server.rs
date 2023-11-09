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
use std::sync::atomic::AtomicU32;
use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;

use crate::future::{ConnectionGuard, ServerHandle, StopHandle};
use crate::middleware::rpc::{RpcService, RpcServiceBuilder, RpcServiceCfg, RpcServiceT, TransportProtocol};
use crate::transport::ws::BackgroundTaskParams;
use crate::transport::{http, ws};

use futures_util::future::{self, Either, FutureExt};
use futures_util::io::{BufReader, BufWriter};

use hyper::body::HttpBody;

use jsonrpsee_core::id_providers::RandomIntegerIdProvider;
use jsonrpsee_core::server::helpers::{prepare_error, MethodResponseResult};
use jsonrpsee_core::server::{BatchResponseBuilder, BoundedSubscriptions, MethodResponse, MethodSink, Methods};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::{Error, JsonRawValue, TEN_MB_SIZE_BYTES};

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
	server_cfg: ServerConfig,
	rpc_middleware: RpcServiceBuilder<RpcMiddleware>,
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
		f.debug_struct("Server").field("listener", &self.listener).field("server_cfg", &self.server_cfg).finish()
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
	HttpMiddleware: Layer<TowerServiceNoHttp<RpcMiddleware>> + Send + 'static,
	<HttpMiddleware as Layer<TowerServiceNoHttp<RpcMiddleware>>>::Service: Send
		+ Service<
			hyper::Request<hyper::Body>,
			Response = hyper::Response<B>,
			Error = Box<(dyn StdError + Send + Sync + 'static)>,
		>,
	<<HttpMiddleware as Layer<TowerServiceNoHttp<RpcMiddleware>>>::Service as Service<hyper::Request<hyper::Body>>>::Future:
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

		match self.server_cfg.tokio_runtime.take() {
			Some(rt) => rt.spawn(self.start_inner(methods, stop_handle)),
			None => tokio::spawn(self.start_inner(methods, stop_handle)),
		};

		ServerHandle::new(stop_tx)
	}

	async fn start_inner(self, methods: Methods, stop_handle: StopHandle) {
		let mut id: u32 = 0;
		let connection_guard = ConnectionGuard::new(self.server_cfg.max_connections as usize);
		let listener = self.listener;

		let stopped = stop_handle.clone().shutdown();
		tokio::pin!(stopped);

		let (drop_on_completion, mut process_connection_awaiter) = mpsc::channel::<()>(1);

		loop {
			match try_accept_conn(&listener, stopped).await {
				AcceptConnection::Established { socket, remote_addr, stop } => {


					process_connection(ProcessConnection {
						http_middleware: &self.http_middleware,
						rpc_middleware: self.rpc_middleware.clone(),
						remote_addr,
						methods: methods.clone(),
						stop_handle: stop_handle.clone(),
						conn_id: id,
						server_cfg: self.server_cfg.clone(),
						conn_guard: &connection_guard,
						socket,
						drop_on_completion: drop_on_completion.clone(),
					});
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

/// Static server configuration which is shared per connection.
#[derive(Debug, Clone)]
pub struct ServerConfig {
	/// Maximum size in bytes of a request.
	pub(crate) max_request_body_size: u32,
	/// Maximum size in bytes of a response.
	pub(crate) max_response_body_size: u32,
	/// Maximum number of incoming connections allowed.
	pub(crate) max_connections: u32,
	/// Maximum number of subscriptions per connection.
	pub(crate) max_subscriptions_per_connection: u32,
	/// Whether batch requests are supported by this server or not.
	pub(crate) batch_requests_config: BatchRequestConfig,
	/// Custom tokio runtime to run the server on.
	pub(crate) tokio_runtime: Option<tokio::runtime::Handle>,
	/// Enable HTTP.
	pub(crate) enable_http: bool,
	/// Enable WS.
	pub(crate) enable_ws: bool,
	/// Number of messages that server is allowed to `buffer` until backpressure kicks in.
	pub(crate) message_buffer_capacity: u32,
	/// Ping settings.
	pub(crate) ping_config: PingConfig,
	/// ID provider.
	pub(crate) id_provider: Arc<dyn IdProvider>,
}

#[derive(Debug, Clone)]
pub struct ServerConfigBuilder {
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
	/// Enable HTTP.
	enable_http: bool,
	/// Enable WS.
	enable_ws: bool,
	/// Number of messages that server is allowed to `buffer` until backpressure kicks in.
	message_buffer_capacity: u32,
	/// Ping settings.
	ping_config: PingConfig,
	/// ID provider.
	id_provider: Arc<dyn IdProvider>,
}

/// Builder for [`TowerService`].
#[derive(Debug, Clone)]
pub struct TowerServiceBuilder<RpcMiddleware, HttpMiddleware> {
	/// ServerConfig
	pub(crate) server_cfg: ServerConfig,
	/// RPC middleware.
	pub(crate) rpc_middleware: RpcServiceBuilder<RpcMiddleware>,
	/// HTTP middleware.
	pub(crate) http_middleware: tower::ServiceBuilder<HttpMiddleware>,
	/// Connection ID.
	pub(crate) conn_id: Arc<AtomicU32>,
	/// Connection guard.
	pub(crate) conn_guard: ConnectionGuard,
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

/// Connection related state that is needed
/// to execute JSON-RPC calls.
#[derive(Debug, Clone)]
pub struct ConnectionState {
	/// Stop handle.
	pub(crate) stop_handle: StopHandle,
	/// Connection ID
	pub(crate) conn_id: u32,
	/// Connection guard.
	pub(crate) _conn_permit: Arc<OwnedSemaphorePermit>,
}

impl ConnectionState {
	/// Create a new connection state.
	pub fn new(stop_handle: StopHandle, conn_id: u32, conn_permit: OwnedSemaphorePermit) -> ConnectionState {
		Self { stop_handle, conn_id, _conn_permit: Arc::new(conn_permit) }
	}
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

impl Default for ServerConfig {
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
			id_provider: Arc::new(RandomIntegerIdProvider),
		}
	}
}

impl ServerConfig {
	/// Create a new builder for the [`ServerConfig`].
	pub fn builder() -> ServerConfigBuilder {
		ServerConfigBuilder::default()
	}
}

impl Default for ServerConfigBuilder {
	fn default() -> Self {
		let this = ServerConfig::default();

		ServerConfigBuilder {
			max_request_body_size: this.max_request_body_size,
			max_response_body_size: this.max_response_body_size,
			max_connections: this.max_connections,
			max_subscriptions_per_connection: this.max_subscriptions_per_connection,
			batch_requests_config: this.batch_requests_config,
			enable_http: this.enable_http,
			enable_ws: this.enable_ws,
			message_buffer_capacity: this.message_buffer_capacity,
			ping_config: this.ping_config,
			id_provider: this.id_provider,
		}
	}
}

impl ServerConfigBuilder {
	/// Create a new [`ServerConfigBuilder`].
	pub fn new() -> Self {
		Self::default()
	}

	/// See [`Builder::max_request_body_size`](method@Builder::max_request_body_size) for documentation.
	pub fn max_request_body_size(mut self, size: u32) -> Self {
		self.max_request_body_size = size;
		self
	}

	/// See [`Builder::max_response_body_size`](method@Builder::max_response_body_size) for documentation.
	pub fn max_response_body_size(mut self, size: u32) -> Self {
		self.max_response_body_size = size;
		self
	}

	/// See [`Builder::max_connections`](method@Builder::max_connections) for documentation.
	pub fn max_connections(mut self, max: u32) -> Self {
		self.max_connections = max;
		self
	}

	/// See [`Builder::set_batch_request_config`](method@Builder::set_batch_request_config) for documentation.
	pub fn set_batch_request_config(mut self, cfg: BatchRequestConfig) -> Self {
		self.batch_requests_config = cfg;
		self
	}

	/// See [`Builder::max_subscriptions_per_connection`](method@Builder::max_subscriptions_per_connection) for documentation.
	pub fn max_subscriptions_per_connection(mut self, max: u32) -> Self {
		self.max_subscriptions_per_connection = max;
		self
	}

	/// See [`Builder::http_only`](method@Builder::http_only) for documentation.
	pub fn http_only(mut self) -> Self {
		self.enable_http = true;
		self.enable_ws = false;
		self
	}

	/// See [`Builder::ws_only`](method@Builder::ws_only) for documentation.
	pub fn ws_only(mut self) -> Self {
		self.enable_http = false;
		self.enable_ws = true;
		self
	}

	/// See [`Builder::set_message_buffer_capacity`](method@Builder::set_message_buffer_capacity) for documentation.
	pub fn set_message_buffer_capacity(mut self, c: u32) -> Self {
		self.message_buffer_capacity = c;
		self
	}

	/// See [`Builder::ping_interval`](method@Builder::ping_interval) for documentation.
	pub fn ping_interval(mut self, config: PingConfig) -> Result<Self, Error> {
		if let PingConfig::WithInactivityCheck { ping_interval, inactive_limit } = config {
			if ping_interval >= inactive_limit {
				return Err(Error::Custom("`inactive_limit` must be bigger than `ping_interval` to work".into()));
			}
		}

		self.ping_config = config;
		Ok(self)
	}

	/// See [`Builder::set_id_provider`] for documentation.
	pub fn set_id_provider<I: IdProvider + 'static>(mut self, id_provider: I) -> Self {
		self.id_provider = Arc::new(id_provider);
		self
	}
}

/// Builder to configure and create a JSON-RPC server
#[derive(Debug)]
pub struct Builder<HttpMiddleware, RpcMiddleware> {
	server_cfg: ServerConfig,
	rpc_middleware: RpcServiceBuilder<RpcMiddleware>,
	http_middleware: tower::ServiceBuilder<HttpMiddleware>,
}

impl Default for Builder<Identity, Identity> {
	fn default() -> Self {
		Builder {
			server_cfg: ServerConfig::default(),
			rpc_middleware: RpcServiceBuilder::new(),
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

impl<RpcMiddleware: Clone, HttpMiddleware: Clone> TowerServiceBuilder<RpcMiddleware, HttpMiddleware> {
	/// Build a tower service.
	pub fn build(
		&self,
		methods: impl Into<Methods>,
		stop_handle: StopHandle,
	) -> TowerService<RpcMiddleware, HttpMiddleware> {
		let conn_id = self.conn_id.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

		let rpc_middleware = TowerServiceNoHttp {
			rpc_middleware: self.rpc_middleware.clone(),
			inner: ServiceData {
				methods: methods.into(),
				stop_handle,
				conn_id,
				conn_guard: self.conn_guard.clone(),
				server_cfg: self.server_cfg.clone(),
			},
		};

		TowerService { rpc_middleware, http_middleware: self.http_middleware.clone() }
	}

	/// Configure the connection id.
	///
	/// This is incremented every time `build` is called.
	pub fn connection_id(mut self, id: u32) -> Self {
		self.conn_id = Arc::new(AtomicU32::new(id));
		self
	}

	/// Configure the max allowed connections on the server.
	pub fn max_connections(mut self, limit: u32) -> Self {
		self.conn_guard = ConnectionGuard::new(limit as usize);
		self
	}

	/// Configure rpc middleware.
	pub fn set_rpc_middleware<T>(self, rpc_middleware: RpcServiceBuilder<T>) -> TowerServiceBuilder<T, HttpMiddleware> {
		TowerServiceBuilder {
			server_cfg: self.server_cfg,
			rpc_middleware,
			http_middleware: self.http_middleware,
			conn_id: self.conn_id,
			conn_guard: self.conn_guard,
		}
	}

	/// Configure http middleware.
	pub fn set_http_middleware<T>(
		self,
		http_middleware: tower::ServiceBuilder<T>,
	) -> TowerServiceBuilder<RpcMiddleware, T> {
		TowerServiceBuilder {
			server_cfg: self.server_cfg,
			rpc_middleware: self.rpc_middleware,
			http_middleware,
			conn_id: self.conn_id,
			conn_guard: self.conn_guard,
		}
	}
}

impl<HttpMiddleware, RpcMiddleware> Builder<HttpMiddleware, RpcMiddleware> {
	/// Set the maximum size of a request body in bytes. Default is 10 MiB.
	pub fn max_request_body_size(mut self, size: u32) -> Self {
		self.server_cfg.max_request_body_size = size;
		self
	}

	/// Set the maximum size of a response body in bytes. Default is 10 MiB.
	pub fn max_response_body_size(mut self, size: u32) -> Self {
		self.server_cfg.max_response_body_size = size;
		self
	}

	/// Set the maximum number of connections allowed. Default is 100.
	pub fn max_connections(mut self, max: u32) -> Self {
		self.server_cfg.max_connections = max;
		self
	}

	/// Configure how [batch requests](https://www.jsonrpc.org/specification#batch) shall be handled
	/// by the server.
	///
	/// Default: batch requests are allowed and can be arbitrary big but the maximum payload size is limited.
	pub fn set_batch_request_config(mut self, cfg: BatchRequestConfig) -> Self {
		self.server_cfg.batch_requests_config = cfg;
		self
	}

	/// Set the maximum number of connections allowed. Default is 1024.
	pub fn max_subscriptions_per_connection(mut self, max: u32) -> Self {
		self.server_cfg.max_subscriptions_per_connection = max;
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
		Builder { server_cfg: self.server_cfg, rpc_middleware, http_middleware: self.http_middleware }
	}

	/// Configure a custom [`tokio::runtime::Handle`] to run the server on.
	///
	/// Default: [`tokio::spawn`]
	pub fn custom_tokio_runtime(mut self, rt: tokio::runtime::Handle) -> Self {
		self.server_cfg.tokio_runtime = Some(rt);
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

		self.server_cfg.ping_config = config;
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
		self.server_cfg.id_provider = Arc::new(id_provider);
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
		Builder { server_cfg: self.server_cfg, http_middleware, rpc_middleware: self.rpc_middleware }
	}

	/// Configure the server to only serve JSON-RPC HTTP requests.
	///
	/// Default: both http and ws are enabled.
	pub fn http_only(mut self) -> Self {
		self.server_cfg.enable_http = true;
		self.server_cfg.enable_ws = false;
		self
	}

	/// Configure the server to only serve JSON-RPC WebSocket requests.
	///
	/// That implies that server just denies HTTP requests which isn't a WebSocket upgrade request
	///
	/// Default: both http and ws are enabled.
	pub fn ws_only(mut self) -> Self {
		self.server_cfg.enable_http = false;
		self.server_cfg.enable_ws = true;
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
		self.server_cfg.message_buffer_capacity = c;
		self
	}

	/// Convert the server builder to a [`TowerServiceBuilder`].
	///
	/// This can be used to utilize the [`TowerService`] from jsonrpsee.
	///
	/// # Examples
	///
	/// ```no_run
	/// use hyper::service::{make_service_fn, service_fn};
	/// use hyper::server::conn::AddrStream;
	/// use jsonrpsee_server::{Methods, ServerHandle, ws, stop_channel};
	/// use tower::Service;
	/// use std::{error::Error as StdError, net::SocketAddr};
	///
	/// fn run_server() -> ServerHandle {
	///     let addr = SocketAddr::from(([127, 0, 0, 1], 0));
	///     let (stop_handle, server_handle) = stop_channel();
	///     let svc_builder = jsonrpsee_server::Server::builder().max_connections(33).to_service_builder();
	///     let methods = Methods::new();
	///     let stop_handle2 = stop_handle.clone();
	///
	///     let make_service = make_service_fn(move |_conn: &AddrStream| {
	///         // You may use `conn` or the actual HTTP request to get connection related details.
	///         let stop_handle = stop_handle2.clone();
	///         let svc_builder = svc_builder.clone();
	///         let methods = methods.clone();
	///
	///         async move {
	///             let stop_handle = stop_handle.clone();
	///             let svc_builder = svc_builder.clone();
	///             let methods = methods.clone();
	///
	///             Ok::<_, Box<dyn StdError + Send + Sync>>(service_fn(move |req| {
	///                 let svc_builder = svc_builder.clone();
	///                 let methods = methods.clone();
	///                 let stop_handle = stop_handle.clone();
	///                 let mut svc = svc_builder.build(methods, stop_handle);
	///
	///                 // It's not possible to know whether the websocket upgrade handshake failed or not here.
	///                 let is_websocket = ws::is_upgrade_request(&req);
	///
	///                 if is_websocket {
	///                     println!("websocket")
	///                 } else {
	///                     println!("http")
	///                 }
	///
	///                 /// Call the jsonrpsee service which
	///                 /// may upgrade it to a WebSocket connection
	///                 /// or treat it as "ordinary HTTP request".
	///                 svc.call(req)
	///             }))
	///         }
	///     });
	///
	///     let server = hyper::Server::bind(&addr).serve(make_service);
	///
	///     tokio::spawn(async move {
	///         let graceful = server.with_graceful_shutdown(async move { stop_handle.shutdown().await });
	///         graceful.await.unwrap()
	///     });
	///
	///     server_handle
	/// }
	/// ```
	pub fn to_service_builder(self) -> TowerServiceBuilder<RpcMiddleware, HttpMiddleware> {
		let max_conns = self.server_cfg.max_connections as usize;

		TowerServiceBuilder {
			server_cfg: self.server_cfg,
			rpc_middleware: self.rpc_middleware,
			http_middleware: self.http_middleware,
			conn_id: Arc::new(AtomicU32::new(0)),
			conn_guard: ConnectionGuard::new(max_conns),
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
	pub async fn build(self, addrs: impl ToSocketAddrs) -> Result<Server<HttpMiddleware, RpcMiddleware>, Error> {
		let listener = TcpListener::bind(addrs).await?;

		Ok(Server {
			listener,
			server_cfg: self.server_cfg,
			rpc_middleware: self.rpc_middleware,
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
			server_cfg: self.server_cfg,
			rpc_middleware: self.rpc_middleware,
			http_middleware: self.http_middleware,
		})
	}
}

/// Data required by the server to handle requests.
#[derive(Debug, Clone)]
struct ServiceData {
	/// Registered server methods.
	methods: Methods,
	/// Stop handle.
	stop_handle: StopHandle,
	/// Connection ID
	conn_id: u32,
	/// Connection guard.
	conn_guard: ConnectionGuard,
	/// ServerConfig
	server_cfg: ServerConfig,
}

/// jsonrpsee tower service
///
/// This will enable both `http_middleware` and `rpc_middleware`
/// that may be enabled by [`Builder`] or [`TowerServiceBuilder`].
#[derive(Debug)]
pub struct TowerService<RpcMiddleware, HttpMiddleware> {
	rpc_middleware: TowerServiceNoHttp<RpcMiddleware>,
	http_middleware: tower::ServiceBuilder<HttpMiddleware>,
}

impl<RpcMiddleware, HttpMiddleware> hyper::service::Service<hyper::Request<hyper::Body>>
	for TowerService<RpcMiddleware, HttpMiddleware>
where
	RpcMiddleware: for<'a> tower::Layer<RpcService> + Clone,
	<RpcMiddleware as Layer<RpcService>>::Service: Send + Sync + 'static,
	for<'a> <RpcMiddleware as Layer<RpcService>>::Service: RpcServiceT<'a>,
	HttpMiddleware: Layer<TowerServiceNoHttp<RpcMiddleware>> + Send + 'static,
	<HttpMiddleware as Layer<TowerServiceNoHttp<RpcMiddleware>>>::Service: Send
		+ Service<
			hyper::Request<hyper::Body>,
			Response = hyper::Response<hyper::Body>,
			Error = Box<(dyn StdError + Send + Sync + 'static)>,
		>,
	<<HttpMiddleware as Layer<TowerServiceNoHttp<RpcMiddleware>>>::Service as Service<hyper::Request<hyper::Body>>>::Future:
		Send + 'static,
{
	type Response = hyper::Response<hyper::Body>;
	type Error = Box<dyn StdError + Send + Sync + 'static>;
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

	/// Opens door for back pressure implementation.
	fn poll_ready(&mut self, _: &mut std::task::Context) -> Poll<Result<(), Self::Error>> {
		Poll::Ready(Ok(()))
	}

	fn call(&mut self, request: hyper::Request<hyper::Body>) -> Self::Future {
		Box::pin(self.http_middleware.service(self.rpc_middleware.clone()).call(request))
	}
}

/// jsonrpsee tower service without HTTP specific middleware.
///
/// # Note
/// This is similar to [`hyper::service::service_fn`].
#[derive(Debug, Clone)]
pub struct TowerServiceNoHttp<L> {
	inner: ServiceData,
	rpc_middleware: RpcServiceBuilder<L>,
}

impl<RpcMiddleware> hyper::service::Service<hyper::Request<hyper::Body>> for TowerServiceNoHttp<RpcMiddleware>
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
		let conn_guard = &self.inner.conn_guard;
		let stop_handle = self.inner.stop_handle.clone();
		let conn_id = self.inner.conn_id;

		tracing::trace!("{:?}", request);

		let Some(conn_permit) = conn_guard.try_acquire() else {
			return async move { Ok(http::response::too_many_requests()) }.boxed();
		};

		let conn = ConnectionState::new(stop_handle.clone(), conn_id, conn_permit);

		let max_conns = conn_guard.max_connections();
		let curr_conns = max_conns - conn_guard.available_connections();
		tracing::debug!("Accepting new connection {}/{}", curr_conns, max_conns);

		let is_upgrade_request = is_upgrade_request(&request);

		if self.inner.server_cfg.enable_ws && is_upgrade_request {
			let this = self.inner.clone();

			let mut server = soketto::handshake::http::Server::new();

			let response = match server.receive_request(&request) {
				Ok(response) => {
					let (tx, rx) = mpsc::channel::<String>(this.server_cfg.message_buffer_capacity as usize);
					let sink = MethodSink::new(tx);

					// On each method call the `pending_calls` is cloned
					// then when all pending_calls are dropped
					// a graceful shutdown can occur.
					let (pending_calls, pending_calls_completed) = mpsc::channel::<()>(1);

					let cfg = RpcServiceCfg::CallsAndSubscriptions {
						bounded_subscriptions: BoundedSubscriptions::new(
							this.server_cfg.max_subscriptions_per_connection,
						),
						id_provider: this.server_cfg.id_provider.clone(),
						sink: sink.clone(),
						_pending_calls: pending_calls,
					};

					let rpc_service = RpcService::new(
						this.methods.clone(),
						this.server_cfg.max_response_body_size as usize,
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
							ws_builder.set_max_message_size(this.server_cfg.max_request_body_size as usize);
							let (sender, receiver) = ws_builder.finish();

							let params = BackgroundTaskParams {
								server_cfg: this.server_cfg,
								conn,
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
		} else if self.inner.server_cfg.enable_http && !is_upgrade_request {
			let this = &self.inner;
			let max_response_size = this.server_cfg.max_response_body_size;
			let max_request_size = this.server_cfg.max_request_body_size;
			let methods = this.methods.clone();
			let batch_config = this.server_cfg.batch_requests_config;

			let rpc_service = self.rpc_middleware.service(RpcService::new(
				methods,
				max_response_size as usize,
				this.conn_id as usize,
				RpcServiceCfg::OnlyCalls,
			));

			Box::pin(
				http::call_with_service(request, batch_config, max_request_size, rpc_service, max_response_size)
					.map(Ok),
			)
		} else {
			Box::pin(async { http::response::denied() }.map(Ok))
		}
	}
}

struct ProcessConnection<'a, HttpMiddleware, RpcMiddleware> {
	http_middleware: &'a tower::ServiceBuilder<HttpMiddleware>,
	rpc_middleware: RpcServiceBuilder<RpcMiddleware>,
	conn_guard: &'a ConnectionGuard,
	conn_id: u32,
	server_cfg: ServerConfig,
	stop_handle: StopHandle,
	socket: TcpStream,
	drop_on_completion: mpsc::Sender<()>,
	remote_addr: SocketAddr,
	methods: Methods,
}

#[instrument(name = "connection", skip_all, fields(remote_addr = %params.remote_addr, conn_id = %params.conn_id), level = "INFO")]
fn process_connection<'a, RpcMiddleware, HttpMiddleware, U>(
	params: ProcessConnection<HttpMiddleware, RpcMiddleware>

) where
	RpcMiddleware: 'static,
	HttpMiddleware: Layer<TowerServiceNoHttp<RpcMiddleware>> + Send + 'static,
	<HttpMiddleware as Layer<TowerServiceNoHttp<RpcMiddleware>>>::Service: Send
		+ 'static
		+ Service<
			hyper::Request<hyper::Body>,
			Response = hyper::Response<U>,
			Error = Box<(dyn StdError + Send + Sync + 'static)>,
		>,
	<<HttpMiddleware as Layer<TowerServiceNoHttp<RpcMiddleware>>>::Service as Service<hyper::Request<hyper::Body>>>::Future:
		Send + 'static,
	U: HttpBody + Send + 'static,
	<U as HttpBody>::Error: Send + Sync + StdError,
	<U as HttpBody>::Data: Send,
{
	let ProcessConnection {
		http_middleware,
		rpc_middleware,
		conn_guard,
		conn_id,
		server_cfg,
		socket,
		stop_handle,
		drop_on_completion,
		methods,
		..
	} = params;

	if let Err(e) = socket.set_nodelay(true) {
		tracing::warn!("Could not set NODELAY on socket: {:?}", e);
		return;
	}

	let tower_service = TowerServiceNoHttp {
		inner: ServiceData {
			server_cfg,
			methods,
			stop_handle: stop_handle.clone(),
			conn_id,
			conn_guard: conn_guard.clone(),
		},
		rpc_middleware,
	};

	let service = http_middleware.service(tower_service);

	tokio::spawn(async {
		to_http_service(socket, service, stop_handle).in_current_span().await;
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
