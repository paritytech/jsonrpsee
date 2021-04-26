use crate::v2::params::{Id, JsonRpcNotificationParams, JsonRpcParams, TwoPointZero};
use beef::Cow;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

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

// /// TODO: docs
// #[derive(Deserialize, Debug)]
// pub struct JsonRpcBatchRequest<'a>(
// 	#[serde(borrow)]
// 	pub
// 	Vec<JsonRpcRequest<'a>>
// );

/// TODO: docs
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum SingleOrBatch<'a> {
	Single(#[serde(borrow)] JsonRpcRequest<'a>),
	Batch(#[serde(borrow)] Vec<JsonRpcRequest<'a>>),
	// Batch(JsonRpcBatchRequest<'a>),
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

/// Serializable [JSON-RPC object](https://www.jsonrpc.org/specification#request-object)
#[derive(Serialize, Debug)]
pub struct JsonRpcCallSer<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	pub method: &'a str,
	/// Request ID
	pub id: Id,
	/// Parameter values of the request.
	pub params: JsonRpcParams<'a>,
}

impl<'a> JsonRpcCallSer<'a> {
	/// Create a new serializable JSON-RPC request.
	pub fn new(id: Id, method: &'a str, params: JsonRpcParams<'a>) -> Self {
		Self { jsonrpc: TwoPointZero, id, method, params }
	}
}

/// Serializable [JSON-RPC notification object](https://www.jsonrpc.org/specification#request-object)
#[derive(Serialize, Debug)]
pub struct JsonRpcNotificationSer<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	pub method: &'a str,
	/// Parameter values of the request.
	pub params: JsonRpcParams<'a>,
}

impl<'a> JsonRpcNotificationSer<'a> {
	/// Create a new serializable JSON-RPC request.
	pub fn new(method: &'a str, params: JsonRpcParams<'a>) -> Self {
		Self { jsonrpc: TwoPointZero, method, params }
	}
}
