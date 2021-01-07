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

use alloc::{format, string::String, vec::Vec};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::value::from_value;

use super::{Error, JsonValue};

/// Request parameters
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Params {
	/// No parameters
	None,
	/// Array of values
	Array(Vec<JsonValue>),
	/// Map of values
	Map(serde_json::Map<String, JsonValue>),
}

impl Params {
	/// Parse incoming `Params` into expected common.
	pub fn parse<D>(self) -> Result<D, Error>
	where
		D: DeserializeOwned,
	{
		let value: JsonValue = self.into();
		from_value(value).map_err(|e| Error::invalid_params(format!("Invalid params: {}.", e)))
	}

	/// Check for no params, returns Err if any params
	pub fn expect_no_params(self) -> Result<(), Error> {
		match self {
			Params::None => Ok(()),
			Params::Array(ref v) if v.is_empty() => Ok(()),
			p => Err(Error::invalid_params_with_details("No parameters were expected", p)),
		}
	}
}

impl From<Params> for JsonValue {
	fn from(params: Params) -> JsonValue {
		match params {
			Params::Array(vec) => JsonValue::Array(vec),
			Params::Map(map) => JsonValue::Object(map),
			Params::None => JsonValue::Null,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::Params;
	use crate::jsonrpc::{Error, ErrorCode, JsonValue};

	#[test]
	fn params_deserialization() {
		let s = r#"[null, true, -1, 4, 2.3, "hello", [0], {"key": "value"}, []]"#;
		let deserialized: Params = serde_json::from_str(s).unwrap();

		let mut map = serde_json::Map::new();
		map.insert("key".to_string(), JsonValue::String("value".to_string()));

		assert_eq!(
			Params::Array(vec![
				JsonValue::Null,
				JsonValue::Bool(true),
				JsonValue::from(-1),
				JsonValue::from(4),
				JsonValue::from(2.3),
				JsonValue::String("hello".to_string()),
				JsonValue::Array(vec![JsonValue::from(0)]),
				JsonValue::Object(map),
				JsonValue::Array(vec![]),
			]),
			deserialized
		);
	}

	#[test]
	fn should_return_meaningful_error_when_deserialization_fails() {
		// given
		let s = r#"[1, true]"#;
		let params = || serde_json::from_str::<Params>(s).unwrap();

		// when
		let v1: Result<(Option<u8>, String), Error> = params().parse();
		let v2: Result<(u8, bool, String), Error> = params().parse();
		let err1 = v1.unwrap_err();
		let err2 = v2.unwrap_err();

		// then
		assert_eq!(err1.code, ErrorCode::InvalidParams);
		assert_eq!(err1.message, "Invalid params: invalid type: boolean `true`, expected a string.");
		assert_eq!(err1.data, None);
		assert_eq!(err2.code, ErrorCode::InvalidParams);
		assert_eq!(err2.message, "Invalid params: invalid length 2, expected a tuple of size 3.");
		assert_eq!(err2.data, None);
	}

	#[test]
	fn single_param_parsed_as_tuple() {
		let params: (u64,) = Params::Array(vec![JsonValue::from(1)]).parse().unwrap();
		assert_eq!(params, (1,));
	}
}
