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

use std::io;

use crate::{to_json_raw_value, Error};
use futures_channel::mpsc;
use futures_util::task::{Context, Poll};
use futures_util::{Sink, StreamExt};
use jsonrpsee_types::error::{
	CallError, ErrorCode, ErrorObject, ErrorResponse, CALL_EXECUTION_FAILED_CODE, OVERSIZED_RESPONSE_CODE,
	OVERSIZED_RESPONSE_MSG, UNKNOWN_ERROR_CODE,
};
use jsonrpsee_types::{Id, InvalidRequest, Response};
use serde::Serialize;
use std::pin::Pin;

/// Bounded writer that allows writing at most `max_len` bytes.
///
/// ```
///    use std::io::Write;
///
///    use jsonrpsee_core::server::helpers::BoundedWriter;
///
///    let mut writer = BoundedWriter::new(10);
///    (&mut writer).write("hello".as_bytes()).unwrap();
///    assert_eq!(std::str::from_utf8(&writer.into_bytes()).unwrap(), "hello");
/// ```
#[derive(Debug)]
pub struct BoundedWriter {
	max_len: usize,
	buf: Vec<u8>,
}

impl BoundedWriter {
	/// Create a new bounded writer.
	pub fn new(max_len: usize) -> Self {
		Self { max_len, buf: Vec::with_capacity(128) }
	}

	/// Consume the writer and extract the written bytes.
	pub fn into_bytes(self) -> Vec<u8> {
		self.buf
	}
}

impl<'a> io::Write for &'a mut BoundedWriter {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		let len = self.buf.len() + buf.len();
		if self.max_len >= len {
			self.buf.extend_from_slice(buf);
			Ok(buf.len())
		} else {
			Err(io::Error::new(io::ErrorKind::OutOfMemory, "Memory capacity exceeded"))
		}
	}

	fn flush(&mut self) -> io::Result<()> {
		Ok(())
	}
}

/// Sink that is used to send back the result to the server for a specific method.
#[derive(Clone, Debug)]
pub struct MethodSink {
	/// Channel sender
	tx: mpsc::UnboundedSender<String>,
	/// Max response size in bytes for a executed call.
	max_response_size: u32,
}

impl Sink<String> for MethodSink {
	type Error = Error;

	fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		Pin::new(&mut self.tx).poll_ready(cx).map_err(Into::into)
	}

	fn start_send(mut self: Pin<&mut Self>, item: String) -> Result<(), Self::Error> {
		Pin::new(&mut self.tx).start_send(item).map_err(Into::into)
	}

	fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		Pin::new(&mut self.tx).poll_flush(cx).map_err(Into::into)
	}

	fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		Pin::new(&mut self.tx).poll_close(cx).map_err(Into::into)
	}
}

impl MethodSink {
	/// Create a new `MethodSink` with unlimited response size
	pub fn new(tx: mpsc::UnboundedSender<String>) -> Self {
		MethodSink { tx, max_response_size: u32::MAX }
	}

	/// Create a new `MethodSink` with a limited response size
	pub fn new_with_limit(tx: mpsc::UnboundedSender<String>, max_response_size: u32) -> Self {
		MethodSink { tx, max_response_size }
	}

	/// Returns whether this channel is closed without needing a context.
	pub fn is_closed(&self) -> bool {
		self.tx.is_closed()
	}

	/// Send a JSON-RPC response to the client. If the serialization of `result` exceeds `max_response_size`,
	/// an error will be sent instead.
	pub fn send_response(&self, id: Id, result: impl Serialize) -> bool {
		let mut writer = BoundedWriter::new(self.max_response_size as usize);

		let json = match serde_json::to_writer(&mut writer, &Response::new(result, id.clone())) {
			Ok(_) => {
				// Safety - serde_json does not emit invalid UTF-8.
				unsafe { String::from_utf8_unchecked(writer.into_bytes()) }
			}
			Err(err) => {
				tracing::error!("Error serializing response: {:?}", err);

				if err.is_io() {
					let data = to_json_raw_value(&format!("Exceeded max limit {}", self.max_response_size)).ok();
					let err = ErrorObject {
						code: ErrorCode::ServerError(OVERSIZED_RESPONSE_CODE),
						message: OVERSIZED_RESPONSE_MSG.into(),
						data: data.as_deref(),
					};
					return self.send_error(id, err);
				} else {
					return self.send_error(id, ErrorCode::InternalError.into());
				}
			}
		};

		if let Err(err) = self.tx.unbounded_send(json) {
			tracing::error!("Error sending response to the client: {:?}", err);
			false
		} else {
			true
		}
	}

	/// Send a JSON-RPC error to the client
	pub fn send_error(&self, id: Id, error: ErrorObject) -> bool {
		let json = match serde_json::to_string(&ErrorResponse::new(error, id)) {
			Ok(json) => json,
			Err(err) => {
				tracing::error!("Error serializing error message: {:?}", err);

				return false;
			}
		};

		if let Err(err) = self.tx.unbounded_send(json) {
			tracing::error!("Could not send error response to the client: {:?}", err)
		}

		false
	}

	/// Helper for sending the general purpose `Error` as a JSON-RPC errors to the client
	pub fn send_call_error(&self, id: Id, err: Error) -> bool {
		let (code, message, data) = match err {
			Error::Call(CallError::InvalidParams(e)) => (ErrorCode::InvalidParams, e.to_string(), None),
			Error::Call(CallError::Failed(e)) => {
				(ErrorCode::ServerError(CALL_EXECUTION_FAILED_CODE), e.to_string(), None)
			}
			Error::Call(CallError::Custom { code, message, data }) => (code.into(), message, data),
			// This should normally not happen because the most common use case is to
			// return `Error::Call` in `register_async_method`.
			e => (ErrorCode::ServerError(UNKNOWN_ERROR_CODE), e.to_string(), None),
		};

		let err = ErrorObject { code, message: message.into(), data: data.as_deref() };

		self.send_error(id, err)
	}

	/// Send a raw JSON-RPC message to the client, `MethodSink` does not check verify the validity
	/// of the JSON being sent.
	pub fn send_raw(&self, raw_json: String) -> Result<(), mpsc::TrySendError<String>> {
		self.tx.unbounded_send(raw_json)
	}

	/// Close the channel for any further messages.
	pub fn close(&self) {
		self.tx.close_channel();
	}
}

/// Figure out if this is a sufficiently complete request that we can extract an [`Id`] out of, or just plain
/// unparseable garbage.
pub fn prepare_error(data: &[u8]) -> (Id<'_>, ErrorCode) {
	match serde_json::from_slice::<InvalidRequest>(data) {
		Ok(InvalidRequest { id }) => (id, ErrorCode::InvalidRequest),
		Err(_) => (Id::Null, ErrorCode::ParseError),
	}
}

/// Read all the results of all method calls in a batch request from the ['Stream']. Format the result into a single
/// `String` appropriately wrapped in `[`/`]`.
pub async fn collect_batch_response(rx: mpsc::UnboundedReceiver<String>) -> String {
	let mut buf = String::with_capacity(2048);
	buf.push('[');
	let mut buf = rx
		.fold(buf, |mut acc, response| async move {
			acc.push_str(&response);
			acc.push(',');
			acc
		})
		.await;
	// Remove trailing comma
	buf.pop();
	buf.push(']');
	buf
}

#[cfg(test)]
mod tests {
	use super::{BoundedWriter, Id, Response};

	#[test]
	fn bounded_serializer_work() {
		let mut writer = BoundedWriter::new(100);
		let result = "success";

		assert!(serde_json::to_writer(&mut writer, &Response::new(result, Id::Number(1))).is_ok());
		assert_eq!(String::from_utf8(writer.into_bytes()).unwrap(), r#"{"jsonrpc":"2.0","result":"success","id":1}"#);
	}

	#[test]
	fn bounded_serializer_cap_works() {
		let mut writer = BoundedWriter::new(100);
		// NOTE: `"` is part of the serialization so 101 characters.
		assert!(serde_json::to_writer(&mut writer, &"x".repeat(99)).is_err());
	}
}
