use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

use crate::logger::{self, Logger, TransportProtocol};

use futures_util::future::Either;
use futures_util::stream::{FuturesOrdered, StreamExt};
use http::Method;
use jsonrpsee_core::error::GenericTransportError;
use jsonrpsee_core::http_helpers::read_body;
use jsonrpsee_core::server::helpers::{prepare_error, BatchResponse, BatchResponseBuilder, MethodResponse};
use jsonrpsee_core::server::rpc_module::MethodKind;
use jsonrpsee_core::server::{resource_limiting::Resources, rpc_module::Methods};
use jsonrpsee_core::tracing::{rx_log_from_json, tx_log_from_str};
use jsonrpsee_core::JsonRawValue;
use jsonrpsee_types::error::{ErrorCode, BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG};
use jsonrpsee_types::{ErrorObject, Id, InvalidRequest, Notification, Params, Request};
use tokio::sync::OwnedSemaphorePermit;
use tracing::instrument;

type Notif<'a> = Notification<'a, Option<&'a JsonRawValue>>;

/// Checks that content type of received request is valid for JSON-RPC.
pub(crate) fn content_type_is_json(request: &hyper::Request<hyper::Body>) -> bool {
	is_json(request.headers().get(http::header::CONTENT_TYPE))
}

/// Returns true if the `content_type` header indicates a valid JSON message.
pub(crate) fn is_json(content_type: Option<&hyper::header::HeaderValue>) -> bool {
	content_type.and_then(|val| val.to_str().ok()).map_or(false, |content| {
		content.eq_ignore_ascii_case("application/json")
			|| content.eq_ignore_ascii_case("application/json; charset=utf-8")
			|| content.eq_ignore_ascii_case("application/json;charset=utf-8")
	})
}

pub(crate) async fn reject_connection(socket: tokio::net::TcpStream) {
	async fn reject(_req: hyper::Request<hyper::Body>) -> Result<hyper::Response<hyper::Body>, Infallible> {
		Ok(response::too_many_requests())
	}

	if let Err(e) = hyper::server::conn::Http::new().serve_connection(socket, hyper::service::service_fn(reject)).await
	{
		tracing::warn!("Error when trying to deny connection: {:?}", e);
	}
}

#[derive(Debug)]
pub(crate) struct ProcessValidatedRequest<'a, L: Logger> {
	pub(crate) request: hyper::Request<hyper::Body>,
	pub(crate) logger: &'a L,
	pub(crate) methods: Methods,
	pub(crate) resources: Resources,
	pub(crate) max_request_body_size: u32,
	pub(crate) max_response_body_size: u32,
	pub(crate) max_log_length: u32,
	pub(crate) batch_requests_supported: bool,
	pub(crate) request_start: L::Instant,
}

/// Process a verified request, it implies a POST request with content type JSON.
pub(crate) async fn process_validated_request<L: Logger>(
	input: ProcessValidatedRequest<'_, L>,
) -> hyper::Response<hyper::Body> {
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
		Err(GenericTransportError::TooLarge) => return response::too_large(max_request_body_size),
		Err(GenericTransportError::Malformed) => return response::malformed(),
		Err(GenericTransportError::Inner(e)) => {
			tracing::error!("Internal error reading request body: {}", e);
			return response::internal_error();
		}
	};

	// Single request or notification
	if is_single {
		let call = CallData {
			conn_id: 0,
			logger,
			methods: &methods,
			max_response_body_size,
			max_log_length,
			resources: &resources,
			request_start,
		};
		let response = process_single_request(body, call).await;
		logger.on_response(&response.result, request_start, TransportProtocol::Http);
		response::ok_response(response.result)
	}
	// Batch of requests or notifications
	else if !batch_requests_supported {
		let err = MethodResponse::error(
			Id::Null,
			ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, &BATCHES_NOT_SUPPORTED_MSG, None),
		);
		logger.on_response(&err.result, request_start, TransportProtocol::Http);
		response::ok_response(err.result)
	}
	// Batch of requests or notifications
	else {
		let response = process_batch_request(Batch {
			data: body,
			call: CallData {
				conn_id: 0,
				logger,
				methods: &methods,
				max_response_body_size,
				max_log_length,
				resources: &resources,
				request_start,
			},
		})
		.await;
		logger.on_response(&response.result, request_start, TransportProtocol::Http);
		response::ok_response(response.result)
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Batch<'a, L: Logger> {
	data: Vec<u8>,
	call: CallData<'a, L>,
}

#[derive(Debug, Clone)]
pub(crate) struct CallData<'a, L: Logger> {
	conn_id: usize,
	logger: &'a L,
	methods: &'a Methods,
	max_response_body_size: u32,
	max_log_length: u32,
	resources: &'a Resources,
	request_start: L::Instant,
}

// Batch responses must be sent back as a single message so we read the results from each
// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
// complete batch response back to the client over `tx`.
#[instrument(name = "batch", skip(b), level = "TRACE")]
pub(crate) async fn process_batch_request<L>(b: Batch<'_, L>) -> BatchResponse
where
	L: Logger,
{
	let Batch { data, call } = b;

	if let Ok(batch) = serde_json::from_slice::<Vec<&JsonRawValue>>(&data) {
		let mut got_notif = false;
		let mut batch_response = BatchResponseBuilder::new_with_limit(call.max_response_body_size as usize);

		let mut pending_calls: FuturesOrdered<_> = batch
			.into_iter()
			.filter_map(|v| {
				if let Ok(req) = serde_json::from_str::<Request>(v.get()) {
					Some(Either::Right(execute_call(req, call.clone())))
				} else if let Ok(_notif) = serde_json::from_str::<Notification<&JsonRawValue>>(v.get()) {
					// notifications should not be answered.
					got_notif = true;
					None
				} else {
					// valid JSON but could be not parsable as `InvalidRequest`
					let id = match serde_json::from_str::<InvalidRequest>(v.get()) {
						Ok(err) => err.id,
						Err(_) => Id::Null,
					};

					Some(Either::Left(async {
						MethodResponse::error(id, ErrorObject::from(ErrorCode::InvalidRequest))
					}))
				}
			})
			.collect();

		while let Some(response) = pending_calls.next().await {
			if let Err(too_large) = batch_response.append(&response) {
				return too_large;
			}
		}

		if got_notif && batch_response.is_empty() {
			BatchResponse { result: String::new(), success: true }
		} else {
			batch_response.finish()
		}
	} else {
		BatchResponse::error(Id::Null, ErrorObject::from(ErrorCode::ParseError))
	}
}

pub(crate) async fn process_single_request<L: Logger>(data: Vec<u8>, call: CallData<'_, L>) -> MethodResponse {
	if let Ok(req) = serde_json::from_slice::<Request>(&data) {
		execute_call_with_tracing(req, call).await
	} else if let Ok(notif) = serde_json::from_slice::<Notif>(&data) {
		execute_notification(notif, call.max_log_length)
	} else {
		let (id, code) = prepare_error(&data);
		MethodResponse::error(id, ErrorObject::from(code))
	}
}

#[instrument(name = "method_call", fields(method = req.method.as_ref()), skip(call, req), level = "TRACE")]
pub(crate) async fn execute_call_with_tracing<'a, L: Logger>(
	req: Request<'a>,
	call: CallData<'_, L>,
) -> MethodResponse {
	execute_call(req, call).await
}

pub(crate) async fn execute_call<L: Logger>(req: Request<'_>, call: CallData<'_, L>) -> MethodResponse {
	let CallData { resources, methods, logger, max_response_body_size, max_log_length, conn_id, request_start } = call;

	rx_log_from_json(&req, call.max_log_length);

	let params = Params::new(req.params.map(|params| params.get()));
	let name = &req.method;
	let id = req.id;

	let response = match methods.method_with_name(name) {
		None => {
			logger.on_call(name, params.clone(), logger::MethodKind::Unknown, TransportProtocol::Http);
			MethodResponse::error(id, ErrorObject::from(ErrorCode::MethodNotFound))
		}
		Some((name, method)) => match &method.inner() {
			MethodKind::Sync(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::MethodCall, TransportProtocol::Http);

				match method.claim(name, resources) {
					Ok(guard) => {
						let r = (callback)(id, params, max_response_body_size as usize);
						drop(guard);
						r
					}
					Err(err) => {
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
						MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy))
					}
				}
			}
			MethodKind::Async(callback) => {
				logger.on_call(name, params.clone(), logger::MethodKind::MethodCall, TransportProtocol::Http);
				match method.claim(name, resources) {
					Ok(guard) => {
						let id = id.into_owned();
						let params = params.into_owned();

						(callback)(id, params, conn_id, max_response_body_size as usize, Some(guard)).await
					}
					Err(err) => {
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
						MethodResponse::error(id, ErrorObject::from(ErrorCode::ServerIsBusy))
					}
				}
			}
			MethodKind::Subscription(_) | MethodKind::Unsubscription(_) => {
				logger.on_call(name, params.clone(), logger::MethodKind::Unknown, TransportProtocol::Http);
				tracing::error!("Subscriptions not supported on HTTP");
				MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError))
			}
		},
	};

	tx_log_from_str(&response.result, max_log_length);
	logger.on_result(name, response.success, request_start, TransportProtocol::Http);
	response
}

#[instrument(name = "notification", fields(method = notif.method.as_ref()), skip(notif, max_log_length), level = "TRACE")]
fn execute_notification(notif: Notif, max_log_length: u32) -> MethodResponse {
	rx_log_from_json(&notif, max_log_length);
	let response = MethodResponse { result: String::new(), success: true };
	tx_log_from_str(&response.result, max_log_length);
	response
}

pub(crate) struct HandleRequest<L: Logger> {
	pub(crate) methods: Methods,
	pub(crate) resources: Resources,
	pub(crate) max_request_body_size: u32,
	pub(crate) max_response_body_size: u32,
	pub(crate) max_log_length: u32,
	pub(crate) batch_requests_supported: bool,
	pub(crate) logger: L,
	pub(crate) conn: Arc<OwnedSemaphorePermit>,
	pub(crate) remote_addr: SocketAddr,
}

pub(crate) async fn handle_request<L: Logger>(
	request: hyper::Request<hyper::Body>,
	input: HandleRequest<L>,
) -> hyper::Response<hyper::Body> {
	let HandleRequest {
		methods,
		resources,
		max_request_body_size,
		max_response_body_size,
		max_log_length,
		batch_requests_supported,
		logger,
		conn,
		remote_addr,
	} = input;

	let request_start = logger.on_request(TransportProtocol::Http);

	// Only the `POST` method is allowed.
	let res = match *request.method() {
		Method::POST if content_type_is_json(&request) => {
			process_validated_request(ProcessValidatedRequest {
				request,
				methods,
				resources,
				max_request_body_size,
				max_response_body_size,
				max_log_length,
				batch_requests_supported,
				logger: &logger,
				request_start,
			})
			.await
		}
		// Error scenarios:
		Method::POST => response::unsupported_content_type(),
		_ => response::method_not_allowed(),
	};

	drop(conn);
	logger.on_disconnect(remote_addr, TransportProtocol::Http);

	res
}

pub(crate) mod response {
	use jsonrpsee_types::error::reject_too_big_request;
	use jsonrpsee_types::error::{ErrorCode, ErrorResponse};
	use jsonrpsee_types::Id;

	const JSON: &str = "application/json; charset=utf-8";
	const TEXT: &str = "text/plain";

	/// Create a response for json internal error.
	pub(crate) fn internal_error() -> hyper::Response<hyper::Body> {
		let error = serde_json::to_string(&ErrorResponse::borrowed(ErrorCode::InternalError.into(), Id::Null))
			.expect("built from known-good data; qed");

		from_template(hyper::StatusCode::INTERNAL_SERVER_ERROR, error, JSON)
	}

	/// Create a text/plain response for not allowed hosts.
	pub(crate) fn host_not_allowed() -> hyper::Response<hyper::Body> {
		from_template(hyper::StatusCode::FORBIDDEN, "Provided Host header is not whitelisted.\n".to_owned(), TEXT)
	}

	/// Create a text/plain response for disallowed method used.
	pub(crate) fn method_not_allowed() -> hyper::Response<hyper::Body> {
		from_template(
			hyper::StatusCode::METHOD_NOT_ALLOWED,
			"Used HTTP Method is not allowed. POST or OPTIONS is required\n".to_owned(),
			TEXT,
		)
	}

	/// Create a json response for oversized requests (413)
	pub(crate) fn too_large(limit: u32) -> hyper::Response<hyper::Body> {
		let error = serde_json::to_string(&ErrorResponse::borrowed(reject_too_big_request(limit), Id::Null))
			.expect("built from known-good data; qed");

		from_template(hyper::StatusCode::PAYLOAD_TOO_LARGE, error, JSON)
	}

	/// Create a json response for empty or malformed requests (400)
	pub(crate) fn malformed() -> hyper::Response<hyper::Body> {
		let error = serde_json::to_string(&ErrorResponse::borrowed(ErrorCode::ParseError.into(), Id::Null))
			.expect("built from known-good data; qed");

		from_template(hyper::StatusCode::BAD_REQUEST, error, JSON)
	}

	/// Create a response body.
	fn from_template<S: Into<hyper::Body>>(
		status: hyper::StatusCode,
		body: S,
		content_type: &'static str,
	) -> hyper::Response<hyper::Body> {
		hyper::Response::builder()
			.status(status)
			.header("content-type", hyper::header::HeaderValue::from_static(content_type))
			.body(body.into())
			// Parsing `StatusCode` and `HeaderValue` is infalliable but
			// parsing body content is not.
			.expect("Unable to parse response body for type conversion")
	}

	/// Create a valid JSON response.
	pub(crate) fn ok_response(body: String) -> hyper::Response<hyper::Body> {
		from_template(hyper::StatusCode::OK, body, JSON)
	}

	/// Create a response for unsupported content type.
	pub(crate) fn unsupported_content_type() -> hyper::Response<hyper::Body> {
		from_template(
			hyper::StatusCode::UNSUPPORTED_MEDIA_TYPE,
			"Supplied content type is not allowed. Content-Type: application/json is required\n".to_owned(),
			TEXT,
		)
	}

	/// Create a response for when the server is busy and can't accept more requests.
	pub(crate) fn too_many_requests() -> hyper::Response<hyper::Body> {
		from_template(
			hyper::StatusCode::TOO_MANY_REQUESTS,
			"Too many connections. Please try again later.".to_owned(),
			TEXT,
		)
	}

	/// Create a response for when the server denied the request.
	pub(crate) fn denied() -> hyper::Response<hyper::Body> {
		from_template(hyper::StatusCode::FORBIDDEN, "".to_owned(), TEXT)
	}
}
