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
use bae::FromAttributes;
use proc_macro2::{Delimiter, Span, TokenStream as TokenStream2, TokenTree};
use syn::spanned::Spanned;

/// Input for the `#[rpc(...)]` attribute macro.
#[derive(Debug, Clone, FromAttributes)]
pub(crate) struct Rpc {
	/// Switch denoting that server trait must be generated.
	/// Assuming that trait to which attribute is applied is named `Foo`, the generated
	/// server trait will have `FooServer` name.
	pub server: Option<()>,
	/// Switch denoting that client extension trait must be generated.
	/// Assuming that trait to which attribute is applied is named `Foo`, the generated
	/// client trait will have `FooClient` name.
	pub client: Option<()>,
	/// Optional prefix for RPC namespace.
	pub namespace: Option<syn::LitStr>,
}

impl Rpc {
	/// Returns `true` if at least one of `server` or `client` attributes is present.
	pub(crate) fn is_correct(&self) -> bool {
		self.server.is_some() || self.client.is_some()
	}

	/// Returns `true` if server implementation was requested.
	pub(crate) fn needs_server(&self) -> bool {
		self.server.is_some()
	}

	/// Returns `true` if client implementation was requested.
	pub(crate) fn needs_client(&self) -> bool {
		self.client.is_some()
	}
}

#[derive(Debug)]
pub(crate) struct Attr {
	pub path: syn::Path,
	pub arguments: Vec<Argument>,
}

#[derive(Debug)]
pub(crate) struct Argument {
	pub label: syn::Ident,
	pub body: ArgumentBody,
}

#[non_exhaustive]
#[derive(Debug)]
pub(crate) enum ArgumentBody {
	Value(TokenStream2),
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

		let mut value_buf = Vec::new();

		while let Some(token) = tokens.next() {
			let label = match token {
				TokenTree::Ident(ident) => ident,
				_ => return Err(syn::Error::new(token.span(), "Expected argument identifier")),
			};

			match tokens.next() {
				Some(TokenTree::Punct(punct)) if punct_is(&punct, '=') => (),
				_ => return Err(syn::Error::new(label.span(), "Expected `=` after the argument identifier")),
			}

			for token in &mut tokens {
				match token {
					TokenTree::Punct(punct) if punct_is(&punct, ',') => break,
					_ => value_buf.push(token),
				}
			}

			if value_buf.is_empty() {
				return Err(syn::Error::new(label.span(), "Missing value after `=`"));
			}

			let body = ArgumentBody::Value(value_buf.drain(..).collect());

			arguments.push(Argument { label, body });
		}

		let path = attr.path;

		Ok(Attr { path, arguments })
	}

	/// Returns an error if any of the arguments in this attribute use a label not allowed on the list
	pub fn only_allowed(&self, allowed: &[&str]) -> syn::Result<()> {
		for argument in self.arguments.iter() {
			if !allowed.iter().any(|allowed_ident| argument.label == allowed_ident) {
				let mut err_str = format!("Unknown argument `{}`, expected one of: ", argument.label);

				let mut allowed = allowed.iter();

				if let Some(first) = allowed.next() {
					err_str.push('`');
					err_str.push_str(first);
				}

				for tail in allowed {
					err_str.push_str("`, `");
					err_str.push_str(tail);
				}

				err_str.push('`');

				return Err(syn::Error::new(argument.label.span(), err_str));
			}
		}

		Ok(())
	}

	/// Returns an argument for a given name, returns an error if there are multiple arguments with the same name
	pub fn get_argument(&self, name: &str) -> syn::Result<Option<&Argument>> {
		let mut needle = None;

		for probe in self.arguments.iter() {
			if probe.label == name {
				if let Some(old) = needle.replace(probe) {
					return Err(syn::Error::new(old.label.span(), format!("Duplicate argument `{}`", name)));
				}
			}
		}

		Ok(needle)
	}

	/// Returns an argument for a given name, returns an error if there are multiple arguments with the same name,
	/// or if there is no argument with this name
	pub fn require_argument(&self, argument: &str) -> syn::Result<&Argument> {
		match self.get_argument(argument)? {
			Some(arg) => Ok(arg),
			None => Err(syn::Error::new(self.path.span(), format!("Missing argument `{}`", argument))),
		}
	}
}

impl Argument {
	pub fn value<T>(&self) -> syn::Result<T>
	where
		T: syn::parse::Parse,
	{
		match self.body {
			ArgumentBody::Value(ref value) => syn::parse2(value.clone()),
			// _ => Err(syn::Error::new(self.label.span(), format!("`{}` is missing value", self.label))),
		}
	}

	pub fn lit_str(&self) -> syn::Result<String> {
		self.value::<syn::LitStr>().map(|lit| lit.value())
	}
}
