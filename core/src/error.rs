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

use serde::Serialize;
use serde_json::value::RawValue;

#[derive(Debug)]
pub(crate) enum InnerSubscriptionErr {
	String(String),
	Json(Box<RawValue>),
}

/// Error returned when a subscription fails.
///
/// It's recommended to use `SubscriptionErr::from_json` to create a new instance of this error
/// because using the `String` representation may not very ergonomic for clients to parse.
#[derive(Debug)]
pub struct SubscriptionErr(pub(crate) InnerSubscriptionErr);

impl Serialize for SubscriptionErr {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		match &self.0 {
			InnerSubscriptionErr::String(s) => serializer.serialize_str(s),
			InnerSubscriptionErr::Json(json) => json.serialize(serializer),
		}
	}
}

impl<T: ToString> From<T> for SubscriptionErr {
	fn from(val: T) -> Self {
		Self(InnerSubscriptionErr::String(val.to_string()))
	}
}

impl SubscriptionErr {
	/// Create a new `SubscriptionErr` from a JSON value.
	pub fn from_json(t: &impl Serialize) -> Result<Self, serde_json::Error> {
		let json = serde_json::value::to_raw_value(t)?;
		Ok(Self(InnerSubscriptionErr::Json(json)))
	}

	/// Create a new `SubscriptionErr` from a String.
	pub fn from_string(s: String) -> Self {
		SubscriptionErr::from(s)
	}
}

/// The error returned when registering a method or subscription failed.
#[derive(Debug, Clone, thiserror::Error)]
pub enum RegisterMethodError {
	/// Method was already registered.
	#[error("Method: {0} was already registered")]
	AlreadyRegistered(String),
	/// Subscribe and unsubscribe method names are the same.
	#[error("Cannot use the same method name for subscribe and unsubscribe, used: {0}")]
	SubscriptionNameConflict(String),
	/// Method with that name has not yet been registered.
	#[error("Method: {0} has not yet been registered")]
	MethodNotFound(String),
}
