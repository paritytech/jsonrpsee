use crate::JsonValue;
use serde::{de::Deserializer, ser::Serializer, Deserialize};
use std::fmt;
use thiserror::Error;

/// [JSON-RPC Error object](https://www.jsonrpc.org/specification#error_object)
#[derive(Error, Debug, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcErrorObject {
	/// Error code
	pub code: ErrorCode,
	/// Message
	pub message: String,
	/// Optional data
	pub data: Option<JsonValue>,
}

impl fmt::Display for JsonRpcErrorObject {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}: {}: {:?}", self.code.code(), self.message, self.data)
	}
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
/// Reserved for implementation-defined server-errors.
pub const SERVER_ERROR_CODE_RANGE_START: i32 = -32000;
/// Reserved for implementation-defined server-errors.
pub const SERVER_ERROR_CODE_RANGE_END: i32 = 32099;

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
/// Application defined error which is not in the reserved space (-32000..=-32768)
pub const APPLICATION_ERROR_MSG: &str = "Application error";

/// JSONRPC error code
#[derive(Error, Debug, PartialEq, Copy, Clone)]
pub enum ErrorCode {
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
	/// Application defined error which is not in the reserved space (-32000..=-32768)
	ApplicationError(i32),
}

impl ErrorCode {
	/// Returns integer code value
	pub const fn code(&self) -> i32 {
		match *self {
			ErrorCode::ParseError => PARSE_ERROR_CODE,
			ErrorCode::InvalidRequest => INVALID_REQUEST_CODE,
			ErrorCode::MethodNotFound => METHOD_NOT_FOUND_CODE,
			ErrorCode::InvalidParams => INVALID_PARAMS_CODE,
			ErrorCode::InternalError => INTERNAL_ERROR_CODE,
			ErrorCode::ServerError(code) => code,
			ErrorCode::ApplicationError(code) => code,
		}
	}

	/// Returns the message for the given error code.
	pub const fn message(&self) -> &str {
		match self {
			ErrorCode::ParseError => PARSE_ERROR_MSG,
			ErrorCode::InvalidRequest => INVALID_REQUEST_MSG,
			ErrorCode::MethodNotFound => METHOD_NOT_FOUND_MSG,
			ErrorCode::InvalidParams => INVALID_PARAMS_MSG,
			ErrorCode::InternalError => INTERNAL_ERROR_MSG,
			ErrorCode::ServerError(_) => SERVER_ERROR_MSG,
			ErrorCode::ApplicationError(_) => APPLICATION_ERROR_MSG,
		}
	}
}

impl fmt::Display for ErrorCode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.code())
	}
}

impl From<i32> for ErrorCode {
	fn from(code: i32) -> Self {
		match code {
			PARSE_ERROR_CODE => ErrorCode::ParseError,
			INVALID_REQUEST_CODE => ErrorCode::InvalidRequest,
			METHOD_NOT_FOUND_CODE => ErrorCode::MethodNotFound,
			INVALID_PARAMS_CODE => ErrorCode::InvalidParams,
			INTERNAL_ERROR_CODE => ErrorCode::InternalError,
			SERVER_ERROR_CODE_RANGE_START..=SERVER_ERROR_CODE_RANGE_END => ErrorCode::ServerError(code),
			code => ErrorCode::ApplicationError(code),
		}
	}
}

impl<'a> serde::Deserialize<'a> for ErrorCode {
	fn deserialize<D>(deserializer: D) -> Result<ErrorCode, D::Error>
	where
		D: Deserializer<'a>,
	{
		let code: i32 = serde::Deserialize::deserialize(deserializer)?;
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
