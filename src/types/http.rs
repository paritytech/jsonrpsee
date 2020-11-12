//! Shared HTTP types

use futures::StreamExt;
use std::io::{Error, ErrorKind};

/// Default maximum request body size (10 MB).
const DEFAULT_MAX_BODY_SIZE_TEN_MB: u32 = 10 * 1024 * 1024;

/// HTTP configuration.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HttpConfig {
	/// Maximum request body size in bytes.
	pub max_request_body_size: u32,
}

impl Default for HttpConfig {
	fn default() -> Self {
		Self { max_request_body_size: DEFAULT_MAX_BODY_SIZE_TEN_MB }
	}
}

/// Read response body from a received request with configured `HTTP` settings.
// TODO: move somewhere else!!!
pub async fn read_http_body(
	header: &hyper::HeaderMap,
	mut body: hyper::Body,
	config: HttpConfig,
) -> Result<Vec<u8>, Error> {
	let body_size = read_http_content_length(&header).unwrap_or(0);

	if body_size > config.max_request_body_size {
		return Err(Error::new(
			ErrorKind::Other,
			format!("HTTP request body too large, got: {} max: {}", body_size, config.max_request_body_size),
		));
	}

	let mut received_data = Vec::with_capacity(body_size as usize);

	while let Some(chunk) = body.next().await {
		let chunk = chunk.map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
		let body_length = chunk.len() + received_data.len();
		if body_length > config.max_request_body_size as usize {
			return Err(Error::new(
				ErrorKind::Other,
				format!("HTTP request body too large, got: {} max: {}", body_length, config.max_request_body_size),
			));
		}
		received_data.extend_from_slice(&chunk);
	}
	Ok(received_data)
}

// Read `content_length` from HTTP Header.
//
// Returns `Some(val)` if `content_length` contains exactly one value.
// None otherwise.
fn read_http_content_length(headers: &hyper::header::HeaderMap) -> Option<u32> {
	let values = headers.get_all("content-length");
	let mut iter = values.iter();
	let content_length = iter.next()?;
	if iter.next().is_some() {
		return None;
	}

	// HTTP Content-Length indicates number of bytes in decimal.
	let length = content_length.to_str().ok()?;
	u32::from_str_radix(length, 10).ok()
}

#[cfg(test)]
mod tests {
	use super::{read_http_body, read_http_content_length, HttpConfig};
	use crate::types::jsonrpc;

	#[tokio::test]
	async fn body_to_request_works() {
		let s = r#"[{"a":"hello"}]"#;
		let expected: jsonrpc::Request = serde_json::from_str(s).unwrap();
		let body = hyper::Body::from(s.to_owned());
		let headers = hyper::header::HeaderMap::new();
		let bytes = read_http_body(&headers, body, HttpConfig::default()).await.unwrap();
		let req: jsonrpc::Request = serde_json::from_slice(&bytes).unwrap();
		assert_eq!(req, expected);
	}

	#[tokio::test]
	async fn body_to_bytes_size_limit_works() {
		let headers = hyper::header::HeaderMap::new();
		let body = hyper::Body::from(vec![0; 128]);
		assert!(read_http_body(&headers, body, HttpConfig { max_request_body_size: 127 }).await.is_err());
	}

	#[test]
	fn read_content_length_works() {
		let mut headers = hyper::header::HeaderMap::new();
		headers.insert(hyper::header::CONTENT_LENGTH, "177".parse().unwrap());
		assert_eq!(read_http_content_length(&headers), Some(177));

		headers.append(hyper::header::CONTENT_LENGTH, "999".parse().unwrap());
		assert_eq!(read_http_content_length(&headers), None);
	}
}
