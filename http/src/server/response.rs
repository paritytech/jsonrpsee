//! Contains common builders for hyper responses.

/// Create a response for plaintext internal error.
pub fn internal_error<T: Into<String>>(msg: T) -> hyper::Response<hyper::Body> {
    from_template(
        hyper::StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal Server Error: {}", msg.into()),
    )
}

/// Create a json response for service unavailable.
pub fn service_unavailable<T: Into<String>>(msg: T) -> hyper::Response<hyper::Body> {
    hyper::Response::builder()
        .status(hyper::StatusCode::SERVICE_UNAVAILABLE)
        .header(
            "Content-Type",
            hyper::header::HeaderValue::from_static("application/json; charset=utf-8"),
        )
        .body(hyper::Body::from(msg.into()))
        .expect("Unable to parse response body for type conversion")
}

/// Create a response for not allowed hosts.
pub fn host_not_allowed() -> hyper::Response<hyper::Body> {
    from_template(
        hyper::StatusCode::FORBIDDEN,
        "Provided Host header is not whitelisted.\n".to_owned(),
    )
}

/// Create a response for unsupported content type.
pub fn unsupported_content_type() -> hyper::Response<hyper::Body> {
    from_template(
        hyper::StatusCode::UNSUPPORTED_MEDIA_TYPE,
        "Supplied content type is not allowed. Content-Type: application/json is required\n"
            .to_owned(),
    )
}

/// Create a response for disallowed method used.
pub fn method_not_allowed() -> hyper::Response<hyper::Body> {
    from_template(
        hyper::StatusCode::METHOD_NOT_ALLOWED,
        "Used HTTP Method is not allowed. POST or OPTIONS is required\n".to_owned(),
    )
}

/// CORS invalid
pub fn invalid_allow_origin() -> hyper::Response<hyper::Body> {
    from_template(
        hyper::StatusCode::FORBIDDEN,
        "Origin of the request is not whitelisted. CORS headers would not be sent and any side-effects were cancelled as well.\n".to_owned(),
    )
}

/// CORS header invalid
pub fn invalid_allow_headers() -> hyper::Response<hyper::Body> {
    from_template(
        hyper::StatusCode::FORBIDDEN,
        "Requested headers are not allowed for CORS. CORS headers would not be sent and any side-effects were cancelled as well.\n".to_owned(),
    )
}

/// Create a response for bad request
pub fn bad_request<S: Into<String>>(msg: S) -> hyper::Response<hyper::Body> {
    from_template(hyper::StatusCode::BAD_REQUEST, msg.into())
}

/// Create a response for too large (413)
pub fn too_large<S: Into<String>>(msg: S) -> hyper::Response<hyper::Body> {
    from_template(hyper::StatusCode::PAYLOAD_TOO_LARGE, msg.into())
}

/// Create a response for a template.
fn from_template(status: hyper::StatusCode, body: String) -> hyper::Response<hyper::Body> {
    hyper::Response::builder()
        .status(status)
        .header(
            "content-type",
            hyper::header::HeaderValue::from_static("text/plain; charset=utf-8"),
        )
        .body(hyper::Body::from(body))
        // Parsing `StatusCode` and `HeaderValue` is infalliable but
        // parsing body content is not.
        .expect("Unable to parse response body for type conversion")
}
