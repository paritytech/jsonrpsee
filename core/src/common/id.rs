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

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// Request Id
#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Id<'a> {
    /// Numeric id
    Num(u64),
    /// String id
    Str(Cow<'a, str>),
}

impl<'a> Id<'a> {
    /// Returns a `'static` version of this struct, potentially performing necessary memory
    /// allocations.
    pub fn to_owned(&self) -> Id<'static> {
        match self {
            Id::Num(n) => Id::Num(*n),
            Id::Str(Cow::Owned(s)) => Id::Str(Cow::Owned(s.clone())),
            Id::Str(Cow::Borrowed(s)) => Id::Str(Cow::Owned(s.to_string())),
        }
    }

    /// Returns a `'static` version of this struct, potentially performing necessary memory
    /// allocations.
    pub fn into_owned(self) -> Id<'static> {
        match self {
            Id::Num(n) => Id::Num(n),
            Id::Str(Cow::Owned(s)) => Id::Str(Cow::Owned(s)),
            Id::Str(Cow::Borrowed(s)) => Id::Str(Cow::Owned(s.to_string())),
        }
    }
}

impl<'a> From<u64> for Id<'a> {
    fn from(id: u64) -> Id<'a> {
        Id::Num(id)
    }
}

impl<'a> From<String> for Id<'a> {
    fn from(id: String) -> Id<'a> {
        Id::Str(id.into())
    }
}

impl<'a> From<&'a str> for Id<'a> {
    fn from(id: &'a str) -> Id<'a> {
        Id::Str(id.into())
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

        let s = r#"[0, 2, "3"]"#;
        let deserialized: Vec<Id> = serde_json::from_str(s).unwrap();
        assert_eq!(
            deserialized,
            vec![Id::Num(0), Id::Num(2), Id::Str("3".into())]
        );
    }

    #[test]
    fn id_serialization() {
        let d = vec![
            Id::Num(0),
            Id::Num(2),
            Id::Num(3),
            Id::Str("3".to_owned()),
            Id::Str("test".to_owned()),
        ];
        let serialized = serde_json::to_string(&d).unwrap();
        assert_eq!(serialized, r#"[0,2,3,"3","test"]"#);
    }
}
