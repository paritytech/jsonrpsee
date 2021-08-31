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
