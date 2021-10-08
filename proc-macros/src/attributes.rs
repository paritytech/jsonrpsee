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

use crate::helpers::punct_is;
use proc_macro2::{Delimiter, Span, TokenStream as TokenStream2, TokenTree};
use std::fmt;
use syn::spanned::Spanned;

#[derive(Debug)]
pub(crate) struct Attr {
	pub path: syn::Path,
	pub arguments: Vec<Argument>,
}

#[derive(Debug)]
pub(crate) struct Argument {
	pub label: syn::Ident,
	pub kind: ArgumentKind,
}

#[non_exhaustive]
#[derive(Debug)]
pub(crate) enum ArgumentKind {
	Flag,
	Value(TokenStream2),
	// Group(Vec<TokenStream2>),
}

impl Attr {
	pub fn find_and_parse(hay: &[syn::Attribute], name: &str, host: Span) -> syn::Result<Attr> {
		let syn_attr = hay
			.iter()
			.find(|syn_attr| syn_attr.path.is_ident(name))
			.ok_or_else(|| syn::Error::new(host, format!("Missing attribute `#[{}]`", name)))?;

		Self::from_syn(syn_attr.clone())
	}

	/// Parses `syn::Attribute` with plain `TokenStream` into a more robust `Attr` with
	/// a collection `Arguments`.
	pub fn from_syn(attr: syn::Attribute) -> syn::Result<Attr> {
		let span = attr.tokens.span();
		let mut tokens = attr.tokens.clone().into_iter();
		let mut arguments = Vec::new();

		let mut tokens = match tokens.next() {
			Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
				match tokens.next() {
					None => (),
					Some(token) => return Err(syn::Error::new(token.span(), "Unexpected token after `(...)` group")),
				}
				group.stream().into_iter()
			}
			None => {
				return Ok(Attr { path: attr.path, arguments: Vec::new() });
			}
			_ => return Err(syn::Error::new(span, "Expected `(...)`")),
		};

		while let Some(token) = tokens.next() {
			let label = match token {
				TokenTree::Ident(ident) => ident,
				_ => return Err(syn::Error::new(token.span(), "Expected argument identifier")),
			};

			let kind = match tokens.next() {
				Some(TokenTree::Punct(punct)) if punct_is(&punct, '=') => Self::parse_value(label.span(), &mut tokens)?,
				Some(TokenTree::Punct(punct)) if punct_is(&punct, ',') => ArgumentKind::Flag,
				None => ArgumentKind::Flag,
				_ => return Err(syn::Error::new(label.span(), "Expected `=`, or `,` after the argument identifier")),
			};

			arguments.push(Argument { label, kind });
		}

		let path = attr.path;

		Ok(Attr { path, arguments })
	}

	fn parse_value(span: Span, tokens: impl Iterator<Item = TokenTree>) -> syn::Result<ArgumentKind> {
		let value: TokenStream2 =
			tokens.take_while(|token| !matches!(token, TokenTree::Punct(punct) if punct_is(&punct, ','))).collect();

		if value.is_empty() {
			return Err(syn::Error::new(span, "Missing value after `=`"));
		}

		Ok(ArgumentKind::Value(value))
	}

	/// Attempt to get a list of `Argument`s from a list of names in order.
	///
	/// Errors if there is an argument with a name that's not on the list, or if there is a duplicate definition.
	pub fn retain<const N: usize>(self, allowed: [&str; N]) -> syn::Result<[syn::Result<Argument>; N]> {
		assert!(N != 0, "Calling `Attr::retain` with an empty `allowed` list, this is a bug, please report it");

		let mut result: [syn::Result<Argument>; N] =
			allowed.map(|name| Err(syn::Error::new(self.path.span(), MissingArgument(name))));

		for argument in self.arguments {
			if let Some(idx) = allowed.iter().position(|allowed_ident| argument.label == allowed_ident) {
				// If this position in the `result` array already contains an argument,
				// it means we got a duplicate definition
				if let Ok(old) = &result[idx] {
					return Err(syn::Error::new(old.label.span(), format!("Duplicate argument `{}`", old.label)));
				}

				result[idx] = Ok(argument);
			} else {
				return Err(syn::Error::new(argument.label.span(), UnknownArgument(&argument.label, &allowed)));
			}
		}

		Ok(result)
	}
}

struct MissingArgument<'a>(&'a str);

struct UnknownArgument<'a, T>(T, &'a [&'a str]);

impl fmt::Display for MissingArgument<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let MissingArgument(missing) = self;

		write!(f, "Missing argument `{}`", missing)
	}
}

impl<T: fmt::Display> fmt::Display for UnknownArgument<'_, T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let UnknownArgument(unknown, expected) = self;

		write!(f, "Unknown argument `{}`, expected one of: ", unknown)?;

		let mut expected = expected.iter();

		if let Some(first) = expected.next() {
			write!(f, "`{}`", first)?;
		}

		for tail in expected {
			write!(f, ", `{}`", tail)?;
		}

		Ok(())
	}
}

impl Argument {
	pub fn flag(self) -> syn::Result<()> {
		match self.kind {
			ArgumentKind::Flag => Ok(()),
			ArgumentKind::Value(value) => {
				Err(syn::Error::new(value.span(), "Expected a flag argument without a value"))
			}
		}
	}

	/// Asserts that the argument is `key = value` pair and parses the value into `T`
	pub fn value<T>(self) -> syn::Result<T>
	where
		T: syn::parse::Parse,
	{
		match self.kind {
			ArgumentKind::Value(value) => syn::parse2(value),
			ArgumentKind::Flag => Err(syn::Error::new(self.label.span(), "Expected `=` after the argument identifier")),
			// ArgumentKind::Group(group) => {
			// 	let span = match (group.first(), group.last()) {
			// 		(Some(start), Some(end)) => {
			// 			start.span().join(end.span())
			// 		},
			// 		_ => None,
			// 	}.unwrap_or_else(|| self.label.span());

			// 	Err(syn::Error::new(span, format!("Expected a value assignment for `{}`, but got a group instead", self.label)))
			// }
		}
	}

	/// Asserts tthat the argument is `key = "string"` and gets the value of the string
	pub fn string(self) -> syn::Result<String> {
		self.value::<syn::LitStr>().map(|lit| lit.value())
	}
}

pub trait ArgumentExt {
	fn flag(self) -> syn::Result<bool>;
}

impl ArgumentExt for syn::Result<Argument> {
	fn flag(self) -> syn::Result<bool> {
		match self {
			Err(_) => Ok(false),
			Ok(argument) => {
				argument.flag()?;
				Ok(true)
			}
		}
	}
}
