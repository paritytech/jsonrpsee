use crate::v2::params::{JsonRpcNotificationParamsAlloc, TwoPointZero};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

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

/// JSON-RPC subscription response.
#[derive(Deserialize, Debug)]
pub struct JsonRpcSubscriptionResponse<T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Params.
	pub params: JsonRpcNotificationParamsAlloc<T>,
}

/// JSON-RPC notification response.
#[derive(Deserialize, Debug)]
pub struct JsonRpcNotifResponse<'a, T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Method
	pub method: &'a str,
	/// Params.
	pub params: T,
}
