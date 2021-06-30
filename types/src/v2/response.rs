use crate::v2::params::{Id, TwoPointZero};
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

#[cfg(test)]
mod tests {
	use super::{Id, JsonRpcResponse, TwoPointZero};

	#[test]
	fn serialize_call_response() {
		let ser =
			serde_json::to_string(&JsonRpcResponse { jsonrpc: TwoPointZero, result: "ok", id: Id::Number(1) }).unwrap();
		let exp = r#"{"jsonrpc":"2.0","result":"ok","id":1}"#;
		assert_eq!(ser, exp);
	}

	#[test]
	fn deserialize_call() {
		let exp = JsonRpcResponse { jsonrpc: TwoPointZero, result: 99_u64, id: Id::Number(11) };
		let dsr: JsonRpcResponse<u64> = serde_json::from_str(r#"{"jsonrpc":"2.0", "result":99, "id":11}"#).unwrap();
		assert_eq!(dsr.jsonrpc, exp.jsonrpc);
		assert_eq!(dsr.result, exp.result);
		assert_eq!(dsr.id, exp.id);
	}
}
