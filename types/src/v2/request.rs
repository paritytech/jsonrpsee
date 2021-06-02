use crate::v2::params::{Id, JsonRpcParams, OwnedId, TwoPointZero};
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

/// Owned version of [`JsonRpcRequest`].
#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct OwnedJsonRpcRequest {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Request ID
	pub id: OwnedId,
	/// Name of the method to be invoked.
	pub method: String,
	/// Parameter values of the request.
	pub params: Option<String>,
}

impl OwnedJsonRpcRequest {
	/// Converts `OwnedJsonRpcRequest` into borrowed `JsonRpcRequest`.
	pub fn borrowed(&self) -> JsonRpcRequest<'_> {
		JsonRpcRequest {
			jsonrpc: self.jsonrpc,
			id: self.id.borrowed(),
			method: Cow::borrowed(self.method.as_ref()),
			params: self.params.as_ref().map(|s| {
				// Note: while this object *may* be created from something that is not a `JsonRpcRequest` object, using
				// an invalid field to construct it would be a logical invariant break.
				serde_json::from_str(&s)
					.expect("OwnedJsonRpcRequest is only created from JsonRpcRequest, so this conversion must be safe")
			}),
		}
	}
}

impl<'a> From<JsonRpcRequest<'a>> for OwnedJsonRpcRequest {
	fn from(borrowed: JsonRpcRequest<'a>) -> Self {
		Self {
			jsonrpc: borrowed.jsonrpc,
			id: borrowed.id.into(),
			method: borrowed.method.as_ref().to_owned(),
			params: borrowed.params.map(|p| p.get().to_owned()),
		}
	}
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
	use super::{
		Id, JsonRpcCallSer, JsonRpcInvalidRequest, JsonRpcNotification, JsonRpcNotificationSer, JsonRpcParams,
		JsonRpcRequest, TwoPointZero,
	};
	use serde_json::{value::RawValue, Value};

	fn assert_request<'a>(request: JsonRpcRequest<'a>, id: Id<'a>, method: &str, params: Option<&str>) {
		assert_eq!(request.jsonrpc, TwoPointZero);
		assert_eq!(request.id, id);
		assert_eq!(request.method, method);
		assert_eq!(request.params.map(RawValue::get), params);
	}

	/// Checks that we can deserialize the object with or without non-mandatory fields.
	#[test]
	fn deserialize_request() {
		let method = "subtract";
		let params = "[42, 23]";

		let test_vector = vec![
			// With all fields set.
			(r#"{"jsonrpc":"2.0", "method":"subtract", "params":[42, 23], "id":1}"#, Id::Number(1), Some(params)),
			// Without params field
			(r#"{"jsonrpc":"2.0", "method":"subtract", "id":null}"#, Id::Null, None),
		];

		for (ser, id, params) in test_vector.into_iter() {
			let request = serde_json::from_str(ser).unwrap();
			assert_request(request, id, method, params);
		}
	}

	#[test]
	fn deserialize_valid_notif_works() {
		let ser = r#"{"jsonrpc":"2.0","method":"say_hello","params":[]}"#;
		let dsr: JsonRpcNotification = serde_json::from_str(ser).unwrap();
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

	/// Checks that we can serialize the object with or without non-mandatory fields.
	#[test]
	fn serialize_call() {
		let method = "subtract";
		let id = Id::Number(1); // It's enough to check one variant, since the type itself also has tests.
		let params: JsonRpcParams = vec![Value::Number(42.into()), Value::Number(23.into())].into(); // Same as above.
		let test_vector = &[
			// With all fields set.
			(
				r#"{"jsonrpc":"2.0","method":"subtract","id":1,"params":[42,23]}"#,
				Some(id.clone()),
				Some(params.clone()),
			),
			// Without ID field.
			(r#"{"jsonrpc":"2.0","method":"subtract","id":null,"params":[42,23]}"#, None, Some(params)),
			// Without params field
			(r#"{"jsonrpc":"2.0","method":"subtract","id":1,"params":null}"#, Some(id), None),
			// Without params and ID.
			(r#"{"jsonrpc":"2.0","method":"subtract","id":null,"params":null}"#, None, None),
		];

		for (ser, id, params) in test_vector.iter().cloned() {
			let request = serde_json::to_string(&JsonRpcCallSer {
				jsonrpc: TwoPointZero,
				method,
				id: id.unwrap_or(Id::Null),
				params: params.unwrap_or(JsonRpcParams::NoParams),
			})
			.unwrap();

			assert_eq!(&request, ser);
		}
	}

	#[test]
	fn serialize_notif() {
		let exp = r#"{"jsonrpc":"2.0","method":"say_hello","params":["hello"]}"#;
		let req = JsonRpcNotificationSer::new("say_hello", vec!["hello".into()].into());
		let ser = serde_json::to_string(&req).unwrap();
		assert_eq!(exp, ser);
	}
}
