//! jsonrpc request

use serde_derive::{Serialize, Deserialize};
use super::{Id, Params, Version};

/// Represents jsonrpc request which is a method call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MethodCall {
	/// A String specifying the version of the JSON-RPC protocol.
	pub jsonrpc: Option<Version>,
	/// A String containing the name of the method to be invoked.
	pub method: String,
	/// A Structured value that holds the parameter values to be used
	/// during the invocation of the method. This member MAY be omitted.
	#[serde(default = "default_params")]
	pub params: Params,
	/// An identifier established by the Client that MUST contain a String,
	/// Number, or NULL value if included. If it is not included it is assumed
	/// to be a notification.
	pub id: Id,
}

/// Represents jsonrpc request which is a notification.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Notification {
	/// A String specifying the version of the JSON-RPC protocol.
	pub jsonrpc: Option<Version>,
	/// A String containing the name of the method to be invoked.
	pub method: String,
	/// A Structured value that holds the parameter values to be used
	/// during the invocation of the method. This member MAY be omitted.
	#[serde(default = "default_params")]
	pub params: Params,
}

/// Represents single jsonrpc call.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Call {
	/// Call method
	MethodCall(MethodCall),
	/// Fire notification
	Notification(Notification),
	/// Invalid call
	Invalid {
		/// Call id (if known)
		#[serde(default = "default_id")]
		id: Id,
	},
}

fn default_params() -> Params {
	Params::None
}

fn default_id() -> Id {
	Id::Null
}

impl From<MethodCall> for Call {
	fn from(mc: MethodCall) -> Self {
		Call::MethodCall(mc)
	}
}

impl From<Notification> for Call {
	fn from(n: Notification) -> Self {
		Call::Notification(n)
	}
}

/// Represents jsonrpc request.
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Request {
	/// Single request (call)
	Single(Call),
	/// Batch of requests (calls)
	Batch(Vec<Call>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::Value;

	#[test]
	fn method_call_serialize() {
		use serde_json;
		use serde_json::Value;

		let m = MethodCall {
			jsonrpc: Some(Version::V2),
			method: "update".to_owned(),
			params: Params::Array(vec![Value::from(1), Value::from(2)]),
			id: Id::Num(1),
		};

		let serialized = serde_json::to_string(&m).unwrap();
		assert_eq!(
			serialized,
			r#"{"jsonrpc":"2.0","method":"update","params":[1,2],"id":1}"#
		);
	}

	#[test]
	fn notification_serialize() {
		use serde_json;
		use serde_json::Value;

		let n = Notification {
			jsonrpc: Some(Version::V2),
			method: "update".to_owned(),
			params: Params::Array(vec![Value::from(1), Value::from(2)]),
		};

		let serialized = serde_json::to_string(&n).unwrap();
		assert_eq!(serialized, r#"{"jsonrpc":"2.0","method":"update","params":[1,2]}"#);
	}

	#[test]
	fn call_serialize() {
		use serde_json;
		use serde_json::Value;

		let n = Call::Notification(Notification {
			jsonrpc: Some(Version::V2),
			method: "update".to_owned(),
			params: Params::Array(vec![Value::from(1)]),
		});

		let serialized = serde_json::to_string(&n).unwrap();
		assert_eq!(serialized, r#"{"jsonrpc":"2.0","method":"update","params":[1]}"#);
	}

	#[test]
	fn request_serialize_batch() {
		use serde_json;

		let batch = Request::Batch(vec![
			Call::MethodCall(MethodCall {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![Value::from(1), Value::from(2)]),
				id: Id::Num(1),
			}),
			Call::Notification(Notification {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![Value::from(1)]),
			}),
		]);

		let serialized = serde_json::to_string(&batch).unwrap();
		assert_eq!(serialized, r#"[{"jsonrpc":"2.0","method":"update","params":[1,2],"id":1},{"jsonrpc":"2.0","method":"update","params":[1]}]"#);
	}

	#[test]
	fn notification_deserialize() {
		use serde_json;
		use serde_json::Value;

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": [1,2]}"#;
		let deserialized: Notification = serde_json::from_str(s).unwrap();

		assert_eq!(
			deserialized,
			Notification {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![Value::from(1), Value::from(2)])
			}
		);

		let s = r#"{"jsonrpc": "2.0", "method": "foobar"}"#;
		let deserialized: Notification = serde_json::from_str(s).unwrap();

		assert_eq!(
			deserialized,
			Notification {
				jsonrpc: Some(Version::V2),
				method: "foobar".to_owned(),
				params: Params::None,
			}
		);

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": [1,2], "id": 1}"#;
		let deserialized: Result<Notification, _> = serde_json::from_str(s);
		assert!(deserialized.is_err());
	}

	#[test]
	fn call_deserialize() {
		use serde_json;

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": [1]}"#;
		let deserialized: Call = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Call::Notification(Notification {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![Value::from(1)])
			})
		);

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": [1], "id": 1}"#;
		let deserialized: Call = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Call::MethodCall(MethodCall {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![Value::from(1)]),
				id: Id::Num(1)
			})
		);

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": [], "id": 1}"#;
		let deserialized: Call = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Call::MethodCall(MethodCall {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::Array(vec![]),
				id: Id::Num(1)
			})
		);

		let s = r#"{"jsonrpc": "2.0", "method": "update", "params": null, "id": 1}"#;
		let deserialized: Call = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Call::MethodCall(MethodCall {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::None,
				id: Id::Num(1)
			})
		);

		let s = r#"{"jsonrpc": "2.0", "method": "update", "id": 1}"#;
		let deserialized: Call = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Call::MethodCall(MethodCall {
				jsonrpc: Some(Version::V2),
				method: "update".to_owned(),
				params: Params::None,
				id: Id::Num(1)
			})
		);
	}

	#[test]
	fn request_deserialize_batch() {
		use serde_json;

		let s = r#"[{}, {"jsonrpc": "2.0", "method": "update", "params": [1,2], "id": 1},{"jsonrpc": "2.0", "method": "update", "params": [1]}]"#;
		let deserialized: Request = serde_json::from_str(s).unwrap();
		assert_eq!(
			deserialized,
			Request::Batch(vec![
				Call::Invalid { id: Id::Null },
				Call::MethodCall(MethodCall {
					jsonrpc: Some(Version::V2),
					method: "update".to_owned(),
					params: Params::Array(vec![Value::from(1), Value::from(2)]),
					id: Id::Num(1)
				}),
				Call::Notification(Notification {
					jsonrpc: Some(Version::V2),
					method: "update".to_owned(),
					params: Params::Array(vec![Value::from(1)])
				})
			])
		)
	}

	#[test]
	fn request_invalid_returns_id() {
		use serde_json;

		let s = r#"{"id":120,"method":"my_method","params":["foo", "bar"],"extra_field":[]}"#;
		let deserialized: Request = serde_json::from_str(s).unwrap();

		match deserialized {
			Request::Single(Call::Invalid { id: Id::Num(120) }) => {}
			_ => panic!("Request wrongly deserialized: {:?}", deserialized),
		}
	}
}
