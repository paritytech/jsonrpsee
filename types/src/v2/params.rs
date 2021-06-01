use crate::error::CallError;
use alloc::collections::BTreeMap;
use beef::Cow;
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
	pub fn parse<T>(self) -> Result<T, CallError>
	where
		T: Deserialize<'a>,
	{
		let params = self.0.unwrap_or("null");
		serde_json::from_str(params).map_err(|_| CallError::InvalidParams)
	}

	/// Attempt to parse only the first parameter from an array into type T
	pub fn one<T>(self) -> Result<T, CallError>
	where
		T: Deserialize<'a>,
	{
		self.parse::<[T; 1]>().map(|[res]| res)
	}
}

/// Owned version of [`RpcParams`].
#[derive(Clone, Debug)]
pub struct OwnedRpcParams(Option<String>);

impl OwnedRpcParams {
	/// Converts `OwnedRpcParams` into borrowed [`RpcParams`].
	pub fn borrowed(&self) -> RpcParams<'_> {
		RpcParams(self.0.as_ref().map(|s| s.as_ref()))
	}
}

impl<'a> From<RpcParams<'a>> for OwnedRpcParams {
	fn from(borrowed: RpcParams<'a>) -> Self {
		Self(borrowed.0.map(Into::into))
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
			Self::Number(n) => Some(n),
			_ => None,
		}
	}

	/// If the Id is a String, returns the associated str. Returns None otherwise.
	pub fn as_str(&self) -> Option<&str> {
		match self {
			Self::Str(s) => Some(s.as_ref()),
			_ => None,
		}
	}

	/// If the ID is Null, returns (). Returns None otherwise.
	pub fn as_null(&self) -> Option<()> {
		match self {
			Self::Null => Some(()),
			_ => None,
		}
	}
}

/// Owned version of [`Id`] that allocates memory for the `Number` and `Str` variants.
#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum OwnedId {
	/// Null
	Null,
	/// Numeric id
	Number(u64),
	/// String id
	Str(String),
}

impl OwnedId {
	/// Converts `OwnedId` into borrowed `Id`.
	pub fn borrowed(&self) -> Id<'_> {
		match self {
			Self::Null => Id::Null,
			Self::Number(num) => Id::Number(*num),
			Self::Str(str) => Id::Str(Cow::borrowed(str)),
		}
	}
}

impl<'a> From<Id<'a>> for OwnedId {
	fn from(borrowed: Id<'a>) -> Self {
		match borrowed {
			Id::Null => Self::Null,
			Id::Number(num) => Self::Number(num),
			Id::Str(num) => Self::Str(num.as_ref().to_owned()),
		}
	}
}

#[cfg(test)]
mod test {
	use super::{Cow, Id, JsonRpcParams, JsonValue, RpcParams, SubscriptionId, TwoPointZero};

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
		assert_eq!(deserialized, Id::Str(Cow::const_str("2x")));

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

	#[test]
	fn params_serialize() {
		let test_vector = &[
			("null", JsonRpcParams::NoParams),
			("[42,23]", JsonRpcParams::Array(serde_json::from_str("[42,23]").unwrap())),
			(
				r#"{"a":42,"b":null,"c":"aa"}"#,
				JsonRpcParams::Map(serde_json::from_str(r#"{"a":42,"b":null,"c":"aa"}"#).unwrap()),
			),
		];

		for (initial_ser, params) in test_vector {
			let serialized = serde_json::to_string(params).unwrap();
			assert_eq!(&serialized, initial_ser);
		}
	}

	#[test]
	fn params_parse() {
		let none = RpcParams::new(None);
		assert!(none.one::<u64>().is_err());

		let array_params = RpcParams::new(Some("[1, 2, 3]"));
		let arr: Result<[u64; 3], _> = array_params.parse();
		assert!(arr.is_ok());

		let arr: Result<(u64, u64, u64), _> = array_params.parse();
		assert!(arr.is_ok());

		let array_one = RpcParams::new(Some("[1]"));
		let one: Result<u64, _> = array_one.one();
		assert!(one.is_ok());

		let object_params = RpcParams::new(Some(r#"{"beef":99,"dinner":0}"#));
		let obj: Result<JsonValue, _> = object_params.parse();
		assert!(obj.is_ok());
	}

	#[test]
	fn two_point_zero_serde_works() {
		let initial_ser = r#""2.0""#;
		// The fact that it was deserialized is enough.
		let two_point_zero: TwoPointZero = serde_json::from_str(initial_ser).unwrap();
		let serialized = serde_json::to_string(&two_point_zero).unwrap();
		assert_eq!(serialized, initial_ser);
	}

	#[test]
	fn subscription_id_serde_works() {
		let test_vector = &[("42", SubscriptionId::Num(42)), (r#""one""#, SubscriptionId::Str("one".into()))];

		for (initial_ser, expected) in test_vector {
			let id: SubscriptionId = serde_json::from_str(initial_ser).unwrap();
			assert_eq!(&id, expected);
			let serialized = serde_json::to_string(&id).unwrap();
			assert_eq!(&serialized, initial_ser);
		}
	}
}
