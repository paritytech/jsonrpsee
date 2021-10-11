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

use proc_macro2::{Span, TokenStream as TokenStream2, TokenTree};
use std::fmt;
use syn::parse::{Parse, ParseStream, Parser};
use syn::punctuated::Punctuated;
use syn::{spanned::Spanned, Attribute, Error, Token};

pub(crate) struct AttributeMeta {
	pub path: syn::Path,
	pub arguments: Punctuated<Argument, Token![,]>,
}

pub(crate) struct Argument {
	pub label: syn::Ident,
	pub tokens: TokenStream2,
}

#[derive(Debug, Clone)]
pub struct Resource {
	pub name: syn::LitStr,
	pub assign: Token![=],
	pub value: syn::LitInt,
}

impl Parse for Argument {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let label = input.parse()?;

		let mut tokens = Vec::new();

		while !input.peek(Token![,]) {
			match input.parse::<TokenTree>() {
				Ok(token) => tokens.push(token),
				Err(_) => break,
			}
		}

		Ok(Argument { label, tokens: tokens.into_iter().collect() })
	}
}

impl Parse for Resource {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		Ok(Resource { name: input.parse()?, assign: input.parse()?, value: input.parse()? })
	}
}

fn parenthesized<T: Parse>(input: ParseStream) -> syn::Result<Punctuated<T, Token![,]>> {
	let content;

	syn::parenthesized!(content in input);

	content.parse_terminated(T::parse)
}

impl AttributeMeta {
	/// Parses `Attribute` with plain `TokenStream` into a more robust `AttributeMeta` with
	/// a collection `Arguments`.
	pub fn parse(attr: Attribute) -> syn::Result<AttributeMeta> {
		let path = attr.path;
		let arguments = parenthesized.parse2(attr.tokens)?;

		Ok(AttributeMeta { path, arguments })
	}

	/// Attempt to get a list of `Argument`s from a list of names in order.
	///
	/// Errors if there is an argument with a name that's not on the list, or if there is a duplicate definition.
	pub fn retain<const N: usize>(self, allowed: [&str; N]) -> syn::Result<[Result<Argument, MissingArgument>; N]> {
		assert!(
			N != 0,
			"Calling `AttributeMeta::retain` with an empty `allowed` list, this is a bug, please report it"
		);

		let mut result: [Result<Argument, _>; N] = allowed.map(|name| Err(MissingArgument(self.path.span(), name)));

		for argument in self.arguments {
			if let Some(idx) = allowed.iter().position(|probe| argument.label == probe) {
				// If this position in the `result` array already contains an argument,
				// it means we got a duplicate definition
				if let Ok(old) = &result[idx] {
					return Err(Error::new(old.label.span(), format!("Duplicate argument `{}`", old.label)));
				}

				result[idx] = Ok(argument);
			} else {
				let mut err_str = format!("Unknown argument `{}`, expected one of: `", &argument.label);

				err_str.push_str(allowed[0]);
				err_str.extend(allowed[1..].iter().flat_map(|&label| ["`, `", label]));
				err_str.push('`');

				return Err(Error::new(argument.label.span(), err_str));
			}
		}

		Ok(result)
	}
}

pub(crate) struct MissingArgument<'a>(Span, &'a str);

impl fmt::Display for MissingArgument<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let MissingArgument(_, missing) = self;

		write!(f, "Missing argument `{}`", missing)
	}
}

impl From<MissingArgument<'_>> for Error {
	fn from(missing: MissingArgument) -> Self {
		Error::new(missing.0, missing)
	}
}

impl Argument {
	/// Asserts that the argument is just a simple `flag` without any value present
	pub fn flag(self) -> syn::Result<()> {
		if self.tokens.is_empty() {
			Ok(())
		} else {
			Err(Error::new(self.tokens.span(), "Expected a flag argument"))
		}
	}

	/// Asserts that the argument is `key = value` pair and parses the value into `T`
	pub fn value<T: Parse>(self) -> syn::Result<T> {
		fn value_parser<T: Parse>(stream: ParseStream) -> syn::Result<T> {
			stream.parse::<Token![=]>()?;
			stream.parse()
		}

		value_parser.parse2(self.tokens)
	}

	pub fn group<T>(self) -> syn::Result<Punctuated<T, Token![,]>>
	where
		T: Parse,
	{
		parenthesized.parse2(self.tokens)
	}

	/// Asserts that the argument is `key = "string"` and gets the value of the string
	pub fn string(self) -> syn::Result<String> {
		self.value::<syn::LitStr>().map(|lit| lit.value())
	}
}

pub(crate) fn optional<T, F>(arg: Result<Argument, MissingArgument>, transform: F) -> syn::Result<Option<T>>
where
	F: Fn(Argument) -> syn::Result<T>,
{
	arg.ok().map(transform).transpose()
}
