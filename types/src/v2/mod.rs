use crate::{error::RpcError, Cow, Error};
use alloc::collections::BTreeMap;
use serde::de::{self, DeserializeOwned, Deserializer, Unexpected, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use std::fmt;
use thiserror::Error;

/// JSON-RPC related error types.
pub mod error;

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
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcNotification<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	pub method: &'a str,
	/// Parameter values of the request.
	pub params: JsonRpcNotificationParams<'a>,
}

/// JSON-RPC parameter values for subscriptions.
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcNotificationParams<'a> {
	/// Subscription ID
	pub subscription: u64,
	/// Result.
	#[serde(borrow)]
	pub result: &'a RawValue,
}

/// JSON-RPC successful response object.
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcResponse<'a, T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Result.
	pub result: T,
	/// Request ID
	#[serde(borrow)]
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

/// [Failed JSON-RPC response object](https://www.jsonrpc.org/specification#response_object).
#[derive(Error, Debug, Deserialize, PartialEq)]
pub struct JsonRpcErrorAlloc {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	#[serde(rename = "error")]
	/// Error object.
	pub inner: error::JsonRpcErrorObject,
	/// Request ID.
	pub id: u64,
}

impl fmt::Display for JsonRpcErrorAlloc {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.inner)
	}
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

/// [JSON-RPC parameters](https://www.jsonrpc.org/specification#parameter_structures)
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum JsonRpcParams<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	/// No params.
	NoParams,
	/// Positional params.
	Array(&'a [T]),
	/// Params by name.
	//
	// TODO(niklasad1): maybe take a reference here but BTreeMap needs allocation anyway.
	Map(BTreeMap<&'a str, &'a T>),
}

// TODO(niklasad1): this is a little weird but nice if `None.into()` works.
impl<'a, T> From<Option<&'a T>> for JsonRpcParams<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	fn from(_raw: Option<&'a T>) -> Self {
		Self::NoParams
	}
}

impl<'a, T> From<BTreeMap<&'a str, &'a T>> for JsonRpcParams<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	fn from(map: BTreeMap<&'a str, &'a T>) -> Self {
		Self::Map(map)
	}
}

impl<'a, T> From<&'a [T]> for JsonRpcParams<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	fn from(arr: &'a [T]) -> Self {
		Self::Array(arr)
	}
}

/// Serializable [JSON-RPC object](https://www.jsonrpc.org/specification#request-object)
#[derive(Serialize, Debug)]
pub struct JsonRpcCallSer<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	pub method: &'a str,
	/// Request ID
	pub id: u64,
	/// Parameter values of the request.
	pub params: JsonRpcParams<'a, T>,
}

impl<'a, T> JsonRpcCallSer<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	/// Create a new serializable JSON-RPC request.
	pub fn new(id: u64, method: &'a str, params: JsonRpcParams<'a, T>) -> Self {
		Self { jsonrpc: TwoPointZero, id, method, params }
	}
}

/// Serializable [JSON-RPC notification object](https://www.jsonrpc.org/specification#request-object)
#[derive(Serialize, Debug)]
pub struct JsonRpcNotificationSer<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	pub method: &'a str,
	/// Parameter values of the request.
	pub params: JsonRpcParams<'a, T>,
}

impl<'a, T> JsonRpcNotificationSer<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	/// Create a new serializable JSON-RPC request.
	pub fn new(method: &'a str, params: JsonRpcParams<'a, T>) -> Self {
		Self { jsonrpc: TwoPointZero, method, params }
	}
}

/// JSON-RPC parameter values for subscriptions.
#[derive(Deserialize, Debug)]
pub struct JsonRpcNotificationParamsAlloc<T> {
	/// Subscription ID
	pub subscription: SubscriptionId,
	/// Result.
	pub result: T,
}

/// JSON-RPC notification response.
// NOTE(niklasad1): basically the same as Maciej version but I wanted to support Strings too.
// Maybe make subscription ID generic?!
#[derive(Deserialize, Debug)]
pub struct JsonRpcNotifAlloc<T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Params.
	pub params: JsonRpcNotificationParamsAlloc<T>,
}

/// Id of a subscription, communicated by the server.
#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum SubscriptionId {
	/// Numeric id
	Num(u64),
	/// String id
	Str(String),
}

/// Parse request ID from RawValue.
pub fn parse_request_id<T: DeserializeOwned>(raw: Option<&RawValue>) -> Result<T, crate::Error> {
	match raw {
		None => Err(Error::InvalidRequestId),
		Some(v) => {
			let val = serde_json::from_str(v.get()).map_err(Error::ParseError)?;
			Ok(val)
		}
	}
}
