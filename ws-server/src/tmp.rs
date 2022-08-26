use std::error::Error as StdError;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::io::{BufReader, BufWriter};
use futures_util::{Future, FutureExt};
use http::Method;
use jsonrpsee_core::http_helpers;
use jsonrpsee_core::logger::{HttpLogger, WsLogger};
use jsonrpsee_core::server::http_utils::{self, response as http_response, ProcessValidatedRequest};
use jsonrpsee_core::server::{access_control::AccessControl, resource_limiting::Resources, rpc_module::Methods};
use jsonrpsee_core::traits::IdProvider;
use soketto::handshake::http::is_upgrade_request;
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::future::StopMonitor;
use crate::server::background_task;

/// Data required by the server to handle requests.
#[derive(Debug, Clone)]
pub struct ServiceData<HL: HttpLogger, WL: WsLogger> {
	/// Remote server address.
	pub remote_addr: SocketAddr,
	/// Registered server methods.
	pub methods: Methods,
	/// Access control.
	pub acl: AccessControl,
	/// Tracker for currently used resources on the server.
	pub resources: Resources,
	/// Max request body size.
	pub max_request_body_size: u32,
	/// Max response body size.
	pub max_response_body_size: u32,
	/// Max length for logging for request and response
	///
	/// Logs bigger than this limit will be truncated.
	pub max_log_length: u32,
	/// Whether batch requests are supported by this server or not.
	pub batch_requests_supported: bool,
	pub id_provider: Arc<dyn IdProvider>,
	pub ping_interval: Duration,
	pub stop_server: StopMonitor,
	pub max_subscriptions_per_connection: u32,
	pub conn_id: u32,
	pub ws_logger: WL,
	pub http_logger: HL,
}

impl<HL: HttpLogger, WL: WsLogger> ServiceData<HL, WL> {
	/// Default behavior for handling the RPC requests.
	async fn handle_request(self, request: hyper::Request<hyper::Body>) -> hyper::Response<hyper::Body> {
		let ServiceData {
			remote_addr,
			methods,
			acl,
			resources,
			max_request_body_size,
			max_response_body_size,
			max_log_length,
			batch_requests_supported,
			http_logger,
			..
		} = self;

		let request_start = http_logger.on_request(remote_addr, &request);

		let host = match http_helpers::read_header_value(request.headers(), "host") {
			Some(origin) => origin,
			None => return http_response::malformed(),
		};
		let maybe_origin = http_helpers::read_header_value(request.headers(), "origin");

		if let Err(e) = acl.verify_host(host) {
			tracing::warn!("Denied request: {}", e);
			return http_response::host_not_allowed();
		}

		if let Err(e) = acl.verify_origin(maybe_origin, host) {
			tracing::warn!("Denied request: {}", e);
			return http_response::origin_rejected(maybe_origin);
		}

		// Only the `POST` method is allowed.
		match *request.method() {
			Method::POST if http_utils::content_type_is_json(&request) => {
				http_utils::process_validated_request(ProcessValidatedRequest {
					request,
					methods,
					resources,
					max_request_body_size,
					max_response_body_size,
					max_log_length,
					batch_requests_supported,
					logger: http_logger,
					request_start,
				})
				.await
			}
			// Error scenarios:
			Method::POST => http_response::unsupported_content_type(),
			_ => http_response::method_not_allowed(),
		}
	}
}

/// JsonRPSee service compatible with `tower`.
///
/// # Note
/// This is similar to [`hyper::service::service_fn`].
#[derive(Debug)]
pub struct TowerService<HL: HttpLogger, WL: WsLogger> {
	pub inner: ServiceData<HL, WL>,
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

		if is_upgrade_request(&request) {
			let mut server = soketto::handshake::http::Server::new();

			let response = match server.receive_request(&request) {
				Ok(response) => {
					data.ws_logger.on_connect(data.remote_addr, request.headers());

					tokio::spawn(async move {
						tracing::trace!("waiting on upgrade request");

						let upgraded = match hyper::upgrade::on(request).await {
							Ok(u) => u,
							Err(e) => panic!("WebSocket upgrade request failed: {:?}", e),
						};

						tracing::trace!("upgrade request ok");

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
					hyper::Response::new(hyper::Body::from(format!("Could not upgrade connection: {}", e.to_string())))
				}
			};

			async { Ok(response) }.boxed()
		} else {
			// The request wasn't an upgrade request; let's treat it as a standard HTTP request:
			Box::pin(data.handle_request(request).map(Ok))
		}
	}
}
