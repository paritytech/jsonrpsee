//! Client side JSON-RPC types.

use crate::v2::TwoPointZero;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
/// JSON-RPC method call.
pub struct JsonRpcMethodOwned(pub String);

impl JsonRpcMethodOwned {
	/// Get inner.
	pub fn inner(&self) -> &str {
		self.0.as_str()
	}
}

/// Serializable JSON-RPC method call.
#[derive(Serialize, Debug, PartialEq)]
pub struct JsonRpcMethod<'a>(&'a str);

impl<'a> From<&'a str> for JsonRpcMethod<'a> {
	fn from(raw: &'a str) -> Self {
		Self(raw)
	}
}

impl<'a> JsonRpcMethod<'a> {
	/// Get inner representation of the method.
	pub fn inner(&self) -> &'a str {
		self.0
	}

	/// To owned.
	pub fn to_owned(&self) -> JsonRpcMethodOwned {
		JsonRpcMethodOwned(self.0.to_owned())
	}
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum JsonRpcParams<'a, T>
where
	T: Serialize + std::fmt::Debug + PartialEq,
{
	NoParams,
	Array(Vec<&'a T>),
}

impl<'a, T> From<Option<&'a T>> for JsonRpcParams<'a, T>
where
	T: Serialize + std::fmt::Debug + PartialEq,
{
	fn from(_raw: Option<&'a T>) -> Self {
		Self::NoParams
	}
}

impl<'a, T> From<Vec<&'a T>> for JsonRpcParams<'a, T>
where
	T: Serialize + std::fmt::Debug + PartialEq,
{
	fn from(arr: Vec<&'a T>) -> Self {
		Self::Array(arr)
	}
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum JsonRpcRequest<'a, T>
where
	T: Serialize + std::fmt::Debug + PartialEq + 'a,
{
	Single(JsonRpcCall<'a, T>),
	Batch(Vec<JsonRpcCall<'a, T>>),
	Notif(JsonRpcNotification<'a, T>),
}

impl<'a, T> From<JsonRpcCall<'a, T>> for JsonRpcRequest<'a, T>
where
	T: Serialize + std::fmt::Debug + PartialEq,
{
	fn from(call: JsonRpcCall<'a, T>) -> Self {
		JsonRpcRequest::Single(call)
	}
}
impl<'a, T> From<Vec<JsonRpcCall<'a, T>>> for JsonRpcRequest<'a, T>
where
	T: Serialize + std::fmt::Debug + PartialEq,
{
	fn from(batch: Vec<JsonRpcCall<'a, T>>) -> Self {
		JsonRpcRequest::Batch(batch)
	}
}

impl<'a, T> From<JsonRpcNotification<'a, T>> for JsonRpcRequest<'a, T>
where
	T: Serialize + std::fmt::Debug + PartialEq,
{
	fn from(notif: JsonRpcNotification<'a, T>) -> Self {
		JsonRpcRequest::Notif(notif)
	}
}

/// Serializable [JSON-RPC object](https://www.jsonrpc.org/specification#request-object)
#[derive(Serialize, Debug)]
pub struct JsonRpcCall<'a, T>
where
	T: Serialize + std::fmt::Debug + PartialEq,
{
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	#[serde(borrow)]
	pub method: &'a str,
	/// Request ID
	pub id: u64,
	/// Parameter values of the request.
	#[serde(borrow)]
	pub params: JsonRpcParams<'a, T>,
}

impl<'a, T> JsonRpcCall<'a, T>
where
	T: Serialize + std::fmt::Debug + PartialEq,
{
	/// Create a new serializable JSON-RPC request.
	pub fn new(id: u64, method: &'a str, params: JsonRpcParams<'a, T>) -> Self {
		Self { jsonrpc: TwoPointZero, id, method, params }
	}
}

/// Serializable [JSON-RPC notification object](https://www.jsonrpc.org/specification#request-object)
#[derive(Serialize, Debug)]
pub struct JsonRpcNotification<'a, T>
where
	T: Serialize + std::fmt::Debug + PartialEq,
{
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	#[serde(borrow)]
	pub method: &'a str,
	/// Parameter values of the request.
	#[serde(borrow)]
	pub params: JsonRpcParams<'a, T>,
}

impl<'a, T> JsonRpcNotification<'a, T>
where
	T: Serialize + std::fmt::Debug + PartialEq,
{
	/// Create a new serializable JSON-RPC request.
	pub fn new(method: &'a str, params: JsonRpcParams<'a, T>) -> Self {
		Self { jsonrpc: TwoPointZero, method, params }
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcResponseObject<T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Result.
	pub result: T,
	/// Request ID
	pub id: u64,
}

/// JSON-RPC parameter values for subscriptions.
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcNotificationParams<T> {
	/// Subscription ID
	pub subscription: SubscriptionId,
	/// Result.
	pub result: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcResponseNotif<T> {
	pub jsonrpc: TwoPointZero,
	pub params: JsonRpcNotificationParams<T>,
}

/// Represent the different JSON-RPC responses.
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponse<T> {
	/// Single response.
	Single(JsonRpcResponseObject<T>),
	/// Batch response.
	Batch(Vec<JsonRpcResponseObject<T>>),
	/// Notification response used for subscriptions.
	Subscription(JsonRpcResponseNotif<T>),
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
