use std::convert::Infallible;

use crate::server::BatchRequestConfig;

use jsonrpsee_core::error::GenericTransportError;
use jsonrpsee_core::http_helpers::read_body;
use jsonrpsee_core::server::helpers::{batch_response_error, prepare_error, BatchResponseBuilder, MethodResponse};
use jsonrpsee_core::JsonRawValue;
use jsonrpsee_types::error::{
	reject_too_big_batch_request, ErrorCode, BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG,
};
use jsonrpsee_types::{ErrorObject, Id, InvalidRequest, Notification, Request};
use tower::Service;

type Notif<'a> = Notification<'a, Option<&'a JsonRawValue>>;

/// Process a verified request, it implies a POST request with content type JSON.
pub(crate) async fn process_validated_request<S>(
	request: hyper::Request<hyper::Body>,
	batch_config: BatchRequestConfig,
	max_request_size: u32,
	max_response_size: u32,
	mut rpc_service: S,
) -> hyper::Response<hyper::Body>
where
	for<'a> S: Service<Request<'a>, Response = MethodResponse, Error = jsonrpsee_core::Error>,
{
	let (parts, body) = request.into_parts();

	let (body, is_single) = match read_body(&parts.headers, body, max_request_size).await {
		Ok(r) => r,
		Err(GenericTransportError::TooLarge) => return response::too_large(max_request_size),
		Err(GenericTransportError::Malformed) => return response::malformed(),
		Err(GenericTransportError::Inner(e)) => {
			tracing::warn!("Internal error reading request body: {}", e);
			return response::internal_error();
		}
	};

	// Single request or notification
	if is_single {
		if let Ok(req) = serde_json::from_slice(&body) {
			let rp = rpc_service.call(req).await.unwrap();
			response::ok_response(rp.result)
		} else if let Ok(_notif) = serde_json::from_slice::<Notif>(&body) {
			response::ok_response(String::new())
		} else {
			let (id, code) = prepare_error(&body);
			let rp = MethodResponse::error(id, ErrorObject::from(code));
			response::ok_response(rp.result)
		}
	}
	// Batch of requests.
	else {
		let max_len = match batch_config {
			BatchRequestConfig::Disabled => {
				let response = MethodResponse::error(
					Id::Null,
					ErrorObject::borrowed(BATCHES_NOT_SUPPORTED_CODE, BATCHES_NOT_SUPPORTED_MSG, None),
				);
				return response::ok_response(response.result);
			}
			BatchRequestConfig::Limit(limit) => limit as usize,
			BatchRequestConfig::Unlimited => usize::MAX,
		};

		if let Ok(batch) = serde_json::from_slice::<Vec<&JsonRawValue>>(&body) {
			if batch.len() > max_len {
				return response::ok_response(batch_response_error(Id::Null, reject_too_big_batch_request(max_len)));
			}

			let mut got_notif = false;
			let mut batch_response = BatchResponseBuilder::new_with_limit(max_response_size as usize);

			for call in batch {
				if let Ok(req) = serde_json::from_str::<Request>(call.get()) {
					let rp = rpc_service.call(req).await.unwrap();

					if let Err(too_large) = batch_response.append(&rp) {
						return response::ok_response(too_large);
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

					let rp = MethodResponse::error(id, ErrorObject::from(ErrorCode::InvalidRequest));

					if let Err(too_large) = batch_response.append(&rp) {
						return response::ok_response(too_large);
					}
				}
			}

			if got_notif && batch_response.is_empty() {
				response::ok_response(String::new())
			} else {
				response::ok_response(batch_response.finish())
			}
		} else {
			response::ok_response(batch_response_error(Id::Null, ErrorObject::from(ErrorCode::ParseError)))
		}
	}
}

/// Checks that content type of received request is valid for JSON-RPC.
pub(crate) fn content_type_is_json(request: &hyper::Request<hyper::Body>) -> bool {
	is_json(request.headers().get(hyper::header::CONTENT_TYPE))
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
		tracing::debug!("HTTP serve connection failed {:?}", e);
	}
}

pub(crate) mod response {
	use jsonrpsee_types::error::{reject_too_big_request, ErrorCode};
	use jsonrpsee_types::{ErrorObjectOwned, Id, Response, ResponsePayload};

	const JSON: &str = "application/json; charset=utf-8";
	const TEXT: &str = "text/plain";

	/// Create a response for json internal error.
	pub(crate) fn internal_error() -> hyper::Response<hyper::Body> {
		let err = ResponsePayload::error(ErrorObjectOwned::from(ErrorCode::InternalError));
		let rp = Response::new(err, Id::Null);
		let error = serde_json::to_string(&rp).expect("built from known-good data; qed");

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
		let err = ResponsePayload::error(reject_too_big_request(limit));
		let rp = Response::new(err, Id::Null);
		let error = serde_json::to_string(&rp).expect("JSON serialization infallible; qed");

		from_template(hyper::StatusCode::PAYLOAD_TOO_LARGE, error, JSON)
	}

	/// Create a json response for empty or malformed requests (400)
	pub(crate) fn malformed() -> hyper::Response<hyper::Body> {
		let rp = Response::new(ErrorCode::ParseError.into(), Id::Null);
		let error = serde_json::to_string(&rp).expect("JSON serialization infallible; qed");

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
