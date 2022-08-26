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
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures_channel::mpsc;
use futures_util::future::FutureExt;
use futures_util::stream::StreamExt;
use hyper::body::HttpBody;
use hyper::server::conn::AddrStream;
use hyper::server::{conn::AddrIncoming, Builder as HyperBuilder};
use hyper::service::{make_service_fn, Service};
use hyper::{Body, Error as HyperError};
use jsonrpsee_core::error::Error;
use jsonrpsee_core::logger::HttpLogger as Logger;
use jsonrpsee_core::server::access_control::AccessControl;
use jsonrpsee_core::server::http_utils::{handle_request, HandleRequest};
use jsonrpsee_core::server::resource_limiting::Resources;
use jsonrpsee_core::server::rpc_module::Methods;
use jsonrpsee_core::TEN_MB_SIZE_BYTES;
use std::error::Error as StdError;
use tokio::net::{TcpListener, ToSocketAddrs};
use tower::layer::util::Identity;
use tower::Layer;

/// Builder to create JSON-RPC HTTP server.
#[derive(Debug)]
pub struct Builder<B = Identity, L = ()> {
	/// Access control based on HTTP headers.
	access_control: AccessControl,
	resources: Resources,
	max_request_body_size: u32,
	max_response_body_size: u32,
	batch_requests_supported: bool,
	/// Custom tokio runtime to run the server on.
	tokio_runtime: Option<tokio::runtime::Handle>,
	logger: L,
	max_log_length: u32,
	service_builder: tower::ServiceBuilder<B>,
}

impl Default for Builder {
	fn default() -> Self {
		Self {
			access_control: AccessControl::default(),
			max_request_body_size: TEN_MB_SIZE_BYTES,
			max_response_body_size: TEN_MB_SIZE_BYTES,
			batch_requests_supported: true,
			resources: Resources::default(),
			tokio_runtime: None,
			logger: (),
			max_log_length: 4096,
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
	/// Add a logger to the builder [`Logger`](../jsonrpsee_core/logger/trait.Logger.html).
	///
	/// # Examples
	///
	/// ```
	/// use std::{time::Instant, net::SocketAddr};
	/// use hyper::Request;
	///
	/// use jsonrpsee_core::logger::{HttpLogger, Headers, MethodKind, Params};
	/// use jsonrpsee_http_server::HttpServerBuilder;
	///
	/// #[derive(Clone)]
	/// struct MyLogger;
	///
	/// impl HttpLogger for MyLogger {
	///     type Instant = Instant;
	///
	///     // Called once the HTTP request is received, it may be a single JSON-RPC call
	///     // or batch.
	///     fn on_request(&self, _remote_addr: SocketAddr, _request: &Request<hyper::Body>) -> Instant {
	///         Instant::now()
	///     }
	///
	///     // Called once a single JSON-RPC method call is processed, it may be called multiple times
	///     // on batches.
	///     fn on_call(&self, method_name: &str, params: Params, kind: MethodKind) {
	///         println!("Call to method: '{}' params: {:?}, kind: {}", method_name, params, kind);
	///     }
	///
	///     // Called once a single JSON-RPC call is completed, it may be called multiple times
	///     // on batches.
	///     fn on_result(&self, method_name: &str, success: bool, started_at: Instant) {
	///         println!("Call to '{}' took {:?}", method_name, started_at.elapsed());
	///     }
	///
	///     // Called the entire JSON-RPC is completed, called on once for both single calls or batches.
	///     fn on_response(&self, result: &str, started_at: Instant) {
	///         println!("complete JSON-RPC response: {}, took: {:?}", result, started_at.elapsed());
	///     }
	/// }
	///
	/// let builder = HttpServerBuilder::new().set_logger(MyLogger);
	/// ```
	pub fn set_logger<T: Logger>(self, logger: T) -> Builder<B, T> {
		Builder {
			access_control: self.access_control,
			max_request_body_size: self.max_request_body_size,
			max_response_body_size: self.max_response_body_size,
			batch_requests_supported: self.batch_requests_supported,
			resources: self.resources,
			tokio_runtime: self.tokio_runtime,
			logger,
			max_log_length: self.max_log_length,
			service_builder: self.service_builder,
		}
	}

	/// Sets the maximum size of a request body in bytes (default is 10 MiB).
	pub fn max_request_body_size(mut self, size: u32) -> Self {
		self.max_request_body_size = size;
		self
	}

	/// Sets the maximum size of a response body in bytes (default is 10 MiB).
	pub fn max_response_body_size(mut self, size: u32) -> Self {
		self.max_response_body_size = size;
		self
	}

	/// Sets access control settings.
	pub fn set_access_control(mut self, acl: AccessControl) -> Self {
		self.access_control = acl;
		self
	}

	/// Enables or disables support of [batch requests](https://www.jsonrpc.org/specification#batch).
	/// By default, support is enabled.
	pub fn batch_requests_supported(mut self, supported: bool) -> Self {
		self.batch_requests_supported = supported;
		self
	}

	/// Register a new resource kind. Errors if `label` is already registered, or if the number of
	/// registered resources on this server instance would exceed 8.
	///
	/// See the module documentation for [`resource_limiting`](../jsonrpsee_utils/server/resource_limiting/index.html#resource-limiting)
	/// for details.
	pub fn register_resource(mut self, label: &'static str, capacity: u16, default: u16) -> Result<Self, Error> {
		self.resources.register(label, capacity, default)?;

		Ok(self)
	}

	/// Configure a custom [`tokio::runtime::Handle`] to run the server on.
	///
	/// Default: [`tokio::spawn`]
	pub fn custom_tokio_runtime(mut self, rt: tokio::runtime::Handle) -> Self {
		self.tokio_runtime = Some(rt);
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
	/// use jsonrpsee_http_server::HttpServerBuilder;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let builder = tower::ServiceBuilder::new()
	///         .timeout(Duration::from_secs(2));
	///
	///     let server = HttpServerBuilder::new()
	///         .set_middleware(builder)
	///         .build("127.0.0.1:0".parse::<SocketAddr>().unwrap())
	///         .await
	///         .unwrap();
	/// }
	/// ```
	pub fn set_middleware<T>(self, service_builder: tower::ServiceBuilder<T>) -> Builder<T, L> {
		Builder {
			access_control: self.access_control,
			max_request_body_size: self.max_request_body_size,
			max_response_body_size: self.max_response_body_size,
			batch_requests_supported: self.batch_requests_supported,
			resources: self.resources,
			tokio_runtime: self.tokio_runtime,
			logger: self.logger,
			max_log_length: self.max_log_length,
			service_builder,
		}
	}

	/// Finalizes the configuration of the server with customized TCP settings on the socket and on hyper.
	///
	/// # Examples
	///
	/// ```rust
	/// use jsonrpsee_http_server::HttpServerBuilder;
	/// use socket2::{Domain, Socket, Type};
	/// use std::net::TcpListener;
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
	///   socket.listen(4096).unwrap();
	///
	///   let listener: TcpListener = socket.into();
	///   let local_addr = listener.local_addr().ok();
	///
	///   // hyper does some settings on the provided socket, ensure that nothing breaks our "expected settings".
	///
	///   let listener = hyper::Server::from_tcp(listener)
	///     .unwrap()
	///     .tcp_sleep_on_accept_errors(true)
	///     .tcp_keepalive(None)
	///     .tcp_nodelay(true);
	///
	///   let server = HttpServerBuilder::new().build_from_hyper(listener, addr).unwrap();
	/// }
	/// ```
	pub fn build_from_hyper(
		self,
		listener: hyper::server::Builder<AddrIncoming>,
		local_addr: SocketAddr,
	) -> Result<Server<B, L>, Error> {
		Ok(Server {
			access_control: self.access_control,
			listener,
			local_addr: Some(local_addr),
			max_request_body_size: self.max_request_body_size,
			max_response_body_size: self.max_response_body_size,
			batch_requests_supported: self.batch_requests_supported,
			resources: self.resources,
			tokio_runtime: self.tokio_runtime,
			logger: self.logger,
			max_log_length: self.max_log_length,
			service_builder: self.service_builder,
		})
	}

	/// Finalizes the configuration of the server with customized TCP settings on the socket.
	/// Note, that [`hyper`] might overwrite some of the TCP settings on the socket
	/// if you want full-control of socket settings use [`Builder::build_from_hyper`] instead.
	///
	/// ```rust
	/// use jsonrpsee_http_server::HttpServerBuilder;
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
	///   let server = HttpServerBuilder::new().build_from_tcp(socket).unwrap();
	/// }
	/// ```
	pub fn build_from_tcp(self, listener: impl Into<StdTcpListener>) -> Result<Server<B, L>, Error> {
		let listener = listener.into();
		let local_addr = listener.local_addr().ok();

		let listener = hyper::Server::from_tcp(listener)?;

		Ok(Server {
			listener,
			local_addr,
			access_control: self.access_control,
			max_request_body_size: self.max_request_body_size,
			max_response_body_size: self.max_response_body_size,
			batch_requests_supported: self.batch_requests_supported,
			resources: self.resources,
			tokio_runtime: self.tokio_runtime,
			logger: self.logger,
			max_log_length: self.max_log_length,
			service_builder: self.service_builder,
		})
	}

	/// Finalizes the configuration of the server.
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
	///   assert!(jsonrpsee_http_server::HttpServerBuilder::default().build(occupied_addr).await.is_err());
	///   assert!(jsonrpsee_http_server::HttpServerBuilder::default().build(addrs).await.is_ok());
	/// }
	/// ```
	pub async fn build(self, addrs: impl ToSocketAddrs) -> Result<Server<B, L>, Error> {
		let listener = TcpListener::bind(addrs).await?.into_std()?;

		let local_addr = listener.local_addr().ok();
		let listener = hyper::Server::from_tcp(listener)?.tcp_nodelay(true);

		Ok(Server {
			listener,
			local_addr,
			access_control: self.access_control,
			max_request_body_size: self.max_request_body_size,
			max_response_body_size: self.max_response_body_size,
			batch_requests_supported: self.batch_requests_supported,
			resources: self.resources,
			tokio_runtime: self.tokio_runtime,
			logger: self.logger,
			max_log_length: self.max_log_length,
			service_builder: self.service_builder,
		})
	}
}

/// Handle used to run or stop the server.
#[derive(Debug)]
pub struct ServerHandle {
	stop_sender: mpsc::Sender<()>,
	pub(crate) handle: Option<tokio::task::JoinHandle<()>>,
}

impl ServerHandle {
	/// Requests server to stop. Returns an error if server was already stopped.
	pub fn stop(mut self) -> Result<tokio::task::JoinHandle<()>, Error> {
		let stop = self.stop_sender.try_send(()).map(|_| self.handle.take());
		match stop {
			Ok(Some(handle)) => Ok(handle),
			_ => Err(Error::AlreadyStopped),
		}
	}
}

impl Future for ServerHandle {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let handle = match &mut self.handle {
			Some(handle) => handle,
			None => return Poll::Ready(()),
		};

		handle.poll_unpin(cx).map(|_| ())
	}
}

/// Data required by the server to handle requests.
#[derive(Debug, Clone)]
struct ServiceData<L> {
	/// Remote server address.
	remote_addr: SocketAddr,
	/// Registered server methods.
	methods: Methods,
	/// Access control.
	acl: AccessControl,
	/// Tracker for currently used resources on the server.
	resources: Resources,
	/// User provided logger.
	logger: L,
	/// Max request body size.
	max_request_body_size: u32,
	/// Max response body size.
	max_response_body_size: u32,
	/// Max length for logging for request and response
	///
	/// Logs bigger than this limit will be truncated.
	max_log_length: u32,
	/// Whether batch requests are supported by this server or not.
	batch_requests_supported: bool,
}

impl<L: Logger> ServiceData<L> {
	/// Default behavior for handling the RPC requests.
	async fn handle_request(self, request: hyper::Request<hyper::Body>) -> hyper::Response<hyper::Body> {
		let data = HandleRequest {
			remote_addr: self.remote_addr,
			methods: self.methods,
			acl: self.acl,
			resources: self.resources,
			max_request_body_size: self.max_request_body_size,
			max_response_body_size: self.max_response_body_size,
			max_log_length: self.max_log_length,
			batch_requests_supported: self.batch_requests_supported,
			logger: self.logger,
		};

		handle_request(request, data).await
	}
}

/// JsonRPSee service compatible with `tower`.
///
/// # Note
/// This is similar to [`hyper::service::service_fn`].
#[derive(Debug)]
pub struct TowerService<L> {
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
		let data = self.inner.clone();
		Box::pin(data.handle_request(request).map(Ok))
	}
}

/// An HTTP JSON RPC server.
#[derive(Debug)]
pub struct Server<B = Identity, L = ()> {
	/// Hyper server.
	listener: HyperBuilder<AddrIncoming>,
	/// Local address
	local_addr: Option<SocketAddr>,
	/// Max request body size.
	max_request_body_size: u32,
	/// Max response body size.
	max_response_body_size: u32,
	/// Max length for logging for request and response
	///
	/// Logs bigger than this limit will be truncated.
	max_log_length: u32,
	/// Whether batch requests are supported by this server or not.
	batch_requests_supported: bool,
	/// Access control.
	access_control: AccessControl,
	/// Tracker for currently used resources on the server.
	resources: Resources,
	/// Custom tokio runtime to run the server on.
	tokio_runtime: Option<tokio::runtime::Handle>,
	logger: L,
	service_builder: tower::ServiceBuilder<B>,
}

impl<B, L> Server<B, L> {
	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.local_addr.ok_or_else(|| Error::Custom("Local address not found".into()))
	}
}

// Required trait bounds for the middleware service.
impl<B, U, L> Server<B, L>
where
	L: Logger,
	B: Layer<TowerService<L>> + Send + 'static,
	<B as Layer<TowerService<L>>>::Service: Send
		+ Service<
			hyper::Request<Body>,
			Response = hyper::Response<U>,
			Error = Box<(dyn StdError + Send + Sync + 'static)>,
		>,
	<<B as Layer<TowerService<L>>>::Service as Service<hyper::Request<Body>>>::Future: Send,
	U: HttpBody + Send + 'static,
	<U as HttpBody>::Error: Send + Sync + StdError,
	<U as HttpBody>::Data: Send,
{
	/// Start the server.
	pub fn start(mut self, methods: impl Into<Methods>) -> Result<ServerHandle, Error> {
		let max_request_body_size = self.max_request_body_size;
		let max_response_body_size = self.max_response_body_size;
		let max_log_length = self.max_log_length;
		let acl = self.access_control;
		let (tx, mut rx) = mpsc::channel(1);
		let listener = self.listener;
		let resources = self.resources;
		let logger = self.logger;
		let batch_requests_supported = self.batch_requests_supported;
		let methods = methods.into().initialize_resources(&resources)?;

		let make_service = make_service_fn(move |conn: &AddrStream| {
			let service = TowerService {
				inner: ServiceData {
					remote_addr: conn.remote_addr(),
					methods: methods.clone(),
					acl: acl.clone(),
					resources: resources.clone(),
					logger: logger.clone(),
					max_request_body_size,
					max_response_body_size,
					max_log_length,
					batch_requests_supported,
				},
			};

			let server = self.service_builder.service(service);

			// For every request the `TowerService` is calling into `ServiceData::handle_request`
			// where the RPSee bare implementation resides.
			async move { Ok::<_, HyperError>(server) }
		});

		let rt = match self.tokio_runtime.take() {
			Some(rt) => rt,
			None => tokio::runtime::Handle::current(),
		};

		let handle = rt.spawn(async move {
			let server = listener.serve(make_service);
			let _ = server.with_graceful_shutdown(async move { rx.next().await.map_or((), |_| ()) }).await;
		});

		Ok(ServerHandle { handle: Some(handle), stop_sender: tx })
	}
}
