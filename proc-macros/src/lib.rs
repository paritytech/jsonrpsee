// Copyright 2019 Parity Technologies (UK) Ltd.
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

extern crate proc_macro;

use new::RpcDescription;
use proc_macro::TokenStream;
use quote::quote;

mod api_def;
mod client_builder;
mod helpers;
mod new;

/// Wraps around one or more API definitions and generates an enum.
///
/// The format within this macro must be:
///
/// ```ignore
/// jsonrpsee_proc_macros::rpc_client_api! {
///     Foo { ... }
///     pub(crate) Bar { ... }
/// }
/// ```
///
/// The `Foo` and `Bar` are identifiers, optionally prefixed with a visibility modifier
/// (e.g. `pub`).
///
/// The content of the blocks is the same as the content of a trait definition, except that
/// default implementations for methods are forbidden.
///
/// For each identifier (such as `Foo` and `Bar` in the example above), this macro will generate
/// an enum where each variant corresponds to a function of the definition. Function names are
/// turned into PascalCase to conform to the Rust style guide.
///
/// Additionally, each generated enum has one method per function definition that lets you perform
/// the method has a client.
///
// TODO(niklasad1): Generic type params for individual methods doesn't work
// because how the enum is generated, so for now type params must be declared on the entire enum.
// The reason is that all type params on the enum is bound as a separate variant but
// not generic params i.e, either params or return type.
// To handle that properly, all generic types has to be collected and applied to the enum, see example:
//
// ```rust
// jsonrpsee_rpc_client_api! {
//     Api {
//       // Doesn't work.
//       fn generic_notif<T>(t: T);
// }
// ```
//
// Expands to which doesn't compile:
// ```rust
// enum Api {
//    GenericNotif {
//        t: T,
//    },
// }
// ```
// The code should be expanded to (to compile):
// ```rust
// enum Api<T> {
//    GenericNotif {
//        t: T,
//    },
// }
// ```
#[proc_macro]
pub fn rpc_client_api(input_token_stream: TokenStream) -> TokenStream {
	// Start by parsing the input into what we expect.
	let defs: api_def::ApiDefinitions = match syn::parse(input_token_stream) {
		Ok(d) => d,
		Err(err) => return err.to_compile_error().into(),
	};

	let mut out = Vec::with_capacity(defs.apis.len());
	for api in defs.apis {
		match client_builder::build_client_api(api) {
			Ok(a) => out.push(a),
			Err(err) => return err.to_compile_error().into(),
		};
	}

	TokenStream::from(quote! {
		#(#out)*
	})
}

// New implementation starts here.

/// Main RPC macro. TODO: Add docs
#[proc_macro_attribute]
pub fn rpc(_attr: TokenStream, item: TokenStream) -> TokenStream {
	match rpc_impl(item) {
		Ok(tokens) => tokens,
		Err(err) => err.to_compile_error(),
	}
	.into()
}

/// Convenience form of `rpc` that may use `?` for error handling to avoid boilerplate.
fn rpc_impl(item: TokenStream) -> Result<proc_macro2::TokenStream, syn::Error> {
	let trait_data: syn::ItemTrait = syn::parse(item)?;
	let rpc = RpcDescription::from_item(trait_data)?;
	rpc.render()
}

/// Marker for a method in the RPC trait definition.
#[proc_macro_attribute]
pub fn method(_attr: TokenStream, item: TokenStream) -> TokenStream {
	// We don't modify the input stream, since this attribute only
	// provides additional metadata for `rpc` attribute.
	//
	// This however should be a `proc_macro_attribute`, so rust compiler won't complain about
	// unknown attribute.
	item
}

/// Marker for a subscription in the RPC trait definition.
#[proc_macro_attribute]
pub fn subscription(_attr: TokenStream, item: TokenStream) -> TokenStream {
	// We don't modify the input stream, since this attribute only
	// provides additional metadata for `rpc` attribute.
	//
	// This however should be a `proc_macro_attribute`, so rust compiler won't complain about
	// unknown attribute.
	item
}
