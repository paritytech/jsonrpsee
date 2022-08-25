use std::error::Error as StdError;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::io::{BufReader, BufWriter};
use futures_util::{Future, FutureExt, TryStreamExt};
use http::Method;
use jsonrpsee_core::error::{Error, GenericTransportError};
use jsonrpsee_core::http_helpers::{self, read_body};
use jsonrpsee_core::logger::{self, WsLogger as Logger};
use jsonrpsee_core::server::helpers::{
	prepare_error, BatchResponse, BatchResponseBuilder, BoundedSubscriptions, MethodResponse,
};
use jsonrpsee_core::server::rpc_module::{ConnectionId, MethodKind};
use jsonrpsee_core::server::{access_control::AccessControl, resource_limiting::Resources, rpc_module::Methods};
use jsonrpsee_core::tracing::{rx_log_from_json, tx_log_from_str, RpcTracing};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_core::JsonRawValue;
use jsonrpsee_types::error::{ErrorCode, BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG};
use jsonrpsee_types::{ErrorObject, Id, Notification, Params, Request};
use soketto::{handshake::http::is_upgrade_request, BoxedError};
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};
use tracing_futures::Instrument;

use crate::future::StopMonitor;
use crate::response;
use crate::server::background_task;

type Notif<'a> = Notification<'a, Option<&'a JsonRawValue>>;

/// Data required by the server to handle requests.
#[derive(Debug, Clone)]
pub struct ServiceData {
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
	pub max_connections: u64,
	pub next_conn_id: u32,
	pub open_connections: Arc<()>,
}

impl ServiceData {
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
			..
		} = self;

		//let request_start = logger.on_request(remote_addr, &request);

		let host = match http_helpers::read_header_value(request.headers(), "host") {
			Some(origin) => origin,
			None => return response::malformed(),
		};
		let maybe_origin = http_helpers::read_header_value(request.headers(), "origin");

		if let Err(e) = acl.verify_host(host) {
			tracing::warn!("Denied request: {}", e);
			return response::host_not_allowed();
		}

		if let Err(e) = acl.verify_origin(maybe_origin, host) {
			tracing::warn!("Denied request: {}", e);
			return response::origin_rejected(maybe_origin);
		}

		// Only the `POST` method is allowed.
		match *request.method() {
			Method::POST if content_type_is_json(&request) => {
				process_validated_request(ProcessValidatedRequest {
					request,
					methods,
					resources,
					max_request_body_size,
					max_response_body_size,
					max_log_length,
					batch_requests_supported,
				})
				.await
			}
			// Error scenarios:
			Method::POST => response::unsupported_content_type(),
			_ => response::method_not_allowed(),
		}
	}
}

/// JsonRPSee service compatible with `tower`.
///
/// # Note
/// This is similar to [`hyper::service::service_fn`].
#[derive(Debug)]
pub struct TowerService {
	pub inner: ServiceData,
}

impl hyper::service::Service<hyper::Request<hyper::Body>> for TowerService {
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
		if is_upgrade_request(&request) {
			// busy.
			if Arc::strong_count(&self.inner.open_connections) > self.inner.max_connections as usize {
				return async { Ok(response::internal_error()) }.boxed();
			}

			let data = self.inner.clone();

			tracing::trace!(
				"Accepting new connection: {}/{}",
				Arc::strong_count(&self.inner.open_connections),
				self.inner.max_connections
			);

			let mut server = soketto::handshake::http::Server::new();

			let response = match server.receive_request(&request) {
				Ok(response) => {
					let conn_id = self.inner.next_conn_id;
					self.inner.next_conn_id = conn_id.wrapping_add(1);

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

						let _ = background_task(sender, receiver, conn_id as usize, data).await;
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
			let data = self.inner.clone();
			Box::pin(data.handle_request(request).map(Ok))
		}
	}
}

pub struct ProcessValidatedRequest {
	pub request: hyper::Request<hyper::Body>,
	pub methods: Methods,
	pub resources: Resources,
	pub max_request_body_size: u32,
	pub max_response_body_size: u32,
	pub max_log_length: u32,
	pub batch_requests_supported: bool,
}

/// Process a verified request, it implies a POST request with content type JSON.
pub async fn process_validated_request(input: ProcessValidatedRequest) -> hyper::Response<hyper::Body> {
	let ProcessValidatedRequest {
		request,
		methods,
		resources,
		max_request_body_size,
		max_response_body_size,
		max_log_length,
		batch_requests_supported,
	} = input;

	let (parts, body) = request.into_parts();

	let (body, is_single) = match read_body(&parts.headers, body, max_request_body_size).await {
		Ok(r) => r,
		Err(GenericTransportError::TooLarge) => return response::too_large(max_request_body_size),
		Err(GenericTransportError::Malformed) => return response::malformed(),
		Err(GenericTransportError::Inner(e)) => {
			tracing::error!("Internal error reading request body: {}", e);
			return response::internal_error();
		}
	};

	// Single request or notification
	if is_single {
		let call =
			CallData { conn_id: 0, methods: &methods, max_response_body_size, max_log_length, resources: &resources };
		let response = process_single_request(body, call).await;

		response::ok_response(response.result)
	}
	// Batch of requests or notifications
	else if !batch_requests_supported {
		let err = MethodResponse::error(
			Id::Null,
			ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, &BATCHES_NOT_SUPPORTED_MSG, None),
		);

		response::ok_response(err.result)
	}
	// Batch of requests or notifications
	else {
		let response = process_batch_request(Batch {
			data: body,
			call: CallData {
				conn_id: 0,
				methods: &methods,
				max_response_body_size,
				max_log_length,
				resources: &resources,
			},
		})
		.await;
		response::ok_response(response.result)
	}
}

/// Checks that content type of received request is valid for JSON-RPC.
pub fn content_type_is_json(request: &hyper::Request<hyper::Body>) -> bool {
	is_json(request.headers().get("content-type"))
}

/// Returns true if the `content_type` header indicates a valid JSON message.
pub fn is_json(content_type: Option<&hyper::header::HeaderValue>) -> bool {
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

#[derive(Debug, Clone)]
pub struct Batch<'a> {
	data: Vec<u8>,
	call: CallData<'a>,
}

#[derive(Debug, Clone)]
pub struct CallData<'a> {
	conn_id: usize,
	methods: &'a Methods,
	max_response_body_size: u32,
	max_log_length: u32,
	resources: &'a Resources,
}

#[derive(Debug, Clone)]
pub struct Call<'a> {
	params: Params<'a>,
	name: &'a str,
	call: CallData<'a>,
	id: Id<'a>,
}

// Batch responses must be sent back as a single message so we read the results from each
// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
// complete batch response back to the client over `tx`.
pub async fn process_batch_request(b: Batch<'_>) -> BatchResponse {
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

pub async fn process_single_request(data: Vec<u8>, call: CallData<'_>) -> MethodResponse {
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

pub async fn execute_call(c: Call<'_>) -> MethodResponse {
	let Call { name, id, params, call } = c;
	let CallData { resources, methods, max_response_body_size, max_log_length, conn_id } = call;

	let response = match methods.method_with_name(name) {
		None => MethodResponse::error(id, ErrorObject::from(ErrorCode::MethodNotFound)),
		Some((name, method)) => match &method.inner() {
			MethodKind::Sync(callback) => match method.claim(name, resources) {
				Ok(guard) => {
					let r = (callback)(id, params, max_response_body_size as usize);
					drop(guard);
					r
				}
				Err(err) => {
					tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
					MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy))
				}
			},
			MethodKind::Async(callback) => match method.claim(name, resources) {
				Ok(guard) => {
					let id = id.into_owned();
					let params = params.into_owned();

					(callback)(id, params, conn_id, max_response_body_size as usize, Some(guard)).await
				}
				Err(err) => {
					tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
					MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy))
				}
			},
			MethodKind::Subscription(_) | MethodKind::Unsubscription(_) => {
				tracing::error!("Subscriptions not supported on HTTP");
				MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError))
			}
		},
	};

	tx_log_from_str(&response.result, max_log_length);
	response
}
