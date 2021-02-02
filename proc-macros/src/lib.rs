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

use inflector::Inflector as _;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use std::collections::HashSet;
use syn::spanned::Spanned as _;

mod api_def;

/// Generates client RPC API.
#[proc_macro]
pub fn rpc_client_api(input_token_stream: TokenStream) -> TokenStream {
	// Start by parsing the input into what we expect.
	let defs: api_def::ApiDefinitions = match syn::parse(input_token_stream) {
		Ok(d) => d,
		Err(err) => return err.to_compile_error().into(),
	};

	let mut out = Vec::with_capacity(defs.apis.len());
	for api in defs.apis {
		match build_client_api(api) {
			Ok(a) => out.push(a),
			Err(err) => return err.to_compile_error().into(),
		};
	}

	TokenStream::from(quote! {
		#(#out)*
	})
}

/// Generates the macro output token stream corresponding to a single API.
fn build_client_api(api: api_def::ApiDefinition) -> Result<proc_macro2::TokenStream, syn::Error> {
	let enum_name = &api.name;
	let visibility = &api.visibility;
	// TODO: make sure there's no conflict here
	let mut tweaked_generics = api.generics.clone();

	let client_impl_block = build_client_impl(&api)?;
	//let debug_variants = build_debug_variants(&api)?;

	Ok(quote_spanned!(api.name.span()=>
		// TODO: doesn't work for generics.
		#visibility struct #enum_name #tweaked_generics(core::marker::PhantomData<()>);

		#client_impl_block

		//TODO: debug impl.
	))
}

/// Builds the impl block that allow performing outbound JSON-RPC queries.
///
/// Generates the `impl <enum> { }` block containing functions that perform RPC client calls.
fn build_client_impl(api: &api_def::ApiDefinition) -> Result<proc_macro2::TokenStream, syn::Error> {
	let enum_name = &api.name;

	let (impl_generics_org, type_generics, where_clause_org) = api.generics.split_for_impl();
	let lifetimes_org = api.generics.lifetimes();
	let type_params_org = api.generics.type_params();
	let const_params_org = api.generics.const_params();

	//let is_generic = api.generics.type_params().count() > 0 || api.generics.type_params().count()
	let client_functions = build_client_functions(&api)?;

	Ok(quote_spanned!(api.name.span() =>
		// TODO: order between type_params and const_params is undecided
		impl #impl_generics_org #enum_name #type_generics #where_clause_org {
			#(#client_functions)*
		}
	))
}

/// Builds the functions that allow performing outbound JSON-RPC queries.
///
/// Generates a list of functions that perform RPC client calls.
fn build_client_functions(api: &api_def::ApiDefinition) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
	let visibility = &api.visibility;

	let mut client_functions = Vec::new();
	for function in &api.definitions {
		let f_name = &function.signature.ident;
		let ret_ty = match function.signature.output {
			syn::ReturnType::Default => quote!(()),
			syn::ReturnType::Type(_, ref ty) => quote_spanned!(ty.span()=> #ty),
		};
		let rpc_method_name =
			function.attributes.method.clone().unwrap_or_else(|| function.signature.ident.to_string());

		let mut params_list = Vec::new();
		let mut params_to_json = Vec::new();
		let mut params_to_array = Vec::new();
		let mut params_tys = Vec::new();

		for (param_index, input) in function.signature.inputs.iter().enumerate() {
			let (ty, pat_span, rpc_param_name) = match input {
				syn::FnArg::Receiver(_) => {
					return Err(syn::Error::new(
						input.span(),
						"Having `self` is not allowed in RPC queries definitions",
					));
				}
				syn::FnArg::Typed(syn::PatType { ty, pat, attrs, .. }) => {
					(ty, pat.span(), rpc_param_name(&pat, &attrs)?)
				}
			};

			let generated_param_name =
				syn::Ident::new(&format!("param{}", param_index), proc_macro2::Span::call_site());

			params_tys.push(ty);
			params_list.push(quote_spanned!(pat_span=> #generated_param_name: impl Into<#ty>));
			params_to_json.push(quote_spanned!(pat_span=>
				map.insert(
					#rpc_param_name.to_string(),
					jsonrpsee_types::jsonrpc::to_value(#generated_param_name.into()).unwrap()        // TODO: don't unwrap
				);
			));
			params_to_array.push(quote_spanned!(pat_span =>
				jsonrpsee_types::jsonrpc::to_value(#generated_param_name.into()).unwrap()        // TODO: don't unwrap
			));
		}

		let params_building = if params_list.is_empty() {
			quote! {jsonrpsee_types::jsonrpc::Params::None}
		} else if function.attributes.positional_params {
			quote_spanned!(function.signature.span()=>
				jsonrpsee_types::jsonrpc::Params::Array(vec![
					#(#params_to_array),*
				])
			)
		} else {
			let params_list_len = params_list.len();
			quote_spanned!(function.signature.span()=>
				jsonrpsee_types::jsonrpc::Params::Map({
					let mut map = jsonrpsee_types::jsonrpc::JsonMap::with_capacity(#params_list_len);
					#(#params_to_json)*
					map
				})
			)
		};

		let is_notification = function.is_void_ret_type();
		let function_body = if is_notification {
			quote_spanned!(function.signature.span()=>
				client.notification(#rpc_method_name, #params_building).await
			)
		} else {
			quote_spanned!(function.signature.span()=>
				client.request(#rpc_method_name, #params_building).await
			)
		};

		client_functions.push(quote_spanned!(function.signature.span()=>
			// TODO: what if there's a conflict between `client` and a param name?
			#visibility async fn #f_name(client: &impl jsonrpsee_types::traits::Client #(, #params_list)*) -> core::result::Result<#ret_ty, jsonrpsee_types::error::Error>
			where
				#ret_ty: jsonrpsee_types::jsonrpc::DeserializeOwned
				#(, #params_tys: jsonrpsee_types::jsonrpc::Serialize)*
			{
				#function_body
			}
		));
	}

	Ok(client_functions)
}

// TODO: better docs
fn build_debug_variants(api: &api_def::ApiDefinition) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
	let enum_name = &api.name;
	let mut debug_variants = Vec::new();
	for function in &api.definitions {
		let variant_name = snake_case_to_camel_case(&function.signature.ident);
		debug_variants.push(quote_spanned!(function.signature.ident.span()=>
			#enum_name::#variant_name { /* TODO: params */ .. } => {
				f.debug_struct(stringify!(#enum_name))/* TODO: params */.finish()
			}
		));
	}
	Ok(debug_variants)
}

/// Turns a snake case function name into an UpperCamelCase name suitable to be an enum variant.
fn snake_case_to_camel_case(snake_case: &syn::Ident) -> syn::Ident {
	syn::Ident::new(&snake_case.to_string().to_pascal_case(), snake_case.span())
}

/// Determine the name of the variant in the enum based on the pattern of the function parameter.
fn param_variant_name(pat: &syn::Pat) -> syn::parse::Result<&syn::Ident> {
	match pat {
		// TODO: check other fields of the `PatIdent`
		syn::Pat::Ident(ident) => Ok(&ident.ident),
		_ => unimplemented!(),
	}
}

/// Determine the name of the parameter based on the pattern.
fn rpc_param_name(pat: &syn::Pat, _attrs: &[syn::Attribute]) -> syn::parse::Result<String> {
	// TODO: look in attributes if the user specified a param name
	match pat {
		// TODO: check other fields of the `PatIdent`
		syn::Pat::Ident(ident) => Ok(ident.ident.to_string()),
		_ => unimplemented!(),
	}
}
