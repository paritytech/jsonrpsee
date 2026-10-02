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

use std::{borrow::Cow, collections::BTreeMap};

use crate::{
	Params,
	params::{Id, TwoPointZero},
};
use http::Extensions;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Value, value::RawValue};

const RESERVED_REQUEST_MEMBERS: &[&str] = &["jsonrpc", "id", "method", "params"];

/// Additional top-level members serialized with a JSON-RPC request or notification.
///
/// These members are stored in the request's [`Extensions`] and can be used by RPC middleware
/// to carry protocol extensions without adding them to the JSON-RPC `params` value. The reserved
/// JSON-RPC request members `jsonrpc`, `id`, `method`, and `params` cannot be inserted.
#[derive(Clone, Debug, Default)]
pub struct RequestExtensions {
	members: BTreeMap<String, Value>,
}

impl RequestExtensions {
	/// Inserts a request extension member.
	///
	/// Returns the previous value when the member was already present.
	pub fn insert(
		&mut self,
		name: impl Into<String>,
		value: Value,
	) -> Result<Option<Value>, InvalidRequestExtensionName> {
		let name = name.into();
		if RESERVED_REQUEST_MEMBERS.contains(&name.as_str()) {
			return Err(InvalidRequestExtensionName { name });
		}

		Ok(self.members.insert(name, value))
	}

	/// Returns a request extension member by name.
	pub fn get(&self, name: &str) -> Option<&Value> {
		self.members.get(name)
	}

	/// Removes and returns a request extension member by name.
	pub fn remove(&mut self, name: &str) -> Option<Value> {
		self.members.remove(name)
	}

	/// Returns an iterator over the request extension members.
	pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
		self.members.iter().map(|(name, value)| (name.as_str(), value))
	}

	/// Returns the number of request extension members.
	pub fn len(&self) -> usize {
		self.members.len()
	}

	/// Returns whether there are no request extension members.
	pub fn is_empty(&self) -> bool {
		self.members.is_empty()
	}
}

/// Error returned when a JSON-RPC request extension uses a reserved member name.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
#[error("`{name}` is a reserved JSON-RPC request member")]
pub struct InvalidRequestExtensionName {
	name: String,
}

impl InvalidRequestExtensionName {
	/// Returns the reserved member name.
	pub fn name(&self) -> &str {
		&self.name
	}
}

#[derive(Serialize)]
struct RequestRef<'a, 'request> {
	jsonrpc: &'a TwoPointZero,
	id: &'a Id<'request>,
	method: &'a Cow<'request, str>,
	#[serde(skip_serializing_if = "Option::is_none")]
	params: Option<&'a RawValue>,
	#[serde(flatten)]
	request_extensions: Option<&'a BTreeMap<String, Value>>,
}

#[derive(Deserialize)]
struct RequestDe<'a> {
	jsonrpc: TwoPointZero,
	#[serde(borrow)]
	id: Id<'a>,
	#[serde(borrow)]
	method: Cow<'a, str>,
	#[serde(borrow)]
	params: Option<Cow<'a, RawValue>>,
	#[serde(flatten)]
	request_extensions: BTreeMap<String, Value>,
}

/// JSON-RPC request object as defined in the [spec](https://www.jsonrpc.org/specification#request-object).
#[derive(Debug, Clone)]
pub struct Request<'a> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Request ID
	pub id: Id<'a>,
	/// Name of the method to be invoked.
	pub method: Cow<'a, str>,
	/// Parameter values of the request.
	pub params: Option<Cow<'a, RawValue>>,
	/// The request's extensions.
	pub extensions: Extensions,
}

impl<'a> Request<'a> {
	/// Create new borrowed [`Request`].
	pub fn borrowed(method: &'a str, params: Option<&'a RawValue>, id: Id<'a>) -> Self {
		Self {
			jsonrpc: TwoPointZero,
			id,
			method: Cow::Borrowed(method),
			params: params.map(Cow::Borrowed),
			extensions: Extensions::new(),
		}
	}

	/// Create new owned [`Request`].
	pub fn owned(method: String, params: Option<Box<RawValue>>, id: Id<'a>) -> Self {
		Self {
			jsonrpc: TwoPointZero,
			id,
			method: Cow::Owned(method),
			params: params.map(Cow::Owned),
			extensions: Extensions::new(),
		}
	}

	/// Get the ID of the request.
	pub fn id(&self) -> Id<'a> {
		self.id.clone()
	}

	/// Get the method name of the request.
	pub fn method_name(&self) -> &str {
		&self.method
	}

	/// Get the params of the request.
	pub fn params(&self) -> Params<'_> {
		Params::new(self.params.as_ref().map(|p| RawValue::get(p)))
	}

	/// Returns a reference to the associated extensions.
	pub fn extensions(&self) -> &Extensions {
		&self.extensions
	}

	/// Returns a reference to the associated extensions.
	pub fn extensions_mut(&mut self) -> &mut Extensions {
		&mut self.extensions
	}

	/// Returns the additional top-level JSON-RPC request members, when present.
	pub fn request_extensions(&self) -> Option<&RequestExtensions> {
		self.extensions.get()
	}

	/// Returns the additional top-level JSON-RPC request members for mutation.
	pub fn request_extensions_mut(&mut self) -> &mut RequestExtensions {
		self.extensions.get_or_insert_default()
	}
}

impl Serialize for Request<'_> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		RequestRef {
			jsonrpc: &self.jsonrpc,
			id: &self.id,
			method: &self.method,
			params: self.params.as_deref(),
			request_extensions: self.request_extensions().map(|extensions| &extensions.members),
		}
		.serialize(serializer)
	}
}

impl<'de: 'a, 'a> Deserialize<'de> for Request<'a> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let request = RequestDe::deserialize(deserializer)?;
		let mut extensions = Extensions::new();
		if !request.request_extensions.is_empty() {
			extensions.insert(RequestExtensions { members: request.request_extensions });
		}

		Ok(Self {
			jsonrpc: request.jsonrpc,
			id: request.id,
			method: request.method,
			params: request.params,
			extensions,
		})
	}
}

/// JSON-RPC Invalid request as defined in the [spec](https://www.jsonrpc.org/specification#request-object).
#[derive(Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct InvalidRequest<'a> {
	/// Request ID
	#[serde(borrow)]
	pub id: Id<'a>,
}

/// JSON-RPC notification (a request object without a request ID) as defined in the
/// [spec](https://www.jsonrpc.org/specification#request-object).
#[derive(Debug, Clone)]
pub struct Notification<'a, T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Name of the method to be invoked.
	pub method: Cow<'a, str>,
	/// Parameter values of the request.
	pub params: T,
	/// Extensions of the notification.
	pub extensions: Extensions,
}

impl<'a, T> Notification<'a, T> {
	/// Create a new [`Notification`].
	pub fn new(method: Cow<'a, str>, params: T) -> Self {
		Self { jsonrpc: TwoPointZero, method, params, extensions: Extensions::new() }
	}

	/// Get the method name of the request.
	pub fn method_name(&self) -> &str {
		&self.method
	}

	/// Returns a reference to the associated extensions.
	pub fn extensions(&self) -> &Extensions {
		&self.extensions
	}

	/// Get the params of the request.
	pub fn params(&self) -> &T {
		&self.params
	}

	/// Returns a reference to the associated extensions.
	pub fn extensions_mut(&mut self) -> &mut Extensions {
		&mut self.extensions
	}

	/// Returns the additional top-level JSON-RPC request members, when present.
	pub fn request_extensions(&self) -> Option<&RequestExtensions> {
		self.extensions.get()
	}

	/// Returns the additional top-level JSON-RPC request members for mutation.
	pub fn request_extensions_mut(&mut self) -> &mut RequestExtensions {
		self.extensions.get_or_insert_default()
	}
}

#[derive(Serialize)]
struct NotificationRef<'a, 'request, T> {
	jsonrpc: &'a TwoPointZero,
	method: &'a Cow<'request, str>,
	params: &'a T,
	#[serde(flatten)]
	request_extensions: Option<&'a BTreeMap<String, Value>>,
}

#[derive(Deserialize)]
struct NotificationDe<'a, T> {
	jsonrpc: TwoPointZero,
	#[serde(borrow)]
	method: Cow<'a, str>,
	params: T,
	#[serde(flatten)]
	request_extensions: BTreeMap<String, Value>,
}

impl<T> Serialize for Notification<'_, T>
where
	T: Serialize,
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		NotificationRef {
			jsonrpc: &self.jsonrpc,
			method: &self.method,
			params: &self.params,
			request_extensions: self.request_extensions().map(|extensions| &extensions.members),
		}
		.serialize(serializer)
	}
}

impl<'de: 'a, 'a, T> Deserialize<'de> for Notification<'a, T>
where
	T: Deserialize<'de>,
{
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let notification = NotificationDe::deserialize(deserializer)?;
		let mut extensions = Extensions::new();
		if !notification.request_extensions.is_empty() {
			extensions.insert(RequestExtensions { members: notification.request_extensions });
		}

		Ok(Self { jsonrpc: notification.jsonrpc, method: notification.method, params: notification.params, extensions })
	}
}

#[cfg(test)]
mod test {
	use super::{Id, InvalidRequest, Notification, Request, TwoPointZero};
	use serde_json::{Value, value::RawValue};

	fn assert_request<'a>(request: Request<'a>, id: Id<'a>, method: &str, params: Option<&str>) {
		assert_eq!(request.jsonrpc, TwoPointZero);
		assert_eq!(request.id, id);
		assert_eq!(request.method, method);
		assert_eq!(request.params.as_ref().map(|p| RawValue::get(p)), params);
		assert!(request.request_extensions().is_none());
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
	fn deserialize_call_with_request_extensions() {
		let serialized = r#"{"jsonrpc":"2.0","method":"subtract","params":[42,23],"id":1,"traceparent":"00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01","vendor":{"sampled":true}}"#;
		let request: Request = serde_json::from_str(serialized).unwrap();
		let extensions = request.request_extensions().unwrap();

		assert_eq!(
			extensions.get("traceparent").and_then(Value::as_str),
			Some("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01")
		);
		assert_eq!(extensions.get("vendor").and_then(|value| value.get("sampled")), Some(&Value::Bool(true)));
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
	fn deserialize_notification_with_request_extensions() {
		let serialized = r#"{"jsonrpc":"2.0","method":"say_hello","params":[],"traceparent":"00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01","vendor":{"sampled":true}}"#;
		let notification: Notification<&RawValue> = serde_json::from_str(serialized).unwrap();
		let extensions = notification.request_extensions().unwrap();

		assert_eq!(
			extensions.get("traceparent").and_then(Value::as_str),
			Some("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01")
		);
		assert_eq!(extensions.get("vendor").and_then(|value| value.get("sampled")), Some(&Value::Bool(true)));
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
		let params = Some(serde_json::value::to_raw_value(&[42, 23]).unwrap());

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

		for (ser, id, params, method) in test_vector.iter().cloned() {
			let request =
				serde_json::to_string(&Request::borrowed(method, params.as_deref(), id.unwrap_or(Id::Null))).unwrap();

			assert_eq!(&request, ser);
		}
	}

	#[test]
	fn serialize_call_with_request_extensions() {
		let mut request = Request::borrowed("subtract", None, Id::Number(1));
		request
			.request_extensions_mut()
			.insert("traceparent", Value::String("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01".into()))
			.unwrap();
		request.request_extensions_mut().insert("vendor", serde_json::json!({ "sampled": true })).unwrap();

		let serialized = serde_json::to_string(&request).unwrap();

		assert_eq!(
			serialized,
			r#"{"jsonrpc":"2.0","id":1,"method":"subtract","traceparent":"00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01","vendor":{"sampled":true}}"#
		);
	}

	#[test]
	fn serialize_notification_with_request_extensions() {
		let params = serde_json::value::to_raw_value(&Vec::<u8>::new()).unwrap();
		let mut notification = Notification::new("say_hello".into(), params.as_ref());
		notification
			.request_extensions_mut()
			.insert("traceparent", Value::String("00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01".into()))
			.unwrap();
		notification.request_extensions_mut().insert("vendor", serde_json::json!({ "sampled": true })).unwrap();

		let serialized = serde_json::to_string(&notification).unwrap();

		assert_eq!(
			serialized,
			r#"{"jsonrpc":"2.0","method":"say_hello","params":[],"traceparent":"00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01","vendor":{"sampled":true}}"#
		);
	}

	#[test]
	fn reserved_request_extension_names_are_rejected() {
		let mut request = Request::borrowed("subtract", None, Id::Number(1));

		for name in ["jsonrpc", "id", "method", "params"] {
			let error = request.request_extensions_mut().insert(name, Value::Null).unwrap_err();
			assert_eq!(error.name(), name);
		}
	}
}
