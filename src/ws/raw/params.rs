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

use crate::common;

use alloc::string::String;
use core::fmt;

/// Access to the parameters of a request.
#[derive(Copy, Clone)]
pub struct Params<'a> {
	/// Raw parameters of the request.
	params: &'a common::Params,
}

/// Key referring to a potential parameter of a request.
pub enum ParamKey<'a> {
	/// String key. Only valid when the parameters list is a map.
	String(&'a str),
	/// Integer key. Only valid when the parameters list is an array.
	Index(usize),
}

impl<'a> Params<'a> {
	/// Wraps around a `&common::Params` and provides utility functions for the user.
	pub(crate) fn from(params: &'a common::Params) -> Params<'a> {
		Params { params }
	}

	/// Returns a parameter of the request by name and decodes it.
	///
	/// Returns an error if the parameter doesn't exist or is of the wrong type.
	pub fn get<'k, T>(self, param: impl Into<ParamKey<'k>>) -> Result<T, ()>
	where
		T: serde::de::DeserializeOwned,
	{
		let val = self.get_raw(param).ok_or(())?;
		serde_json::from_value(val.clone()).map_err(|_| ())
	}

	/// Returns a parameter of the request by name.
	pub fn get_raw<'k>(self, param: impl Into<ParamKey<'k>>) -> Option<&'a common::JsonValue> {
		match (self.params, param.into()) {
			(common::Params::None, _) => None,
			(common::Params::Map(map), ParamKey::String(key)) => map.get(key),
			(common::Params::Map(_), ParamKey::Index(_)) => None,
			(common::Params::Array(_), ParamKey::String(_)) => None,
			(common::Params::Array(array), ParamKey::Index(index)) => {
				if index < array.len() {
					Some(&array[index])
				} else {
					None
				}
			}
		}
	}
}

impl<'a> IntoIterator for Params<'a> {
	type Item = (ParamKey<'a>, &'a common::JsonValue);
	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		Iter(match self.params {
			common::Params::None => IterInner::Empty,
			common::Params::Array(arr) => IterInner::Array(arr.iter()),
			common::Params::Map(map) => IterInner::Map(map.iter()),
		})
	}
}

impl<'a> fmt::Debug for Params<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_map().entries(self.into_iter()).finish()
	}
}

impl<'a> AsRef<common::Params> for Params<'a> {
	fn as_ref(&self) -> &common::Params {
		self.params
	}
}

impl<'a> From<Params<'a>> for &'a common::Params {
	fn from(params: Params<'a>) -> &'a common::Params {
		params.params
	}
}

impl<'a> From<&'a str> for ParamKey<'a> {
	fn from(s: &'a str) -> Self {
		ParamKey::String(s)
	}
}

impl<'a> From<&'a String> for ParamKey<'a> {
	fn from(s: &'a String) -> Self {
		ParamKey::String(&s[..])
	}
}

impl<'a> From<usize> for ParamKey<'a> {
	fn from(i: usize) -> Self {
		ParamKey::Index(i)
	}
}

impl<'a> fmt::Debug for ParamKey<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ParamKey::String(s) => fmt::Debug::fmt(s, f),
			ParamKey::Index(s) => fmt::Debug::fmt(s, f),
		}
	}
}

/// Iterator to all the parameters of a request.
pub struct Iter<'a>(IterInner<'a>);

enum IterInner<'a> {
	Empty,
	Map(serde_json::map::Iter<'a>),
	Array(std::slice::Iter<'a, serde_json::Value>),
}

impl<'a> Iterator for Iter<'a> {
	type Item = (ParamKey<'a>, &'a common::JsonValue);

	fn next(&mut self) -> Option<Self::Item> {
		match &mut self.0 {
			IterInner::Empty => None,
			IterInner::Map(iter) => iter.next().map(|(k, v)| (ParamKey::String(&k[..]), v)),
			IterInner::Array(iter) => iter.next().map(|v| (ParamKey::String(""), v)),
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		match &self.0 {
			IterInner::Empty => (0, Some(0)),
			IterInner::Map(iter) => iter.size_hint(),
			IterInner::Array(iter) => iter.size_hint(),
		}
	}
}

impl<'a> ExactSizeIterator for Iter<'a> {}

impl<'a> fmt::Debug for Iter<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("ParamsIter").finish()
	}
}
