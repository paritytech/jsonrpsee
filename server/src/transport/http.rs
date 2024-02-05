use http::Method;
use jsonrpsee_core::{http_helpers::read_body, server::Methods, GenericTransportError};

use crate::{
	middleware::rpc::{RpcService, RpcServiceBuilder, RpcServiceCfg, RpcServiceT},
	server::{handle_rpc_call, ServerConfig},
	BatchRequestConfig, ConnectionState, LOG_TARGET,
};

/// Checks that content type of received request is valid for JSON-RPC.
pub fn content_type_is_json(request: &hyper::Request<hyper::Body>) -> bool {
	is_json(request.headers().get(hyper::header::CONTENT_TYPE))
}

/// Returns true if the `content_type` header indicates a valid JSON message.
pub fn is_json(content_type: Option<&hyper::header::HeaderValue>) -> bool {
	content_type.and_then(|val| val.to_str().ok()).map_or(false, |content| {
		content.eq_ignore_ascii_case("application/json")
			|| content.eq_ignore_ascii_case("application/json; charset=utf-8")
			|| content.eq_ignore_ascii_case("application/json;charset=utf-8")
			|| content.eq_ignore_ascii_case("application/json-rpc")
			|| content.eq_ignore_ascii_case("application/json-rpc;charset=utf-8")
			|| content.eq_ignore_ascii_case("application/json-rpc; charset=utf-8")
	})
}

/// Make JSON-RPC HTTP call with a [`RpcServiceBuilder`]
///
/// Fails if the HTTP request was a malformed JSON-RPC request.
pub async fn call_with_service_builder<L>(
	request: hyper::Request<hyper::Body>,
	server_cfg: ServerConfig,
	conn: ConnectionState,
	methods: impl Into<Methods>,
	rpc_service: RpcServiceBuilder<L>,
) -> hyper::Response<hyper::Body>
where
	L: for<'a> tower::Layer<RpcService>,
	<L as tower::Layer<RpcService>>::Service: Send + Sync + 'static,
	for<'a> <L as tower::Layer<RpcService>>::Service: RpcServiceT<'a>,
{
	let ServerConfig { max_response_body_size, batch_requests_config, max_request_body_size, .. } = server_cfg;

	let rpc_service = rpc_service.service(RpcService::new(
		methods.into(),
		max_response_body_size as usize,
		conn.conn_id as usize,
		RpcServiceCfg::OnlyCalls,
	));

	let rp =
		call_with_service(request, batch_requests_config, max_request_body_size, rpc_service, max_response_body_size)
			.await;

	drop(conn);

	rp
}

/// Make JSON-RPC HTTP call with a service [`RpcServiceT`]
///
/// Fails if the HTTP request was a malformed JSON-RPC request.
pub async fn call_with_service<S>(
	request: hyper::Request<hyper::Body>,
	batch_config: BatchRequestConfig,
	max_request_size: u32,
	rpc_service: S,
	max_response_size: u32,
) -> hyper::Response<hyper::Body>
where
	for<'a> S: RpcServiceT<'a> + Send,
{
	// Only the `POST` method is allowed.
	match *request.method() {
		Method::POST if content_type_is_json(&request) => {
			let (parts, body) = request.into_parts();

			let (body, is_single) = match read_body(&parts.headers, body, max_request_size).await {
				Ok(r) => r,
				Err(GenericTransportError::TooLarge) => return response::too_large(max_request_size),
				Err(GenericTransportError::Malformed) => return response::malformed(),
				Err(GenericTransportError::Inner(e)) => {
					tracing::warn!(target: LOG_TARGET, "Internal error reading request body: {}", e);
					return response::internal_error();
				}
			};

			let rp = handle_rpc_call(&body, is_single, batch_config, max_response_size, &rpc_service).await;

			// If the response is empty it means that it was a notification or empty batch.
			// For HTTP these are just ACK:ed with a empty body.
			response::ok_response(rp.map_or(String::new(), |r| r.into_result()))
		}
		// Error scenarios:
		Method::POST => response::unsupported_content_type(),
		_ => response::method_not_allowed(),
	}
}

/// HTTP response helpers.
pub mod response {
	use jsonrpsee_types::error::{reject_too_big_request, ErrorCode};
	use jsonrpsee_types::{ErrorObjectOwned, Id, Response, ResponsePayload};

	const JSON: &str = "application/json; charset=utf-8";
	const TEXT: &str = "text/plain";

	/// Create a response for json internal error.
	pub fn internal_error() -> hyper::Response<hyper::Body> {
		let err = ResponsePayload::unit_error(ErrorObjectOwned::from(ErrorCode::InternalError));
		let rp = Response::new(err, Id::Null);
		let error = serde_json::to_string(&rp).expect("built from known-good data; qed");

		from_template(hyper::StatusCode::INTERNAL_SERVER_ERROR, error, JSON)
	}

	/// Create a text/plain response for not allowed hosts.
	pub fn host_not_allowed() -> hyper::Response<hyper::Body> {
		from_template(hyper::StatusCode::FORBIDDEN, "Provided Host header is not whitelisted.\n".to_owned(), TEXT)
	}

	/// Create a text/plain response for disallowed method used.
	pub fn method_not_allowed() -> hyper::Response<hyper::Body> {
		from_template(
			hyper::StatusCode::METHOD_NOT_ALLOWED,
			"Used HTTP Method is not allowed. POST or OPTIONS is required\n".to_owned(),
			TEXT,
		)
	}

	/// Create a json response for oversized requests (413)
	pub fn too_large(limit: u32) -> hyper::Response<hyper::Body> {
		let err = ResponsePayload::unit_error(reject_too_big_request(limit));
		let rp = Response::new(err, Id::Null);
		let error = serde_json::to_string(&rp).expect("JSON serialization infallible; qed");

		from_template(hyper::StatusCode::PAYLOAD_TOO_LARGE, error, JSON)
	}

	/// Create a json response for empty or malformed requests (400)
	pub fn malformed() -> hyper::Response<hyper::Body> {
		let rp = Response::new(ResponsePayload::unit_error(ErrorCode::ParseError), Id::Null);
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
	pub fn ok_response(body: String) -> hyper::Response<hyper::Body> {
		from_template(hyper::StatusCode::OK, body, JSON)
	}

	/// Create a response for unsupported content type.
	pub fn unsupported_content_type() -> hyper::Response<hyper::Body> {
		from_template(
			hyper::StatusCode::UNSUPPORTED_MEDIA_TYPE,
			"Supplied content type is not allowed. Content-Type: application/json is required\n".to_owned(),
			TEXT,
		)
	}

	/// Create a response for when the server is busy and can't accept more requests.
	pub fn too_many_requests() -> hyper::Response<hyper::Body> {
		from_template(
			hyper::StatusCode::TOO_MANY_REQUESTS,
			"Too many connections. Please try again later.".to_owned(),
			TEXT,
		)
	}

	/// Create a response for when the server denied the request.
	pub fn denied() -> hyper::Response<hyper::Body> {
		from_template(hyper::StatusCode::FORBIDDEN, "".to_owned(), TEXT)
	}
}
