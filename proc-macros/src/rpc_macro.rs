//! Declaration of the JSON RPC generator procedural macros.

use crate::{attributes, respan::Respan};

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Attribute;

#[derive(Debug, Clone)]
pub struct RpcMethod {
	pub name: syn::LitStr,
	pub params: Vec<(syn::PatIdent, syn::Type)>,
	pub returns: Option<syn::Type>,
	pub signature: syn::TraitItemMethod,
}

impl RpcMethod {
	pub fn from_item(mut method: syn::TraitItemMethod) -> Result<Self, syn::Error> {
		let attributes = attributes::Method::from_attributes(&method.attrs).respan(&method.attrs.first())?;
		let sig = method.sig.clone();
		let name = attributes.name;
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

		Ok(Self { name, params, returns, signature: method })
	}
}

#[derive(Debug, Clone)]
pub struct RpcSubscription {
	pub name: syn::LitStr,
	pub unsub_method: syn::LitStr,
	pub params: Vec<(syn::PatIdent, syn::Type)>,
	pub item: syn::Type,
	pub signature: syn::TraitItemMethod,
}

impl RpcSubscription {
	pub fn from_item(mut sub: syn::TraitItemMethod) -> Result<Self, syn::Error> {
		let attributes = attributes::Subscription::from_attributes(&sub.attrs).respan(&sub.attrs.first())?;
		let sig = sub.sig.clone();
		let name = attributes.name;
		let unsub_method = attributes.unsub;
		let item = attributes.item;
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

		Ok(Self { name, unsub_method, params, item, signature: sub })
	}
}

#[derive(Debug)]
pub struct RpcDescription {
	/// Path to the `jsonrpsee` client types part.
	pub(crate) jsonrpsee_client_path: Option<TokenStream2>,
	/// Path to the `jsonrpsee` server types part.
	pub(crate) jsonrpsee_server_path: Option<TokenStream2>,
	/// Data about RPC declaration
	pub(crate) attrs: attributes::Rpc,
	/// Trait definition in which all the attributes were stripped.
	pub(crate) trait_def: syn::ItemTrait,
	/// List of RPC methods defined in the trait.
	pub(crate) methods: Vec<RpcMethod>,
	/// List of RPC subscritpions defined in the trait.
	pub(crate) subscriptions: Vec<RpcSubscription>,
}

impl RpcDescription {
	pub fn from_item(attr: syn::Attribute, mut item: syn::ItemTrait) -> Result<Self, syn::Error> {
		let attrs = attributes::Rpc::from_attributes(&[attr.clone()]).respan(&attr)?;
		if !attrs.is_correct() {
			return Err(syn::Error::new_spanned(&item.ident, "Either 'server' or 'client' attribute must be applied"));
		}

		let jsonrpsee_client_path = crate::helpers::find_jsonrpsee_client_crate().ok();
		let jsonrpsee_server_path = crate::helpers::find_jsonrpsee_server_crate().ok();

		if attrs.needs_client() && jsonrpsee_client_path.is_none() {
			return Err(syn::Error::new_spanned(&item.ident, "Unable to locate 'jsonrpsee' client dependency"));
		}
		if attrs.needs_server() && jsonrpsee_server_path.is_none() {
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
					if !matches!(method.sig.output, syn::ReturnType::Default) {
						return Err(syn::Error::new_spanned(&method, "Subscription methods must not return anything"));
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

		Ok(Self { jsonrpsee_client_path, jsonrpsee_server_path, attrs, trait_def: item, methods, subscriptions })
	}

	pub fn render(self) -> Result<TokenStream2, syn::Error> {
		let server_impl = if self.attrs.needs_server() { self.render_server()? } else { TokenStream2::new() };
		let client_impl = if self.attrs.needs_client() { self.render_client()? } else { TokenStream2::new() };

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
	pub(crate) fn rpc_identifier(&self, method: &syn::LitStr) -> String {
		if let Some(ns) = &self.attrs.namespace {
			format!("{}_{}", ns.value(), method.value())
		} else {
			method.value()
		}
	}
}

fn has_attr(attrs: &[Attribute], ident: &str) -> bool {
	for attr in attrs.iter().filter_map(|a| a.path.get_ident()) {
		if attr == ident {
			return true;
		}
	}
	false
}
