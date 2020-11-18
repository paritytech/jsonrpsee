// Copyright 2019 Parity Technologies (UK) Ltd.
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

//! Utility methods relying on hyper

use crate::types::error::GenericTransportError;
use crate::types::http::HttpConfig;
use futures::StreamExt;

/// Read a hyper response with configured `HTTP` settings.
///
/// Returns `Ok(bytes)` if the body was in valid size range.
/// Returns `Err` if the body was too large or the body couldn't be read.
pub async fn read_response_to_body(
	headers: &hyper::HeaderMap,
	mut body: hyper::Body,
	config: HttpConfig,
) -> Result<Vec<u8>, GenericTransportError<hyper::Error>> {
	// NOTE(niklasad1): Bigger values than `u32::MAX` will be regarded as zero here and that's very unlikely to occur.
	// Thus, in those causes we rely on the while loop to allocate instead of `pre-allocate`.
	let body_size = read_header_content_length(&headers).unwrap_or(0);

	if body_size > config.max_request_body_size {
		return Err(GenericTransportError::TooLarge);
	}

	let mut received_data = Vec::with_capacity(body_size as usize);

	while let Some(chunk) = body.next().await {
		let chunk = chunk.map_err(|e| GenericTransportError::Inner(e))?;
		let body_length = chunk.len() + received_data.len();
		if body_length > config.max_request_body_size as usize {
			return Err(GenericTransportError::TooLarge);
		}
		received_data.extend_from_slice(&chunk);
	}
	Ok(received_data)
}

/// Read `content_length` from HTTP Header that fits into `u32`
///
/// NOTE: There's no specific hard limit on `content_length` in HTTP specification, thus this method might reject valid `content_length`
fn read_header_content_length(headers: &hyper::header::HeaderMap) -> Option<u32> {
	let length = read_header(headers, "content-length")?;
	// HTTP Content-Length indicates number of bytes in decimal.
	u32::from_str_radix(length, 10).ok()
}

/// Extracts string value of a single header in request.
///
/// Returns `Some(val)` if the header contains exactly one value.
/// None otherwise.
pub fn read_header_value<'a>(headers: &'a hyper::header::HeaderMap, header_name: &str) -> Option<&'a str> {
	let mut values = headers.get_all(header_name).iter();
	let val = values.next()?;
	if values.next().is_none() {
		val.to_str().ok()
	} else {
		None
	}
}

#[cfg(test)]
mod tests {
	use super::{read_header_content_length, read_response_to_body, HttpConfig};
	use crate::types::jsonrpc;

	#[tokio::test]
	async fn body_to_request_works() {
		let s = r#"[{"a":"hello"}]"#;
		let expected: jsonrpc::Request = serde_json::from_str(s).unwrap();
		let body = hyper::Body::from(s.to_owned());
		let headers = hyper::header::HeaderMap::new();
		let bytes = read_response_to_body(&headers, body, HttpConfig::default()).await.unwrap();
		let req: jsonrpc::Request = serde_json::from_slice(&bytes).unwrap();
		assert_eq!(req, expected);
	}

	#[tokio::test]
	async fn body_to_bytes_size_limit_works() {
		let headers = hyper::header::HeaderMap::new();
		let body = hyper::Body::from(vec![0; 128]);
		assert!(read_response_to_body(&headers, body, HttpConfig { max_request_body_size: 127 }).await.is_err());
	}

	#[test]
	fn read_content_length_works() {
		let mut headers = hyper::header::HeaderMap::new();
		headers.insert(hyper::header::CONTENT_LENGTH, "177".parse().unwrap());
		assert_eq!(read_header_content_length(&headers), Some(177));

		headers.append(hyper::header::CONTENT_LENGTH, "999".parse().unwrap());
		assert_eq!(read_header_content_length(&headers), None);
	}

	#[test]
	fn read_content_length_too_big_value() {
		let mut headers = hyper::header::HeaderMap::new();
		headers.insert(hyper::header::CONTENT_LENGTH, "18446744073709551616".parse().unwrap());
		assert_eq!(read_header_content_length(&headers), None);
	}
}
