//! Utility methods relying on hyper
use hyper;

/// Extracts string value of a single header in request.
pub fn read_header<'a>(req: &'a hyper::Request<hyper::Body>, header_name: &str) -> Option<&'a str> {
	req.headers().get(header_name).and_then(|v| v.to_str().ok())
}
