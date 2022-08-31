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

//! Types to handle JSON-RPC request parameters according to the [spec](https://www.jsonrpc.org/specification#parameter_structures).
//! Some types come with a "*Ser" variant that implements [`serde::Serialize`]; these are used in the client.

use std::fmt;

use crate::error::CallError;
use alloc::collections::BTreeMap;
use anyhow::anyhow;
use beef::Cow;
use serde::de::{self, Deserializer, Unexpected, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
use serde_json::Value as JsonValue;

#[doc(hidden)]
pub mod __reexports {
	pub use crate::params::ToRpcParams;
	pub use crate::params::UnnamedParams;
	pub use crate::params::UnnamedParamsBuilder;
}

/// JSON-RPC v2 marker type.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

/// Parameters sent with an incoming JSON-RPC request.
///
/// The data containing the params is a `Cow<&str>` and can either be a borrowed `&str` of JSON from an incoming
/// [`super::request::Request`] (which in turn borrows it from the input buffer that is shared between requests);
/// or, it can be an owned [`String`].
#[derive(Clone, Debug)]
pub struct Params<'a>(Option<Cow<'a, str>>);

impl<'a> Params<'a> {
	/// Create params
	pub fn new(raw: Option<&'a str>) -> Self {
		Self(raw.map(|r| r.trim().into()))
	}

	/// Returns true if the contained JSON is an object
	pub fn is_object(&self) -> bool {
		let json: &str = match self.0 {
			Some(ref cow) => cow,
			None => return false,
		};
		json.starts_with('{')
	}

	/// Obtain a sequence parser, [`ParamsSequence`].
	///
	/// This allows sequential parsing of the incoming params, using an `Iterator`-style API and is useful when the RPC
	/// request has optional parameters at the tail that may or may not be present.
	pub fn sequence(&self) -> ParamsSequence {
		let json = match self.0.as_ref() {
			// It's assumed that params is `[a,b,c]`, if empty regard as no params.
			Some(json) if json == "[]" => "",
			Some(json) => json,
			None => "",
		};
		ParamsSequence(json)
	}

	/// Attempt to parse all parameters as an array or map into type `T`.
	pub fn parse<T>(&'a self) -> Result<T, CallError>
	where
		T: Deserialize<'a>,
	{
		// NOTE(niklasad1): Option::None is serialized as `null` so we provide that here.
		let params = self.0.as_ref().map(AsRef::as_ref).unwrap_or("null");
		serde_json::from_str(params).map_err(|e| CallError::InvalidParams(e.into()))
	}

	/// Attempt to parse parameters as an array of a single value of type `T`, and returns that value.
	pub fn one<T>(&'a self) -> Result<T, CallError>
	where
		T: Deserialize<'a>,
	{
		self.parse::<[T; 1]>().map(|[res]| res)
	}

	/// Convert `Params<'a>` to `Params<'static>` so that it can be moved across threads.
	///
	/// This will cause an allocation if the params internally are using a borrowed JSON slice.
	pub fn into_owned(self) -> Params<'static> {
		Params(self.0.map(|s| Cow::owned(s.into_owned())))
	}

	/// Return the length of underlying JSON string in number of bytes.
	pub fn len_bytes(&self) -> usize {
		match self.0 {
			Some(ref cow) => cow.len(),
			None => 0,
		}
	}
}

/// An `Iterator`-like parser for a sequence of [`Params`].
///
/// This will parse the params one at a time, and allows for graceful handling of optional parameters at the tail; other
/// use cases are likely better served by [`Params::parse`]. The reason this is not an actual [`Iterator`] is that
/// params parsing (often) yields values of different types.
///
/// Regards empty array `[]` as no parameters provided.
#[derive(Debug)]
pub struct ParamsSequence<'a>(&'a str);

impl<'a> ParamsSequence<'a> {
	fn next_inner<T>(&mut self) -> Option<Result<T, CallError>>
	where
		T: Deserialize<'a>,
	{
		let mut json = self.0;
		tracing::trace!("[next_inner] Params JSON: {:?}", json);
		match json.as_bytes().first()? {
			b']' => {
				self.0 = "";

				tracing::trace!("[next_inner] Reached end of sequence.");
				return None;
			}
			b'[' | b',' => json = &json[1..],
			_ => {
				let errmsg = format!("Invalid params. Expected one of '[', ']' or ',' but found {:?}", json);
				tracing::error!("[next_inner] {}", errmsg);
				return Some(Err(CallError::InvalidParams(anyhow!(errmsg))));
			}
		}

		let mut iter = serde_json::Deserializer::from_str(json).into_iter::<T>();

		match iter.next()? {
			Ok(value) => {
				self.0 = json[iter.byte_offset()..].trim_start();

				Some(Ok(value))
			}
			Err(e) => {
				tracing::error!(
					"[next_inner] Deserialization to {:?} failed. Error: {:?}, input JSON: {:?}",
					std::any::type_name::<T>(),
					e,
					json
				);
				self.0 = "";

				Some(Err(CallError::InvalidParams(e.into())))
			}
		}
	}

	/// Parse the next parameter to type `T`
	///
	/// ```
	/// # use jsonrpsee_types::params::Params;
	/// let params = Params::new(Some(r#"[true, 10, "foo"]"#));
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
	#[allow(clippy::should_implement_trait)]
	pub fn next<T>(&mut self) -> Result<T, CallError>
	where
		T: Deserialize<'a>,
	{
		match self.next_inner() {
			Some(result) => result,
			None => Err(CallError::InvalidParams(anyhow!("No more params"))),
		}
	}

	/// Parse the next optional parameter to type `Option<T>`.
	///
	/// The result will be `None` for `null`, and for missing values in the supplied JSON array.
	///
	/// ```
	/// # use jsonrpsee_types::params::Params;
	/// let params = Params::new(Some(r#"[1, 2, null]"#));
	/// let mut seq = params.sequence();
	///
	/// let params: [Option<u32>; 4] = [
	///     seq.optional_next().unwrap(),
	///     seq.optional_next().unwrap(),
	///     seq.optional_next().unwrap(),
	///     seq.optional_next().unwrap(),
	/// ];
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
pub enum ParamsSer<'a> {
	/// Positional params (heap allocated).
	Array(Vec<JsonValue>),
	/// Positional params (slice).
	ArrayRef(&'a [JsonValue]),
	/// Params by name.
	Map(BTreeMap<&'a str, JsonValue>),
}

impl<'a> From<BTreeMap<&'a str, JsonValue>> for ParamsSer<'a> {
	fn from(map: BTreeMap<&'a str, JsonValue>) -> Self {
		Self::Map(map)
	}
}

impl<'a> From<Vec<JsonValue>> for ParamsSer<'a> {
	fn from(arr: Vec<JsonValue>) -> Self {
		Self::Array(arr)
	}
}

impl<'a> From<&'a [JsonValue]> for ParamsSer<'a> {
	fn from(slice: &'a [JsonValue]) -> Self {
		Self::ArrayRef(slice)
	}
}

/// Id of a subscription, communicated by the server.
#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum SubscriptionId<'a> {
	/// Numeric id
	Num(u64),
	/// String id
	#[serde(borrow)]
	Str(Cow<'a, str>),
}

impl<'a> From<SubscriptionId<'a>> for JsonValue {
	fn from(sub_id: SubscriptionId) -> Self {
		match sub_id {
			SubscriptionId::Num(n) => n.into(),
			SubscriptionId::Str(s) => s.into_owned().into(),
		}
	}
}

impl<'a> From<u64> for SubscriptionId<'a> {
	fn from(sub_id: u64) -> Self {
		Self::Num(sub_id)
	}
}

impl<'a> From<String> for SubscriptionId<'a> {
	fn from(sub_id: String) -> Self {
		Self::Str(sub_id.into())
	}
}

impl<'a> TryFrom<JsonValue> for SubscriptionId<'a> {
	type Error = ();

	fn try_from(json: JsonValue) -> Result<SubscriptionId<'a>, ()> {
		match json {
			JsonValue::String(s) => Ok(SubscriptionId::Str(s.into())),
			JsonValue::Number(n) => {
				if let Some(n) = n.as_u64() {
					Ok(SubscriptionId::Num(n))
				} else {
					Err(())
				}
			}
			_ => Err(()),
		}
	}
}

impl<'a> SubscriptionId<'a> {
	/// Convert `SubscriptionId<'a>` to `SubscriptionId<'static>` so that it can be moved across threads.
	///
	/// This can cause an allocation if the id is a string.
	pub fn into_owned(self) -> SubscriptionId<'static> {
		match self {
			SubscriptionId::Num(num) => SubscriptionId::Num(num),
			SubscriptionId::Str(s) => SubscriptionId::Str(Cow::owned(s.into_owned())),
		}
	}
}

/// Request Id
#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize, PartialOrd, Ord)]
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

/// Helper module for building parameters.
mod params_builder {
	use serde::Serialize;

	/// Initial number of bytes for a parameter length.
	const PARAM_BYTES_CAPACITY: usize = 128;

	/// Generic parameter builder that serializes parameters to bytes.
	/// This produces a JSON compatible String.
	///
	/// The implementation relies on `Vec<u8>` to hold the serialized
	/// parameters in memory for the following reasons:
	///   1. Other serialization methods than `serde_json::to_writer` would internally
	///      have an extra heap allocation for temporarily holding the value in memory.
	///   2. `io::Write` is not implemented for `String` required for serialization.
	#[derive(Debug)]
	pub struct ParamsBuilder {
		bytes: Vec<u8>,
		end: char,
	}

	impl ParamsBuilder {
		/// Construct a new [`ParamsBuilder`] with custom start and end tokens.
		/// The inserted values are wrapped by the _start_ and _end_ tokens.
		fn new(start: char, end: char) -> Self {
			let mut bytes = Vec::with_capacity(PARAM_BYTES_CAPACITY);
			bytes.push(start as u8);
			ParamsBuilder { bytes, end }
		}

		/// Construct a new [`ParamsBuilder`] for positional parameters equivalent to a JSON array object.
		pub fn positional() -> Self {
			Self::new('[', ']')
		}

		/// Construct a new [`ParamsBuilder`] for named parameters equivalent to a JSON map object.
		pub fn named() -> Self {
			Self::new('{', '}')
		}

		/// Insert a named value (key, value) pair into the builder.
		/// The _name_ and _value_ are delimited by the `:` token.
		pub fn insert_named<P: Serialize>(&mut self, name: &str, value: P) -> Result<(), serde_json::Error> {
			serde_json::to_writer(&mut self.bytes, name)?;
			self.bytes.push(b':');
			serde_json::to_writer(&mut self.bytes, &value)?;
			self.bytes.push(b',');

			Ok(())
		}

		/// Insert a plain value into the builder.
		pub fn insert<P: Serialize>(&mut self, value: P) -> Result<(), serde_json::Error> {
			serde_json::to_writer(&mut self.bytes, &value)?;
			self.bytes.push(b',');

			Ok(())
		}

		/// Finish the building process and return a JSON compatible string.
		pub fn build(mut self) -> String {
			let idx = self.bytes.len() - 1;
			if self.bytes[idx] == b',' {
				self.bytes[idx] = self.end as u8;
			} else {
				self.bytes.push(self.end as u8);
			}

			// Safety: This is safe because we do not emit invalid UTF-8.
			unsafe { String::from_utf8_unchecked(self.bytes) }
		}
	}
}

/// Parameter builder that serializes named value parameters to a JSON compatible string.
/// This is the equivalent of a JSON Map object `{ key: value }`.
///
/// # Examples
///
/// ```rust
///
/// use jsonrpsee_types::NamedParamsBuilder;
///
/// let mut builder = NamedParamsBuilder::new();
/// builder.insert("param1", 1);
/// builder.insert("param2", "abc");
/// let params = builder.build();
///
/// // Use RPC parameters...
/// ```
#[derive(Debug)]
pub struct NamedParamsBuilder(params_builder::ParamsBuilder);

impl NamedParamsBuilder {
	/// Construct a new [`NamedParamsBuilder`].
	pub fn new() -> Self {
		Self::default()
	}

	/// Insert a named value (key, value) pair into the builder.
	/// The _name_ and _value_ are delimited by the `:` token.
	pub fn insert<P: Serialize>(&mut self, name: &str, value: P) -> Result<(), serde_json::Error> {
		self.0.insert_named(name, value)
	}

	/// Finish the building process and return a JSON compatible string.
	pub fn build(self) -> NamedParams {
		NamedParams(self.0.build())
	}
}

impl Default for NamedParamsBuilder {
	fn default() -> Self {
		Self(params_builder::ParamsBuilder::named())
	}
}

/// Named RPC parameters stored as a JSON Map object `{ key: value }`.
#[derive(Clone, Debug)]
pub struct NamedParams(String);

impl ToRpcParams for NamedParams {
	fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error> {
		RawValue::from_string(self.0).map(Some)
	}
}

/// Parameter builder that serializes plain value parameters to a JSON compatible string.
/// This is the equivalent of a JSON Array object `[ value0, value1, .., valueN ]`.
///
/// # Examples
///
/// ```rust
///
/// use jsonrpsee_types::UnnamedParamsBuilder;
///
/// let mut builder = UnnamedParamsBuilder::new();
/// builder.insert("param1");
/// builder.insert(1);
/// let params = builder.build();
///
/// // Use RPC parameters...
/// ```
#[derive(Debug)]
pub struct UnnamedParamsBuilder(params_builder::ParamsBuilder);

impl UnnamedParamsBuilder {
	/// Construct a new [`UnnamedParamsBuilder`].
	pub fn new() -> Self {
		Self::default()
	}

	/// Insert a plain value into the builder.
	pub fn insert<P: Serialize>(&mut self, value: P) -> Result<(), serde_json::Error> {
		self.0.insert(value)
	}

	/// Finish the building process and return a JSON compatible string.
	pub fn build(self) -> UnnamedParams {
		UnnamedParams(self.0.build())
	}
}

impl Default for UnnamedParamsBuilder {
	fn default() -> Self {
		Self(params_builder::ParamsBuilder::positional())
	}
}

/// Unnamed RPC parameters stored as a JSON Array object `[ value0, value1, .., valueN ]`.
#[derive(Clone, Debug)]
pub struct UnnamedParams(String);

impl ToRpcParams for UnnamedParams {
	fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error> {
		RawValue::from_string(self.0).map(Some)
	}
}

/// Marker trait for types that can be serialized as JSON compatible strings.
///
/// This trait ensures the correctness of the RPC parameters.
///
/// # Note
///
/// Please consider using the [`UnnamedParamsBuilder`] and [`NamedParamsBuilder`] than
/// implementing this trait.
///
/// # Examples
///
/// - Implementation for hard-coded strings
///
/// ```rust
///
/// use jsonrpsee_types::ToRpcParams;
/// use serde_json::value::RawValue;
///
/// struct ManualParam;
///
/// impl ToRpcParams for ManualParam {
///     fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error> {
///         // Manually define a valid JSONRPC parameter.
///         RawValue::from_string("[1, \"2\", 3]".to_string()).map(Some)
///     }
/// }
/// ```
///
/// - Implementation for JSON serializable structures
///
/// ```rust
/// use jsonrpsee_types::ToRpcParams;
/// use serde_json::value::RawValue;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct SerParam {
///     param_1: u8,
///     param_2: String,
/// };
///
/// impl ToRpcParams for SerParam {
///     fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error> {
///         let s = String::from_utf8(serde_json::to_vec(&self)?).expect("Valid UTF8 format");
///         RawValue::from_string(s).map(Some)
///     }
/// }
/// ```
pub trait ToRpcParams {
	/// Consume and serialize the type as a JSON raw value.
	fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error>;
}

/// Initial number of parameters in a batch request.
const BATCH_PARAMS_NUM_CAPACITY: usize = 4;

/// Request builder that serializes RPC parameters to construct a valid batch parameter.
/// This is the equivalent of chaining multiple RPC requests.
#[derive(Clone, Debug, Default)]
pub struct BatchRequestBuilder<'a>(Vec<(&'a str, Option<Box<RawValue>>)>);

impl<'a> BatchRequestBuilder<'a> {
	/// Construct a new [`BatchRequestBuilder`].
	pub fn new() -> Self {
		Self(Vec::with_capacity(BATCH_PARAMS_NUM_CAPACITY))
	}

	/// Inserts the RPC method with provided parameters into the builder.
	pub fn insert<Params: ToRpcParams>(&mut self, method: &'a str, value: Params) -> Result<(), serde_json::Error> {
		self.0.push((method, value.to_rpc_params()?));
		Ok(())
	}

	/// Finish the building process and return a valid batch parameter.
	pub fn build(self) -> Vec<(&'a str, Option<Box<RawValue>>)> {
		self.0
	}
}

/// Empty RPC parameters that perform no allocation.
#[derive(Clone, Debug)]
pub struct EmptyParams;

/// Custom implementation for empty RPC parameters.
impl ToRpcParams for EmptyParams {
	fn to_rpc_params(self) -> Result<Option<Box<RawValue>>, serde_json::Error> {
		Ok(None)
	}
}

#[cfg(test)]
mod test {
	use super::{Cow, Id, JsonValue, Params, ParamsSer, SubscriptionId, TwoPointZero};
	use crate::response::SubscriptionPayload;

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

		let s = r#"[null, 0, 2, "\"3"]"#;
		let deserialized: Vec<Id> = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized, vec![Id::Null, Id::Number(0), Id::Number(2), Id::Str("\"3".into())]);
	}

	#[test]
	fn id_serialization() {
		let d =
			vec![Id::Null, Id::Number(0), Id::Number(2), Id::Number(3), Id::Str("\"3".into()), Id::Str("test".into())];
		let serialized = serde_json::to_string(&d).unwrap();
		assert_eq!(serialized, r#"[null,0,2,3,"\"3","test"]"#);
	}

	#[test]
	fn params_serialize() {
		let test_vector = &[
			("[]", ParamsSer::Array(serde_json::from_str("[]").unwrap())),
			("[42,23]", ParamsSer::Array(serde_json::from_str("[42,23]").unwrap())),
			(
				r#"{"a":42,"b":null,"c":"aa"}"#,
				ParamsSer::Map(serde_json::from_str(r#"{"a":42,"b":null,"c":"aa"}"#).unwrap()),
			),
		];

		for (initial_ser, params) in test_vector {
			let serialized = serde_json::to_string(params).unwrap();
			assert_eq!(&serialized, initial_ser);
		}
	}

	#[test]
	fn params_parse() {
		let none = Params::new(None);
		assert!(none.sequence().next::<u64>().is_err());
		assert!(none.parse::<Option<u64>>().is_ok());
		assert_eq!(none.len_bytes(), 0);

		let array_params = Params::new(Some("[1, 2, 3]"));
		assert_eq!(array_params.len_bytes(), 9);
		let arr: Result<[u64; 3], _> = array_params.parse();
		assert!(arr.is_ok());

		let mut seq = array_params.sequence();

		assert_eq!(seq.next::<u64>().unwrap(), 1);
		assert_eq!(seq.next::<u64>().unwrap(), 2);
		assert_eq!(seq.next::<u64>().unwrap(), 3);
		assert!(seq.next::<u64>().is_err());

		let array_one = Params::new(Some("[1]"));
		assert_eq!(array_one.len_bytes(), 3);
		let one: Result<u64, _> = array_one.one();
		assert!(one.is_ok());

		let object_params = Params::new(Some(r#"{"beef":99,"dinner":0}"#));
		assert_eq!(object_params.len_bytes(), 22);
		let obj: Result<JsonValue, _> = object_params.parse();
		assert!(obj.is_ok());
	}

	#[test]
	fn params_parse_empty_json() {
		let array_params = Params::new(Some("[]"));
		let arr: Result<Vec<u64>, _> = array_params.parse();
		assert!(arr.is_ok());

		let obj_params = Params::new(Some("{}"));
		let obj: Result<JsonValue, _> = obj_params.parse();
		assert!(obj.is_ok());
	}

	#[test]
	fn params_sequence_borrows() {
		let params = Params::new(Some(r#"["foo", "bar"]"#));
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
		let ser = serde_json::to_string(&SubscriptionPayload { subscription: SubscriptionId::Num(12), result: "goal" })
			.unwrap();
		let exp = r#"{"subscription":12,"result":"goal"}"#;
		assert_eq!(ser, exp);
	}

	#[test]
	fn subscription_params_deserialize_work() {
		let ser = r#"{"subscription":"9","result":"offside"}"#;
		assert!(
			serde_json::from_str::<SubscriptionPayload<()>>(ser).is_err(),
			"invalid type should not be deserializable"
		);
		let dsr: SubscriptionPayload<JsonValue> = serde_json::from_str(ser).unwrap();
		assert_eq!(dsr.subscription, SubscriptionId::Str("9".into()));
		assert_eq!(dsr.result, serde_json::json!("offside"));
	}

	#[test]
	fn params_sequence_optional_ignore_empty() {
		let params = Params::new(Some(r#"["foo", "bar"]"#));
		let mut seq = params.sequence();

		assert_eq!(seq.optional_next::<&str>().unwrap(), Some("foo"));
		assert_eq!(seq.optional_next::<&str>().unwrap(), Some("bar"));

		let params = Params::new(Some(r#"[]"#));
		let mut seq = params.sequence();
		assert!(seq.optional_next::<&str>().unwrap().is_none());

		let params = Params::new(Some(r#"   []		"#));
		let mut seq = params.sequence();
		assert!(seq.optional_next::<&str>().unwrap().is_none());

		let params = Params::new(Some(r#"{}"#));
		let mut seq = params.sequence();
		assert!(seq.optional_next::<&str>().is_err(), "JSON object not supported by RpcSequence");

		let params = Params::new(Some(r#"[12, "[]", [], {}]"#));
		let mut seq = params.sequence();
		assert_eq!(seq.optional_next::<u64>().unwrap(), Some(12));
		assert_eq!(seq.optional_next::<&str>().unwrap(), Some("[]"));
		assert_eq!(seq.optional_next::<Vec<u8>>().unwrap(), Some(vec![]));
		assert_eq!(seq.optional_next::<serde_json::Value>().unwrap(), Some(serde_json::json!({})));
	}

	#[test]
	fn params_sequence_optional_nesting_works() {
		let nested = Params::new(Some(r#"[1, [2], [3, 4], [[5], [6,7], []], {"named":7}]"#));
		let mut seq = nested.sequence();
		assert_eq!(seq.optional_next::<i8>().unwrap(), Some(1));
		assert_eq!(seq.optional_next::<[i8; 1]>().unwrap(), Some([2]));
		assert_eq!(seq.optional_next::<Vec<u16>>().unwrap(), Some(vec![3, 4]));
		assert_eq!(seq.optional_next::<Vec<Vec<u32>>>().unwrap(), Some(vec![vec![5], vec![6, 7], vec![]]));
		assert_eq!(seq.optional_next::<serde_json::Value>().unwrap(), Some(serde_json::json!({"named":7})));
	}
}
