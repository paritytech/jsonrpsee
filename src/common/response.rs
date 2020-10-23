// Copyright 2019 Parity Technologies (UK) Ltd.
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

use super::{Error, Id, JsonValue, Version};

use alloc::{
	string::{String, ToString as _},
	vec,
	vec::Vec,
};
use serde::{Deserialize, Serialize};

/// Synchronous response
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Response {
	/// Single response
	Single(Output),
	/// Response to batch request (batch of responses)
	Batch(Vec<Output>),
	/// Notification to an active subscription.
	Notif(SubscriptionNotif),
}

/// Successful response
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Success {
	/// Protocol version
	pub jsonrpc: Version,
	/// Result
	pub result: JsonValue,
	/// Correlation id
	pub id: Id,
}

/// Unsuccessful response
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Failure {
	/// Protocol version
	pub jsonrpc: Version,
	/// Error
	pub error: Error,
	/// Correlation id
	pub id: Id,
}

/// Represents output - failure or success
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Output {
	/// Success
	Success(Success),
	/// Failure
	Failure(Failure),
}

/// Server notification about something the client is subscribed to.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionNotif {
	/// Protocol version
	pub jsonrpc: Version,
	/// A String containing the name of the method that was used for the subscription.
	pub method: String,
	/// Parameters of the notification.
	pub params: SubscriptionNotifParams,
}

/// Field of a [`SubscriptionNotif`].
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionNotifParams {
	/// Subscription id, as communicated during the subscription.
	pub subscription: SubscriptionId,
	/// Actual data that the server wants to communicate to us.
	pub result: JsonValue,
}

/// Id of a subscription, communicated by the server.
#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum SubscriptionId {
	/// Numeric id
	Num(u64),
	/// String id
	Str(String),
}

impl Output {
	/// Creates new output given `Result`, `Id` and `Version`.
	pub fn from(result: Result<JsonValue, Error>, id: Id, jsonrpc: Version) -> Self {
		match result {
			Ok(result) => Output::Success(Success { id, jsonrpc, result }),
			Err(error) => Output::Failure(Failure { id, jsonrpc, error }),
		}
	}

	/// Get the jsonrpc protocol version.
	pub fn version(&self) -> Version {
		match *self {
			Output::Success(ref s) => s.jsonrpc,
			Output::Failure(ref f) => f.jsonrpc,
		}
	}

	/// Get the correlation id.
	pub fn id(&self) -> &Id {
		match *self {
			Output::Success(ref s) => &s.id,
			Output::Failure(ref f) => &f.id,
		}
	}
}

impl From<Output> for Result<JsonValue, Error> {
	/// Convert into a result. Will be `Ok` if it is a `Success` and `Err` if `Failure`.
	fn from(output: Output) -> Result<JsonValue, Error> {
		match output {
			Output::Success(s) => Ok(s.result),
			Output::Failure(f) => Err(f.error),
		}
	}
}

impl Response {
	/// Creates new `Response` with given error and `Version`
	pub fn from(error: impl Into<Error>, jsonrpc: Version) -> Self {
		Failure { id: Id::Null, jsonrpc, error: error.into() }.into()
	}

	/// Deserialize `Response` from given JSON string.
	///
	/// This method will handle an empty string as empty batch response.
	pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
		if s.is_empty() {
			Ok(Response::Batch(vec![]))
		} else {
			serde_json::from_str(s)
		}
	}
}

impl From<Failure> for Response {
	fn from(failure: Failure) -> Self {
		Response::Single(Output::Failure(failure))
	}
}

impl From<Success> for Response {
	fn from(success: Success) -> Self {
		Response::Single(Output::Success(success))
	}
}

impl SubscriptionId {
	/// Turns the subcription ID into a string.
	pub fn into_string(self) -> String {
		match self {
			SubscriptionId::Num(n) => n.to_string(),
			SubscriptionId::Str(s) => s,
		}
	}
}

#[test]
fn success_output_serialize() {
	use serde_json;
	use serde_json::Value;

	let so = Output::Success(Success { jsonrpc: Version::V2, result: Value::from(1), id: Id::Num(1) });

	let serialized = serde_json::to_string(&so).unwrap();
	assert_eq!(serialized, r#"{"jsonrpc":"2.0","result":1,"id":1}"#);
}

#[test]
fn success_output_deserialize() {
	use serde_json;
	use serde_json::Value;

	let dso = r#"{"jsonrpc":"2.0","result":1,"id":1}"#;

	let deserialized: Output = serde_json::from_str(dso).unwrap();
	assert_eq!(deserialized, Output::Success(Success { jsonrpc: Version::V2, result: Value::from(1), id: Id::Num(1) }));
}

#[test]
fn failure_output_serialize() {
	use serde_json;

	let fo = Output::Failure(Failure { jsonrpc: Version::V2, error: Error::parse_error(), id: Id::Num(1) });

	let serialized = serde_json::to_string(&fo).unwrap();
	assert_eq!(serialized, r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}"#);
}

#[test]
fn failure_output_deserialize() {
	use serde_json;

	let dfo = r#"{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}"#;

	let deserialized: Output = serde_json::from_str(dfo).unwrap();
	assert_eq!(
		deserialized,
		Output::Failure(Failure { jsonrpc: Version::V2, error: Error::parse_error(), id: Id::Num(1) })
	);
}

#[test]
fn single_response_deserialize() {
	use serde_json;
	use serde_json::Value;

	let dsr = r#"{"jsonrpc":"2.0","result":1,"id":1}"#;

	let deserialized: Response = serde_json::from_str(dsr).unwrap();
	assert_eq!(
		deserialized,
		Response::Single(Output::Success(Success { jsonrpc: Version::V2, result: Value::from(1), id: Id::Num(1) }))
	);
}

#[test]
fn batch_response_deserialize() {
	use serde_json;
	use serde_json::Value;

	let dbr = r#"[{"jsonrpc":"2.0","result":1,"id":1},{"jsonrpc":"2.0","error":{"code":-32700,"message":"Parse error"},"id":1}]"#;

	let deserialized: Response = serde_json::from_str(dbr).unwrap();
	assert_eq!(
		deserialized,
		Response::Batch(vec![
			Output::Success(Success { jsonrpc: Version::V2, result: Value::from(1), id: Id::Num(1) }),
			Output::Failure(Failure { jsonrpc: Version::V2, error: Error::parse_error(), id: Id::Num(1) })
		])
	);
}

#[test]
fn handle_incorrect_responses() {
	use serde_json;

	let dsr = r#"
{
	"id": 2,
	"jsonrpc": "2.0",
	"result": "0x62d3776be72cc7fa62cad6fe8ed873d9bc7ca2ee576e400d987419a3f21079d5",
	"error": {
		"message": "VM Exception while processing transaction: revert",
		"code": -32000,
		"data": {}
	}
}"#;

	let deserialized: Result<Response, _> = serde_json::from_str(dsr);
	assert!(deserialized.is_err(), "Expected error when deserializing invalid payload.");
}

#[test]
fn should_parse_empty_response_as_batch() {
	use serde_json;

	let dsr = r#""#;

	let deserialized1: Result<Response, _> = serde_json::from_str(dsr);
	let deserialized2: Result<Response, _> = Response::from_json(dsr);
	assert!(deserialized1.is_err(), "Empty string is not valid JSON, so we should get an error.");
	assert_eq!(deserialized2.unwrap(), Response::Batch(vec![]));
}
