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

//! Contains common builders for hyper responses.

use jsonrpsee_types::error::reject_too_big_request;

use crate::types::error::{ErrorCode, ErrorResponse};
use crate::types::Id;

const JSON: &str = "application/json; charset=utf-8";
const TEXT: &str = "text/plain";

/// Create a response for json internal error.
pub fn internal_error() -> hyper::Response<hyper::Body> {
	let error = serde_json::to_string(&ErrorResponse::borrowed(ErrorCode::InternalError.into(), Id::Null))
		.expect("built from known-good data; qed");

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

/// Create a text/plain response for invalid "Origin" headers.
pub fn invalid_allow_origin() -> hyper::Response<hyper::Body> {
	from_template(hyper::StatusCode::FORBIDDEN, "Origin of the request is not whitelisted.\n".to_owned(), TEXT)
}

/// Create a json response for oversized requests (413)
pub fn too_large(limit: u32) -> hyper::Response<hyper::Body> {
	let error = serde_json::to_string(&ErrorResponse::borrowed(reject_too_big_request(limit), Id::Null))
		.expect("built from known-good data; qed");

	from_template(hyper::StatusCode::PAYLOAD_TOO_LARGE, error, JSON)
}

/// Create a json response for empty or malformed requests (400)
pub fn malformed() -> hyper::Response<hyper::Body> {
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
