use crate::error::InvalidParams;
use alloc::collections::BTreeMap;
use std::borrow::Cow;
use serde::de::{self, Deserializer, Unexpected, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::{value::RawValue, Value as JsonValue};

use std::fmt;

/// JSON-RPC parameter values for subscriptions.
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcNotificationParams<'a> {
	/// Subscription ID
	pub subscription: u64,
	/// Result.
	#[serde(borrow)]
	pub result: &'a RawValue,
}

/// JSON-RPC parameter values for subscriptions with support for number and strings.
#[derive(Deserialize, Debug)]
pub struct JsonRpcNotificationParamsAlloc<T> {
	/// Subscription ID
	pub subscription: SubscriptionId,
	/// Result.
	pub result: T,
}

/// JSON-RPC v2 marker type.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TwoPointZero;

struct TwoPointZeroVisitor;

impl<'de> Visitor<'de> for TwoPointZeroVisitor {
	type Value = TwoPointZero;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str(r#"a string "2.0""#)
	}

	fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		match s {
			"2.0" => Ok(TwoPointZero),
			_ => Err(de::Error::invalid_value(Unexpected::Str(s), &self)),
		}
	}
}

impl<'de> Deserialize<'de> for TwoPointZero {
	fn deserialize<D>(deserializer: D) -> Result<TwoPointZero, D::Error>
	where
		D: Deserializer<'de>,
	{
		deserializer.deserialize_str(TwoPointZeroVisitor)
	}
}

impl Serialize for TwoPointZero {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str("2.0")
	}
}

/// Parameters sent with the RPC request
#[derive(Clone, Copy, Debug)]
pub struct RpcParams<'a>(Option<&'a str>);

impl<'a> RpcParams<'a> {
	/// Create params
	pub fn new(raw: Option<&'a str>) -> Self {
		Self(raw)
	}

	/// Attempt to parse all parameters as array or map into type T
	pub fn parse<T>(self) -> Result<T, InvalidParams>
	where
		T: Deserialize<'a>,
	{
		match self.0 {
			None => Err(InvalidParams),
			Some(params) => serde_json::from_str(params).map_err(|_| InvalidParams),
		}
	}

	/// Attempt to parse only the first parameter from an array into type T
	pub fn one<T>(self) -> Result<T, InvalidParams>
	where
		T: Deserialize<'a>,
	{
		self.parse::<[T; 1]>().map(|[res]| res)
	}
}

/// [Serializable JSON-RPC parameters](https://www.jsonrpc.org/specification#parameter_structures)
///
/// If your type implement `Into<JsonValue>` call that favor of `serde_json::to:value` to
/// construct the parameters. Because `serde_json::to_value` serializes the type which
/// allocates whereas `Into<JsonValue>` doesn't in most cases.
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum JsonRpcParams<'a> {
	/// No params.
	NoParams,
	/// Positional params (heap allocated)
	Array(Vec<JsonValue>),
	/// Positional params (slice)
	ArrayRef(&'a [JsonValue]),
	/// Params by name.
	Map(BTreeMap<&'a str, JsonValue>),
}

impl<'a> From<BTreeMap<&'a str, JsonValue>> for JsonRpcParams<'a> {
	fn from(map: BTreeMap<&'a str, JsonValue>) -> Self {
		Self::Map(map)
	}
}

impl<'a> From<Vec<JsonValue>> for JsonRpcParams<'a> {
	fn from(arr: Vec<JsonValue>) -> Self {
		Self::Array(arr)
	}
}

impl<'a> From<&'a [JsonValue]> for JsonRpcParams<'a> {
	fn from(slice: &'a [JsonValue]) -> Self {
		Self::ArrayRef(slice)
	}
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

impl From<SubscriptionId> for JsonValue {
	fn from(sub_id: SubscriptionId) -> Self {
		match sub_id {
			SubscriptionId::Num(n) => n.into(),
			SubscriptionId::Str(s) => s.into(),
		}
	}
}

/// Request Id
#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Id<'a> {
	/// Null
	Null,
	/// Numeric id
	Number(u64),
	/// String id
	#[serde(borrow)]
	Str(Cow<'a, str>),
}

impl<'a> Id<'a> {
	/// If the Id is a number, returns the associated number. Returns None otherwise.
	pub fn as_number(&self) -> Option<&u64> {
		match self {
			Id::Number(n) => Some(n),
			_ => None,
		}
	}

	/// If the Id is a String, returns the associated str. Returns None otherwise.
	pub fn as_str(&self) -> Option<&str> {
		match self {
			Id::Str(s) => Some(s),
			_ => None,
		}
	}

	/// If the ID is Null, returns (). Returns None otherwise.
	pub fn as_null(&self) -> Option<()> {
		match self {
			Id::Null => Some(()),
			_ => None,
		}
	}

	/// Creates owned data from borrowed data, allocates only for Strings.
	pub fn to_owned(&self) -> Id<'static> {
		match self {
			Id::Null => Id::Null,
			Id::Number(n) => Id::Number(*n),
			Id::Str(Cow::Borrowed(s)) => Id::Str(Cow::Owned(s.to_string())),
			Id::Str(Cow::Owned(s)) => Id::Str(Cow::Owned(s.clone())),
		}
	}
}

#[cfg(test)]
mod test {
	use super::{Cow, Id};

	#[test]
	fn id_deserialization() {
		let s = r#""2""#;
		let deserialized: Id = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized, Id::Str("2".into()));

		let s = r#"2"#;
		let deserialized: Id = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized, Id::Number(2));

		let s = r#""2x""#;
		let deserialized: Id = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized, Id::Str(Cow::Borrowed("2x")));

		let s = r#"[1337]"#;
		assert!(serde_json::from_str::<Id>(s).is_err());

		let s = r#"[null, 0, 2, "3"]"#;
		let deserialized: Vec<Id> = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized, vec![Id::Null, Id::Number(0), Id::Number(2), Id::Str("3".into())]);
	}

	#[test]
	fn id_serialization() {
		let d =
			vec![Id::Null, Id::Number(0), Id::Number(2), Id::Number(3), Id::Str("3".into()), Id::Str("test".into())];
		let serialized = serde_json::to_string(&d).unwrap();
		assert_eq!(serialized, r#"[null,0,2,3,"3","test"]"#);
	}
}
