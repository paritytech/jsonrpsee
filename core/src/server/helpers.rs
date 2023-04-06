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
use std::time::Duration;

use crate::tracing::tx_log_from_str;
use crate::Error;
use jsonrpsee_types::error::{ErrorCode, ErrorObject, OVERSIZED_RESPONSE_CODE, OVERSIZED_RESPONSE_MSG};
use jsonrpsee_types::{Id, InvalidRequest, Response, ResponsePayload};
use serde::Serialize;
use serde_json::value::to_raw_value;
use tokio::sync::mpsc::{self, Permit};

use super::{DisconnectError, SendTimeoutError, SubscriptionMessage, TrySendError};

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
#[derive(Debug, Clone)]
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
	/// Channel sender.
	tx: mpsc::Sender<String>,
	/// Max response size in bytes for a executed call.
	max_response_size: u32,
	/// Max log length.
	max_log_length: u32,
}

impl MethodSink {
	/// Create a new `MethodSink` with unlimited response size.
	pub fn new(tx: mpsc::Sender<String>) -> Self {
		MethodSink { tx, max_response_size: u32::MAX, max_log_length: u32::MAX }
	}

	/// Create a new `MethodSink` with a limited response size.
	pub fn new_with_limit(tx: mpsc::Sender<String>, max_response_size: u32, max_log_length: u32) -> Self {
		MethodSink { tx, max_response_size, max_log_length }
	}

	/// Returns whether this channel is closed without needing a context.
	pub fn is_closed(&self) -> bool {
		self.tx.is_closed()
	}

	/// Same as [`tokio::sync::mpsc::Sender::closed`].
	///
	/// # Cancel safety
	/// This method is cancel safe. Once the channel is closed,
	/// it stays closed forever and all future calls to closed will return immediately.
	pub async fn closed(&self) {
		self.tx.closed().await
	}

	/// Get the max response size.
	pub const fn max_response_size(&self) -> u32 {
		self.max_response_size
	}

	/// Attempts to send out the message immediately and fails if the underlying
	/// connection has been closed or if the message buffer is full.
	///
	/// Returns the message if the send fails such that either can be thrown away or re-sent later.
	pub fn try_send(&mut self, msg: String) -> Result<(), TrySendError> {
		tx_log_from_str(&msg, self.max_log_length);
		self.tx.try_send(msg).map_err(Into::into)
	}

	/// Async send which will wait until there is space in channel buffer or that the subscription is disconnected.
	pub async fn send(&self, msg: String) -> Result<(), DisconnectError> {
		tx_log_from_str(&msg, self.max_log_length);
		self.tx.send(msg).await.map_err(Into::into)
	}

	/// Similar to to `MethodSink::send` but only waits for a limited time.
	pub async fn send_timeout(&self, msg: String, timeout: Duration) -> Result<(), SendTimeoutError> {
		tx_log_from_str(&msg, self.max_log_length);
		self.tx.send_timeout(msg, timeout).await.map_err(Into::into)
	}

	/// Waits for channel capacity. Once capacity to send one message is available, it is reserved for the caller.
	pub async fn reserve(&self) -> Result<MethodSinkPermit, DisconnectError> {
		match self.tx.reserve().await {
			Ok(permit) => Ok(MethodSinkPermit { tx: permit, max_log_length: self.max_log_length }),
			Err(_) => Err(DisconnectError(SubscriptionMessage::empty())),
		}
	}
}

/// A method sink with reserved spot in the bounded queue.
#[derive(Debug)]
pub struct MethodSinkPermit<'a> {
	tx: Permit<'a, String>,
	max_log_length: u32,
}

impl<'a> MethodSinkPermit<'a> {
	/// Send a JSON-RPC error to the client
	pub fn send_error(self, id: Id, err: ErrorObject) {
		let json = serde_json::to_string(&Response::new(ResponsePayload::<()>::Error(err.into_owned()), id))
			.expect("valid JSON; qed");

		self.send_raw(json)
	}

	/// Helper for sending the general purpose `Error` as a JSON-RPC errors to the client.
	pub fn send_call_error(self, id: Id, err: Error) {
		self.send_error(id, err.into())
	}

	/// Send a raw JSON-RPC message to the client, `MethodSink` does not check the validity
	/// of the JSON being sent.
	pub fn send_raw(self, json: String) {
		self.tx.send(json.clone());
		tx_log_from_str(&json, self.max_log_length);
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

/// Represent the response to method call.
#[derive(Debug, Clone)]
pub struct MethodResponse {
	/// Serialized JSON-RPC response,
	pub result: String,
	/// Indicates whether the call was successful or not.
	pub success: bool,
}

impl MethodResponse {
	/// Send a JSON-RPC response to the client. If the serialization of `result` exceeds `max_response_size`,
	/// an error will be sent instead.
	pub fn response<T>(id: Id, result: ResponsePayload<T>, max_response_size: usize) -> Self
	where
		T: Serialize + Clone,
	{
		let mut writer = BoundedWriter::new(max_response_size);

		match serde_json::to_writer(&mut writer, &Response::new(result, id.clone())) {
			Ok(_) => {
				// Safety - serde_json does not emit invalid UTF-8.
				let result = unsafe { String::from_utf8_unchecked(writer.into_bytes()) };
				Self { result, success: true }
			}
			Err(err) => {
				tracing::error!("Error serializing response: {:?}", err);

				if err.is_io() {
					let data = to_raw_value(&format!("Exceeded max limit of {max_response_size}")).ok();
					let err = ResponsePayload::error_borrowed(ErrorObject::borrowed(
						OVERSIZED_RESPONSE_CODE,
						&OVERSIZED_RESPONSE_MSG,
						data.as_deref(),
					));
					let result =
						serde_json::to_string(&Response::new(err, id)).expect("JSON serialization infallible; qed");

					Self { result, success: false }
				} else {
					let result = serde_json::to_string(&Response::new(ErrorCode::InternalError.into(), id))
						.expect("JSON serialization infallible; qed");
					Self { result, success: false }
				}
			}
		}
	}

	/// Create a `MethodResponse` from an error.
	pub fn error<'a>(id: Id, err: impl Into<ErrorObject<'a>>) -> Self {
		let err = ResponsePayload::error_borrowed(err);
		let result = serde_json::to_string(&Response::new(err, id)).expect("JSON serialization infallible; qed");
		Self { result, success: false }
	}
}

/// Builder to build a `BatchResponse`.
#[derive(Debug, Clone, Default)]
pub struct BatchResponseBuilder {
	/// Serialized JSON-RPC response,
	result: String,
	/// Max limit for the batch
	max_response_size: usize,
}

impl BatchResponseBuilder {
	/// Create a new batch response builder with limit.
	pub fn new_with_limit(limit: usize) -> Self {
		let mut initial = String::with_capacity(2048);
		initial.push('[');

		Self { result: initial, max_response_size: limit }
	}

	/// Append a result from an individual method to the batch response.
	///
	/// Fails if the max limit is exceeded and returns to error response to
	/// return early in order to not process method call responses which are thrown away anyway.
	pub fn append(&mut self, response: &MethodResponse) -> Result<(), String> {
		// `,` will occupy one extra byte for each entry
		// on the last item the `,` is replaced by `]`.
		let len = response.result.len() + self.result.len() + 1;

		if len > self.max_response_size {
			Err(batch_response_error(Id::Null, ErrorObject::from(ErrorCode::InvalidRequest)))
		} else {
			self.result.push_str(&response.result);
			self.result.push(',');
			Ok(())
		}
	}

	/// Check if the batch is empty.
	pub fn is_empty(&self) -> bool {
		self.result.len() <= 1
	}

	/// Finish the batch response
	pub fn finish(mut self) -> String {
		if self.result.len() == 1 {
			batch_response_error(Id::Null, ErrorObject::from(ErrorCode::InvalidRequest))
		} else {
			self.result.pop();
			self.result.push(']');
			self.result
		}
	}
}

/// Create a JSON-RPC error response.
pub fn batch_response_error(id: Id, err: impl Into<ErrorObject<'static>>) -> String {
	let err = ResponsePayload::error_borrowed(err);
	serde_json::to_string(&Response::new(err, id)).expect("JSON serialization infallible; qed")
}

#[cfg(test)]
mod tests {
	use super::{BatchResponseBuilder, BoundedWriter, Id, MethodResponse, Response};
	use jsonrpsee_types::ResponsePayload;

	#[test]
	fn bounded_serializer_work() {
		let mut writer = BoundedWriter::new(100);
		let result = ResponsePayload::result(&"success");
		let rp = &Response::new(result, Id::Number(1));

		assert!(serde_json::to_writer(&mut writer, rp).is_ok());
		assert_eq!(String::from_utf8(writer.into_bytes()).unwrap(), r#"{"jsonrpc":"2.0","result":"success","id":1}"#);
	}

	#[test]
	fn bounded_serializer_cap_works() {
		let mut writer = BoundedWriter::new(100);
		// NOTE: `"` is part of the serialization so 101 characters.
		assert!(serde_json::to_writer(&mut writer, &"x".repeat(99)).is_err());
	}

	#[test]
	fn batch_with_single_works() {
		let method = MethodResponse::response(Id::Number(1), ResponsePayload::result_borrowed(&"a"), usize::MAX);
		assert_eq!(method.result.len(), 37);

		// Recall a batch appends two bytes for the `[]`.
		let mut builder = BatchResponseBuilder::new_with_limit(39);
		builder.append(&method).unwrap();
		let batch = builder.finish();

		assert_eq!(batch, r#"[{"jsonrpc":"2.0","result":"a","id":1}]"#)
	}

	#[test]
	fn batch_with_multiple_works() {
		let m1 = MethodResponse::response(Id::Number(1), ResponsePayload::result_borrowed(&"a"), usize::MAX);
		assert_eq!(m1.result.len(), 37);

		// Recall a batch appends two bytes for the `[]` and one byte for `,` to append a method call.
		// so it should be 2 + (37 * n) + (n-1)
		let limit = 2 + (37 * 2) + 1;
		let mut builder = BatchResponseBuilder::new_with_limit(limit);
		builder.append(&m1).unwrap();
		builder.append(&m1).unwrap();
		let batch = builder.finish();

		assert_eq!(batch, r#"[{"jsonrpc":"2.0","result":"a","id":1},{"jsonrpc":"2.0","result":"a","id":1}]"#)
	}

	#[test]
	fn batch_empty_err() {
		let batch = BatchResponseBuilder::new_with_limit(1024).finish();

		let exp_err = r#"{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid request"},"id":null}"#;
		assert_eq!(batch, exp_err);
	}

	#[test]
	fn batch_too_big() {
		let method = MethodResponse::response(Id::Number(1), ResponsePayload::result_borrowed(&"a".repeat(28)), 128);
		assert_eq!(method.result.len(), 64);

		let batch = BatchResponseBuilder::new_with_limit(63).append(&method).unwrap_err();

		let exp_err = r#"{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid request"},"id":null}"#;
		assert_eq!(batch, exp_err);
	}
}
