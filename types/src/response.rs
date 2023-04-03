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
use std::marker::PhantomData;

use crate::params::{Id, SubscriptionId, TwoPointZero};
use crate::request::Notification;
use crate::ErrorObjectOwned;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// JSON-RPC response object as defined in the [spec](https://www.jsonrpc.org/specification#response_object).
#[derive(Debug)]
pub struct Response<'a, T> {
	/// JSON-RPC version.
	pub jsonrpc: Option<TwoPointZero>,
	/// Result or error.
	pub result_or_error: PartialResponse<T>,
	/// Request ID
	pub id: Id<'a>,
}

impl<'a, T> Response<'a, T> {
	/// Create a new [`Response`].
	pub fn new(result_or_error: PartialResponse<T>, id: Id<'a>) -> Response<'a, T> {
		Response { jsonrpc: Some(TwoPointZero), result_or_error, id }
	}

	/// Create an owned [`Response`].
	pub fn into_owned(self) -> Response<'static, T> {
		Response { jsonrpc: self.jsonrpc, result_or_error: self.result_or_error, id: self.id.into_owned() }
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

/// Represent the result or error field of the JSON-RPC response object
///
/// It can be:
///
/// ```json
/// "result":<value>
/// "error":{"code":<code>,"message":<msg>,"data":<data>}
/// ```
#[derive(Debug, PartialEq)]
pub enum PartialResponse<T> {
	/// Corresponds to successful JSON-RPC response with the field `result`.
	Result(T),
	/// Corresponds to failed JSON-RPC response with a error object with the field `error.
	Error(ErrorObjectOwned),
}

impl<T> PartialResponse<T> {
	/// Create successful partial response i.e, the `result field`
	pub fn result(t: T) -> Self {
		Self::Result(t)
	}
}

impl PartialResponse<()> {
	/// Create successful partial response i.e, the `result field`
	pub fn error(e: impl Into<ErrorObjectOwned>) -> Self {
		Self::Error(e.into())
	}
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Response<'de, T> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		enum Field {
			Jsonrpc,
			Result,
			Error,
			Id,
		}

		impl<'de> Deserialize<'de> for Field {
			fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
			where
				D: Deserializer<'de>,
			{
				struct FieldVisitor;

				impl<'de> serde::de::Visitor<'de> for FieldVisitor {
					type Value = Field;

					fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
						formatter.write_str("`jsonrpc`, `result/error` and `error`")
					}

					fn visit_str<E>(self, value: &str) -> Result<Field, E>
					where
						E: serde::de::Error,
					{
						match value {
							"jsonrpc" => Ok(Field::Jsonrpc),
							"result" => Ok(Field::Result),
							"error" => Ok(Field::Error),
							"id" => Ok(Field::Id),
							_ => Err(serde::de::Error::unknown_field(value, FIELDS)),
						}
					}
				}
				deserializer.deserialize_identifier(FieldVisitor)
			}
		}

		struct Visitor<T>(PhantomData<T>);

		impl<T> Visitor<T> {
			fn new() -> Visitor<T> {
				Visitor(PhantomData)
			}
		}

		impl<'de, T> serde::de::Visitor<'de> for Visitor<T>
		where
			T: Deserialize<'de>,
		{
			type Value = Response<'de, T>;

			fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
				formatter.write_str("struct Duration")
			}

			fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
			where
				V: serde::de::MapAccess<'de>,
			{
				let mut jsonrpc = None;
				let mut result = None;
				let mut error = None;
				let mut id = None;
				while let Some(key) = map.next_key()? {
					match key {
						Field::Result => {
							if result.is_some() {
								return Err(serde::de::Error::duplicate_field("result"));
							}
							result = Some(map.next_value()?);
						}
						Field::Error => {
							if error.is_some() {
								return Err(serde::de::Error::duplicate_field("error"));
							}
							error = Some(map.next_value()?);
						}
						Field::Id => {
							if id.is_some() {
								return Err(serde::de::Error::duplicate_field("id"));
							}
							id = Some(map.next_value()?);
						}
						Field::Jsonrpc => {
							if jsonrpc.is_some() {
								return Err(serde::de::Error::duplicate_field("jsonrpc"));
							}
							jsonrpc = Some(map.next_value()?);
						}
					}
				}

				let id = id.ok_or_else(|| serde::de::Error::missing_field("id"))?;

				let response = match (jsonrpc, result, error) {
					(_, Some(_), Some(_)) => {
						return Err(serde::de::Error::duplicate_field("result and error are mutually exclusive"))
					}
					(Some(jsonrpc), Some(result), None) => {
						Response { jsonrpc, result_or_error: PartialResponse::Result(result), id }
					}
					(Some(jsonrpc), None, Some(err)) => {
						Response { jsonrpc, result_or_error: PartialResponse::Error(err), id }
					}
					(None, Some(result), _) => {
						Response { jsonrpc: None, result_or_error: PartialResponse::Result(result), id }
					}
					(None, _, Some(err)) => {
						Response { jsonrpc: None, result_or_error: PartialResponse::Error(err), id }
					}
					(_, None, None) => return Err(serde::de::Error::missing_field("result/error")),
				};

				Ok(response)
			}
		}

		const FIELDS: &'static [&'static str] = &["jsonrpc", "result/error", "id"];
		deserializer.deserialize_struct("Response", FIELDS, Visitor::new())
	}
}

impl<'a, T: Serialize> Serialize for Response<'a, T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut s = serializer.serialize_struct("Response", 3)?;

		if let Some(field) = &self.jsonrpc {
			s.serialize_field("jsonrpc", field)?;
		}

		match &self.result_or_error {
			PartialResponse::Error(err) => s.serialize_field("error", err)?,
			PartialResponse::Result(r) => s.serialize_field("result", r)?,
		};

		s.serialize_field("id", &self.id)?;
		s.end()
	}
}

#[cfg(test)]
mod tests {
	use crate::{response::PartialResponse, ErrorObjectOwned};

	use super::{Id, Response, TwoPointZero};

	#[test]
	fn serialize_call_ok_response() {
		let ser = serde_json::to_string(&Response {
			jsonrpc: Some(TwoPointZero),
			result_or_error: PartialResponse::Result("ok"),
			id: Id::Number(1),
		})
		.unwrap();
		let exp = r#"{"jsonrpc":"2.0","result":"ok","id":1}"#;
		assert_eq!(ser, exp);
	}

	#[test]
	fn serialize_call_err_response() {
		let ser = serde_json::to_string(&Response {
			jsonrpc: Some(TwoPointZero),
			result_or_error: PartialResponse::error(ErrorObjectOwned::owned(1, "lo", None::<()>)),
			id: Id::Number(1),
		})
		.unwrap();
		let exp = r#"{"jsonrpc":"2.0","error":{"code":1,"message":"lo"},"id":1}"#;
		assert_eq!(ser, exp);
	}

	#[test]
	fn serialize_call_response_missing_version_field() {
		let ser = serde_json::to_string(&Response {
			jsonrpc: None,
			result_or_error: PartialResponse::Result("ok"),
			id: Id::Number(1),
		})
		.unwrap();
		let exp = r#"{"result":"ok","id":1}"#;
		assert_eq!(ser, exp);
	}

	#[test]
	fn deserialize_success_call() {
		let exp = Response {
			jsonrpc: Some(TwoPointZero),
			result_or_error: PartialResponse::Result(99_u64),
			id: Id::Number(11),
		};
		let dsr: Response<u64> = serde_json::from_str(r#"{"jsonrpc":"2.0", "result":99, "id":11}"#).unwrap();
		assert_eq!(dsr.jsonrpc, exp.jsonrpc);
		assert_eq!(dsr.result_or_error, exp.result_or_error);
		assert_eq!(dsr.id, exp.id);
	}

	#[test]
	fn deserialize_err_call() {
		let exp = Response {
			jsonrpc: Some(TwoPointZero),
			result_or_error: PartialResponse::error(ErrorObjectOwned::owned(1, "lo", None::<()>)),
			id: Id::Number(11),
		};
		let dsr: Response<()> =
			serde_json::from_str(r#"{"jsonrpc":"2.0","error":{"code":1,"message":"lo"},"id":11}"#).unwrap();
		assert_eq!(dsr.jsonrpc, exp.jsonrpc);
		assert_eq!(dsr.result_or_error, exp.result_or_error);
		assert_eq!(dsr.id, exp.id);
	}

	#[test]
	fn deserialize_call_missing_version_field() {
		let exp = Response { jsonrpc: None, result_or_error: PartialResponse::Result(99_u64), id: Id::Number(11) };
		let dsr: Response<u64> = serde_json::from_str(r#"{"jsonrpc":null, "result":99, "id":11}"#).unwrap();
		assert_eq!(dsr.jsonrpc, exp.jsonrpc);
		assert_eq!(dsr.result_or_error, exp.result_or_error);
		assert_eq!(dsr.id, exp.id);
	}
}
