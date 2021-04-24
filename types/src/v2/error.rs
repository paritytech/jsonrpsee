use crate::v2::params::{Id, TwoPointZero};
use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::value::{RawValue, Value as JsonValue};
use std::fmt;
use thiserror::Error;

/// [Failed JSON-RPC response object](https://www.jsonrpc.org/specification#response_object).
#[derive(Serialize, Debug)]
pub struct JsonRpcError<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Error.
	pub error: JsonRpcErrorObject<'a>,
	/// Request ID
	pub id: Option<&'a RawValue>,
}
/// [Failed JSON-RPC response object with allocations](https://www.jsonrpc.org/specification#response_object).
#[derive(Error, Debug, Deserialize, PartialEq)]
pub struct JsonRpcErrorAlloc {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// JSON-RPC error object.
	pub error: JsonRpcErrorObjectAlloc,
	/// Request ID.
	pub id: Id,
}

impl fmt::Display for JsonRpcErrorAlloc {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}: {:?}: {:?}", self.jsonrpc, self.error, self.id)
	}
}

/// JSON-RPC error object.
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcErrorObjectAlloc {
	/// Code
	pub code: JsonRpcErrorCode,
	/// Message
	pub message: String,
	/// Optional data
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<JsonValue>,
}

impl From<JsonRpcErrorCode> for JsonRpcErrorObjectAlloc {
	fn from(code: JsonRpcErrorCode) -> Self {
		Self { message: code.message().to_owned(), code, data: None }
	}
}

/// JSON-RPC error object with no extra allocations.
#[derive(Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcErrorObject<'a> {
	/// Code
	pub code: JsonRpcErrorCode,
	/// Message
	pub message: &'a str,
	/// Optional data
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<&'a RawValue>,
}

/// Parse error code.
pub const PARSE_ERROR_CODE: i32 = -32700;
/// Internal error code.
pub const INTERNAL_ERROR_CODE: i32 = -32603;
/// Invalid params error code.
pub const INVALID_PARAMS_CODE: i32 = -32602;
/// Invalid request error code.
pub const INVALID_REQUEST_CODE: i32 = -32600;
/// Method not found error code.
pub const METHOD_NOT_FOUND_CODE: i32 = -32601;

/// Parse error message
pub const PARSE_ERROR_MSG: &str = "Parse error";
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
		match *self {
			JsonRpcErrorCode::ParseError => PARSE_ERROR_CODE,
			JsonRpcErrorCode::InvalidRequest => INVALID_REQUEST_CODE,
			JsonRpcErrorCode::MethodNotFound => METHOD_NOT_FOUND_CODE,
			JsonRpcErrorCode::InvalidParams => INVALID_PARAMS_CODE,
			JsonRpcErrorCode::InternalError => INTERNAL_ERROR_CODE,
			JsonRpcErrorCode::ServerError(code) => code,
		}
	}

	/// Returns the message for the given error code.
	pub const fn message(&self) -> &str {
		match self {
			JsonRpcErrorCode::ParseError => PARSE_ERROR_MSG,
			JsonRpcErrorCode::InvalidRequest => INVALID_REQUEST_MSG,
			JsonRpcErrorCode::MethodNotFound => METHOD_NOT_FOUND_MSG,
			JsonRpcErrorCode::InvalidParams => INVALID_PARAMS_MSG,
			JsonRpcErrorCode::InternalError => INTERNAL_ERROR_MSG,
			JsonRpcErrorCode::ServerError(_) => SERVER_ERROR_MSG,
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
		match code {
			PARSE_ERROR_CODE => JsonRpcErrorCode::ParseError,
			INVALID_REQUEST_CODE => JsonRpcErrorCode::InvalidRequest,
			METHOD_NOT_FOUND_CODE => JsonRpcErrorCode::MethodNotFound,
			INVALID_PARAMS_CODE => JsonRpcErrorCode::InvalidParams,
			INTERNAL_ERROR_CODE => JsonRpcErrorCode::InternalError,
			code => JsonRpcErrorCode::ServerError(code),
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
	use super::{
		Id, JsonRpcError, JsonRpcErrorAlloc, JsonRpcErrorCode, JsonRpcErrorObject, JsonRpcErrorObjectAlloc,
		TwoPointZero,
	};

	#[test]
	fn deserialize_works() {
		let ser = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":null}"#;
		let err: JsonRpcErrorAlloc = serde_json::from_str(ser).unwrap();
		assert_eq!(err.jsonrpc, TwoPointZero);
		assert_eq!(
			err.error,
			JsonRpcErrorObjectAlloc { code: JsonRpcErrorCode::ParseError, message: "Parse error".into(), data: None }
		);
		assert_eq!(err.id, Id::Null);
	}

	#[test]
	fn deserialize_with_optional_data() {
		let ser = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error", "data":"vegan"},"id":null}"#;
		let err: JsonRpcErrorAlloc = serde_json::from_str(ser).unwrap();
		assert_eq!(err.jsonrpc, TwoPointZero);
		assert_eq!(
			err.error,
			JsonRpcErrorObjectAlloc {
				code: JsonRpcErrorCode::ParseError,
				message: "Parse error".into(),
				data: Some("vegan".into())
			}
		);
		assert_eq!(err.id, Id::Null);
	}

	#[test]
	fn serialize_works() {
		let exp = r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"Internal error"},"id":1337}"#;
		let raw_id = serde_json::value::to_raw_value(&1337).unwrap();
		let err = JsonRpcError {
			jsonrpc: TwoPointZero,
			error: JsonRpcErrorObject { code: JsonRpcErrorCode::InternalError, message: "Internal error", data: None },
			id: Some(&*raw_id),
		};
		let ser = serde_json::to_string(&err).unwrap();
		assert_eq!(exp, ser);
	}
}
