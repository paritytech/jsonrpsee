pub mod response;

use std::net::SocketAddr;

use crate::error::GenericTransportError;
use crate::http_helpers::{self, read_body};
use crate::logger::{self, HttpLogger as Logger};
use crate::server::helpers::{prepare_error, BatchResponse, BatchResponseBuilder, MethodResponse};
use crate::server::rpc_module::MethodKind;
use crate::server::{resource_limiting::Resources, rpc_module::Methods};
use crate::tracing::{rx_log_from_json, tx_log_from_str, RpcTracing};
use crate::JsonRawValue;
use futures_util::TryStreamExt;
use http::Method;
use jsonrpsee_types::error::{ErrorCode, BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG};
use jsonrpsee_types::{ErrorObject, Id, Notification, Params, Request};
use tracing_futures::Instrument;

use super::access_control::AccessControl;

type Notif<'a> = Notification<'a, Option<&'a JsonRawValue>>;

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

#[derive(Debug)]
pub struct ProcessValidatedRequest<L: Logger> {
	pub request: hyper::Request<hyper::Body>,
	pub logger: L,
	pub methods: Methods,
	pub resources: Resources,
	pub max_request_body_size: u32,
	pub max_response_body_size: u32,
	pub max_log_length: u32,
	pub batch_requests_supported: bool,
	pub request_start: L::Instant,
}

/// Process a verified request, it implies a POST request with content type JSON.
pub async fn process_validated_request<L: Logger>(input: ProcessValidatedRequest<L>) -> hyper::Response<hyper::Body> {
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
			logger: &logger,
			methods: &methods,
			max_response_body_size,
			max_log_length,
			resources: &resources,
			request_start,
		};
		let response = process_single_request(body, call).await;
		logger.on_response(&response.result, request_start);
		response::ok_response(response.result)
	}
	// Batch of requests or notifications
	else if !batch_requests_supported {
		let err = MethodResponse::error(
			Id::Null,
			ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, &BATCHES_NOT_SUPPORTED_MSG, None),
		);
		logger.on_response(&err.result, request_start);
		response::ok_response(err.result)
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
		response::ok_response(response.result)
	}
}

#[derive(Debug, Clone)]
pub struct Batch<'a, L: Logger> {
	data: Vec<u8>,
	call: CallData<'a, L>,
}

#[derive(Debug, Clone)]
pub struct CallData<'a, L: Logger> {
	conn_id: usize,
	logger: &'a L,
	methods: &'a Methods,
	max_response_body_size: u32,
	max_log_length: u32,
	resources: &'a Resources,
	request_start: L::Instant,
}

#[derive(Debug, Clone)]
pub struct Call<'a, L: Logger> {
	params: Params<'a>,
	name: &'a str,
	call: CallData<'a, L>,
	id: Id<'a>,
}

// Batch responses must be sent back as a single message so we read the results from each
// request in the batch and read the results off of a new channel, `rx_batch`, and then send the
// complete batch response back to the client over `tx`.
pub async fn process_batch_request<L>(b: Batch<'_, L>) -> BatchResponse
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

pub async fn process_single_request<L: Logger>(data: Vec<u8>, call: CallData<'_, L>) -> MethodResponse {
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

pub async fn execute_call<L: Logger>(c: Call<'_, L>) -> MethodResponse {
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
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
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
						tracing::error!("[Methods::execute_with_resources] failed to lock resources: {}", err);
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

pub struct HandleRequest<L: Logger> {
	pub remote_addr: SocketAddr,
	pub methods: Methods,
	pub acl: AccessControl,
	pub resources: Resources,
	pub max_request_body_size: u32,
	pub max_response_body_size: u32,
	pub max_log_length: u32,
	pub batch_requests_supported: bool,
	pub logger: L,
}

pub async fn handle_request<L: Logger>(
	request: hyper::Request<hyper::Body>,
	input: HandleRequest<L>,
) -> hyper::Response<hyper::Body> {
	let HandleRequest {
		remote_addr,
		methods,
		acl,
		resources,
		max_request_body_size,
		max_response_body_size,
		max_log_length,
		batch_requests_supported,
		logger,
	} = input;

	let request_start = logger.on_request(remote_addr, &request);

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
				logger,
				request_start,
			})
			.await
		}
		// Error scenarios:
		Method::POST => response::unsupported_content_type(),
		_ => response::method_not_allowed(),
	}
}
