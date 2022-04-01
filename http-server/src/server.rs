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

use std::cmp;
use std::future::Future;
use std::net::{SocketAddr, TcpListener as StdTcpListener};
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::response::{internal_error, malformed};
use crate::{response, AccessControl};
use futures_channel::mpsc;
use futures_util::{future::join_all, stream::StreamExt, FutureExt};
use hyper::header::{HeaderMap, HeaderValue};
use hyper::server::{conn::AddrIncoming, Builder as HyperBuilder};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Error as HyperError, Method};
use jsonrpsee_core::error::{Error, GenericTransportError};
use jsonrpsee_core::http_helpers::{self, read_body};
use jsonrpsee_core::middleware::Middleware;
use jsonrpsee_core::server::helpers::{collect_batch_response, prepare_error, MethodSink};
use jsonrpsee_core::server::resource_limiting::Resources;
use jsonrpsee_core::server::rpc_module::{MethodKind, Methods};
use jsonrpsee_core::TEN_MB_SIZE_BYTES;
use jsonrpsee_types::error::ErrorCode;
use jsonrpsee_types::{Id, Notification, Params, Request};
use serde_json::value::RawValue;
use tokio::net::{TcpListener, ToSocketAddrs};

/// Builder to create JSON-RPC HTTP server.
#[derive(Debug)]
pub struct Builder<M = ()> {
	access_control: AccessControl,
	resources: Resources,
	max_request_body_size: u32,
	max_response_body_size: u32,
	/// Custom tokio runtime to run the server on.
	tokio_runtime: Option<tokio::runtime::Handle>,
	middleware: M,
}

impl Default for Builder {
	fn default() -> Self {
		Self {
			max_request_body_size: TEN_MB_SIZE_BYTES,
			max_response_body_size: TEN_MB_SIZE_BYTES,
			resources: Resources::default(),
			access_control: AccessControl::default(),
			tokio_runtime: None,
			middleware: (),
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
	/// Add a middleware to the builder [`Middleware`](../jsonrpsee_core/middleware/trait.Middleware.html).
	///
	/// ```
	/// use std::time::Instant;
	///
	/// use jsonrpsee_core::middleware::Middleware;
	/// use jsonrpsee_http_server::HttpServerBuilder;
	///
	/// #[derive(Clone)]
	/// struct MyMiddleware;
	///
	/// impl Middleware for MyMiddleware {
	///     type Instant = Instant;
	///
	///     fn on_request(&self) -> Instant {
	///         Instant::now()
	///     }
	///
	///     fn on_result(&self, name: &str, success: bool, started_at: Instant) {
	///         println!("Call to '{}' took {:?}", name, started_at.elapsed());
	///     }
	/// }
	///
	/// let builder = HttpServerBuilder::new().set_middleware(MyMiddleware);
	/// ```
	pub fn set_middleware<T: Middleware>(self, middleware: T) -> Builder<T> {
		Builder {
			max_request_body_size: self.max_request_body_size,
			max_response_body_size: self.max_response_body_size,
			resources: self.resources,
			access_control: self.access_control,
			tokio_runtime: self.tokio_runtime,
			middleware,
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

	/// Finalizes the configuration of the server with customized TCP settings on the socket and on hyper.
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
	) -> Result<Server<M>, Error> {
		Ok(Server {
			listener,
			local_addr: Some(local_addr),
			access_control: self.access_control,
			max_request_body_size: self.max_request_body_size,
			max_response_body_size: self.max_response_body_size,
			resources: self.resources,
			tokio_runtime: self.tokio_runtime,
			middleware: self.middleware,
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
	pub fn build_from_tcp(self, listener: impl Into<StdTcpListener>) -> Result<Server<M>, Error> {
		let listener = listener.into();
		let local_addr = listener.local_addr().ok();

		let listener = hyper::Server::from_tcp(listener)?;

		Ok(Server {
			listener,
			local_addr,
			access_control: self.access_control,
			max_request_body_size: self.max_request_body_size,
			max_response_body_size: self.max_response_body_size,
			resources: self.resources,
			tokio_runtime: self.tokio_runtime,
			middleware: self.middleware,
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
	pub async fn build(self, addrs: impl ToSocketAddrs) -> Result<Server<M>, Error> {
		let listener = TcpListener::bind(addrs).await?.into_std()?;

		let local_addr = listener.local_addr().ok();
		let listener = hyper::Server::from_tcp(listener)?.tcp_nodelay(true);

		Ok(Server {
			listener,
			local_addr,
			access_control: self.access_control,
			max_request_body_size: self.max_request_body_size,
			max_response_body_size: self.max_response_body_size,
			resources: self.resources,
			tokio_runtime: self.tokio_runtime,
			middleware: self.middleware,
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

/// An HTTP JSON RPC server.
#[derive(Debug)]
pub struct Server<M = ()> {
	/// Hyper server.
	listener: HyperBuilder<AddrIncoming>,
	/// Local address
	local_addr: Option<SocketAddr>,
	/// Max request body size.
	max_request_body_size: u32,
	/// Max response body size.
	max_response_body_size: u32,
	/// Access control
	access_control: AccessControl,
	/// Tracker for currently used resources on the server
	resources: Resources,
	/// Custom tokio runtime to run the server on.
	tokio_runtime: Option<tokio::runtime::Handle>,
	middleware: M,
}

impl<M: Middleware> Server<M> {
	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.local_addr.ok_or_else(|| Error::Custom("Local address not found".into()))
	}

	/// Start the server.
	pub fn start(mut self, methods: impl Into<Methods>) -> Result<ServerHandle, Error> {
		let max_request_body_size = self.max_request_body_size;
		let max_response_body_size = self.max_response_body_size;
		let access_control = self.access_control;
		let (tx, mut rx) = mpsc::channel(1);
		let listener = self.listener;
		let resources = self.resources;
		let middleware = self.middleware;
		let methods = methods.into().initialize_resources(&resources)?;

		let make_service = make_service_fn(move |_| {
			let methods = methods.clone();
			let access_control = access_control.clone();
			let resources = resources.clone();
			let middleware = middleware.clone();

			async move {
				Ok::<_, HyperError>(service_fn(move |request| {
					let methods = methods.clone();
					let access_control = access_control.clone();
					let resources = resources.clone();
					let middleware = middleware.clone();

					// Run some validation on the http request, then read the body and try to deserialize it into one of
					// two cases: a single RPC request or a batch of RPC requests.
					async move {
						if let Err(e) = access_control_is_valid(&access_control, &request) {
							return Ok::<_, HyperError>(e);
						}

						// Only `POST` and `OPTIONS` methods are allowed.
						match *request.method() {
							// An OPTIONS request is a CORS preflight request. We've done our access check
							// above so we just need to tell the browser that the request is OK.
							Method::OPTIONS => {
								let origin = match http_helpers::read_header_value(request.headers(), "origin") {
									Some(origin) => origin,
									None => return Ok(malformed()),
								};
								let allowed_headers = access_control.allowed_headers().to_cors_header_value();
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
								let mut res = process_validated_request(
									request,
									middleware,
									methods,
									resources,
									max_request_body_size,
									max_response_body_size,
								)
								.await?;

								if let Some(origin) = origin {
									res.headers_mut().insert("access-control-allow-origin", origin);
								}
								Ok(res)
							}
							// Error scenarios:
							Method::POST => Ok(response::unsupported_content_type()),
							_ => Ok(response::method_not_allowed()),
						}
					}
				}))
			}
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

// Checks to that access control of the received request is the same as configured.
fn access_control_is_valid(
	access_control: &AccessControl,
	request: &hyper::Request<hyper::Body>,
) -> Result<(), hyper::Response<hyper::Body>> {
	if access_control.deny_host(request) {
		return Err(response::host_not_allowed());
	}
	if access_control.deny_cors_origin(request) {
		return Err(response::invalid_allow_origin());
	}
	if access_control.deny_cors_header(request) {
		return Err(response::invalid_allow_headers());
	}
	Ok(())
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

/// Process a verified request, it implies a POST request with content type JSON.
async fn process_validated_request(
	request: hyper::Request<hyper::Body>,
	middleware: impl Middleware,
	methods: Methods,
	resources: Resources,
	max_request_body_size: u32,
	max_response_body_size: u32,
) -> Result<hyper::Response<hyper::Body>, HyperError> {
	let (parts, body) = request.into_parts();

	let (body, mut is_single) = match read_body(&parts.headers, body, max_request_body_size).await {
		Ok(r) => r,
		Err(GenericTransportError::TooLarge) => return Ok(response::too_large()),
		Err(GenericTransportError::Malformed) => return Ok(response::malformed()),
		Err(GenericTransportError::Inner(e)) => {
			tracing::error!("Internal error reading request body: {}", e);
			return Ok(response::internal_error());
		}
	};

	let request_start = middleware.on_request();

	// NOTE(niklasad1): it's a channel because it's needed for batch requests.
	let (tx, mut rx) = mpsc::unbounded::<String>();
	let sink = MethodSink::new_with_limit(tx, max_response_body_size);

	type Notif<'a> = Notification<'a, Option<&'a RawValue>>;

	// Single request or notification
	if is_single {
		if let Ok(req) = serde_json::from_slice::<Request>(&body) {
			let method = req.method.as_ref();
			middleware.on_call(method);

			let id = req.id.clone();
			let params = Params::new(req.params.map(|params| params.get()));

			let result = match methods.method_with_name(method) {
				None => {
					sink.send_error(req.id, ErrorCode::MethodNotFound.into());
					false
				}
				Some((name, method_callback)) => match method_callback.inner() {
					MethodKind::Sync(callback) => match method_callback.claim(&req.method, &resources) {
						Ok(guard) => {
							let result = (callback)(id, params, &sink);
							drop(guard);
							result
						}
						Err(err) => {
							tracing::error!("[Methods::execute_with_resources] failed to lock resources: {:?}", err);
							sink.send_error(req.id, ErrorCode::ServerIsBusy.into());
							false
						}
					},
					MethodKind::Async(callback) => match method_callback.claim(name, &resources) {
						Ok(guard) => {
							let result =
								(callback)(id.into_owned(), params.into_owned(), sink.clone(), 0, Some(guard)).await;
							result
						}
						Err(err) => {
							tracing::error!("[Methods::execute_with_resources] failed to lock resources: {:?}", err);
							sink.send_error(req.id, ErrorCode::ServerIsBusy.into());
							false
						}
					},
					MethodKind::Subscription(_) => {
						tracing::error!("Subscriptions not supported on HTTP");
						sink.send_error(req.id, ErrorCode::InternalError.into());
						false
					}
				},
			};
			middleware.on_result(&req.method, result, request_start);
		} else if let Ok(_req) = serde_json::from_slice::<Notif>(&body) {
			return Ok::<_, HyperError>(response::ok_response("".into()));
		} else {
			let (id, code) = prepare_error(&body);
			sink.send_error(id, code.into());
		}
	// Batch of requests or notifications
	} else if let Ok(batch) = serde_json::from_slice::<Vec<Request>>(&body) {
		if !batch.is_empty() {
			let middleware = &middleware;

			join_all(batch.into_iter().filter_map(move |req| {
				let id = req.id.clone();
				let params = Params::new(req.params.map(|params| params.get()));

				match methods.method_with_name(&req.method) {
					None => {
						sink.send_error(req.id, ErrorCode::MethodNotFound.into());
						None
					}
					Some((name, method_callback)) => match method_callback.inner() {
						MethodKind::Sync(callback) => match method_callback.claim(name, &resources) {
							Ok(guard) => {
								let result = (callback)(id, params, &sink);
								middleware.on_result(name, result, request_start);
								drop(guard);
								None
							}
							Err(err) => {
								tracing::error!(
									"[Methods::execute_with_resources] failed to lock resources: {:?}",
									err
								);
								sink.send_error(req.id, ErrorCode::ServerIsBusy.into());
								middleware.on_result(name, false, request_start);
								None
							}
						},
						MethodKind::Async(callback) => match method_callback.claim(name, &resources) {
							Ok(guard) => {
								let sink = sink.clone();
								let id = id.into_owned();
								let params = params.into_owned();
								let callback = callback.clone();

								Some(async move {
									let result = (callback)(id, params, sink, 0, Some(guard)).await;
									middleware.on_result(name, result, request_start);
								})
							}
							Err(err) => {
								tracing::error!(
									"[Methods::execute_with_resources] failed to lock resources: {:?}",
									err
								);
								sink.send_error(req.id, ErrorCode::ServerIsBusy.into());
								middleware.on_result(name, false, request_start);
								None
							}
						},
						MethodKind::Subscription(_) => {
							tracing::error!("Subscriptions not supported on HTTP");
							sink.send_error(req.id, ErrorCode::InternalError.into());
							middleware.on_result(&req.method, false, request_start);
							None
						}
					},
				}
			}))
			.await;
		} else {
			// "If the batch rpc call itself fails to be recognized as an valid JSON or as an
			// Array with at least one value, the response from the Server MUST be a single
			// Response object." – The Spec.
			is_single = true;
			sink.send_error(Id::Null, ErrorCode::InvalidRequest.into());
		}
	} else if let Ok(_batch) = serde_json::from_slice::<Vec<Notif>>(&body) {
		return Ok(response::ok_response("".into()));
	} else {
		// "If the batch rpc call itself fails to be recognized as an valid JSON or as an
		// Array with at least one value, the response from the Server MUST be a single
		// Response object." – The Spec.
		is_single = true;
		let (id, code) = prepare_error(&body);
		sink.send_error(id, code.into());
	}

	// Closes the receiving half of a channel without dropping it. This prevents any further
	// messages from being sent on the channel.
	rx.close();
	let response = if is_single {
		rx.next().await.expect("Sender is still alive managed by us above; qed")
	} else {
		collect_batch_response(rx).await
	};
	tracing::debug!("[service_fn] sending back: {:?}", &response[..cmp::min(response.len(), 1024)]);
	middleware.on_response(request_start);
	Ok(response::ok_response(response))
}
