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

//! Types to handle JSON-RPC requests according to the [spec](https://www.jsonrpc.org/specification#request-object).
//! Some types come with a "*Ser" variant that implements [`serde::Serialize`]; these are used in the client.

use std::borrow::Cow as StdCow;

use crate::params::{Id, TwoPointZero};
use beef::Cow;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

/// JSON-RPC request object as defined in the [spec](https://www.jsonrpc.org/specification#request-object).
#[derive(Serialize, Deserialize, Debug)]
pub struct Request<'a> {
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

impl<'a> Request<'a> {
	/// Create a new [`Request`].
	pub fn new(method: Cow<'a, str>, params: Option<&'a RawValue>, id: Id<'a>) -> Self {
		Self { jsonrpc: TwoPointZero, id, method, params }
	}
}

/// JSON-RPC Invalid request as defined in the [spec](https://www.jsonrpc.org/specification#request-object).
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct InvalidRequest<'a> {
	/// Request ID
	#[serde(borrow)]
	pub id: Id<'a>,
}

/// JSON-RPC notification (a request object without a request ID) as defined in the
/// [spec](https://www.jsonrpc.org/specification#request-object).
#[derive(Serialize, Deserialize, Debug)]
pub struct Notification<'a, T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	#[serde(borrow)]
	pub method: Cow<'a, str>,
	/// Parameter values of the request.
	pub params: T,
}

impl<'a, T> Notification<'a, T> {
	/// Create a new [`Notification`].
	pub fn new(method: Cow<'a, str>, params: T) -> Self {
		Self { jsonrpc: TwoPointZero, method, params }
	}
}

/// Serializable [JSON-RPC object](https://www.jsonrpc.org/specification#request-object).
#[derive(Serialize, Debug)]
pub struct RequestSer<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Request ID
	pub id: Id<'a>,
	/// Name of the method to be invoked.
	// NOTE: as this type only implements serialize `#[serde(borrow)]` is not needed.
	pub method: Cow<'a, str>,
	/// Parameter values of the request.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub params: Option<StdCow<'a, RawValue>>,
}

impl<'a> RequestSer<'a> {
	/// Create a borrowed serializable JSON-RPC method call.
	pub fn borrowed(id: &'a Id<'a>, method: &'a impl AsRef<str>, params: Option<&'a RawValue>) -> Self {
		Self {
			jsonrpc: TwoPointZero,
			id: id.clone(),
			method: method.as_ref().into(),
			params: params.map(StdCow::Borrowed),
		}
	}

	/// Create a owned serializable JSON-RPC method call.
	pub fn owned(id: Id<'a>, method: impl Into<String>, params: Option<Box<RawValue>>) -> Self {
		Self { jsonrpc: TwoPointZero, id, method: method.into().into(), params: params.map(StdCow::Owned) }
	}
}

/// Serializable [JSON-RPC notification object](https://www.jsonrpc.org/specification#request-object).
#[derive(Serialize, Debug)]
pub struct NotificationSer<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	// NOTE: as this type only implements serialize `#[serde(borrow)]` is not needed.
	pub method: Cow<'a, str>,
	/// Parameter values of the request.
	#[serde(skip_serializing_if = "Option::is_none")]
	pub params: Option<StdCow<'a, RawValue>>,
}

impl<'a> NotificationSer<'a> {
	/// Create a borrowed serializable JSON-RPC notification.
	pub fn borrowed(method: &'a impl AsRef<str>, params: Option<&'a RawValue>) -> Self {
		Self { jsonrpc: TwoPointZero, method: method.as_ref().into(), params: params.map(StdCow::Borrowed) }
	}

	/// Create an owned serializable JSON-RPC notification.
	pub fn owned(method: impl Into<String>, params: Option<Box<RawValue>>) -> Self {
		Self { jsonrpc: TwoPointZero, method: method.into().into(), params: params.map(StdCow::Owned) }
	}
}

#[cfg(test)]
mod test {
	use super::{Id, InvalidRequest, Notification, NotificationSer, Request, RequestSer, StdCow, TwoPointZero};
	use serde_json::value::RawValue;

	fn assert_request<'a>(request: Request<'a>, id: Id<'a>, method: &str, params: Option<&str>) {
		assert_eq!(request.jsonrpc, TwoPointZero);
		assert_eq!(request.id, id);
		assert_eq!(request.method, method);
		assert_eq!(request.params.map(RawValue::get), params);
	}

	/// Checks that we can deserialize the object with or without non-mandatory fields.
	#[test]
	fn deserialize_call() {
		let method = "subtract";
		let params = "[42, 23]";

		let test_vector = vec![
			// With all fields set.
			(
				r#"{"jsonrpc":"2.0", "method":"subtract", "params":[42, 23], "id":1}"#,
				Id::Number(1),
				Some(params),
				method,
			),
			// Without params field
			(r#"{"jsonrpc":"2.0", "method":"subtract", "id":null}"#, Id::Null, None, method),
			// Escaped method name.
			(r#"{"jsonrpc":"2.0", "method":"\"m", "id":null}"#, Id::Null, None, "\"m"),
		];

		for (ser, id, params, method) in test_vector.into_iter() {
			let request = serde_json::from_str(ser).unwrap();
			assert_request(request, id, method, params);
		}
	}

	#[test]
	fn deserialize_call_escaped_method_name() {
		let ser = r#"{"jsonrpc":"2.0","id":1,"method":"\"m\""}"#;
		let req: Request = serde_json::from_str(ser).unwrap();
		assert_request(req, Id::Number(1), "\"m\"", None);
	}

	#[test]
	fn deserialize_valid_notif_works() {
		let ser = r#"{"jsonrpc":"2.0","method":"say_hello","params":[]}"#;
		let dsr: Notification<&RawValue> = serde_json::from_str(ser).unwrap();
		assert_eq!(dsr.method, "say_hello");
		assert_eq!(dsr.jsonrpc, TwoPointZero);
	}

	#[test]
	fn deserialize_valid_notif_escaped_method() {
		let ser = r#"{"jsonrpc":"2.0","method":"\"m\"","params":[]}"#;
		let dsr: Notification<&RawValue> = serde_json::from_str(ser).unwrap();
		assert_eq!(dsr.method, "\"m\"");
		assert_eq!(dsr.jsonrpc, TwoPointZero);
	}

	#[test]
	fn deserialize_call_bad_id_should_fail() {
		let ser = r#"{"jsonrpc":"2.0","method":"say_hello","params":[],"id":{}}"#;
		assert!(serde_json::from_str::<Request>(ser).is_err());
	}

	#[test]
	fn deserialize_invalid_request() {
		let s = r#"{"id":120,"method":"my_method","params":["foo", "bar"],"extra_field":[]}"#;
		let deserialized: InvalidRequest = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized, InvalidRequest { id: Id::Number(120) });
	}

	/// Checks that we can serialize the object with or without non-mandatory fields.
	#[test]
	fn serialize_call() {
		let method = "subtract";
		let id = Id::Number(1); // It's enough to check one variant, since the type itself also has tests.
		let params = Some(RawValue::from_string("[42,23]".into()).unwrap());

		let test_vector: &[(&'static str, Option<_>, Option<_>, &'static str)] = &[
			// With all fields set.
			(
				r#"{"jsonrpc":"2.0","id":1,"method":"subtract","params":[42,23]}"#,
				Some(id.clone()),
				params.clone(),
				method,
			),
			// Escaped method name.
			(r#"{"jsonrpc":"2.0","id":1,"method":"\"m"}"#, Some(id.clone()), None, "\"m"),
			// Without ID field.
			(r#"{"jsonrpc":"2.0","id":null,"method":"subtract","params":[42,23]}"#, None, params, method),
			// Without params field
			(r#"{"jsonrpc":"2.0","id":1,"method":"subtract"}"#, Some(id), None, method),
			// Without params and ID.
			(r#"{"jsonrpc":"2.0","id":null,"method":"subtract"}"#, None, None, method),
		];

		for (ser, id, params, method) in test_vector.into_iter().cloned() {
			let request = serde_json::to_string(&RequestSer {
				jsonrpc: TwoPointZero,
				method: method.into(),
				id: id.unwrap_or(Id::Null),
				params: params.map(StdCow::Owned),
			})
			.unwrap();

			assert_eq!(&request, ser);
		}
	}

	#[test]
	fn serialize_notif() {
		let exp = r#"{"jsonrpc":"2.0","method":"say_hello","params":["hello"]}"#;
		let params = Some(RawValue::from_string(r#"["hello"]"#.into()).unwrap());
		let req = NotificationSer::owned("say_hello", params);
		let ser = serde_json::to_string(&req).unwrap();
		assert_eq!(exp, ser);
	}

	#[test]
	fn serialize_notif_escaped_method_name() {
		let exp = r#"{"jsonrpc":"2.0","method":"\"method\""}"#;
		let req = NotificationSer::owned("\"method\"", None);
		let ser = serde_json::to_string(&req).unwrap();
		assert_eq!(exp, ser);
	}
}
