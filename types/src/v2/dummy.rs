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

/// [Successful JSON-RPC object](https://www.jsonrpc.org/specification#response_object).
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

/// JSON-RPC notification response.
#[derive(Deserialize, Debug)]
pub struct JsonRpcResponseNotif<T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Params.
	pub params: JsonRpcNotificationParams<T>,
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

#[cfg(test)]
mod tests {

	#[test]
	fn deser_error() {}
}
