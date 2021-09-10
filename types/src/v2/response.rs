// Copyright 2019-2021 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::v2::params::{Id, TwoPointZero};
use serde::{Deserialize, Serialize};

/// JSON-RPC successful response object.
#[derive(Serialize, Deserialize, Debug, Clone)]
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
