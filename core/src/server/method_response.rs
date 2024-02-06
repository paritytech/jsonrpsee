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

use crate::server::{BoundedWriter, LOG_TARGET};
use std::task::Poll;

use futures_util::{Future, FutureExt};
use jsonrpsee_types::error::{
	reject_too_big_batch_response, ErrorCode, ErrorObject, OVERSIZED_RESPONSE_CODE, OVERSIZED_RESPONSE_MSG,
};
use jsonrpsee_types::{ErrorObjectOwned, Id, Response, ResponsePayload as InnerResponsePayload};
use serde::Serialize;
use serde_json::value::to_raw_value;

#[derive(Debug, Clone)]
enum ResponseKind {
	MethodCall,
	Subscription,
	Batch,
}

/// Represents a response to a method call.
///
/// NOTE: A subscription is also a method call but it's
/// possible determine whether a method response
/// is "subscription" or "ordinary method call"
/// by calling [`MethodResponse::is_subscription`]
#[derive(Debug)]
pub struct MethodResponse {
	/// Serialized JSON-RPC response,
	result: String,
	/// Indicates whether the call was successful or not.
	success_or_error: MethodResponseResult,
	/// Indicates whether the call was a subscription response.
	kind: ResponseKind,
	/// Optional callback that may be utilized to notif
	/// that the method response has been processed
	on_close: Option<MethodResponseNotifyTx>,
}

impl MethodResponse {
	/// Returns whether the call was successful.
	pub fn is_success(&self) -> bool {
		self.success_or_error.is_success()
	}

	/// Returns whether the call failed.
	pub fn is_error(&self) -> bool {
		self.success_or_error.is_error()
	}

	/// Returns whether the response is a subscription response.
	pub fn is_subscription(&self) -> bool {
		matches!(self.kind, ResponseKind::Subscription)
	}

	/// Returns whether the response is a method response.
	pub fn is_method_call(&self) -> bool {
		matches!(self.kind, ResponseKind::MethodCall)
	}

	/// Returns whether the response is a batch response.
	pub fn is_batch(&self) -> bool {
		matches!(self.kind, ResponseKind::Batch)
	}

	/// Consume the method response and extract the serialized response.
	pub fn into_result(self) -> String {
		self.result
	}

	/// Extract the serialized response as a String.
	pub fn to_result(&self) -> String {
		self.result.clone()
	}

	/// Consume the method response and extract the parts.
	pub fn into_parts(self) -> (String, Option<MethodResponseNotifyTx>) {
		(self.result, self.on_close)
	}

	/// Get the error code
	///
	/// Returns `Some(error code)` if the call failed.
	pub fn as_error_code(&self) -> Option<i32> {
		self.success_or_error.as_error_code()
	}

	/// Get a reference to the serialized response.
	pub fn as_result(&self) -> &str {
		&self.result
	}

	/// Create a method response from [`BatchResponse`].
	pub fn from_batch(batch: BatchResponse) -> Self {
		Self {
			result: batch.0,
			success_or_error: MethodResponseResult::Success,
			kind: ResponseKind::Batch,
			on_close: None,
		}
	}

	/// This is similar to [`MethodResponse::response`] but sets a flag to indicate
	/// that response is a subscription.
	pub fn subscription_response<T>(id: Id, result: ResponsePayload<T>, max_response_size: usize) -> Self
	where
		T: Serialize + Clone,
	{
		let mut rp = Self::response(id, result, max_response_size);
		rp.kind = ResponseKind::Subscription;
		rp
	}

	/// Create a new method response.
	///
	/// If the serialization of `result` exceeds `max_response_size` then
	/// the response is changed to an JSON-RPC error object.
	pub fn response<T>(id: Id, rp: ResponsePayload<T>, max_response_size: usize) -> Self
	where
		T: Serialize + Clone,
	{
		let mut writer = BoundedWriter::new(max_response_size);

		let success_or_error = if let InnerResponsePayload::Error(ref e) = rp.inner {
			MethodResponseResult::Failed(e.code())
		} else {
			MethodResponseResult::Success
		};

		let kind = ResponseKind::MethodCall;

		match serde_json::to_writer(&mut writer, &Response::new(rp.inner, id.clone())) {
			Ok(_) => {
				// Safety - serde_json does not emit invalid UTF-8.
				let result = unsafe { String::from_utf8_unchecked(writer.into_bytes()) };

				Self { result, success_or_error, kind, on_close: rp.on_exit }
			}
			Err(err) => {
				tracing::error!(target: LOG_TARGET, "Error serializing response: {:?}", err);

				if err.is_io() {
					let data = to_raw_value(&format!("Exceeded max limit of {max_response_size}")).ok();
					let err_code = OVERSIZED_RESPONSE_CODE;

					let err = InnerResponsePayload::<()>::error_borrowed(ErrorObject::borrowed(
						err_code,
						OVERSIZED_RESPONSE_MSG,
						data.as_deref(),
					));
					let result =
						serde_json::to_string(&Response::new(err, id)).expect("JSON serialization infallible; qed");

					Self {
						result,
						success_or_error: MethodResponseResult::Failed(err_code),
						kind,
						on_close: rp.on_exit,
					}
				} else {
					let err = ErrorCode::InternalError;
					let payload = jsonrpsee_types::ResponsePayload::<()>::error(err);
					let result =
						serde_json::to_string(&Response::new(payload, id)).expect("JSON serialization infallible; qed");
					Self {
						result,
						success_or_error: MethodResponseResult::Failed(err.code()),
						kind,
						on_close: rp.on_exit,
					}
				}
			}
		}
	}

	/// This is similar to [`MethodResponse::error`] but sets a flag to indicate
	/// that error is a subscription.
	pub fn subscription_error<'a>(id: Id, err: impl Into<ErrorObject<'a>>) -> Self {
		let mut rp = Self::error(id, err);
		rp.kind = ResponseKind::Subscription;
		rp
	}

	/// Create a [`MethodResponse`] from a JSON-RPC error.
	pub fn error<'a>(id: Id, err: impl Into<ErrorObject<'a>>) -> Self {
		let err: ErrorObject = err.into();
		let err_code = err.code();
		let err = InnerResponsePayload::<()>::error_borrowed(err);
		let result = serde_json::to_string(&Response::new(err, id)).expect("JSON serialization infallible; qed");
		Self {
			result,
			success_or_error: MethodResponseResult::Failed(err_code),
			kind: ResponseKind::MethodCall,
			on_close: None,
		}
	}
}

/// Represent the outcome of a method call success or failed.
#[derive(Debug, Copy, Clone)]
enum MethodResponseResult {
	/// The method call was successful.
	Success,
	/// The method call failed with error code.
	Failed(i32),
}

impl MethodResponseResult {
	/// Returns whether the call was successful.
	fn is_success(&self) -> bool {
		matches!(self, MethodResponseResult::Success)
	}

	/// Returns whether the call failed.
	fn is_error(&self) -> bool {
		matches!(self, MethodResponseResult::Failed(_))
	}

	/// Get the error code
	///
	/// Returns `Some(error code)` if the call failed.
	fn as_error_code(&self) -> Option<i32> {
		match self {
			Self::Failed(e) => Some(*e),
			_ => None,
		}
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
	pub fn append(&mut self, response: &MethodResponse) -> Result<(), MethodResponse> {
		// `,` will occupy one extra byte for each entry
		// on the last item the `,` is replaced by `]`.
		let len = response.result.len() + self.result.len() + 1;

		if len > self.max_response_size {
			Err(MethodResponse::error(Id::Null, reject_too_big_batch_response(self.max_response_size)))
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
	pub fn finish(mut self) -> BatchResponse {
		if self.result.len() == 1 {
			BatchResponse(batch_response_error(Id::Null, ErrorObject::from(ErrorCode::InvalidRequest)))
		} else {
			self.result.pop();
			self.result.push(']');
			BatchResponse(self.result)
		}
	}
}

/// Serialized batch response.
#[derive(Debug, Clone)]
pub struct BatchResponse(String);

/// Create a JSON-RPC error response.
pub fn batch_response_error(id: Id, err: impl Into<ErrorObject<'static>>) -> String {
	let err = InnerResponsePayload::<()>::error_borrowed(err);
	serde_json::to_string(&Response::new(err, id)).expect("JSON serialization infallible; qed")
}

/// Similar to [`jsonrpsee_types::ResponsePayload`] but possible to with an async-like
/// API to detect when a method response has been sent.
#[derive(Debug)]
pub struct ResponsePayload<'a, T>
where
	T: Clone,
{
	inner: InnerResponsePayload<'a, T>,
	on_exit: Option<MethodResponseNotifyTx>,
}

impl<'a, T: Clone> From<InnerResponsePayload<'a, T>> for ResponsePayload<'a, T> {
	fn from(inner: InnerResponsePayload<'a, T>) -> Self {
		Self { inner, on_exit: None }
	}
}

impl<'a, T> ResponsePayload<'a, T>
where
	T: Clone,
{
	/// Create a successful owned response payload.
	pub fn success(t: T) -> Self {
		InnerResponsePayload::success(t).into()
	}

	/// Create a successful borrowed response payload.
	pub fn success_borrowed(t: &'a T) -> Self {
		InnerResponsePayload::success_borrowed(t).into()
	}

	/// Create an error response payload.
	pub fn error(e: impl Into<ErrorObjectOwned>) -> Self {
		InnerResponsePayload::error(e.into()).into()
	}

	/// Create a borrowd error response payload.
	pub fn error_borrowed(e: impl Into<ErrorObject<'a>>) -> Self {
		InnerResponsePayload::error_borrowed(e.into()).into()
	}

	/// Consumes the [`ResponsePayload`] and produces new [`ResponsePayload`] and a future
	/// [`MethodResponseFuture`] that will be resolved once the response has been processed.
	///
	/// If this has been called more than once then this will overwrite
	/// the old result the previous future(s) will be resolved with error.
	pub fn notify_on_completion(mut self) -> (Self, MethodResponseFuture) {
		let (tx, rx) = response_channel();
		self.on_exit = Some(tx);
		(self, rx)
	}

	/// Convert the response payload into owned.
	pub fn into_owned(self) -> ResponsePayload<'static, T> {
		ResponsePayload { inner: self.inner.into_owned(), on_exit: self.on_exit }
	}
}

impl<'a, T> From<ErrorCode> for ResponsePayload<'a, T>
where
	T: Clone,
{
	fn from(code: ErrorCode) -> Self {
		let err: ErrorObject = code.into();
		Self::error(err)
	}
}

/// Create a channel to be used in combination with [`ResponsePayload`] to
/// notify when a method call has been processed.
fn response_channel() -> (MethodResponseNotifyTx, MethodResponseFuture) {
	let (tx, rx) = tokio::sync::oneshot::channel();
	(MethodResponseNotifyTx(tx), MethodResponseFuture(rx))
}

/// Sends a message once the method response has been processed.
#[derive(Debug)]
pub struct MethodResponseNotifyTx(tokio::sync::oneshot::Sender<NotifyMsg>);

impl MethodResponseNotifyTx {
	/// Send a notify message.
	pub fn notify(self, is_success: bool) {
		let msg = if is_success { NotifyMsg::Ok } else { NotifyMsg::Err };
		_ = self.0.send(msg);
	}
}

/// Future that resolves when the method response has been processed.
#[derive(Debug)]
pub struct MethodResponseFuture(tokio::sync::oneshot::Receiver<NotifyMsg>);

/// A message that that tells whether notification
/// was succesful or not.
#[derive(Debug, Copy, Clone)]
pub enum NotifyMsg {
	/// The response was succesfully processed.
	Ok,
	/// The response was the wrong kind
	/// such an error response when
	/// one expected a succesful response.
	Err,
}

/// Method response error.
#[derive(Debug, Copy, Clone)]
pub enum MethodResponseError {
	/// The connection was closed.
	Closed,
	/// The response was a JSON-RPC error.
	JsonRpcError,
}

impl Future for MethodResponseFuture {
	type Output = Result<(), MethodResponseError>;

	fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
		match self.0.poll_unpin(cx) {
			Poll::Ready(Ok(NotifyMsg::Ok)) => Poll::Ready(Ok(())),
			Poll::Ready(Ok(NotifyMsg::Err)) => Poll::Ready(Err(MethodResponseError::JsonRpcError)),
			Poll::Ready(Err(_)) => Poll::Ready(Err(MethodResponseError::Closed)),
			Poll::Pending => Poll::Pending,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{BatchResponseBuilder, MethodResponse, ResponsePayload};
	use jsonrpsee_types::Id;

	#[test]
	fn batch_with_single_works() {
		let method = MethodResponse::response(Id::Number(1), ResponsePayload::success_borrowed(&"a"), usize::MAX);
		assert_eq!(method.result.len(), 37);

		// Recall a batch appends two bytes for the `[]`.
		let mut builder = BatchResponseBuilder::new_with_limit(39);
		builder.append(&method).unwrap();
		let batch = builder.finish();

		assert_eq!(batch.0, r#"[{"jsonrpc":"2.0","result":"a","id":1}]"#)
	}

	#[test]
	fn batch_with_multiple_works() {
		let m1 = MethodResponse::response(Id::Number(1), ResponsePayload::success_borrowed(&"a"), usize::MAX);
		assert_eq!(m1.result.len(), 37);

		// Recall a batch appends two bytes for the `[]` and one byte for `,` to append a method call.
		// so it should be 2 + (37 * n) + (n-1)
		let limit = 2 + (37 * 2) + 1;
		let mut builder = BatchResponseBuilder::new_with_limit(limit);
		builder.append(&m1).unwrap();
		builder.append(&m1).unwrap();
		let batch = builder.finish();

		assert_eq!(batch.0, r#"[{"jsonrpc":"2.0","result":"a","id":1},{"jsonrpc":"2.0","result":"a","id":1}]"#)
	}

	#[test]
	fn batch_empty_err() {
		let batch = BatchResponseBuilder::new_with_limit(1024).finish();

		let exp_err = r#"{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid request"},"id":null}"#;
		assert_eq!(batch.0, exp_err);
	}

	#[test]
	fn batch_too_big() {
		let method = MethodResponse::response(Id::Number(1), ResponsePayload::success_borrowed(&"a".repeat(28)), 128);
		assert_eq!(method.result.len(), 64);

		let batch = BatchResponseBuilder::new_with_limit(63).append(&method).unwrap_err();

		let exp_err = r#"{"jsonrpc":"2.0","error":{"code":-32011,"message":"The batch response was too large","data":"Exceeded max limit of 63"},"id":null}"#;
		assert_eq!(batch.result, exp_err);
	}
}
