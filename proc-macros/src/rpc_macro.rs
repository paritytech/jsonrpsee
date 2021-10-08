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

//! Declaration of the JSON RPC generator procedural macros.

use crate::{
	attributes::{Argument, Attr},
	helpers::extract_doc_comments,
};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{spanned::Spanned, Attribute};

#[derive(Debug, Clone)]
pub struct RpcMethod {
	pub name: String,
	pub docs: TokenStream2,
	pub params: Vec<(syn::PatIdent, syn::Type)>,
	pub returns: Option<syn::Type>,
	pub signature: syn::TraitItemMethod,
	pub aliases: Vec<String>,
}

impl RpcMethod {
	pub fn from_item(mut method: syn::TraitItemMethod) -> syn::Result<Self> {
		let attr = Attr::find_and_parse(&method.attrs, "method", method.span())?;
		let [aliases, name] = attr.retain(["aliases", "name"])?;

		let sig = method.sig.clone();
		let name = name?.string()?;
		let docs = extract_doc_comments(&method.attrs);
		let aliases = parse_aliases(aliases)?;

		let params: Vec<_> = sig
			.inputs
			.into_iter()
			.filter_map(|arg| match arg {
				syn::FnArg::Receiver(_) => None,
				syn::FnArg::Typed(arg) => match *arg.pat {
					syn::Pat::Ident(name) => Some((name, *arg.ty)),
					_ => panic!("Identifier in signature must be an ident"),
				},
			})
			.collect();

		let returns = match sig.output {
			syn::ReturnType::Default => None,
			syn::ReturnType::Type(_, output) => Some(*output),
		};

		// We've analyzed attributes and don't need them anymore.
		method.attrs.clear();

		Ok(Self { aliases, name, params, returns, signature: method, docs })
	}
}

#[derive(Debug, Clone)]
pub struct RpcSubscription {
	pub name: String,
	pub docs: TokenStream2,
	pub unsubscribe: String,
	pub params: Vec<(syn::PatIdent, syn::Type)>,
	pub item: syn::Type,
	pub signature: syn::TraitItemMethod,
	pub aliases: Vec<String>,
	pub unsubscribe_aliases: Vec<String>,
}

impl RpcSubscription {
	pub fn from_item(mut sub: syn::TraitItemMethod) -> syn::Result<Self> {
		let attr = Attr::find_and_parse(&sub.attrs, "subscription", sub.span())?;
		let [aliases, item, name, unsubscribe_aliases] =
			attr.retain(["aliases", "item", "name", "unsubscribe_aliases"])?;

		let sig = sub.sig.clone();
		let name = name?.string()?;
		let docs = extract_doc_comments(&sub.attrs);
		let unsubscribe = build_unsubscribe_method(&name);
		let item = item?.value()?;
		let aliases = parse_aliases(aliases)?;
		let unsubscribe_aliases = parse_aliases(unsubscribe_aliases)?;

		let params: Vec<_> = sig
			.inputs
			.into_iter()
			.filter_map(|arg| match arg {
				syn::FnArg::Receiver(_) => None,
				syn::FnArg::Typed(arg) => match *arg.pat {
					syn::Pat::Ident(name) => Some((name, *arg.ty)),
					_ => panic!("Identifier in signature must be an ident"),
				},
			})
			.collect();

		// We've analyzed attributes and don't need them anymore.
		sub.attrs.clear();

		Ok(Self { name, unsubscribe, unsubscribe_aliases, params, item, signature: sub, aliases, docs })
	}
}

#[derive(Debug)]
pub struct RpcDescription {
	/// Path to the `jsonrpsee` client types part.
	pub(crate) jsonrpsee_client_path: Option<TokenStream2>,
	/// Path to the `jsonrpsee` server types part.
	pub(crate) jsonrpsee_server_path: Option<TokenStream2>,
	/// Switch denoting that server trait must be generated.
	/// Assuming that trait to which attribute is applied is named `Foo`, the generated
	/// server trait will have `FooServer` name.
	pub(crate) needs_server: bool,
	/// Switch denoting that client extension trait must be generated.
	/// Assuming that trait to which attribute is applied is named `Foo`, the generated
	/// client trait will have `FooClient` name.
	pub(crate) needs_client: bool,
	/// Optional prefix for RPC namespace.
	pub(crate) namespace: Option<String>,
	/// Trait definition in which all the attributes were stripped.
	pub(crate) trait_def: syn::ItemTrait,
	/// List of RPC methods defined in the trait.
	pub(crate) methods: Vec<RpcMethod>,
	/// List of RPC subscriptions defined in the trait.
	pub(crate) subscriptions: Vec<RpcSubscription>,
}

impl RpcDescription {
	pub fn from_item(attr: syn::Attribute, mut item: syn::ItemTrait) -> syn::Result<Self> {
		let [client, server, namespace] = Attr::from_syn(attr)?.retain(["client", "server", "namespace"])?;

		let needs_server = server.ok().map(Argument::flag).transpose()?.is_some();
		let needs_client = client.ok().map(Argument::flag).transpose()?.is_some();
		let namespace = namespace.ok().map(Argument::string).transpose()?;

		if !needs_server && !needs_client {
			return Err(syn::Error::new_spanned(&item.ident, "Either 'server' or 'client' attribute must be applied"));
		}

		let jsonrpsee_client_path = crate::helpers::find_jsonrpsee_client_crate().ok();
		let jsonrpsee_server_path = crate::helpers::find_jsonrpsee_server_crate().ok();

		if needs_client && jsonrpsee_client_path.is_none() {
			return Err(syn::Error::new_spanned(&item.ident, "Unable to locate 'jsonrpsee' client dependency"));
		}
		if needs_server && jsonrpsee_server_path.is_none() {
			return Err(syn::Error::new_spanned(&item.ident, "Unable to locate 'jsonrpsee' server dependency"));
		}

		item.attrs.clear(); // Remove RPC attributes.

		let mut methods = Vec::new();
		let mut subscriptions = Vec::new();

		// Go through all the methods in the trait and collect methods and
		// subscriptions.
		for entry in item.items.iter() {
			if let syn::TraitItem::Method(method) = entry {
				if method.sig.receiver().is_none() {
					return Err(syn::Error::new_spanned(&method.sig, "First argument of the trait must be '&self'"));
				}

				let mut is_method = false;
				let mut is_sub = false;
				if has_attr(&method.attrs, "method") {
					is_method = true;

					let method_data = RpcMethod::from_item(method.clone())?;
					methods.push(method_data);
				}
				if has_attr(&method.attrs, "subscription") {
					is_sub = true;
					if is_method {
						return Err(syn::Error::new_spanned(
							&method,
							"Element cannot be both subscription and method at the same time",
						));
					}
					if method.sig.asyncness.is_some() {
						return Err(syn::Error::new_spanned(&method, "Subscription methods must not be `async`"));
					}

					let sub_data = RpcSubscription::from_item(method.clone())?;
					subscriptions.push(sub_data);
				}

				if !is_method && !is_sub {
					return Err(syn::Error::new_spanned(
						&method,
						"Methods must have either 'method' or 'subscription' attribute",
					));
				}
			} else {
				return Err(syn::Error::new_spanned(&entry, "Only methods allowed in RPC traits"));
			}
		}

		if methods.is_empty() && subscriptions.is_empty() {
			return Err(syn::Error::new_spanned(&item, "RPC cannot be empty"));
		}

		Ok(Self {
			jsonrpsee_client_path,
			jsonrpsee_server_path,
			needs_server,
			needs_client,
			namespace,
			trait_def: item,
			methods,
			subscriptions,
		})
	}

	pub fn render(self) -> Result<TokenStream2, syn::Error> {
		let server_impl = if self.needs_server { self.render_server()? } else { TokenStream2::new() };
		let client_impl = if self.needs_client { self.render_client()? } else { TokenStream2::new() };

		Ok(quote! {
			#server_impl
			#client_impl
		})
	}

	/// Formats the identifier as a path relative to the resolved
	/// `jsonrpsee` client path.
	pub(crate) fn jrps_client_item(&self, item: impl quote::ToTokens) -> TokenStream2 {
		let jsonrpsee = self.jsonrpsee_client_path.as_ref().unwrap();
		quote! { #jsonrpsee::#item }
	}

	/// Formats the identifier as a path relative to the resolved
	/// `jsonrpsee` server path.
	pub(crate) fn jrps_server_item(&self, item: impl quote::ToTokens) -> TokenStream2 {
		let jsonrpsee = self.jsonrpsee_server_path.as_ref().unwrap();
		quote! { #jsonrpsee::#item }
	}

	/// Based on the namespace, renders the full name of the RPC method/subscription.
	/// Examples:
	/// For namespace `foo` and method `makeSpam`, result will be `foo_makeSpam`.
	/// For no namespace and method `makeSpam` it will be just `makeSpam.
	pub(crate) fn rpc_identifier(&self, method: &str) -> String {
		if let Some(ns) = &self.namespace {
			format!("{}_{}", ns, method.trim())
		} else {
			method.to_string()
		}
	}
}

fn parse_aliases(arg: syn::Result<Argument>) -> syn::Result<Vec<String>> {
	let aliases = arg.ok().map(Argument::string).transpose()?;

	Ok(aliases.map(|a| a.split(',').map(Into::into).collect()).unwrap_or_default())
}

fn has_attr(attrs: &[Attribute], ident: &str) -> bool {
	for attr in attrs.iter().filter_map(|a| a.path.get_ident()) {
		if attr == ident {
			return true;
		}
	}
	false
}

fn build_unsubscribe_method(existing_method: &str) -> String {
	let method = existing_method.trim();
	let mut new_method = String::from("unsubscribe");
	if method.starts_with("subscribe") {
		new_method.extend(method.chars().skip(9));
	} else {
		new_method.push_str(method);
	}
	new_method
}
