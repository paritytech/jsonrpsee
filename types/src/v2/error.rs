use crate::v2::params::{Id, TwoPointZero};
use serde::de::{Deserializer, Error as DeserializeError, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::fmt;
use thiserror::Error;

/// [Failed JSON-RPC response object](https://www.jsonrpc.org/specification#response_object).
#[derive(Serialize, Debug)]
pub struct JsonRpcError<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Error.
	pub error: ErrorCode,
	/// Request ID
	pub id: Option<&'a RawValue>,
}
/// [Failed JSON-RPC response object with allocations](https://www.jsonrpc.org/specification#response_object).
#[derive(Error, Debug, Deserialize, PartialEq)]
pub struct JsonRpcErrorAlloc {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Error object.
	pub error: ErrorCode,
	/// Request ID.
	pub id: Id,
}

impl fmt::Display for JsonRpcErrorAlloc {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}: {}: {:?}", self.jsonrpc, self.error, self.id)
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

/// Expected field to be found in the deserialization visitor.
const ERROR_CODE_KEY: &str = "code";

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
		write!(f, "{}: {}", self.code(), self.message())
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
		let code = deserializer.deserialize_map(ErrorCodeVisitor)?;
		Ok(code)
	}
}

impl serde::Serialize for ErrorCode {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut map = serializer.serialize_map(Some(2))?;
		map.serialize_entry("code", &self.code())?;
		map.serialize_entry("message", self.message())?;
		map.end()
	}
}

struct ErrorCodeVisitor;

impl<'de> Visitor<'de> for ErrorCodeVisitor {
	type Value = ErrorCode;

	// Format a message stating what data this Visitor expects to receive.
	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(ERROR_CODE_KEY)
	}

	fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
	where
		M: MapAccess<'de>,
	{
		let mut res = None;

		loop {
			match access.next_entry::<&str, i32>() {
				Ok(Some((key, val))) if key == ERROR_CODE_KEY && res.is_none() => {
					res = Some(Ok(val.into()));
				}
				Ok(Some((key, _))) if key == ERROR_CODE_KEY => {
					res = Some(Err(DeserializeError::duplicate_field(ERROR_CODE_KEY)));
				}
				Ok(None) => break,
				// traverse the entire map otherwise it will err,
				_ => (),
			}
		}

		match res {
			Some(res) => res,
			None => Err(DeserializeError::missing_field(ERROR_CODE_KEY)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{ErrorCode, Id, JsonRpcError, JsonRpcErrorAlloc, TwoPointZero};

	#[test]
	fn deserialize_works() {
		let ser = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":null}"#;
		let err: JsonRpcErrorAlloc = serde_json::from_str(ser).unwrap();
		assert_eq!(err.jsonrpc, TwoPointZero);
		assert_eq!(err.error, ErrorCode::ParseError);
		assert_eq!(err.id, Id::Null);
		let ser = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error", "data":"vegan"},"id":null}"#;
		let err: JsonRpcErrorAlloc = serde_json::from_str(ser).unwrap();
		assert_eq!(err.jsonrpc, TwoPointZero);
		assert_eq!(err.error, ErrorCode::ParseError);
		assert_eq!(err.id, Id::Null);
	}

	#[test]
	fn deserialize_with_unknown_fields() {
		let ser = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error", "data":"vegan", "lol":1337},"id":null}"#;
		let err: JsonRpcErrorAlloc = serde_json::from_str(ser).unwrap();
		assert_eq!(err.jsonrpc, TwoPointZero);
		assert_eq!(err.error, ErrorCode::ParseError);
		assert_eq!(err.id, Id::Null);
	}

	#[test]
	fn serialize_works() {
		let exp = r#"{"jsonrpc":"2.0","error":{"code":-32603,"message":"Internal error"},"id":1337}"#;
		let raw_id = serde_json::value::to_raw_value(&1337).unwrap();
		let err = JsonRpcError { jsonrpc: TwoPointZero, error: ErrorCode::InternalError, id: Some(&*raw_id) };
		let ser = serde_json::to_string(&err).unwrap();
		assert_eq!(exp, ser);
	}
}
