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

#[derive(Debug)]
/// JSON-RPC method call.
pub struct JsonRpcParamsOwned(pub Option<String>);

impl JsonRpcParamsOwned {
	/// Get inner.
	pub fn inner(&self) -> Option<&str> {
		self.0.as_ref().map(|p| p.as_str())
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

/// Serializable JSON-RPC params.
#[derive(Serialize, Debug, PartialEq)]
pub struct JsonRpcParams<'a>(Option<&'a str>);

impl<'a> From<Option<&'a str>> for JsonRpcParams<'a> {
	fn from(raw: Option<&'a str>) -> Self {
		Self(raw)
	}
}

impl<'a> JsonRpcParams<'a> {
	/// Get inner representation of the params.
	pub fn inner(&self) -> Option<&'a str> {
		self.0
	}

	/// To owned.
	pub fn to_owned(&self) -> JsonRpcParamsOwned {
		JsonRpcParamsOwned(self.0.map(ToOwned::to_owned))
	}
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum JsonRpcRequest<'a> {
	Single(JsonRpcCall<'a>),
	Batch(Vec<JsonRpcCall<'a>>),
	Notif(JsonRpcNotification<'a>),
}

impl<'a> From<JsonRpcCall<'a>> for JsonRpcRequest<'a> {
	fn from(call: JsonRpcCall<'a>) -> Self {
		JsonRpcRequest::Single(call)
	}
}
impl<'a> From<Vec<JsonRpcCall<'a>>> for JsonRpcRequest<'a> {
	fn from(batch: Vec<JsonRpcCall<'a>>) -> Self {
		JsonRpcRequest::Batch(batch)
	}
}
impl<'a> From<JsonRpcNotification<'a>> for JsonRpcRequest<'a> {
	fn from(notif: JsonRpcNotification<'a>) -> Self {
		JsonRpcRequest::Notif(notif)
	}
}

/// Serializable [JSON-RPC object](https://www.jsonrpc.org/specification#request-object)
#[derive(Serialize, Debug)]
pub struct JsonRpcCall<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	#[serde(borrow)]
	pub method: &'a str,
	/// Request ID
	pub id: u64,
	/// Parameter values of the request.
	#[serde(borrow)]
	pub params: Option<&'a str>,
}

impl<'a> JsonRpcCall<'a> {
	/// Create a new serializable JSON-RPC request.
	pub fn new(id: u64, method: &'a str, params: Option<&'a str>) -> Self {
		Self { jsonrpc: TwoPointZero, id, method, params }
	}
}

/// Serializable [JSON-RPC notification object](https://www.jsonrpc.org/specification#request-object)
#[derive(Serialize, Debug)]
pub struct JsonRpcNotification<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	#[serde(borrow)]
	pub method: &'a str,
	/// Parameter values of the request.
	#[serde(borrow)]
	pub params: Option<&'a str>,
}

impl<'a> JsonRpcNotification<'a> {
	/// Create a new serializable JSON-RPC request.
	pub fn new(method: &'a str, params: Option<&'a str>) -> Self {
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
