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
use crate::types::error::{ErrorCode, ErrorObject, BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG};
use crate::types::{Id, Request};
use futures_channel::mpsc;
use futures_util::future::{Either, FutureExt};
use futures_util::io::{BufReader, BufWriter};
use futures_util::stream::StreamExt;
use futures_util::{AsyncRead, TryStreamExt};
use http::header::{HOST, ORIGIN};
use http::{HeaderMap, HeaderValue};
use hyper::upgrade::Upgraded;
use jsonrpsee_core::id_providers::RandomIntegerIdProvider;
use jsonrpsee_core::logger::{self, WsLogger as Logger};
use jsonrpsee_core::server::access_control::AccessControl;
use jsonrpsee_core::server::helpers::{
	prepare_error, BatchResponse, BatchResponseBuilder, BoundedSubscriptions, MethodResponse, MethodSink,
};
use jsonrpsee_core::server::resource_limiting::Resources;
use jsonrpsee_core::server::rpc_module::{ConnState, ConnectionId, MethodKind, Methods};
use jsonrpsee_core::tracing::{rx_log_from_json, rx_log_from_str, tx_log_from_str, RpcTracing};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::{Error, TEN_MB_SIZE_BYTES};
use jsonrpsee_types::error::{reject_too_big_request, reject_too_many_subscriptions};
use jsonrpsee_types::Params;
use soketto::connection::Error as SokettoError;
use soketto::data::ByteSlice125;
use soketto::handshake::WebSocketKey;
use soketto::handshake::{server::Response, Server as SokettoServer};
use tokio::io::AsyncWrite;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio_stream::wrappers::IntervalStream;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};
use tracing_futures::Instrument;

// ...
use crate::tmp::*;
use hyper::body::HttpBody;
use tower::layer::util::Identity;
use tower::Layer;

/// Default maximum connections allowed.
const MAX_CONNECTIONS: u64 = 100;

type Sender = soketto::Sender<BufReader<BufWriter<Compat<Upgraded>>>>;
type Receiver = soketto::Receiver<BufReader<BufWriter<Compat<Upgraded>>>>;

/// A WebSocket JSON RPC server.
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

impl<B, U, L> Server<B, L>
where
	L: Logger,
	B: Layer<TowerService> + Send + 'static,
	<B as Layer<TowerService>>::Service: Send
		+ tower::Service<
			hyper::Request<hyper::Body>,
			Response = hyper::Response<U>,
			Error = Box<(dyn StdError + Send + Sync + 'static)>,
		>,
	<<B as Layer<TowerService>>::Service as tower::Service<hyper::Request<hyper::Body>>>::Future: Send,
	U: HttpBody + Send + 'static,
	<U as HttpBody>::Error: Send + Sync + StdError,
	<U as HttpBody>::Data: Send,
{
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
		let max_request_body_size = self.cfg.max_request_body_size;
		let max_response_body_size = self.cfg.max_response_body_size;
		let max_log_length = self.cfg.max_log_length;
		let acl = self.cfg.access_control;
		let listener = self.listener;
		let resources = self.resources;
		let _logger = self.logger;
		let batch_requests_supported = self.cfg.batch_requests_supported;
		let id_provider = self.id_provider;
		let stop_server = self.stop_monitor;
		let max_subscriptions_per_connection = self.cfg.max_subscriptions_per_connection;
		let max_connections = self.cfg.max_connections;

		let service = hyper::service::make_service_fn(move |conn: &hyper::server::conn::AddrStream| {
			let service = TowerService {
				inner: ServiceData {
					remote_addr: conn.remote_addr(),
					methods: methods.clone(),
					acl: acl.clone(),
					resources: resources.clone(),
					max_request_body_size,
					max_response_body_size,
					max_log_length,
					batch_requests_supported,
					id_provider: id_provider.clone(),
					ping_interval: self.cfg.ping_interval,
					stop_server: stop_server.clone(),
					max_subscriptions_per_connection,
					max_connections,
					next_conn_id: 0,
					open_connections: Arc::new(()),
				},
			};

			let server = self.service_builder.service(service);

			// For every request the `TowerService` is calling into `ServiceData::handle_request`
			// where the RPSee bare implementation resides.
			async move { Ok::<_, hyper::Error>(server) }
		});

		let server = hyper::Server::from_tcp(listener.into_std().unwrap()).unwrap().serve(service);
		let _ = server.await.unwrap();
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

enum HandshakeResponse<'a, L> {
	Reject {
		status_code: u16,
	},
	Accept {
		conn_id: ConnectionId,
		methods: &'a Methods,
		resources: &'a Resources,
		cfg: &'a Settings,
		stop_monitor: &'a StopMonitor,
		logger: L,
		id_provider: Arc<dyn IdProvider>,
	},
}

pub async fn background_task(
	mut sender: Sender,
	mut receiver: Receiver,
	conn_id: usize,
	svc: ServiceData,
) -> Result<(), Error> {
	let ServiceData {
		methods,
		resources,
		max_request_body_size,
		max_response_body_size,
		max_log_length,
		batch_requests_supported,
		stop_server,
		id_provider,
		ping_interval,
		max_subscriptions_per_connection,
		..
	} = svc;

	// And we can finally transition to a websocket background_task.

	let (tx, mut rx) = mpsc::unbounded::<String>();
	let bounded_subscriptions = BoundedSubscriptions::new(max_subscriptions_per_connection);
	let bounded_subscriptions2 = bounded_subscriptions.clone();

	let stop_server2 = stop_server.clone();
	let sink = MethodSink::new_with_limit(tx, max_response_body_size, max_log_length);

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
					if let Err(err) = send_ws_ping(&mut sender).await {
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

			if let Err(err) = method_executors.select_with(Monitored::new(receive, &stop_server)).await {
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
						resources,
						max_response_body_size,
						max_log_length,
						methods,
						bounded_subscriptions,
						sink: &sink,
						id_provider: &*id_provider,
					};

					match process_single_request(data, call).await {
						MethodResult::JustLogger(r) => {}
						MethodResult::SendAndLogger(r) => {
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
							max_log_length,
							methods,
							bounded_subscriptions,
							sink: &sink,
							id_provider: &*id_provider,
						},
					})
					.await;

					tx_log_from_str(&response.result, max_log_length);
					let _ = sink.send_raw(response.result);
				};

				method_executors.add(Box::pin(fut));
			}
			_ => {
				sink.send_error(Id::Null, ErrorCode::ParseError.into());
			}
		}
	};

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
	/// use jsonrpsee_ws_server::WsServerBuilder;
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
	/// let builder = WsServerBuilder::new().set_logger(MyLogger);
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

async fn send_ws_message(sender: &mut Sender, response: String) -> Result<(), Error> {
	sender.send_text_owned(response).await?;
	sender.flush().await.map_err(Into::into)
}

async fn send_ws_ping(sender: &mut Sender) -> Result<(), Error> {
	tracing::debug!("Send ping");
	// Submit empty slice as "optional" parameter.
	let slice: &[u8] = &[];
	// Byte slice fails if the provided slice is larger than 125 bytes.
	let byte_slice = ByteSlice125::try_from(slice).expect("Empty slice should fit into ByteSlice125");
	sender.send_ping(byte_slice).await?;
	sender.flush().await.map_err(Into::into)
}

#[derive(Debug, Clone)]
struct Batch<'a> {
	data: Vec<u8>,
	call: CallData<'a>,
}

#[derive(Debug, Clone)]
struct CallData<'a> {
	conn_id: usize,
	bounded_subscriptions: BoundedSubscriptions,
	id_provider: &'a dyn IdProvider,
	methods: &'a Methods,
	max_response_body_size: u32,
	max_log_length: u32,
	resources: &'a Resources,
	sink: &'a MethodSink,
}

#[derive(Debug, Clone)]
struct Call<'a> {
	params: Params<'a>,
	name: &'a str,
	call: CallData<'a>,
	id: Id<'a>,
}

enum MethodResult {
	JustLogger(MethodResponse),
	SendAndLogger(MethodResponse),
}

impl MethodResult {
	fn as_inner(&self) -> &MethodResponse {
		match &self {
			Self::JustLogger(r) => r,
			Self::SendAndLogger(r) => r,
		}
	}
}

// Batch responses must be sent back as a single message so we read the results from each
// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
// complete batch response back to the client over `tx`.
async fn process_batch_request(b: Batch<'_>) -> BatchResponse {
	let Batch { data, call } = b;

	if let Ok(batch) = serde_json::from_slice::<Vec<Request>>(&data) {
		return if !batch.is_empty() {
			let batch = batch.into_iter().map(|req| Ok((req, call.clone())));
			let batch_stream = futures_util::stream::iter(batch);

			let trace = RpcTracing::batch();

			return async {
				let max_response_size = call.max_response_body_size;

				let batch_response = batch_stream
					.try_fold(
						BatchResponseBuilder::new_with_limit(max_response_size as usize),
						|batch_response, (req, call)| async move {
							let params = Params::new(req.params.map(|params| params.get()));
							let response = execute_call(Call { name: &req.method, params, id: req.id, call }).await;
							batch_response.append(response.as_inner())
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
		} else {
			BatchResponse::error(Id::Null, ErrorObject::from(ErrorCode::InvalidRequest))
		};
	}

	let (id, code) = prepare_error(&data);
	BatchResponse::error(id, ErrorObject::from(code))
}

async fn process_single_request(data: Vec<u8>, call: CallData<'_>) -> MethodResult {
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
	} else {
		let (id, code) = prepare_error(&data);
		MethodResult::SendAndLogger(MethodResponse::error(id, ErrorObject::from(code)))
	}
}

/// Execute a call which returns result of the call with a additional sink
/// to fire a signal once the subscription call has been answered.
///
/// Returns `(MethodResponse, None)` on every call that isn't a subscription
/// Otherwise `(MethodResponse, Some(PendingSubscriptionCallTx)`.
async fn execute_call(c: Call<'_>) -> MethodResult {
	let Call { name, id, params, call } = c;
	let CallData {
		resources,
		methods,
		max_response_body_size,
		max_log_length,
		conn_id,
		bounded_subscriptions,
		id_provider,
		sink,
	} = call;

	let response = match methods.method_with_name(name) {
		None => {
			let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::MethodNotFound));
			MethodResult::SendAndLogger(response)
		}
		Some((name, method)) => match &method.inner() {
			MethodKind::Sync(callback) => match method.claim(name, resources) {
				Ok(guard) => {
					let r = (callback)(id, params, max_response_body_size as usize);
					drop(guard);
					MethodResult::SendAndLogger(r)
				}
				Err(err) => {
					tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
					let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy));
					MethodResult::SendAndLogger(response)
				}
			},
			MethodKind::Async(callback) => match method.claim(name, resources) {
				Ok(guard) => {
					let id = id.into_owned();
					let params = params.into_owned();

					let response = (callback)(id, params, conn_id, max_response_body_size as usize, Some(guard)).await;
					MethodResult::SendAndLogger(response)
				}
				Err(err) => {
					tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
					let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy));
					MethodResult::SendAndLogger(response)
				}
			},
			MethodKind::Subscription(callback) => match method.claim(name, resources) {
				Ok(guard) => {
					if let Some(cn) = bounded_subscriptions.acquire() {
						let conn_state = ConnState { conn_id, close_notify: cn, id_provider };
						let response = callback(id.clone(), params, sink.clone(), conn_state, Some(guard)).await;
						MethodResult::JustLogger(response)
					} else {
						let response =
							MethodResponse::error(id, reject_too_many_subscriptions(bounded_subscriptions.max()));
						MethodResult::SendAndLogger(response)
					}
				}
				Err(err) => {
					tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
					let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy));
					MethodResult::SendAndLogger(response)
				}
			},
			MethodKind::Unsubscription(callback) => {
				// Don't adhere to any resource or subscription limits; always let unsubscribing happen!
				let result = callback(id, params, conn_id, max_response_body_size as usize);
				MethodResult::SendAndLogger(result)
			}
		},
	};

	let r = response.as_inner();

	rx_log_from_str(&r.result, max_log_length);
	response
}
