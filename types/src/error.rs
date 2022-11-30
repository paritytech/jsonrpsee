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

use std::fmt;

use crate::params::{Id, TwoPointZero};
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::borrow::Borrow;
use std::borrow::Cow as StdCow;
use thiserror::Error;

/// [Failed JSON-RPC response object](https://www.jsonrpc.org/specification#response_object).
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ErrorResponse<'a> {
	/// JSON-RPC version.
	jsonrpc: TwoPointZero,
	/// Error.
	#[serde(borrow)]
	error: ErrorObject<'a>,
	/// Request ID
	id: Id<'a>,
}

impl<'a> ErrorResponse<'a> {
	/// Create a borrowed `ErrorResponse`.
	pub fn borrowed(error: ErrorObject<'a>, id: Id<'a>) -> Self {
		Self { jsonrpc: TwoPointZero, error, id }
	}

	/// Create a borrowed `ErrorResponse`.
	pub fn owned(error: ErrorObject<'static>, id: Id<'static>) -> Self {
		Self { jsonrpc: TwoPointZero, error, id }
	}

	/// Take ownership of the parameters within, if we haven't already.
	pub fn into_owned(self) -> ErrorResponse<'static> {
		ErrorResponse { jsonrpc: self.jsonrpc, error: self.error.into_owned(), id: self.id.into_owned() }
	}

	/// Get the [`ErrorObject`] of the error response.
	pub fn error_object(&self) -> &ErrorObject {
		&self.error
	}

	/// Get the [`Id`] of the error response.
	pub fn id(&self) -> &Id {
		&self.id
	}
}

impl<'a> fmt::Display for ErrorResponse<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", serde_json::to_string(&self).expect("infallible; qed"))
	}
}
/// The return type of the subscription's method for the rpc server implementation.
///
/// **Note**: The error does not contain any data and is discarded on drop.
pub type SubscriptionResult = Result<(), SubscriptionEmptyError>;

/// The error returned by the subscription's method for the rpc server implementation.
///
/// It contains no data, and neither is the error utilized. It provides an abstraction to make the
/// API more ergonomic while handling errors that may occur during the subscription callback.
#[derive(Debug, Clone, Copy)]
pub struct SubscriptionEmptyError;

impl From<anyhow::Error> for SubscriptionEmptyError {
	fn from(_: anyhow::Error) -> Self {
		SubscriptionEmptyError
	}
}

impl From<CallError> for SubscriptionEmptyError {
	fn from(_: CallError) -> Self {
		SubscriptionEmptyError
	}
}

impl<'a> From<ErrorObject<'a>> for SubscriptionEmptyError {
	fn from(_: ErrorObject<'a>) -> Self {
		SubscriptionEmptyError
	}
}

impl From<SubscriptionAcceptRejectError> for SubscriptionEmptyError {
	fn from(_: SubscriptionAcceptRejectError) -> Self {
		SubscriptionEmptyError
	}
}

/// The error returned while accepting or rejecting a subscription.
#[derive(Debug, Copy, Clone)]
pub enum SubscriptionAcceptRejectError {
	/// The method was already called.
	AlreadyCalled,
	/// The remote peer closed the connection or called the unsubscribe method.
	RemotePeerAborted,
}

/// Owned variant of [`ErrorObject`].
pub type ErrorObjectOwned = ErrorObject<'static>;

/// [Failed JSON-RPC response object](https://www.jsonrpc.org/specification#response_object).
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ErrorObject<'a> {
	/// Code
	code: ErrorCode,
	/// Message
	message: StdCow<'a, str>,
	/// Optional data
	#[serde(skip_serializing_if = "Option::is_none")]
	data: Option<StdCow<'a, RawValue>>,
}

impl<'a> ErrorObject<'a> {
	/// Return the error code
	pub fn code(&self) -> i32 {
		self.code.code()
	}

	/// Return the message
	pub fn message(&self) -> &str {
		self.message.borrow()
	}

	/// Return the data associated with this error, if any
	pub fn data(&self) -> Option<&RawValue> {
		self.data.as_ref().map(|d| d.borrow())
	}

	/// Create a new `ErrorObjectOwned` with optional data.
	pub fn owned<S: Serialize>(code: i32, message: impl Into<String>, data: Option<S>) -> ErrorObject<'static> {
		let data = data.and_then(|d| serde_json::value::to_raw_value(&d).ok());
		ErrorObject { code: code.into(), message: message.into().into(), data: data.map(StdCow::Owned) }
	}

	/// Create a new [`ErrorObject`] with optional data.
	pub fn borrowed(code: i32, message: &'a impl AsRef<str>, data: Option<&'a RawValue>) -> ErrorObject<'a> {
		ErrorObject { code: code.into(), message: StdCow::Borrowed(message.as_ref()), data: data.map(StdCow::Borrowed) }
	}

	/// Take ownership of the parameters within, if we haven't already.
	pub fn into_owned(self) -> ErrorObject<'static> {
		ErrorObject {
			code: self.code,
			message: StdCow::Owned(self.message.into_owned()),
			data: self.data.map(|d| StdCow::Owned(d.into_owned())),
		}
	}

	/// Borrow the current [`ErrorObject`].
	pub fn borrow(&'a self) -> ErrorObject<'a> {
		ErrorObject {
			code: self.code,
			message: StdCow::Borrowed(self.message.borrow()),
			data: self.data.as_ref().map(|d| StdCow::Borrowed(d.borrow())),
		}
	}
}

impl<'a> PartialEq for ErrorObject<'a> {
	fn eq(&self, other: &Self) -> bool {
		let this_raw = self.data.as_ref().map(|r| r.get());
		let other_raw = other.data.as_ref().map(|r| r.get());
		self.code == other.code && self.message == other.message && this_raw == other_raw
	}
}

impl<'a> From<ErrorCode> for ErrorObject<'a> {
	fn from(code: ErrorCode) -> Self {
		Self { code, message: code.message().into(), data: None }
	}
}

impl<'a> From<CallError> for ErrorObject<'a> {
	fn from(error: CallError) -> Self {
		match error {
			CallError::InvalidParams(e) => ErrorObject::owned(INVALID_PARAMS_CODE, e.to_string(), None::<()>),
			CallError::Failed(e) => ErrorObject::owned(CALL_EXECUTION_FAILED_CODE, e.to_string(), None::<()>),
			CallError::Custom(err) => err,
		}
	}
}

/// Parse error code.
pub const PARSE_ERROR_CODE: i32 = -32700;
/// Oversized request error code.
pub const OVERSIZED_REQUEST_CODE: i32 = -32701;
/// Oversized response error code.
pub const OVERSIZED_RESPONSE_CODE: i32 = -32702;
/// Internal error code.
pub const INTERNAL_ERROR_CODE: i32 = -32603;
/// Invalid params error code.
pub const INVALID_PARAMS_CODE: i32 = -32602;
/// Invalid request error code.
pub const INVALID_REQUEST_CODE: i32 = -32600;
/// Method not found error code.
pub const METHOD_NOT_FOUND_CODE: i32 = -32601;
/// Server is busy error code.
pub const SERVER_IS_BUSY_CODE: i32 = -32604;
/// Custom server error when a call failed.
pub const CALL_EXECUTION_FAILED_CODE: i32 = -32000;
/// Unknown error.
pub const UNKNOWN_ERROR_CODE: i32 = -32001;
/// Subscription got closed by the server.
pub const SUBSCRIPTION_CLOSED: i32 = -32003;
/// Subscription got closed by the server.
pub const SUBSCRIPTION_CLOSED_WITH_ERROR: i32 = -32004;
/// Batched requests are not supported by the server.
pub const BATCHES_NOT_SUPPORTED_CODE: i32 = -32005;
/// Subscription limit per connection was exceeded.
pub const TOO_MANY_SUBSCRIPTIONS_CODE: i32 = -32006;

/// Parse error message
pub const PARSE_ERROR_MSG: &str = "Parse error";
/// Oversized request message
pub const OVERSIZED_REQUEST_MSG: &str = "Request is too big";
/// Oversized response message
pub const OVERSIZED_RESPONSE_MSG: &str = "Response is too big";
/// Internal error message.
pub const INTERNAL_ERROR_MSG: &str = "Internal error";
/// Invalid params error message.
pub const INVALID_PARAMS_MSG: &str = "Invalid params";
/// Invalid request error message.
pub const INVALID_REQUEST_MSG: &str = "Invalid request";
/// Method not found error message.
pub const METHOD_NOT_FOUND_MSG: &str = "Method not found";
/// Server is busy error message.
pub const SERVER_IS_BUSY_MSG: &str = "Server is busy, try again later";
/// Reserved for implementation-defined server-errors.
pub const SERVER_ERROR_MSG: &str = "Server error";
/// Batched requests not supported error message.
pub const BATCHES_NOT_SUPPORTED_MSG: &str = "Batched requests are not supported by this server";
/// Subscription limit per connection was exceeded.
pub const TOO_MANY_SUBSCRIPTIONS_MSG: &str = "Too many subscriptions on the connection";

/// JSONRPC error code
#[derive(Error, Debug, PartialEq, Eq, Copy, Clone)]
pub enum ErrorCode {
	/// Invalid JSON was received by the server.
	/// An error occurred on the server while parsing the JSON text.
	ParseError,
	/// The request was too big.
	OversizedRequest,
	/// The JSON sent is not a valid Request object.
	InvalidRequest,
	/// The method does not exist / is not available.
	MethodNotFound,
	/// Server is busy / resources are at capacity.
	ServerIsBusy,
	/// Invalid method parameter(s).
	InvalidParams,
	/// Internal JSON-RPC error.
	InternalError,
	/// Reserved for implementation-defined server-errors.
	ServerError(i32),
}

impl ErrorCode {
	/// Returns integer code value
	pub const fn code(&self) -> i32 {
		use ErrorCode::*;
		match *self {
			ParseError => PARSE_ERROR_CODE,
			OversizedRequest => OVERSIZED_REQUEST_CODE,
			InvalidRequest => INVALID_REQUEST_CODE,
			MethodNotFound => METHOD_NOT_FOUND_CODE,
			ServerIsBusy => SERVER_IS_BUSY_CODE,
			InvalidParams => INVALID_PARAMS_CODE,
			InternalError => INTERNAL_ERROR_CODE,
			ServerError(code) => code,
		}
	}

	/// Returns the message for the given error code.
	pub const fn message(&self) -> &'static str {
		use ErrorCode::*;
		match self {
			ParseError => PARSE_ERROR_MSG,
			OversizedRequest => OVERSIZED_REQUEST_MSG,
			InvalidRequest => INVALID_REQUEST_MSG,
			MethodNotFound => METHOD_NOT_FOUND_MSG,
			ServerIsBusy => SERVER_IS_BUSY_MSG,
			InvalidParams => INVALID_PARAMS_MSG,
			InternalError => INTERNAL_ERROR_MSG,
			ServerError(_) => SERVER_ERROR_MSG,
		}
	}
}

impl fmt::Display for ErrorCode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.code(), self.message())
	}
}

impl From<i32> for ErrorCode {
	fn from(code: i32) -> Self {
		use ErrorCode::*;
		match code {
			PARSE_ERROR_CODE => ParseError,
			OVERSIZED_REQUEST_CODE => OversizedRequest,
			INVALID_REQUEST_CODE => InvalidRequest,
			METHOD_NOT_FOUND_CODE => MethodNotFound,
			INVALID_PARAMS_CODE => InvalidParams,
			INTERNAL_ERROR_CODE => InternalError,
			code => ServerError(code),
		}
	}
}

impl<'a> serde::Deserialize<'a> for ErrorCode {
	fn deserialize<D>(deserializer: D) -> Result<ErrorCode, D::Error>
	where
		D: Deserializer<'a>,
	{
		let code: i32 = Deserialize::deserialize(deserializer)?;
		Ok(ErrorCode::from(code))
	}
}

impl serde::Serialize for ErrorCode {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_i32(self.code())
	}
}

/// Error that occurs when a call failed.
#[derive(Debug, thiserror::Error)]
pub enum CallError {
	/// Invalid params in the call.
	#[error("Invalid params in the call: {0}")]
	InvalidParams(#[source] anyhow::Error),
	/// The call failed (let jsonrpsee assign default error code and error message).
	#[error("RPC call failed: {0}")]
	Failed(#[from] anyhow::Error),
	/// Custom error with specific JSON-RPC error code, message and data.
	#[error("RPC call failed: {0:?}")]
	Custom(ErrorObject<'static>),
}

impl CallError {
	/// Create `CallError` from a generic error.
	pub fn from_std_error<E>(err: E) -> Self
	where
		E: std::error::Error + Send + Sync + 'static,
	{
		CallError::Failed(err.into())
	}
}

/// Helper to get a `JSON-RPC` error object when the maximum number of subscriptions have been exceeded.
pub fn reject_too_many_subscriptions(limit: u32) -> ErrorObject<'static> {
	ErrorObjectOwned::owned(
		TOO_MANY_SUBSCRIPTIONS_CODE,
		TOO_MANY_SUBSCRIPTIONS_MSG,
		Some(format!("Exceeded max limit of {}", limit)),
	)
}

/// Helper to get a `JSON-RPC` error object when the maximum request size limit have been exceeded.
pub fn reject_too_big_request(limit: u32) -> ErrorObject<'static> {
	ErrorObjectOwned::owned(
		OVERSIZED_REQUEST_CODE,
		OVERSIZED_REQUEST_MSG,
		Some(format!("Exceeded max limit of {}", limit)),
	)
}

#[cfg(test)]
mod tests {
	use super::{ErrorCode, ErrorObject, ErrorResponse, Id, TwoPointZero};

	#[test]
	fn deserialize_works() {
		let ser = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":null}"#;
		let exp = ErrorResponse {
			jsonrpc: TwoPointZero,
			error: ErrorObject { code: ErrorCode::ParseError, message: "Parse error".into(), data: None },
			id: Id::Null,
		};
		let err: ErrorResponse = serde_json::from_str(ser).unwrap();
		assert_eq!(exp, err);
	}

	#[test]
	fn deserialize_with_optional_data() {
		let ser = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error", "data":"vegan"},"id":null}"#;
		let data = serde_json::value::to_raw_value(&"vegan").unwrap();
		let exp = ErrorResponse {
			jsonrpc: TwoPointZero,
			error: ErrorObject::owned(ErrorCode::ParseError.code(), "Parse error", Some(data)),
			id: Id::Null,
		};
		let err: ErrorResponse = serde_json::from_str(ser).unwrap();
		assert_eq!(exp, err);
	}

	#[test]
	fn deserialized_error_with_quoted_str() {
		let raw = r#"{
			"error": {
				"code": 1002,
				"message": "desc: \"Could not decode `ChargeAssetTxPayment::asset_id`\" } })",
				"data": "\\\"validate_transaction\\\""
			},
			"id": 7,
			"jsonrpc": "2.0"
		}"#;
		let err: ErrorResponse = serde_json::from_str(raw).unwrap();

		let data = serde_json::value::to_raw_value(&"\\\"validate_transaction\\\"").unwrap();

		assert_eq!(
			err,
			ErrorResponse {
				error: ErrorObject::borrowed(
					1002,
					&"desc: \"Could not decode `ChargeAssetTxPayment::asset_id`\" } })",
					Some(&*data)
				),
				id: Id::Number(7),
				jsonrpc: TwoPointZero,
			}
		);
	}

	#[test]
	fn serialize_works() {
		let exp = r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"Internal error"},"id":1337}"#;
		let err = ErrorResponse {
			jsonrpc: TwoPointZero,
			error: ErrorObject { code: ErrorCode::InternalError, message: "Internal error".into(), data: None },
			id: Id::Number(1337),
		};
		let ser = serde_json::to_string(&err).unwrap();
		assert_eq!(exp, ser);
	}
}
