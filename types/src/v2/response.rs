use crate::v2::params::{Id, JsonRpcNotificationParams, JsonRpcNotificationParamsAlloc, TwoPointZero};
use serde::{Deserialize, Serialize};

/// JSON-RPC successful response object.
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcResponse<'a, T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Result.
	pub result: T,
	/// Request ID
	#[serde(borrow)]
	pub id: Id<'a>,
}

/// JSON-RPC subscription response.
#[derive(Serialize, Debug)]
pub struct JsonRpcSubscriptionResponse<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Method
	pub method: &'a str,
	/// Params.
	pub params: JsonRpcNotificationParams<'a>,
}

/// JSON-RPC subscription response.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcSubscriptionResponseAlloc<'a, T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Method
	pub method: &'a str,
	/// Params.
	pub params: JsonRpcNotificationParamsAlloc<T>,
}

/// JSON-RPC notification response.
#[derive(Deserialize, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcNotifResponse<'a, T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Method
	pub method: &'a str,
	/// Params.
	pub params: T,
}
