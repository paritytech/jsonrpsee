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

use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::borrow::{Borrow, Cow as StdCow};
use thiserror::Error;

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
/// Invalid request error code.
pub const INVALID_REQUEST_CODE: i32 = -32600;
/// Method not found error code.
pub const METHOD_NOT_FOUND_CODE: i32 = -32601;
/// Invalid params error code.
pub const INVALID_PARAMS_CODE: i32 = -32602;
/// Internal error code.
pub const INTERNAL_ERROR_CODE: i32 = -32603;
/// Custom server error when a call failed.
pub const CALL_EXECUTION_FAILED_CODE: i32 = -32000;
/// Unknown error.
pub const UNKNOWN_ERROR_CODE: i32 = -32001;
/// Batched requests are not supported by the server.
pub const BATCHES_NOT_SUPPORTED_CODE: i32 = -32005;
/// Subscription limit per connection was exceeded.
pub const TOO_MANY_SUBSCRIPTIONS_CODE: i32 = -32006;
/// Oversized request error code.
pub const OVERSIZED_REQUEST_CODE: i32 = -32007;
/// Oversized response error code.
pub const OVERSIZED_RESPONSE_CODE: i32 = -32008;
/// Server is busy error code.
pub const SERVER_IS_BUSY_CODE: i32 = -32009;

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
	Custom(ErrorObjectOwned),
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
pub fn reject_too_many_subscriptions(limit: u32) -> ErrorObjectOwned {
	ErrorObjectOwned::owned(
		TOO_MANY_SUBSCRIPTIONS_CODE,
		TOO_MANY_SUBSCRIPTIONS_MSG,
		Some(format!("Exceeded max limit of {limit}")),
	)
}

/// Helper to get a `JSON-RPC` error object when the maximum request size limit have been exceeded.
pub fn reject_too_big_request(limit: u32) -> ErrorObjectOwned {
	ErrorObjectOwned::owned(
		OVERSIZED_REQUEST_CODE,
		OVERSIZED_REQUEST_MSG,
		Some(format!("Exceeded max limit of {limit}")),
	)
}
