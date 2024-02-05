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

use hyper::body::{Buf, HttpBody};

/// Represents error that can when reading with a HTTP body.
#[derive(Debug, thiserror::Error)]
pub enum HttpError {
	/// The HTTP message was too large.
	#[error("The HTTP message was too big")]
	TooLarge,
	/// Malformed request
	#[error("Malformed request")]
	Malformed,
	/// Represents error that can happen when dealing with HTTP streams.
	#[error("{0}")]
	Stream(#[from] hyper::Error),
}

/// Read a data from [`hyper::body::HttpBody`] and return the data if it is valid JSON and within the allowed size range.
///
/// Returns `Ok((bytes, single))` if the body was in valid size range; and a bool indicating whether the JSON-RPC
/// request is a single or a batch.
/// Returns `Err` if the body was too large or the body couldn't be read.
pub async fn read_body<B>(headers: &hyper::HeaderMap, body: B, max_body_size: u32) -> Result<(Vec<u8>, bool), HttpError>
where
	B: HttpBody<Error = hyper::Error> + Send + 'static,
	B::Data: Send,
{
	// NOTE(niklasad1): Values bigger than `u32::MAX` will be turned into zero here. This is unlikely to occur in
	// practice and in that case we fallback to allocating in the while-loop below instead of pre-allocating.
	let body_size = read_header_content_length(headers).unwrap_or(0);

	if body_size > max_body_size {
		return Err(HttpError::TooLarge);
	}

	futures_util::pin_mut!(body);

	// Allocate up to 16KB initially.
	let mut received_data = Vec::with_capacity(std::cmp::min(body_size as usize, 16 * 1024));
	let mut is_single = None;

	while let Some(d) = body.data().await {
		let data = d.map_err(HttpError::Stream)?;

		// If it's the first chunk, trim the whitespaces to determine whether it's valid JSON-RPC call.
		if received_data.is_empty() {
			let first_non_whitespace =
				data.chunk().iter().enumerate().take(128).find(|(_, byte)| !byte.is_ascii_whitespace());

			let skip = match first_non_whitespace {
				Some((idx, b'{')) => {
					is_single = Some(true);
					idx
				}
				Some((idx, b'[')) => {
					is_single = Some(false);
					idx
				}
				_ => return Err(HttpError::Malformed),
			};

			if data.chunk().len() - skip > max_body_size as usize {
				return Err(HttpError::TooLarge);
			}

			// ignore whitespace as these doesn't matter just makes the JSON decoding slower.
			received_data.extend_from_slice(&data.chunk()[skip..]);
		} else {
			if data.chunk().len() + received_data.len() > max_body_size as usize {
				return Err(HttpError::TooLarge);
			}

			received_data.extend_from_slice(data.chunk());
		}
	}

	match is_single {
		Some(single) if !received_data.is_empty() => {
			tracing::trace!(
				target: "jsonrpsee-http",
				"HTTP response body: {}",
				std::str::from_utf8(&received_data).unwrap_or("Invalid UTF-8 data")
			);
			Ok((received_data, single))
		}
		_ => Err(HttpError::Malformed),
	}
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
