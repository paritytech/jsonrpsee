use globset::{GlobBuilder, GlobMatcher};
use std::{fmt, hash};

/// Pattern that can be matched to string.
pub trait Pattern {
	/// Returns true if given string matches the pattern.
	fn matches<T: AsRef<str>>(&self, other: T) -> bool;
}

#[derive(Clone)]
pub struct Matcher(Option<GlobMatcher>, String);
impl Matcher {
	pub fn new(string: &str) -> Matcher {
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
