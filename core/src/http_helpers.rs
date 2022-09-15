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

//! Utility methods relying on hyper

use crate::error::GenericTransportError;
use futures_util::stream::StreamExt;

/// Read a data from a [`hyper::Body`] and return the data if it is valid and within the allowed size range.
///
/// Returns `Ok((bytes, single))` if the body was in valid size range; and a bool indicating whether the JSON-RPC
/// request is a single or a batch.
/// Returns `Err` if the body was too large or the body couldn't be read.
pub async fn read_body(
	headers: &hyper::HeaderMap,
	mut body: hyper::Body,
	max_request_body_size: u32,
) -> Result<(Vec<u8>, bool), GenericTransportError<hyper::Error>> {
	// NOTE(niklasad1): Values bigger than `u32::MAX` will be turned into zero here. This is unlikely to occur in
	// practice and for that case we fallback to allocating in the while-loop below instead of pre-allocating.
	let body_size = read_header_content_length(headers).unwrap_or(0);

	if body_size > max_request_body_size {
		return Err(GenericTransportError::TooLarge);
	}

	let first_chunk =
		body.next().await.ok_or(GenericTransportError::Malformed)?.map_err(GenericTransportError::Inner)?;

	if first_chunk.len() > max_request_body_size as usize {
		return Err(GenericTransportError::TooLarge);
	}

	let first_non_whitespace = first_chunk.iter().find(|byte| !byte.is_ascii_whitespace());

	let single = match first_non_whitespace {
		Some(b'{') => true,
		Some(b'[') => false,
		_ => return Err(GenericTransportError::Malformed),
	};

	let mut received_data = Vec::with_capacity(body_size as usize);
	received_data.extend_from_slice(&first_chunk);

	while let Some(chunk) = body.next().await {
		let chunk = chunk.map_err(GenericTransportError::Inner)?;
		let body_length = chunk.len() + received_data.len();
		if body_length > max_request_body_size as usize {
			return Err(GenericTransportError::TooLarge);
		}
		received_data.extend_from_slice(&chunk);
	}
	Ok((received_data, single))
}

/// Read the `Content-Length` HTTP Header. Must fit into a `u32`; returns `None` otherwise.
///
/// NOTE: There's no specific hard limit on `Content_length` in HTTP specification.
/// Thus this method might reject valid `content_length`
fn read_header_content_length(headers: &hyper::header::HeaderMap) -> Option<u32> {
	let length = read_header_value(headers, hyper::header::CONTENT_LENGTH)?;
	// HTTP Content-Length indicates number of bytes in decimal.
	length.parse::<u32>().ok()
}

/// Returns a string value when there is exactly one value for the given header.
pub fn read_header_value(headers: &hyper::header::HeaderMap, header_name: hyper::header::HeaderName) -> Option<&str> {
	let mut values = headers.get_all(header_name).iter();
	let val = values.next()?;
	if values.next().is_none() {
		val.to_str().ok()
	} else {
		None
	}
}

/// Returns an iterator of all values for a given a header name
pub fn read_header_values<'a>(
	headers: &'a hyper::header::HeaderMap,
	header_name: &str,
) -> hyper::header::GetAll<'a, hyper::header::HeaderValue> {
	headers.get_all(header_name)
}

#[cfg(test)]
mod tests {
	use super::{read_body, read_header_content_length};

	#[tokio::test]
	async fn body_to_bytes_size_limit_works() {
		let headers = hyper::header::HeaderMap::new();
		let body = hyper::Body::from(vec![0; 128]);
		assert!(read_body(&headers, body, 127).await.is_err());
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
