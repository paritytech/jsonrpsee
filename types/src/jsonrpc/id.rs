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

use alloc::string::String;
use serde::{Deserialize, Serialize};

/// Request Id
#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Id {
	/// No id (notification)
	Null,
	/// Numeric id
	Num(u64),
	/// String id
	Str(String),
}

impl Id {
	/// If the `Id` is a number, returns the associated number. Returns None
	/// otherwise.
	pub fn as_number(&self) -> Option<&u64> {
		match self {
			Self::Num(n) => Some(n),
			_ => None,
		}
	}

	/// If the `Id` is an String, returns the associated &str. Returns None
	/// otherwise.
	pub fn as_str(&self) -> Option<&str> {
		match self {
			Self::Str(s) => Some(s),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json;

	#[test]
	fn id_deserialization() {
		let s = r#""2""#;
		let deserialized: Id = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized, Id::Str("2".into()));

		let s = r#"2"#;
		let deserialized: Id = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized, Id::Num(2));

		let s = r#""2x""#;
		let deserialized: Id = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized, Id::Str("2x".to_owned()));

		let s = r#"[null, 0, 2, "3"]"#;
		let deserialized: Vec<Id> = serde_json::from_str(s).unwrap();
		assert_eq!(deserialized, vec![Id::Null, Id::Num(0), Id::Num(2), Id::Str("3".into())]);
	}

	#[test]
	fn id_serialization() {
		let d = vec![Id::Null, Id::Num(0), Id::Num(2), Id::Num(3), Id::Str("3".to_owned()), Id::Str("test".to_owned())];
		let serialized = serde_json::to_string(&d).unwrap();
		assert_eq!(serialized, r#"[null,0,2,3,"3","test"]"#);
	}
}
