use crate::error::CallError;
use alloc::collections::BTreeMap;
use beef::Cow;
use serde::de::{self, Deserializer, Unexpected, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::fmt;

/// JSON-RPC parameter values for subscriptions.
#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcSubscriptionParams<T> {
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

/// Parameters sent with the RPC request.
///
/// The data containing the params is a `Cow<&str>` and can either be a borrowed `&str` of JSON from an incoming
/// [`super::request::JsonRpcRequest`] (which in turn borrows it from the input buffer that is shared between requests);
/// or, it can be an owned `String`.
#[derive(Clone, Debug)]
pub struct RpcParams<'a>(Option<Cow<'a, str>>);

impl<'a> RpcParams<'a> {
	/// Create params
	pub fn new(raw: Option<&'a str>) -> Self {
		Self(raw.map(Into::into))
	}

	/// Returns true if the contained JSON is an object
	pub fn is_object(&self) -> bool {
		let json: &str = match self.0 {
			Some(ref cow) => cow,
			None => return false,
		};

		json.trim_start().starts_with("{")
	}

	/// Obtain a sequence parser, [`RpcParamsSequence`].
	///
	/// This allows sequential parsing of the incoming params, using an `Iterator`-style API and is useful when the RPC
	/// request has optional parameters at the tail that may or may not be present.
	pub fn sequence(&self) -> RpcParamsSequence {
		RpcParamsSequence(self.0.as_ref().map(AsRef::as_ref).unwrap_or(""))
	}

	/// Attempt to parse all parameters as an array or map into type `T`.
	pub fn parse<T>(&'a self) -> Result<T, CallError>
	where
		T: Deserialize<'a>,
	{
		let params = self.0.as_ref().map(AsRef::as_ref).unwrap_or("null");
		serde_json::from_str(params).map_err(|_| CallError::InvalidParams)
	}

	/// Attempt to parse parameters as an array of a single value of type `T`, and returns that value.
	pub fn one<T>(&'a self) -> Result<T, CallError>
	where
		T: Deserialize<'a>,
	{
		self.parse::<[T; 1]>().map(|[res]| res)
	}

	/// Convert `RpcParams<'a>` to `RpcParams<'static>` so that it can be moved across threads.
	///
	/// This will cause an allocation if the params internally are using a borrowed JSON slice.
	pub fn into_owned(self) -> RpcParams<'static> {
		RpcParams(self.0.map(|s| Cow::owned(s.into_owned())))
	}
}

/// An `Iterator`-like parser for a sequence of `RpcParams`.
///
/// This will parse the params one at a time, and allows for graceful handling of optional parameters at the tail; other
/// use cases are likely better served by [`RpcParams::parse`]. The reason this is not an actual [`Iterator`] is that
/// params parsing (often) yields values of different types.
#[derive(Debug)]
pub struct RpcParamsSequence<'a>(&'a str);

impl<'a> RpcParamsSequence<'a> {
	fn next_inner<T>(&mut self) -> Option<Result<T, CallError>>
	where
		T: Deserialize<'a>,
	{
		let mut json = self.0.trim_start();

		match json.as_bytes().get(0)? {
			b']' => {
				self.0 = "";

				return None;
			}
			b'[' | b',' => json = &json[1..],
			_ => return Some(Err(CallError::InvalidParams)),
		}

		let mut iter = serde_json::Deserializer::from_str(json).into_iter::<T>();

		match iter.next()? {
			Ok(value) => {
				self.0 = &json[iter.byte_offset()..];

				Some(Ok(value))
			}
			Err(_) => {
				self.0 = "";

				Some(Err(CallError::InvalidParams))
			}
		}
	}

	/// Parse the next parameter to type `T`
	///
	/// ```
	/// # use jsonrpsee_types::v2::params::RpcParams;
	/// let params = RpcParams::new(Some(r#"[true, 10, "foo"]"#));
	/// let mut seq = params.sequence();
	///
	/// let a: bool = seq.next().unwrap();
	/// let b: i32 = seq.next().unwrap();
	/// let c: &str = seq.next().unwrap();
	///
	/// assert_eq!(a, true);
	/// assert_eq!(b, 10);
	/// assert_eq!(c, "foo");
	/// ```
	pub fn next<T>(&mut self) -> Result<T, CallError>
	where
		T: Deserialize<'a>,
	{
		match self.next_inner() {
			Some(result) => result,
			None => Err(CallError::InvalidParams),
		}
	}

	/// Parse the next optional parameter to type `Option<T>`.
	///
	/// The result will be `None` for `null`, and for missing values in the supplied JSON array.
	///
	/// ```
	/// # use jsonrpsee_types::v2::params::RpcParams;
	/// let params = RpcParams::new(Some(r#"[1, 2, null]"#));
	/// let mut seq = params.sequence();
	///
	/// let params: [Option<u32>; 4] = [
	///     seq.optional_next().unwrap(),
	///     seq.optional_next().unwrap(),
	///     seq.optional_next().unwrap(),
	///     seq.optional_next().unwrap(),
	/// ];;
	///
	/// assert_eq!(params, [Some(1), Some(2), None, None]);
	/// ```
	pub fn optional_next<T>(&mut self) -> Result<Option<T>, CallError>
	where
		T: Deserialize<'a>,
	{
		match self.next_inner::<Option<T>>() {
			Some(result) => result,
			None => Ok(None),
		}
	}
}

/// [Serializable JSON-RPC parameters](https://www.jsonrpc.org/specification#parameter_structures)
///
/// If your type implements `Into<JsonValue>`, call that in favor of `serde_json::to:value` to
/// construct the parameters. Because `serde_json::to_value` serializes the type which allocates
/// whereas `Into<JsonValue>` doesn't in most cases.
#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum JsonRpcParams<'a> {
	/// No params.
	NoParams,
	/// Positional params (heap allocated).
	Array(Vec<JsonValue>),
	/// Positional params (slice).
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

	/// Convert `Id<'a>` to `Id<'static>` so that it can be moved across threads.
	///
	/// This can cause an allocation if the id is a string.
	pub fn into_owned(self) -> Id<'static> {
		match self {
			Id::Null => Id::Null,
			Id::Number(num) => Id::Number(num),
			Id::Str(s) => Id::Str(Cow::owned(s.into_owned())),
		}
	}
}

#[cfg(test)]
mod test {
	use super::{
		Cow, Id, JsonRpcParams, JsonRpcSubscriptionParams, JsonValue, RpcParams, SubscriptionId, TwoPointZero,
	};

	#[test]
	fn id_deserialization() {
		let s = r#""2""#;
		let deserialized: Id = serde_json::from_str(s).unwrap();
		match deserialized {
			Id::Str(ref cow) => {
				assert!(cow.is_borrowed());
				assert_eq!(cow, "2");
			}
			_ => panic!("Expected Id::Str"),
		}

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
		assert!(none.sequence().next::<u64>().is_err());

		let array_params = RpcParams::new(Some("[1, 2, 3]"));
		let arr: Result<[u64; 3], _> = array_params.parse();
		assert!(arr.is_ok());

		let mut seq = array_params.sequence();

		assert_eq!(seq.next::<u64>().unwrap(), 1);
		assert_eq!(seq.next::<u64>().unwrap(), 2);
		assert_eq!(seq.next::<u64>().unwrap(), 3);
		assert!(seq.next::<u64>().is_err());

		let array_one = RpcParams::new(Some("[1]"));
		let one: Result<u64, _> = array_one.one();
		assert!(one.is_ok());

		let object_params = RpcParams::new(Some(r#"{"beef":99,"dinner":0}"#));
		let obj: Result<JsonValue, _> = object_params.parse();
		assert!(obj.is_ok());
	}

	#[test]
	fn params_sequence_borrows() {
		let params = RpcParams::new(Some(r#"["foo", "bar"]"#));
		let mut seq = params.sequence();

		assert_eq!(seq.next::<&str>().unwrap(), "foo");
		assert_eq!(seq.next::<&str>().unwrap(), "bar");
		assert!(seq.next::<&str>().is_err());

		// It's ok to parse the params again.
		let params: (&str, &str) = params.parse().unwrap();
		assert_eq!(params, ("foo", "bar"));
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

	#[test]
	fn subscription_params_serialize_work() {
		let ser =
			serde_json::to_string(&JsonRpcSubscriptionParams { subscription: SubscriptionId::Num(12), result: "goal" })
				.unwrap();
		let exp = r#"{"subscription":12,"result":"goal"}"#;
		assert_eq!(ser, exp);
	}

	#[test]
	fn subscription_params_deserialize_work() {
		let ser = r#"{"subscription":"9","result":"offside"}"#;
		assert!(
			serde_json::from_str::<JsonRpcSubscriptionParams<()>>(ser).is_err(),
			"invalid type should not be deserializable"
		);
		let dsr: JsonRpcSubscriptionParams<JsonValue> = serde_json::from_str(ser).unwrap();
		assert_eq!(dsr.subscription, SubscriptionId::Str("9".into()));
		assert_eq!(dsr.result, serde_json::json!("offside"));
	}
}
