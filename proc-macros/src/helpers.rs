use inflector::Inflector as _;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;

/// Turns a snake case function name into an UpperCamelCase name suitable to be an enum variant.
pub(crate) fn snake_case_to_camel_case(snake_case: &syn::Ident) -> syn::Ident {
	syn::Ident::new(&snake_case.to_string().to_pascal_case(), snake_case.span())
}

/// Determine the name of the variant in the enum based on the pattern of the function parameter.
pub(crate) fn param_variant_name(pat: &syn::Pat) -> syn::parse::Result<&syn::Ident> {
	match pat {
		// TODO: check other fields of the `PatIdent`
		syn::Pat::Ident(ident) => Ok(&ident.ident),
		_ => unimplemented!(),
	}
}

/// Determine the name of the parameter based on the pattern.
pub(crate) fn rpc_param_name(pat: &syn::Pat, _attrs: &[syn::Attribute]) -> syn::parse::Result<String> {
	// TODO: look in attributes if the user specified a param name
	match pat {
		// TODO: check other fields of the `PatIdent`
		syn::Pat::Ident(ident) => Ok(ident.ident.to_string()),
		_ => unimplemented!(),
	}
}

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
			Ok(quote!(#ident::types))
		}
		Ok(FoundCrate::Itself) => panic!("Deriving RPC methods in any of the `jsonrpsee crates` is not supported"),
		Err(_) => match (crate_name(http_name), crate_name(ws_name)) {
			(Ok(FoundCrate::Name(name)), _) | (_, Ok(FoundCrate::Name(name))) => {
				let ident = syn::Ident::new(&name, Span::call_site());
				Ok(quote!(#ident))
			}
			(Ok(FoundCrate::Itself), _) | (_, Ok(FoundCrate::Itself)) => {
				panic!("Deriving RPC methods in any of the `jsonrpsee crates` is not supported")
			}
			(_, Err(e)) => Err(syn::Error::new(Span::call_site(), &e)),
		},
	}
}
