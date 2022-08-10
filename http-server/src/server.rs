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

use std::convert::Infallible;
use std::future::Future;
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::response;
use crate::response::{internal_error, malformed};
use futures_channel::mpsc;
use futures_util::future::FutureExt;
use futures_util::stream::{StreamExt, TryStreamExt};
use futures_util::TryFutureExt;
use hyper::body::HttpBody;
use hyper::header::{HeaderMap, HeaderValue};
use hyper::server::conn::AddrStream;
use hyper::server::{conn::AddrIncoming, Builder as HyperBuilder};
use hyper::service::{make_service_fn, Service};
use hyper::{Body, Error as HyperError, Method};
use jsonrpsee_core::error::{Error, GenericTransportError};
use jsonrpsee_core::http_helpers::{self, read_body};
use jsonrpsee_core::logger::{self, HttpLogger as Logger};
use jsonrpsee_core::server::access_control::AccessControl;
use jsonrpsee_core::server::helpers::{prepare_error, MethodResponse};
use jsonrpsee_core::server::helpers::{BatchResponse, BatchResponseBuilder};
use jsonrpsee_core::server::resource_limiting::Resources;
use jsonrpsee_core::server::rpc_module::{MethodKind, Methods};
use jsonrpsee_core::tracing::{rx_log_from_json, rx_log_from_str, tx_log_from_str, RpcTracing};
use jsonrpsee_core::TEN_MB_SIZE_BYTES;
use jsonrpsee_types::error::{ErrorCode, ErrorObject, BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG};
use jsonrpsee_types::{Id, Notification, Params, Request};
use std::error::Error as StdError;
use serde_json::value::RawValue;
use tokio::net::{TcpListener, ToSocketAddrs};
use tower::layer::util::Identity;
use tower::Layer;
use tracing_futures::Instrument;

type Notif<'a> = Notification<'a, Option<&'a RawValue>>;

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
	health_api: Option<HealthApi>,
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
			health_api: None,
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
			health_api: self.health_api,
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

	/// Enable health endpoint.
	/// Allows you to expose one of the methods under GET /<path> The method will be invoked with no parameters.
	/// Error returned from the method will be converted to status 500 response.
	/// Expects a tuple with (</path>, <rpc-method-name>).
	///
	/// Fails if the path is missing `/`.
	pub fn health_api(mut self, path: impl Into<String>, method: impl Into<String>) -> Result<Self, Error> {
		let path = path.into();

		if !path.starts_with('/') {
			return Err(Error::Custom(format!("Health endpoint path must start with `/` to work, got: {}", path)));
		}

		self.health_api = Some(HealthApi { path, method: method.into() });
		Ok(self)
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
			health_api: self.health_api,
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
			health_api: self.health_api,
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
			health_api: self.health_api,
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
			health_api: self.health_api,
			service_builder: self.service_builder,
		})
	}
}

#[derive(Debug, Clone)]
struct HealthApi {
	path: String,
	method: String,
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
	/// Health API.
	health_api: Option<HealthApi>,
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
	async fn handle_request(
		self,
		request: hyper::Request<hyper::Body>,
	) -> Result<hyper::Response<hyper::Body>, Infallible> {
		let ServiceData {
			remote_addr,
			methods,
			acl,
			resources,
			logger,
			health_api,
			max_request_body_size,
			max_response_body_size,
			max_log_length,
			batch_requests_supported,
		} = self;

		let request_start = logger.on_request(remote_addr, &request);

		let keys = request.headers().keys().map(|k| k.as_str());
		let cors_request_headers = http_helpers::get_cors_request_headers(request.headers());

		let host = match http_helpers::read_header_value(request.headers(), "host") {
			Some(origin) => origin,
			None => return Ok(malformed()),
		};
		let maybe_origin = http_helpers::read_header_value(request.headers(), "origin");

		if let Err(e) = acl.verify_host(host) {
			tracing::warn!("Denied request: {:?}", e);
			return Ok(response::host_not_allowed());
		}

		if let Err(e) = acl.verify_origin(maybe_origin, host) {
			tracing::warn!("Denied request: {:?}", e);
			return Ok(response::invalid_allow_origin());
		}

		if let Err(e) = acl.verify_headers(keys, cors_request_headers) {
			tracing::warn!("Denied request: {:?}", e);
			return Ok(response::invalid_allow_headers());
		}

		// Only `POST` and `OPTIONS` methods are allowed.
		match *request.method() {
			// An OPTIONS request is a CORS preflight request. We've done our access check
			// above so we just need to tell the browser that the request is OK.
			Method::OPTIONS => {
				let origin = match maybe_origin {
					Some(origin) => origin,
					None => return Ok(malformed()),
				};

				let allowed_headers = acl.allowed_headers().to_cors_header_value();
				let allowed_header_bytes = allowed_headers.as_bytes();

				let res = hyper::Response::builder()
					.header("access-control-allow-origin", origin)
					.header("access-control-allow-methods", "POST")
					.header("access-control-allow-headers", allowed_header_bytes)
					.body(hyper::Body::empty())
					.unwrap_or_else(|e| {
						tracing::error!("Error forming preflight response: {}", e);
						internal_error()
					});

				Ok(res)
			}
			// The actual request. If it's a CORS request we need to remember to add
			// the access-control-allow-origin header (despite preflight) to allow it
			// to be read in a browser.
			Method::POST if content_type_is_json(&request) => {
				let origin = return_origin_if_different_from_host(request.headers()).cloned();
				let mut res = process_validated_request(ProcessValidatedRequest {
					request,
					logger,
					methods,
					resources,
					max_request_body_size,
					max_response_body_size,
					max_log_length,
					batch_requests_supported,
					request_start,
				})
				.await?;

				if let Some(origin) = origin {
					res.headers_mut().insert("access-control-allow-origin", origin);
				}
				Ok(res)
			}
			Method::GET => match health_api.as_ref() {
				Some(health) if health.path.as_str() == request.uri().path() => {
					process_health_request(
						health,
						logger,
						methods,
						max_response_body_size,
						request_start,
						max_log_length,
					)
					.await
				}
				_ => Ok(response::method_not_allowed()),
			},
			// Error scenarios:
			Method::POST => Ok(response::unsupported_content_type()),
			_ => Ok(response::method_not_allowed()),
		}
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

	// NOTE(lexnv): The `handle_request` method returns `Result<_, Infallible>`.
	// This is because the RPC service will always return a valid HTTP response (ie return `Ok(_)`).
	//
	// The following associated type is required by the `impl<B, U, L: Logger> Server<B, L>` bounds.
	// It satisfies the server's bounds when the `tower::ServiceBuilder<B>` is not set (ie `B: Identity`).
	type Error = Box<dyn StdError + Send + Sync + 'static>;

	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

	/// Opens door for back pressure implementation.
	fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
		Poll::Ready(Ok(()))
	}

	fn call(&mut self, request: hyper::Request<hyper::Body>) -> Self::Future {
		let data = self.inner.clone();
		// Note that `handle_request` will never return error.
		// The dummy error is set in place to satisfy the server's trait bounds regarding the
		// `tower::ServiceBuilder` and the error will never be mapped.
		Box::pin(data.handle_request(request).map_err(|_| "".into()))
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
	health_api: Option<HealthApi>,
	service_builder: tower::ServiceBuilder<B>,
}

impl<B, U, L: Logger> Server<B, L>
where
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
	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.local_addr.ok_or_else(|| Error::Custom("Local address not found".into()))
	}

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
		let health_api = self.health_api;

		let make_service = make_service_fn(move |conn: &AddrStream| {
			let service = TowerService {
				inner: ServiceData {
					remote_addr: conn.remote_addr(),
					methods: methods.clone(),
					acl: acl.clone(),
					resources: resources.clone(),
					logger: logger.clone(),
					health_api: health_api.clone(),
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

// Checks the origin and host headers. If they both exist, return the origin if it does not match the host.
// If one of them doesn't exist (origin most probably), or they are identical, return None.
fn return_origin_if_different_from_host(headers: &HeaderMap) -> Option<&HeaderValue> {
	if let (Some(origin), Some(host)) = (headers.get("origin"), headers.get("host")) {
		if origin != host {
			Some(origin)
		} else {
			None
		}
	} else {
		None
	}
}

/// Checks that content type of received request is valid for JSON-RPC.
fn content_type_is_json(request: &hyper::Request<hyper::Body>) -> bool {
	is_json(request.headers().get("content-type"))
}

/// Returns true if the `content_type` header indicates a valid JSON message.
fn is_json(content_type: Option<&hyper::header::HeaderValue>) -> bool {
	match content_type.and_then(|val| val.to_str().ok()) {
		Some(content)
			if content.eq_ignore_ascii_case("application/json")
				|| content.eq_ignore_ascii_case("application/json; charset=utf-8")
				|| content.eq_ignore_ascii_case("application/json;charset=utf-8") =>
		{
			true
		}
		_ => false,
	}
}

struct ProcessValidatedRequest<L: Logger> {
	request: hyper::Request<hyper::Body>,
	logger: L,
	methods: Methods,
	resources: Resources,
	max_request_body_size: u32,
	max_response_body_size: u32,
	max_log_length: u32,
	batch_requests_supported: bool,
	request_start: L::Instant,
}

/// Process a verified request, it implies a POST request with content type JSON.
async fn process_validated_request<L: Logger>(
	input: ProcessValidatedRequest<L>,
) -> Result<hyper::Response<hyper::Body>, Infallible> {
	let ProcessValidatedRequest {
		request,
		logger,
		methods,
		resources,
		max_request_body_size,
		max_response_body_size,
		max_log_length,
		batch_requests_supported,
		request_start,
	} = input;

	let (parts, body) = request.into_parts();

	let (body, is_single) = match read_body(&parts.headers, body, max_request_body_size).await {
		Ok(r) => r,
		Err(GenericTransportError::TooLarge) => return Ok(response::too_large(max_request_body_size)),
		Err(GenericTransportError::Malformed) => return Ok(response::malformed()),
		Err(GenericTransportError::Inner(e)) => {
			tracing::error!("Internal error reading request body: {}", e);
			return Ok(response::internal_error());
		}
	};

	// Single request or notification
	if is_single {
		let call = CallData {
			conn_id: 0,
			logger: &logger,
			methods: &methods,
			max_response_body_size,
			max_log_length,
			resources: &resources,
			request_start,
		};
		let response = process_single_request(body, call).await;
		logger.on_response(&response.result, request_start);
		Ok(response::ok_response(response.result))
	}
	// Batch of requests or notifications
	else if !batch_requests_supported {
		let err = MethodResponse::error(
			Id::Null,
			ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, &BATCHES_NOT_SUPPORTED_MSG, None),
		);
		logger.on_response(&err.result, request_start);
		Ok(response::ok_response(err.result))
	}
	// Batch of requests or notifications
	else {
		let response = process_batch_request(Batch {
			data: body,
			call: CallData {
				conn_id: 0,
				logger: &logger,
				methods: &methods,
				max_response_body_size,
				max_log_length,
				resources: &resources,
				request_start,
			},
		})
		.await;
		logger.on_response(&response.result, request_start);
		Ok(response::ok_response(response.result))
	}
}

async fn process_health_request<L: Logger>(
	health_api: &HealthApi,
	logger: L,
	methods: Methods,
	max_response_body_size: u32,
	request_start: L::Instant,
	max_log_length: u32,
) -> Result<hyper::Response<hyper::Body>, Infallible> {
	let trace = RpcTracing::method_call(&health_api.method);
	async {
		tx_log_from_str("HTTP health API", max_log_length);
		let response = match methods.method_with_name(&health_api.method) {
			None => MethodResponse::error(Id::Null, ErrorObject::from(ErrorCode::MethodNotFound)),
			Some((_name, method_callback)) => match method_callback.inner() {
				MethodKind::Sync(callback) => {
					(callback)(Id::Number(0), Params::new(None), max_response_body_size as usize)
				}
				MethodKind::Async(callback) => {
					(callback)(Id::Number(0), Params::new(None), 0, max_response_body_size as usize, None).await
				}
				MethodKind::Subscription(_) | MethodKind::Unsubscription(_) => {
					MethodResponse::error(Id::Null, ErrorObject::from(ErrorCode::InternalError))
				}
			},
		};

		rx_log_from_str(&response.result, max_log_length);
		logger.on_result(&health_api.method, response.success, request_start);
		logger.on_response(&response.result, request_start);

		if response.success {
			#[derive(serde::Deserialize)]
			struct RpcPayload<'a> {
				#[serde(borrow)]
				result: &'a serde_json::value::RawValue,
			}

			let payload: RpcPayload = serde_json::from_str(&response.result)
				.expect("valid JSON-RPC response must have a result field and be valid JSON; qed");
			Ok(response::ok_response(payload.result.to_string()))
		} else {
			Ok(response::internal_error())
		}
	}
	.instrument(trace.into_span())
	.await
}

#[derive(Debug, Clone)]
struct Batch<'a, L: Logger> {
	data: Vec<u8>,
	call: CallData<'a, L>,
}

#[derive(Debug, Clone)]
struct CallData<'a, L: Logger> {
	conn_id: usize,
	logger: &'a L,
	methods: &'a Methods,
	max_response_body_size: u32,
	max_log_length: u32,
	resources: &'a Resources,
	request_start: L::Instant,
}

#[derive(Debug, Clone)]
struct Call<'a, L: Logger> {
	params: Params<'a>,
	name: &'a str,
	call: CallData<'a, L>,
	id: Id<'a>,
}

// Batch responses must be sent back as a single message so we read the results from each
// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
// complete batch response back to the client over `tx`.
async fn process_batch_request<L>(b: Batch<'_, L>) -> BatchResponse
where
	L: Logger,
{
	let Batch { data, call } = b;

	if let Ok(batch) = serde_json::from_slice::<Vec<Request>>(&data) {
		let max_response_size = call.max_response_body_size;
		let batch = batch.into_iter().map(|req| Ok((req, call.clone())));

		let batch_stream = futures_util::stream::iter(batch);

		let trace = RpcTracing::batch();
		return async {
			let batch_response = batch_stream
				.try_fold(
					BatchResponseBuilder::new_with_limit(max_response_size as usize),
					|batch_response, (req, call)| async move {
						let params = Params::new(req.params.map(|params| params.get()));
						let response = execute_call(Call { name: &req.method, params, id: req.id, call }).await;
						batch_response.append(&response)
					},
				)
				.await;

			match batch_response {
				Ok(batch) => batch.finish(),
				Err(batch_err) => batch_err,
			}
		}
		.instrument(trace.into_span())
		.await;
	}

	if let Ok(batch) = serde_json::from_slice::<Vec<Notif>>(&data) {
		return if !batch.is_empty() {
			BatchResponse { result: "".to_string(), success: true }
		} else {
			BatchResponse::error(Id::Null, ErrorObject::from(ErrorCode::InvalidRequest))
		};
	}

	// "If the batch rpc call itself fails to be recognized as an valid JSON or as an
	// Array with at least one value, the response from the Server MUST be a single
	// Response object." – The Spec.
	let (id, code) = prepare_error(&data);
	BatchResponse::error(id, ErrorObject::from(code))
}

async fn process_single_request<L: Logger>(data: Vec<u8>, call: CallData<'_, L>) -> MethodResponse {
	if let Ok(req) = serde_json::from_slice::<Request>(&data) {
		let trace = RpcTracing::method_call(&req.method);
		async {
			rx_log_from_json(&req, call.max_log_length);
			let params = Params::new(req.params.map(|params| params.get()));
			let name = &req.method;
			let id = req.id;
			execute_call(Call { name, params, id, call }).await
		}
		.instrument(trace.into_span())
		.await
	} else if let Ok(req) = serde_json::from_slice::<Notif>(&data) {
		let trace = RpcTracing::notification(&req.method);
		let span = trace.into_span();
		let _enter = span.enter();
		rx_log_from_json(&req, call.max_log_length);

		MethodResponse { result: String::new(), success: true }
	} else {
		let (id, code) = prepare_error(&data);
		MethodResponse::error(id, ErrorObject::from(code))
	}
}

async fn execute_call<L: Logger>(c: Call<'_, L>) -> MethodResponse {
	let Call { name, id, params, call } = c;
	let CallData { resources, methods, logger, max_response_body_size, max_log_length, conn_id, request_start } = call;

	let response = match methods.method_with_name(name) {
		None => {
			logger.on_call(name, params.clone(), logger::MethodKind::Unknown);
			MethodResponse::error(id, ErrorObject::from(ErrorCode::MethodNotFound))
		}
		Some((name, method)) => match &method.inner() {
			MethodKind::Sync(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::MethodCall);

				match method.claim(name, resources) {
					Ok(guard) => {
						let r = (callback)(id, params, max_response_body_size as usize);
						drop(guard);
						r
					}
					Err(err) => {
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {:?}", err);
						MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy))
					}
				}
			}
			MethodKind::Async(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::MethodCall);
				match method.claim(name, resources) {
					Ok(guard) => {
						let id = id.into_owned();
						let params = params.into_owned();

						(callback)(id, params, conn_id, max_response_body_size as usize, Some(guard)).await
					}
					Err(err) => {
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {:?}", err);
						MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy))
					}
				}
			}
			MethodKind::Subscription(_) | MethodKind::Unsubscription(_) => {
				logger.on_call(name, params.clone(), logger::MethodKind::Unknown);
				tracing::error!("Subscriptions not supported on HTTP");
				MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError))
			}
		},
	};

	tx_log_from_str(&response.result, max_log_length);
	logger.on_result(name, response.success, request_start);
	response
}
