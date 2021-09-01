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

use crate::visitor::{FindAllParams, FindSubscriptionParams};
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use std::collections::HashSet;
use syn::{parse_quote, punctuated::Punctuated, visit::Visit, Token};

/// Search for client-side `jsonrpsee` in `Cargo.toml`.
pub(crate) fn find_jsonrpsee_client_crate() -> Result<proc_macro2::TokenStream, syn::Error> {
	find_jsonrpsee_crate("jsonrpsee-http-client", "jsonrpsee-ws-client")
}

/// Search for server-side `jsonrpsee` in `Cargo.toml`.
pub(crate) fn find_jsonrpsee_server_crate() -> Result<proc_macro2::TokenStream, syn::Error> {
	find_jsonrpsee_crate("jsonrpsee-http-server", "jsonrpsee-ws-server")
}

fn find_jsonrpsee_crate(http_name: &str, ws_name: &str) -> Result<proc_macro2::TokenStream, syn::Error> {
	match crate_name("jsonrpsee") {
		Ok(FoundCrate::Name(name)) => {
			let ident = syn::Ident::new(&name, Span::call_site());
			Ok(quote!(#ident))
		}
		Ok(FoundCrate::Itself) => panic!("Deriving RPC methods in any of the `jsonrpsee` crates is not supported"),
		Err(_) => match (crate_name(http_name), crate_name(ws_name)) {
			(Ok(FoundCrate::Name(name)), _) | (_, Ok(FoundCrate::Name(name))) => {
				let ident = syn::Ident::new(&name, Span::call_site());
				Ok(quote!(#ident))
			}
			(Ok(FoundCrate::Itself), _) | (_, Ok(FoundCrate::Itself)) => {
				panic!("Deriving RPC methods in any of the `jsonrpsee` crates is not supported")
			}
			(_, Err(e)) => Err(syn::Error::new(Span::call_site(), &e)),
		},
	}
}

/// Traverses the RPC trait definition and applies the required bounds for the generic type parameters that are used.
/// The bounds applied depend on whether the type parameter is used as a parameter, return value or subscription result
/// and whether it's used in client or server mode.
/// Type params get `Send + Sync + 'static` bounds and input/output parameters get `Serialize` and/or `DeserializeOwned` bounds.
/// Inspired by <https://github.com/paritytech/jsonrpc/blob/master/derive/src/to_delegate.rs#L414>
///
/// ### Example
///
/// ```
///  use jsonrpsee::{proc_macros::rpc, types::JsonRpcResult};
///
///  #[rpc(client, server)]
///  pub trait RpcTrait<A, B, C> {
///    #[method(name = "call")]
///    fn call(&self, a: A) -> JsonRpcResult<B>;
///
///    #[subscription(name = "sub", unsub = "unsub", item = Vec<C>)]
///    fn sub(&self);
///  }
/// ```
///
/// Because the `item` attribute is not parsed as ordinary rust syntax, the `syn::Type` is traversed to find
/// each generic parameter of it.
/// This is used as an additional input before traversing the entire trait.
/// Otherwise, it's not possible to know whether a type parameter is used for subscription result.
pub(crate) fn generate_where_clause(
	item_trait: &syn::ItemTrait,
	sub_tys: &[syn::Type],
	is_client: bool,
) -> Vec<syn::WherePredicate> {
	let visitor = visit_trait(item_trait, sub_tys);
	let additional_where_clause = item_trait.generics.where_clause.clone();

	item_trait
		.generics
		.type_params()
		.map(|ty| {
			let ty_path = syn::TypePath { qself: None, path: ty.ident.clone().into() };
			let mut bounds: Punctuated<syn::TypeParamBound, Token![+]> = parse_quote!(Send + Sync + 'static);

			if is_client {
				if visitor.input_params.contains(&ty.ident) {
					bounds.push(parse_quote!(jsonrpsee::types::Serialize))
				}
				if visitor.ret_params.contains(&ty.ident) || visitor.sub_params.contains(&ty.ident) {
					bounds.push(parse_quote!(jsonrpsee::types::DeserializeOwned))
				}
			} else {
				if visitor.input_params.contains(&ty.ident) {
					bounds.push(parse_quote!(jsonrpsee::types::DeserializeOwned))
				}
				if visitor.ret_params.contains(&ty.ident) || visitor.sub_params.contains(&ty.ident) {
					bounds.push(parse_quote!(jsonrpsee::types::Serialize))
				}
			}

			// Add the trait bounds specified in the trait.
			if let Some(where_clause) = &additional_where_clause {
				for predicate in where_clause.predicates.iter() {
					if let syn::WherePredicate::Type(where_ty) = predicate {
						if let syn::Type::Path(ref predicate) = where_ty.bounded_ty {
							if *predicate == ty_path {
								bounds.extend(where_ty.bounds.clone().into_iter());
							}
						}
					}
				}
			}

			syn::WherePredicate::Type(syn::PredicateType {
				lifetimes: None,
				bounded_ty: syn::Type::Path(ty_path),
				colon_token: <Token![:]>::default(),
				bounds,
			})
		})
		.collect()
}

/// Traverse the RPC trait by first finding the subscription parameters and then all elements
/// needed for generating the `client` and `server` traits/implementations.
fn visit_trait(item_trait: &syn::ItemTrait, sub_tys: &[syn::Type]) -> FindAllParams {
	let type_params: HashSet<_> = item_trait.generics.type_params().map(|t| t.ident.clone()).collect();
	let sub_tys = FindSubscriptionParams::new(type_params).visit(sub_tys);
	let mut visitor = FindAllParams::new(sub_tys);
	visitor.visit_item_trait(item_trait);
	visitor
}

/// Checks whether provided type is an `Option<...>`.
pub(crate) fn is_option(ty: &syn::Type) -> bool {
	if let syn::Type::Path(path) = ty {
		// TODO: https://github.com/paritytech/jsonrpsee/issues/447
		// Probably not the best way to check whether type is an `Option`.
		if path.path.segments.iter().any(|seg| seg.ident == "Option") {
			return true;
		}
	}

	false
}
