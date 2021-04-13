//! Client side JSON-RPC types.

use crate::v2::TwoPointZero;
use alloc::collections::BTreeMap;
use serde::{Deserialize, Serialize};

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
	Array(Vec<&'a T>),
	/// Params by name.
	Map(BTreeMap<&'a str, &'a T>),
}

// FIXME: this is a little weird but nice if `None.into()` works.
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

impl<'a, T> From<Vec<&'a T>> for JsonRpcParams<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	fn from(arr: Vec<&'a T>) -> Self {
		Self::Array(arr)
	}
}

/// Serializable JSON-RPC request object which may be a notification, method call or batch.
#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum JsonRpcRequest<'a, T>
where
	T: Serialize + std::fmt::Debug + 'a,
{
	/// Single method call.
	Single(JsonRpcCall<'a, T>),
	/// Batch.
	Batch(Vec<JsonRpcCall<'a, T>>),
	/// Notification.
	Notif(JsonRpcNotification<'a, T>),
}

impl<'a, T> From<JsonRpcCall<'a, T>> for JsonRpcRequest<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	fn from(call: JsonRpcCall<'a, T>) -> Self {
		JsonRpcRequest::Single(call)
	}
}
impl<'a, T> From<Vec<JsonRpcCall<'a, T>>> for JsonRpcRequest<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	fn from(batch: Vec<JsonRpcCall<'a, T>>) -> Self {
		JsonRpcRequest::Batch(batch)
	}
}

impl<'a, T> From<JsonRpcNotification<'a, T>> for JsonRpcRequest<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	fn from(notif: JsonRpcNotification<'a, T>) -> Self {
		JsonRpcRequest::Notif(notif)
	}
}

/// Serializable [JSON-RPC object](https://www.jsonrpc.org/specification#request-object)
#[derive(Serialize, Debug)]
pub struct JsonRpcCall<'a, T>
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

impl<'a, T> JsonRpcCall<'a, T>
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
pub struct JsonRpcNotification<'a, T>
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

impl<'a, T> JsonRpcNotification<'a, T>
where
	T: Serialize + std::fmt::Debug,
{
	/// Create a new serializable JSON-RPC request.
	pub fn new(method: &'a str, params: JsonRpcParams<'a, T>) -> Self {
		Self { jsonrpc: TwoPointZero, method, params }
	}
}

#[derive(Deserialize, Debug)]
pub struct JsonRpcResponseObject<T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Result.
	pub result: T,
	/// Request ID
	pub id: u64,
}

/// JSON-RPC parameter values for subscriptions.
#[derive(Deserialize, Debug)]
pub struct JsonRpcNotificationParams<T> {
	/// Subscription ID
	pub subscription: SubscriptionId,
	/// Result.
	pub result: T,
}

#[derive(Deserialize, Debug)]
pub struct JsonRpcResponseNotif<T> {
	pub jsonrpc: TwoPointZero,
	pub params: JsonRpcNotificationParams<T>,
}

/// Represent the different JSON-RPC responses.
#[derive(Deserialize, Debug)]
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
