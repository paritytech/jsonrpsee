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

use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;

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
