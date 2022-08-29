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

use crate::future::{FutureDriver, ServerHandle, StopMonitor};
use crate::transport::{http, ws};
use crate::types::error::{ErrorCode, ErrorObject, BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG};
use crate::types::Id;

use futures_channel::mpsc;
use futures_util::future::{Either, FutureExt};
use futures_util::io::{BufReader, BufWriter};
use futures_util::stream::StreamExt;

use hyper::body::HttpBody;
use jsonrpsee_core::id_providers::RandomIntegerIdProvider;
use jsonrpsee_core::logger::{HttpLogger, WsLogger};
use jsonrpsee_core::server::access_control::AccessControl;
use jsonrpsee_core::server::helpers::{BoundedSubscriptions, MethodResponse, MethodSink};
use jsonrpsee_core::server::resource_limiting::Resources;
use jsonrpsee_core::server::rpc_module::Methods;
use jsonrpsee_core::tracing::tx_log_from_str;
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::{http_helpers, Error, TEN_MB_SIZE_BYTES};
use jsonrpsee_types::error::reject_too_big_request;
use soketto::connection::Error as SokettoError;

use soketto::handshake::http::is_upgrade_request;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_stream::wrappers::IntervalStream;
use tokio_util::compat::TokioAsyncReadCompatExt;
use tower::layer::util::Identity;
use tower::{Layer, Service};

/// Default maximum connections allowed.
const MAX_CONNECTIONS: u64 = 100;

/// JSON RPC server.
pub struct Server<B = Identity, HL = (), WL = ()> {
	listener: TcpListener,
	cfg: Settings,
	stop_monitor: StopMonitor,
	resources: Resources,
	http_logger: HL,
	ws_logger: WL,
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

impl<B, HL, WL> Server<B, HL, WL> {
	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.listener.local_addr().map_err(Into::into)
	}

	/// Returns the handle to stop the running server.
	pub fn server_handle(&self) -> ServerHandle {
		self.stop_monitor.handle()
	}
}

impl<B, U, HL, WL> Server<B, HL, WL>
where
	HL: jsonrpsee_core::logger::HttpLogger,
	WL: jsonrpsee_core::logger::WsLogger,
	B: Layer<TowerService<HL, WL>> + Send + 'static,
	<B as Layer<TowerService<HL, WL>>>::Service: Send
		+ Service<
			hyper::Request<hyper::Body>,
			Response = hyper::Response<U>,
			Error = Box<(dyn StdError + Send + Sync + 'static)>,
		>,
	<<B as Layer<TowerService<HL, WL>>>::Service as Service<hyper::Request<hyper::Body>>>::Future: Send,
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
		let acl = self.cfg.access_control;
		let resources = self.resources;
		let listener = self.listener;
		let http_logger = self.http_logger;
		let ws_logger = self.ws_logger;
		let batch_requests_supported = self.cfg.batch_requests_supported;
		let id_provider = self.id_provider;
		let stop_monitor = self.stop_monitor;
		let max_subscriptions_per_connection = self.cfg.max_subscriptions_per_connection;

		let mut id: u32 = 0;
		let mut connections = FutureDriver::default();
		let mut incoming = Monitored::new(Incoming(listener), &stop_monitor);

		loop {
			match connections.select_with(&mut incoming).await {
				Ok((socket, _addr)) => {
					if let Err(e) = socket.set_nodelay(true) {
						tracing::warn!("Could not set NODELAY on socket: {:?}", e);
						continue;
					}

					if connections.count() >= self.cfg.max_connections as usize {
						tracing::warn!("Too many connections. Please try again later.");
						connections.add(http::reject_connection(socket).boxed());
						continue;
					}

					let stop_requested = stop_monitor.stopped();

					let tower_service = TowerService {
						inner: ServiceData {
							remote_addr: socket.local_addr().unwrap(),
							methods: methods.clone(),
							acl: acl.clone(),
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
							ws_logger: ws_logger.clone(),
							http_logger: http_logger.clone(),
						},
					};

					let service = self.service_builder.service(tower_service);

					connections.add(
						async {
							let mut conn =
								hyper::server::conn::Http::new().serve_connection(socket, service).with_upgrades();

							let mut conn = Pin::new(&mut conn);

							tokio::select! {
								res = &mut conn => {
									if let Err(e) = res {
										tracing::error!("Error when processing connection: {:?}", e);
									}
								},
								_ = stop_requested => {
									conn.graceful_shutdown();
								}
							}
						}
						.boxed(),
					);

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

async fn background_task<HL: HttpLogger, WL: WsLogger>(
	mut sender: ws::Sender,
	mut receiver: ws::Receiver,
	svc: ServiceData<HL, WL>,
) -> Result<(), Error> {
	let ServiceData {
		methods,
		resources,
		max_request_body_size,
		max_response_body_size,
		max_log_length,
		batch_requests_supported,
		stop_monitor,
		id_provider,
		ping_interval,
		max_subscriptions_per_connection,
		conn_id,
		ws_logger,
		remote_addr,
		..
	} = svc;

	// And we can finally transition to a websocket background_task.

	let (tx, mut rx) = mpsc::unbounded::<String>();
	let bounded_subscriptions = BoundedSubscriptions::new(max_subscriptions_per_connection);
	let bounded_subscriptions2 = bounded_subscriptions.clone();

	let stop_monitor2 = stop_monitor.clone();
	let sink = MethodSink::new_with_limit(tx, max_response_body_size, max_log_length);

	// Send results back to the client.
	tokio::spawn(async move {
		// Received messages from the WebSocket.
		let mut rx_item = rx.next();

		// Interval to send out continuously `pings`.
		let ping_interval = IntervalStream::new(tokio::time::interval(ping_interval));
		tokio::pin!(ping_interval);
		let mut next_ping = ping_interval.next();

		while !stop_monitor2.shutdown_requested() {
			// Ensure select is cancel-safe by fetching and storing the `rx_item` that did not finish yet.
			// Note: Although, this is cancel-safe already, avoid using `select!` macro for future proofing.
			match futures_util::future::select(rx_item, next_ping).await {
				Either::Left((Some(response), ping)) => {
					// If websocket message send fail then terminate the connection.
					if let Err(err) = ws::send_message(&mut sender, response).await {
						tracing::error!("Terminate connection: WS send error: {}", err);
						break;
					}
					rx_item = rx.next();
					next_ping = ping;
				}
				// Nothing else to receive.
				Either::Left((None, _)) => break,

				// Handle timer intervals.
				Either::Right((_, next_rx)) => {
					if let Err(err) = ws::send_ping(&mut sender).await {
						tracing::error!("Terminate connection: WS send ping error: {}", err);
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
	let logger = &ws_logger;

	let result = loop {
		data.clear();

		{
			// Need the extra scope to drop this pinned future and reclaim access to `data`
			let receive = async {
				// Identical loop to `soketto::receive_data` with debug logs for `Pong` frames.
				loop {
					match receiver.receive(&mut data).await? {
						soketto::Incoming::Data(d) => break Ok(d),
						soketto::Incoming::Pong(_) => tracing::debug!("Received pong"),
						soketto::Incoming::Closed(_) => {
							// The closing reason is already logged by `soketto` trace log level.
							// Return the `Closed` error to avoid logging unnecessary warnings on clean shutdown.
							break Err(SokettoError::Closed);
						}
					}
				}
			};

			tokio::pin!(receive);

			if let Err(err) = method_executors.select_with(Monitored::new(receive, &stop_monitor)).await {
				match err {
					MonitoredError::Selector(SokettoError::Closed) => {
						tracing::debug!("WS transport: Remote peer terminated the connection: {}", conn_id);
						sink.close();
						break Ok(());
					}
					MonitoredError::Selector(SokettoError::MessageTooLarge { current, maximum }) => {
						tracing::warn!(
							"WS transport error: Request length: {} exceeded max limit: {} bytes",
							current,
							maximum
						);
						sink.send_error(Id::Null, reject_too_big_request(max_request_body_size));
						continue;
					}
					// These errors can not be gracefully handled, so just log them and terminate the connection.
					MonitoredError::Selector(err) => {
						tracing::error!("Terminate connection {}: WS error: {}", conn_id, err);
						sink.close();
						break Err(err.into());
					}
					MonitoredError::Shutdown => break Ok(()),
				};
			};
		};

		let request_start = logger.on_request();

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
					let call = ws::CallData {
						conn_id: conn_id as usize,
						resources,
						max_response_body_size,
						max_log_length,
						methods,
						bounded_subscriptions,
						sink: &sink,
						id_provider,
						logger,
						request_start,
					};

					match ws::process_single_request(data, call).await {
						MethodResult::JustLogger(r) => {
							logger.on_response(&r.result, request_start);
						}
						MethodResult::SendAndLogger(r) => {
							logger.on_response(&r.result, request_start);
							let _ = sink.send_raw(r.result);
						}
					};
				}
				.boxed();

				method_executors.add(fut);
			}
			Some(b'[') if !batch_requests_supported => {
				let response = MethodResponse::error(
					Id::Null,
					ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, &BATCHES_NOT_SUPPORTED_MSG, None),
				);
				logger.on_response(&response.result, request_start);
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
					let response = ws::process_batch_request(ws::Batch {
						data,
						call: ws::CallData {
							conn_id: conn_id as usize,
							resources,
							max_response_body_size,
							max_log_length,
							methods,
							bounded_subscriptions,
							sink: &sink,
							id_provider: &*id_provider,
							logger,
							request_start,
						},
					})
					.await;

					tx_log_from_str(&response.result, max_log_length);
					logger.on_response(&response.result, request_start);
					let _ = sink.send_raw(response.result);
				};

				method_executors.add(Box::pin(fut));
			}
			_ => {
				sink.send_error(Id::Null, ErrorCode::ParseError.into());
			}
		}
	};

	logger.on_disconnect(remote_addr);

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
pub struct Builder<B = Identity, HL = (), WL = ()> {
	settings: Settings,
	resources: Resources,
	http_logger: HL,
	ws_logger: WL,
	id_provider: Arc<dyn IdProvider>,
	service_builder: tower::ServiceBuilder<B>,
}

impl Default for Builder {
	fn default() -> Self {
		Builder {
			settings: Settings::default(),
			resources: Resources::default(),
			http_logger: (),
			ws_logger: (),
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

impl<B, HL, WL> Builder<B, HL, WL> {
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

	/// Add a logger to the builder [`Logger`](../jsonrpsee_core/logger/trait.Logger.html).
	///
	/// ```
	/// use std::{time::Instant, net::SocketAddr};
	///
	/// use jsonrpsee_core::logger::{WsLogger, Headers, MethodKind, Params};
	/// use jsonrpsee_server::ServerBuilder;
	///
	/// #[derive(Clone)]
	/// struct MyLogger;
	///
	/// impl WsLogger for MyLogger {
	///     type Instant = Instant;
	///
	///     fn on_connect(&self, remote_addr: SocketAddr, headers: &Headers) {
	///          println!("[MyLogger::on_call] remote_addr: {}, headers: {:?}", remote_addr, headers);
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
	///     fn on_disconnect(&self, remote_addr: SocketAddr) {
	///          println!("[MyLogger::on_disconnect] remote_addr: {}", remote_addr);
	///     }
	/// }
	///
	/// let builder = ServerBuilder::new().set_logger(MyLogger);
	/// ```
	pub fn set_ws_logger<T: WsLogger>(self, ws_logger: T) -> Builder<B, HL, T> {
		Builder {
			settings: self.settings,
			resources: self.resources,
			ws_logger,
			http_logger: self.http_logger,
			id_provider: self.id_provider,
			service_builder: self.service_builder,
		}
	}

	/// TODO unify logger.
	pub fn set_http_logger<T: HttpLogger>(self, http_logger: T) -> Builder<B, T, WL> {
		Builder {
			settings: self.settings,
			resources: self.resources,
			ws_logger: self.ws_logger,
			http_logger,
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
	/// use jsonrpsee_ws_server::{ServerBuilder, RandomStringIdProvider, IdProvider};
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

	/// Sets access control settings.
	pub fn set_access_control(mut self, acl: AccessControl) -> Self {
		self.settings.access_control = acl;
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
	/// use jsonrpsee_http_server::ServerBuilder;
	///
	/// #[tokio::main]
	/// async fn main() {
	///     let builder = tower::ServiceBuilder::new()
	///         .timeout(Duration::from_secs(2));
	///
	///     let server = ServerBuilder::new()
	///         .set_middleware(builder)
	///         .build("127.0.0.1:0".parse::<SocketAddr>().unwrap())
	///         .await
	///         .unwrap();
	/// }
	/// ```
	pub fn set_middleware<T>(self, service_builder: tower::ServiceBuilder<T>) -> Builder<T, HL, WL> {
		Builder {
			settings: self.settings,
			resources: self.resources,
			http_logger: self.http_logger,
			ws_logger: self.ws_logger,
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
	pub async fn build(self, addrs: impl ToSocketAddrs) -> Result<Server<B, HL, WL>, Error> {
		let listener = TcpListener::bind(addrs).await?;
		let stop_monitor = StopMonitor::new();
		let resources = self.resources;
		Ok(Server {
			listener,
			cfg: self.settings,
			stop_monitor,
			resources,
			http_logger: self.http_logger,
			ws_logger: self.ws_logger,
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
struct ServiceData<HL: HttpLogger, WL: WsLogger> {
	/// Remote server address.
	remote_addr: SocketAddr,
	/// Registered server methods.
	methods: Methods,
	/// Access control.
	acl: AccessControl,
	/// Tracker for currently used resources on the server.
	resources: Resources,
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
	/// Subscription ID provider.
	id_provider: Arc<dyn IdProvider>,
	/// Ping interval
	ping_interval: Duration,
	/// Stop handle.
	stop_monitor: StopMonitor,
	/// Max subscriptions per connection.
	max_subscriptions_per_connection: u32,
	/// Connection ID
	conn_id: u32,
	/// WebSocket logger.
	ws_logger: WL,
	/// HTTP logger.
	http_logger: HL,
}

impl<HL: HttpLogger, WL: WsLogger> ServiceData<HL, WL> {
	/// Default behavior for handling the RPC requests.
	async fn handle_request(self, request: hyper::Request<hyper::Body>) -> hyper::Response<hyper::Body> {
		let data = http::HandleRequest {
			remote_addr: self.remote_addr,
			methods: self.methods,
			resources: self.resources,
			max_request_body_size: self.max_request_body_size,
			max_response_body_size: self.max_response_body_size,
			max_log_length: self.max_log_length,
			batch_requests_supported: self.batch_requests_supported,
			logger: self.http_logger,
		};

		http::handle_request(request, data).await
	}
}

/// JsonRPSee service compatible with `tower`.
///
/// # Note
/// This is similar to [`hyper::service::service_fn`].
#[derive(Debug)]
pub struct TowerService<HL: HttpLogger, WL: WsLogger> {
	inner: ServiceData<HL, WL>,
}

impl<HL: HttpLogger, WL: WsLogger> hyper::service::Service<hyper::Request<hyper::Body>> for TowerService<HL, WL> {
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

		let host = match http_helpers::read_header_value(request.headers(), "host") {
			Some(origin) => origin,
			None => return async { Ok(http::response::malformed()) }.boxed(),
		};
		let maybe_origin = http_helpers::read_header_value(request.headers(), "origin");

		if let Err(e) = data.acl.verify_host(host) {
			tracing::warn!("Denied request: {}", e);
			return async { Ok(http::response::host_not_allowed()) }.boxed();
		}

		if let Err(e) = data.acl.verify_origin(maybe_origin, host) {
			let maybe_origin = maybe_origin.map(|o| o.to_owned());
			tracing::warn!("Denied request: {}", e);
			return async { Ok(http::response::origin_rejected(maybe_origin)) }.boxed();
		}

		if is_upgrade_request(&request) {
			let mut server = soketto::handshake::http::Server::new();

			let response = match server.receive_request(&request) {
				Ok(response) => {
					data.ws_logger.on_connect(data.remote_addr, request.headers());

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
						ws_builder.set_max_message_size(data.max_response_body_size as usize);
						let (sender, receiver) = ws_builder.finish();

						let _ = background_task(sender, receiver, data).await;
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
			Box::pin(data.handle_request(request).map(Ok))
		}
	}
}
