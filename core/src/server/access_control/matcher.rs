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

use std::{fmt, hash};

use globset::{GlobBuilder, GlobMatcher};
use tracing::warn;

/// Pattern that can be matched to string.
pub(crate) trait Pattern {
	/// Returns true if given string matches the pattern.
	fn matches<T: AsRef<str>>(&self, other: T) -> bool;
}

#[derive(Clone)]
pub(crate) struct Matcher(Option<GlobMatcher>, String);

impl Matcher {
	pub(crate) fn new(string: &str) -> Matcher {
		Matcher(
			GlobBuilder::new(string)
				.case_insensitive(true)
				.build()
				.map(|g| g.compile_matcher())
				.map_err(|e| warn!("Invalid glob pattern for {}: {:?}", string, e))
				.ok(),
			string.into(),
		)
	}
}

impl Pattern for Matcher {
	fn matches<T: AsRef<str>>(&self, other: T) -> bool {
		let s = other.as_ref();
		match self.0 {
			Some(ref matcher) => matcher.is_match(s),
			None => self.1.eq_ignore_ascii_case(s),
		}
	}
}

impl fmt::Debug for Matcher {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		write!(fmt, "{:?} ({})", self.1, self.0.is_some())
	}
}

impl hash::Hash for Matcher {
	fn hash<H>(&self, state: &mut H)
	where
		H: hash::Hasher,
	{
		self.1.hash(state)
	}
}

impl Eq for Matcher {}
impl PartialEq for Matcher {
	fn eq(&self, other: &Matcher) -> bool {
		self.1.eq(&other.1)
	}
}
