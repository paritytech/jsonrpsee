use serde::de::{self, Deserializer, Unexpected, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Niklas dummy types
pub mod dummy;
/// Error type.
pub mod error;
/// Traits.
pub mod traits;

// TODO: revisit re-exports.
pub use beef::lean::Cow;
pub use error::RpcError;
pub use serde_json::value::{to_raw_value, RawValue};
pub use serde_json::Value as JsonValue;

/// [JSON-RPC request object](https://www.jsonrpc.org/specification#request-object)
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcRequest<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Request ID
	#[serde(borrow)]
	pub id: Option<&'a RawValue>,
	/// Name of the method to be invoked.
	#[serde(borrow)]
	pub method: Cow<'a, str>,
	/// Parameter values of the request.
	#[serde(borrow)]
	pub params: Option<&'a RawValue>,
}

/// Invalid request with known request ID.
#[derive(Deserialize, Debug)]
pub struct JsonRpcInvalidRequest<'a> {
	/// Request ID
	#[serde(borrow)]
	pub id: Option<&'a RawValue>,
}

/// JSON-RPC notification (a request object without a request ID).
#[derive(Serialize, Debug)]
pub struct JsonRpcNotification<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	pub method: &'a str,
	/// Parameter values of the request.
	pub params: JsonRpcNotificationParams<'a>,
}

/// JSON-RPC parameter values for subscriptions.
#[derive(Serialize, Debug)]
pub struct JsonRpcNotificationParams<'a> {
	/// Subscription ID
	pub subscription: u64,
	/// Result.
	pub result: &'a RawValue,
}

/// JSON-RPC successful response object.
#[derive(Serialize, Debug)]
pub struct JsonRpcResponse<'a, T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Result.
	pub result: T,
	/// Request ID
	pub id: Option<&'a RawValue>,
}

/// JSON-RPC error response object.
#[derive(Serialize, Debug)]
pub struct JsonRpcError<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Error.
	pub error: JsonRpcErrorParams<'a>,
	/// Request ID
	pub id: Option<&'a RawValue>,
}

/// [JSON-RPC error object](https://www.jsonrpc.org/specification#error-object)
#[derive(Serialize, Debug)]
pub struct JsonRpcErrorParams<'a> {
	/// Error code.
	pub code: i32,
	/// Error message.
	pub message: &'a str,
}

/// JSON-RPC v2 marker type.
#[derive(Debug, Default, PartialEq)]
pub struct TwoPointZero;

struct TwoPointZeroVisitor;

impl<'de> Visitor<'de> for TwoPointZeroVisitor {
	type Value = TwoPointZero;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(r#"a string "2.0""#)
	}

	fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		match s {
			"2.0" => Ok(TwoPointZero),
			_ => Err(de::Error::invalid_value(Unexpected::Str(s), &self)),
		}
	}
}

impl<'de> Deserialize<'de> for TwoPointZero {
	fn deserialize<D>(deserializer: D) -> Result<TwoPointZero, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_str(TwoPointZeroVisitor)
	}
}

impl Serialize for TwoPointZero {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str("2.0")
	}
}

/// Parameters sent with the RPC request
#[derive(Clone, Copy)]
pub struct RpcParams<'a>(Option<&'a str>);

impl<'a> RpcParams<'a> {
	/// Create params
	pub fn new(raw: Option<&'a str>) -> Self {
		Self(raw)
	}

	/// Attempt to parse all parameters as array or map into type T
	pub fn parse<T>(self) -> Result<T, RpcError>
	where
		T: Deserialize<'a>,
	{
		match self.0 {
			None => Err(RpcError::InvalidParams),
			Some(params) => serde_json::from_str(params).map_err(|_| RpcError::InvalidParams),
		}
	}

	/// Attempt to parse only the first parameter from an array into type T
	pub fn one<T>(self) -> Result<T, RpcError>
	where
		T: Deserialize<'a>,
	{
		self.parse::<[T; 1]>().map(|[res]| res)
	}
}
