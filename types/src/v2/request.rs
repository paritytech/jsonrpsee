use crate::v2::params::{Id, JsonRpcParams, TwoPointZero};
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
	pub id: Id<'a>,
	/// Name of the method to be invoked.
	#[serde(borrow)]
	pub method: Cow<'a, str>,
	/// Parameter values of the request.
	#[serde(borrow)]
	pub params: Option<&'a RawValue>,
}

/// Invalid request with known request ID.
#[derive(Deserialize, Debug, PartialEq)]
pub struct JsonRpcInvalidRequest<'a> {
	/// Request ID
	#[serde(borrow)]
	pub id: Id<'a>,
}

/// JSON-RPC notification (a request object without a request ID).
#[derive(Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct JsonRpcNotification<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	pub method: &'a str,
	/// Parameter values of the request.
	#[serde(borrow)]
	pub params: Option<&'a RawValue>,
}

/// Serializable [JSON-RPC object](https://www.jsonrpc.org/specification#request-object)
#[derive(Serialize, Debug)]
pub struct JsonRpcCallSer<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	pub method: &'a str,
	/// Request ID
	pub id: Id<'a>,
	/// Parameter values of the request.
	pub params: JsonRpcParams<'a>,
}

impl<'a> JsonRpcCallSer<'a> {
	/// Create a new serializable JSON-RPC request.
	pub fn new(id: Id<'a>, method: &'a str, params: JsonRpcParams<'a>) -> Self {
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

#[cfg(test)]
mod test {
	use super::{JsonRpcRequest, JsonRpcNotification, JsonRpcInvalidRequest, TwoPointZero, Id,
		JsonRpcCallSer, JsonRpcNotificationSer
	};

	#[test]
	fn deserialize_valid_call_works() {
		let ser = r#"{"jsonrpc":"2.0","method":"say_hello","params":[1,"bar"],"id":1}"#;
		let dsr: JsonRpcRequest = serde_json::from_str(ser).unwrap();
		assert_eq!(dsr.method, "say_hello");
		assert_eq!(dsr.jsonrpc, TwoPointZero);
	}

	#[test]
	fn deserialize_valid_notif_works() {
		let ser = r#"{"jsonrpc":"2.0","method":"say_hello","params":[]}"#;
		let dsr: JsonRpcNotification = serde_json::from_str(ser).unwrap();
		assert_eq!(dsr.method, "say_hello");
		assert_eq!(dsr.jsonrpc, TwoPointZero);
	}

	#[test]
	fn deserialize_valid_call_without_params_works() {
		let ser = r#"{"jsonrpc":"2.0","method":"say_hello", "id":1}"#;
		let dsr: JsonRpcRequest = serde_json::from_str(ser).unwrap();
		assert_eq!(dsr.method, "say_hello");
		assert_eq!(dsr.jsonrpc, TwoPointZero);
	}

	// TODO(niklasad1): merge the types `JsonRpcParams` and `RpcParams` and remove `RawValue`.
	#[test]
	#[ignore]
	fn deserialize_call_bad_params_should_fail() {
		let ser = r#"{"jsonrpc":"2.0","method":"say_hello","params":"lol","id":1}"#;
		assert!(serde_json::from_str::<JsonRpcRequest>(ser).is_err());
	}

	#[test]
	fn deserialize_call_bad_id_should_fail() {
		let ser = r#"{"jsonrpc":"2.0","method":"say_hello","params":[],"id":{}}"#;
		assert!(serde_json::from_str::<JsonRpcRequest>(ser).is_err());
	}

	#[test]
	fn deserialize_invalid_request() {
		let s = r#"{"id":120,"method":"my_method","params":["foo", "bar"],"extra_field":[]}"#;
		let deserialized: JsonRpcInvalidRequest = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized, JsonRpcInvalidRequest { id: Id::Number(120) });
	}

	#[test]
	fn serialize_call() {
		let exp = r#"{"jsonrpc":"2.0","method":"say_hello","id":"bar","params":[]}"#;
		let req = JsonRpcCallSer::new(Id::Str("bar".into()), "say_hello", vec![].into());
		let ser = serde_json::to_string(&req).unwrap();
		assert_eq!(exp, ser);
	}

	#[test]
	fn serialize_notif() {
		let exp = r#"{"jsonrpc":"2.0","method":"say_hello","params":["hello"]}"#;
		let req = JsonRpcNotificationSer::new("say_hello", vec!["hello".into()].into());
		let ser = serde_json::to_string(&req).unwrap();
		assert_eq!(exp, ser);
	}
}
