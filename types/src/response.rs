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

//! Types pertaining to JSON-RPC responses.

use std::fmt;

use crate::params::{Id, SubscriptionId, TwoPointZero};
use crate::request::Notification;
use serde::{Deserialize, Serialize};

/// JSON-RPC successful response object as defined in the [spec](https://www.jsonrpc.org/specification#response_object).
#[derive(Serialize, Deserialize, Debug)]
pub struct Response<'a, T> {
	/// JSON-RPC version.
	pub jsonrpc: TwoPointZero,
	/// Result.
	pub result: T,
	/// Request ID
	#[serde(borrow)]
	pub id: Id<'a>,
}

impl<'a, T> Response<'a, T> {
	/// Create a new [`Response`].
	pub fn new(result: T, id: Id<'a>) -> Response<'a, T> {
		Response { jsonrpc: TwoPointZero, result, id }
	}

	/// Create an owned [`Response`].
	pub fn into_owned(self) -> Response<'static, T> {
		Response { jsonrpc: self.jsonrpc, result: self.result, id: self.id.into_owned() }
	}
}

impl<'a, T: Serialize> fmt::Display for Response<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", serde_json::to_string(&self).expect("valid JSON; qed"))
	}
}

/// Return value for subscriptions.
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionPayload<'a, T> {
	/// Subscription ID
	#[serde(borrow)]
	pub subscription: SubscriptionId<'a>,
	/// Result.
	pub result: T,
}

/// Subscription response object, embedding a [`SubscriptionPayload`] in the `params` member along with `result` field.
pub type SubscriptionResponse<'a, T> = Notification<'a, SubscriptionPayload<'a, T>>;
/// Subscription response object, embedding a [`SubscriptionPayload`] in the `params` member along with `error` field.
pub type SubscriptionError<'a, T> = Notification<'a, SubscriptionPayloadError<'a, T>>;

/// Error value for subscriptions.
#[derive(Serialize, Deserialize, Debug)]
pub struct SubscriptionPayloadError<'a, T> {
	/// Subscription ID
	#[serde(borrow)]
	pub subscription: SubscriptionId<'a>,
	/// Result.
	pub error: T,
}

#[cfg(test)]
mod tests {
	use super::{Id, Response, TwoPointZero};

	#[test]
	fn serialize_call_response() {
		let ser = serde_json::to_string(&Response { jsonrpc: TwoPointZero, result: "ok", id: Id::Number(1) }).unwrap();
		let exp = r#"{"jsonrpc":"2.0","result":"ok","id":1}"#;
		assert_eq!(ser, exp);
	}

	#[test]
	fn deserialize_call() {
		let exp = Response { jsonrpc: TwoPointZero, result: 99_u64, id: Id::Number(11) };
		let dsr: Response<u64> = serde_json::from_str(r#"{"jsonrpc":"2.0", "result":99, "id":11}"#).unwrap();
		assert_eq!(dsr.jsonrpc, exp.jsonrpc);
		assert_eq!(dsr.result, exp.result);
		assert_eq!(dsr.id, exp.id);
	}
}
