use hyper::{self, header};

use crate::server_utils::{cors, hosts};

/// Extracts string value of a single header in request.
fn read_header<'a>(req: &'a hyper::Request<hyper::Body>, header_name: &str) -> Option<&'a str> {
	req.headers().get(header_name).and_then(|v| v.to_str().ok())
}

/// Returns `true` if Host header in request matches a list of allowed hosts.
pub fn is_host_allowed(request: &hyper::Request<hyper::Body>, allowed_hosts: &Option<Vec<hosts::Host>>) -> bool {
	hosts::is_host_valid(read_header(request, "host"), allowed_hosts)
}

/// Returns a CORS AllowOrigin header that should be returned with that request.
pub fn cors_allow_origin(
	request: &hyper::Request<hyper::Body>,
	cors_domains: &Option<Vec<cors::AccessControlAllowOrigin>>,
) -> cors::AllowCors<header::HeaderValue> {
	cors::get_cors_allow_origin(
		read_header(request, "origin"),
		read_header(request, "host"),
		cors_domains,
	)
	.map(|origin| {
		use self::cors::AccessControlAllowOrigin::*;
		match origin {
			Value(ref val) => {
				header::HeaderValue::from_str(val).unwrap_or_else(|_| header::HeaderValue::from_static("null"))
			}
			Null => header::HeaderValue::from_static("null"),
			Any => header::HeaderValue::from_static("*"),
		}
	})
}

/// Returns the CORS AllowHeaders header that should be returned with that request.
pub fn cors_allow_headers(
	request: &hyper::Request<hyper::Body>,
	cors_allow_headers: &cors::AccessControlAllowHeaders,
) -> cors::AllowCors<Vec<header::HeaderValue>> {
	let headers = request.headers().keys().map(|name| name.as_str());
	let requested_headers = request
		.headers()
		.get_all("access-control-request-headers")
		.iter()
		.filter_map(|val| val.to_str().ok())
		.flat_map(|val| val.split(", "))
		.flat_map(|val| val.split(','));

	cors::get_cors_allow_headers(headers, requested_headers, cors_allow_headers, |name| {
		header::HeaderValue::from_str(name).unwrap_or_else(|_| header::HeaderValue::from_static("unknown"))
	})
}

/// Returns an optional value of `Connection` header that should be included in the response.
/// The second parameter defines if server is configured with keep-alive option.
/// Return value of `true` indicates that no `Connection` header should be returned,
/// `false` indicates `Connection: close`.
pub fn keep_alive(request: &hyper::Request<hyper::Body>, keep_alive: bool) -> bool {
	read_header(request, "connection")
		.map(|val| match (keep_alive, val) {
			// indicate that connection should be closed
			(false, _) | (_, "close") => false,
			// don't include any headers otherwise
			_ => true,
		})
		// if the client header is not present, close connection if we don't keep_alive
		.unwrap_or(keep_alive)
}
