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

use crate::v2::params::{Id, TwoPointZero};
use beef::Cow;
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::fmt;
use thiserror::Error;

/// [Failed JSON-RPC response object](https://www.jsonrpc.org/specification#response_object).
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RpcError<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Error.
	#[serde(borrow)]
	pub error: ErrorObject<'a>,
	/// Request ID
	pub id: Id<'a>,
}

impl<'a> RpcError<'a> {
	/// Create a new `RpcError`.
	pub fn new(error: ErrorObject<'a>, id: Id<'a>) -> Self {
		Self { jsonrpc: TwoPointZero, error, id }
	}
}

impl<'a> fmt::Display for RpcError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", serde_json::to_string(&self).expect("infallible; qed"))
	}
}

/// JSON-RPC error object.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ErrorObject<'a> {
	/// Code
	pub code: ErrorCode,
	/// Message
	pub message: Cow<'a, str>,
	/// Optional data
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(borrow)]
	pub data: Option<&'a RawValue>,
}

impl<'a> ErrorObject<'a> {
	/// Create a new `ErrorObject` with optional data.
	pub fn new(code: ErrorCode, data: Option<&'a RawValue>) -> ErrorObject<'a> {
		Self { code, message: code.message().into(), data }
	}
}

impl<'a> From<ErrorCode> for ErrorObject<'a> {
	fn from(code: ErrorCode) -> Self {
		Self { code, message: code.message().into(), data: None }
	}
}

impl<'a> PartialEq for ErrorObject<'a> {
	fn eq(&self, other: &Self) -> bool {
		let this_raw = self.data.map(|r| r.get());
		let other_raw = other.data.map(|r| r.get());
		self.code == other.code && self.message == other.message && this_raw == other_raw
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
/// Invalid subscription error code.
pub const INVALID_SUBSCRIPTION_CODE: i32 = -32002;

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

/// JSONRPC error code
#[derive(Error, Debug, PartialEq, Copy, Clone)]
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

/// Create a invalid subscription ID error.
pub fn invalid_subscription_err(data: Option<&RawValue>) -> ErrorObject {
	ErrorObject::new(ErrorCode::ServerError(INVALID_SUBSCRIPTION_CODE), data)
}

#[cfg(test)]
mod tests {
	use super::{ErrorCode, ErrorObject, Id, RpcError, TwoPointZero};

	#[test]
	fn deserialize_works() {
		let ser = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":null}"#;
		let exp = RpcError {
			jsonrpc: TwoPointZero,
			error: ErrorObject { code: ErrorCode::ParseError, message: "Parse error".into(), data: None },
			id: Id::Null,
		};
		let err: RpcError = serde_json::from_str(ser).unwrap();
		assert_eq!(exp, err);
	}

	#[test]
	fn deserialize_with_optional_data() {
		let ser = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error", "data":"vegan"},"id":null}"#;
		let data = serde_json::value::to_raw_value(&"vegan").unwrap();
		let exp = RpcError {
			jsonrpc: TwoPointZero,
			error: ErrorObject { code: ErrorCode::ParseError, message: "Parse error".into(), data: Some(&*data) },
			id: Id::Null,
		};
		let err: RpcError = serde_json::from_str(ser).unwrap();
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
		let err: RpcError = serde_json::from_str(raw).unwrap();

		let data = serde_json::value::to_raw_value(&"\\\"validate_transaction\\\"").unwrap();

		assert_eq!(
			err,
			RpcError {
				error: ErrorObject {
					code: 1002.into(),
					message: "desc: \"Could not decode `ChargeAssetTxPayment::asset_id`\" } })".into(),
					data: Some(&*data),
				},
				id: Id::Number(7),
				jsonrpc: TwoPointZero,
			}
		);
	}

	#[test]
	fn serialize_works() {
		let exp = r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"Internal error"},"id":1337}"#;
		let err = RpcError {
			jsonrpc: TwoPointZero,
			error: ErrorObject { code: ErrorCode::InternalError, message: "Internal error".into(), data: None },
			id: Id::Number(1337),
		};
		let ser = serde_json::to_string(&err).unwrap();
		assert_eq!(exp, ser);
	}
}
