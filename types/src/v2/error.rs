use crate::v2::params::{Id, TwoPointZero};
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::fmt;
use thiserror::Error;

/// [Failed JSON-RPC response object](https://www.jsonrpc.org/specification#response_object).
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct JsonRpcError<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Error.
	#[serde(borrow)]
	pub error: JsonRpcErrorObject<'a>,
	/// Request ID
	pub id: Id<'a>,
}

impl<'a> fmt::Display for JsonRpcError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", serde_json::to_string(&self).expect("infallible; qed"))
	}
}

/// JSON-RPC error object.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcErrorObject<'a> {
	/// Code
	pub code: JsonRpcErrorCode,
	/// Message
	pub message: &'a str,
	/// Optional data
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(borrow)]
	pub data: Option<&'a RawValue>,
}

impl<'a> From<JsonRpcErrorCode> for JsonRpcErrorObject<'a> {
	fn from(code: JsonRpcErrorCode) -> Self {
		Self { code, message: code.message(), data: None }
	}
}

impl<'a> PartialEq for JsonRpcErrorObject<'a> {
	fn eq(&self, other: &Self) -> bool {
		let this_raw = self.data.map(|r| r.get());
		let other_raw = self.data.map(|r| r.get());
		self.code == other.code && self.message == other.message && this_raw == other_raw
	}
}

/// Parse error code.
pub const PARSE_ERROR_CODE: i32 = -32700;
/// Oversized request error code.
pub const OVERSIZED_REQUEST_CODE: i32 = -32701;
/// Internal error code.
pub const INTERNAL_ERROR_CODE: i32 = -32603;
/// Invalid params error code.
pub const INVALID_PARAMS_CODE: i32 = -32602;
/// Invalid request error code.
pub const INVALID_REQUEST_CODE: i32 = -32600;
/// Method not found error code.
pub const METHOD_NOT_FOUND_CODE: i32 = -32601;
/// Custom server error when a call failed.
pub const CALL_EXECUTION_FAILED_CODE: i32 = -32000;

/// Parse error message
pub const PARSE_ERROR_MSG: &str = "Parse error";
/// Oversized request message
pub const OVERSIZED_REQUEST_MSG: &str = "Request is too big";
/// Internal error message.
pub const INTERNAL_ERROR_MSG: &str = "Internal error";
/// Invalid params error message.
pub const INVALID_PARAMS_MSG: &str = "Invalid params";
/// Invalid request error message.
pub const INVALID_REQUEST_MSG: &str = "Invalid request";
/// Method not found error message.
pub const METHOD_NOT_FOUND_MSG: &str = "Method not found";
/// Reserved for implementation-defined server-errors.
pub const SERVER_ERROR_MSG: &str = "Server error";

/// JSONRPC error code
#[derive(Error, Debug, PartialEq, Copy, Clone)]
pub enum JsonRpcErrorCode {
	/// Invalid JSON was received by the server.
	/// An error occurred on the server while parsing the JSON text.
	ParseError,
	/// The request was too big.
	OversizedRequest,
	/// The JSON sent is not a valid Request object.
	InvalidRequest,
	/// The method does not exist / is not available.
	MethodNotFound,
	/// Invalid method parameter(s).
	InvalidParams,
	/// Internal JSON-RPC error.
	InternalError,
	/// Reserved for implementation-defined server-errors.
	ServerError(i32),
}

impl JsonRpcErrorCode {
	/// Returns integer code value
	pub const fn code(&self) -> i32 {
		use JsonRpcErrorCode::*;
		match *self {
			ParseError => PARSE_ERROR_CODE,
			OversizedRequest => OVERSIZED_REQUEST_CODE,
			InvalidRequest => INVALID_REQUEST_CODE,
			MethodNotFound => METHOD_NOT_FOUND_CODE,
			InvalidParams => INVALID_PARAMS_CODE,
			InternalError => INTERNAL_ERROR_CODE,
			ServerError(code) => code,
		}
	}

	/// Returns the message for the given error code.
	pub const fn message(&self) -> &'static str {
		use JsonRpcErrorCode::*;
		match self {
			ParseError => PARSE_ERROR_MSG,
			OversizedRequest => OVERSIZED_REQUEST_MSG,
			InvalidRequest => INVALID_REQUEST_MSG,
			MethodNotFound => METHOD_NOT_FOUND_MSG,
			InvalidParams => INVALID_PARAMS_MSG,
			InternalError => INTERNAL_ERROR_MSG,
			ServerError(_) => SERVER_ERROR_MSG,
		}
	}
}

impl fmt::Display for JsonRpcErrorCode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}: {}", self.code(), self.message())
	}
}

impl From<i32> for JsonRpcErrorCode {
	fn from(code: i32) -> Self {
		use JsonRpcErrorCode::*;
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

impl<'a> serde::Deserialize<'a> for JsonRpcErrorCode {
	fn deserialize<D>(deserializer: D) -> Result<JsonRpcErrorCode, D::Error>
	where
		D: Deserializer<'a>,
	{
		let code: i32 = Deserialize::deserialize(deserializer)?;
		Ok(JsonRpcErrorCode::from(code))
	}
}

impl serde::Serialize for JsonRpcErrorCode {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_i32(self.code())
	}
}

#[cfg(test)]
mod tests {
	use super::{Id, JsonRpcError, JsonRpcErrorCode, JsonRpcErrorObject, TwoPointZero};

	#[test]
	fn deserialize_works() {
		let ser = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":null}"#;
		let exp = JsonRpcError {
			jsonrpc: TwoPointZero,
			error: JsonRpcErrorObject { code: JsonRpcErrorCode::ParseError, message: "Parse error", data: None },
			id: Id::Null,
		};
		let err: JsonRpcError = serde_json::from_str(ser).unwrap();
		assert_eq!(exp, err);
	}

	#[test]
	fn deserialize_with_optional_data() {
		let ser = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error", "data":"vegan"},"id":null}"#;
		let data = serde_json::value::to_raw_value(&"vegan").unwrap();
		let exp = JsonRpcError {
			jsonrpc: TwoPointZero,
			error: JsonRpcErrorObject {
				code: JsonRpcErrorCode::ParseError,
				message: "Parse error",
				data: Some(&*data),
			},
			id: Id::Null,
		};
		let err: JsonRpcError = serde_json::from_str(ser).unwrap();
		assert_eq!(exp, err);
	}

	#[test]
	fn serialize_works() {
		let exp = r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"Internal error"},"id":1337}"#;
		let err = JsonRpcError {
			jsonrpc: TwoPointZero,
			error: JsonRpcErrorObject { code: JsonRpcErrorCode::InternalError, message: "Internal error", data: None },
			id: Id::Number(1337),
		};
		let ser = serde_json::to_string(&err).unwrap();
		assert_eq!(exp, ser);
	}
}
