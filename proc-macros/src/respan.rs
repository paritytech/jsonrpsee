//! Module with a trait extension capable of re-spanning `syn` errors.

use quote::ToTokens;

/// Trait capable of changing `Span` set in the `syn::Error` so in case
/// of dependency setting it incorrectly, it is possible to easily create
/// a new error with the correct span.
pub trait Respan<T> {
	fn respan<S: ToTokens>(self, spanned: S) -> Result<T, syn::Error>;
}

impl<T> Respan<T> for Result<T, syn::Error> {
	fn respan<S: ToTokens>(self, spanned: S) -> Result<T, syn::Error> {
		self.map_err(|e| syn::Error::new_spanned(spanned, e))
	}
}
