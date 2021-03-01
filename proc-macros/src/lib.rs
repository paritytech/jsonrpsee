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
use quote::{format_ident, quote, quote_spanned};
use std::collections::HashSet;
use syn::spanned::Spanned as _;

mod api_def;

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
	let generics = api.generics.clone();
	let mut non_used_type_params = HashSet::new();

	let mut variants = Vec::new();
	for function in &api.definitions {
		let variant_name = snake_case_to_camel_case(&function.signature.ident);
		if let syn::ReturnType::Type(_, ty) = &function.signature.output {
			non_used_type_params.insert(ty);
		};

		let mut params_list = Vec::new();

		for input in function.signature.inputs.iter() {
			let (ty, pat_span, param_variant_name) = match input {
				syn::FnArg::Receiver(_) => {
					return Err(syn::Error::new(
						input.span(),
						"Having `self` is not allowed in RPC queries definitions",
					));
				}
				syn::FnArg::Typed(syn::PatType { ty, pat, .. }) => (ty, pat.span(), param_variant_name(&pat)?),
			};
			params_list.push(quote_spanned!(pat_span=> #param_variant_name: #ty));
		}

		variants.push(quote_spanned!(function.signature.ident.span()=>
			#variant_name {
				#(#params_list,)*
			}
		));
	}

	let client_impl_block = build_client_impl(&api)?;
	// TODO(niklasad1): do we want debug impl here?
	//let debug_variants = build_debug_variants(&api)?;

	let mut ret_variants = Vec::new();
	for (idx, ty) in non_used_type_params.into_iter().enumerate() {
		// NOTE(niklasad1): variant names are converted from `snake_case` to `CamelCase`
		// It's impossible to have collisions between `_0, _1, ... _N`
		// Because variant name `_0`, `__0` becomes `0` in `CamelCase`
		// then `0` is not a valid identifier in Rust syntax and the error message is hard to understand.
		// Perhaps document this in macro when it's ready.
		let varname = format_ident!("_{}", idx);
		ret_variants.push(quote_spanned!(ty.span()=> #varname (#ty)));
	}

	Ok(quote_spanned!(api.name.span()=>
		#visibility enum #enum_name #generics {
			 #(#variants,)* #(#[allow(unused)] #ret_variants,)*
		}

		#client_impl_block

		// TODO(niklasad1): do we want debug impl here?
		/*impl #generics core::fmt::Debug  for #enum_name #generics {
			fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
				match self {
					#(#debug_variants,)* _ => f.debug_struct("ReturnType").finish()
				}
			}
		}*/
	))
}

/// Builds the impl block that allow performing outbound JSON-RPC queries.
///
/// Generates the `impl <enum> { }` block containing functions that perform RPC client calls.
fn build_client_impl(api: &api_def::ApiDefinition) -> Result<proc_macro2::TokenStream, syn::Error> {
	let enum_name = &api.name;

	let (impl_generics_org, type_generics, where_clause_org) = api.generics.split_for_impl();
	let client_functions = build_client_functions(&api)?;

	Ok(quote_spanned!(api.name.span() =>
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
					jsonrpsee_types::jsonrpc::to_value(#generated_param_name.into()).map_err(|e| jsonrpsee_types::error::Error::Custom(format!("{:?}", e)))?
				);
			));
			params_to_array.push(quote_spanned!(pat_span =>
				jsonrpsee_types::jsonrpc::to_value(#generated_param_name.into()).map_err(|e| jsonrpsee_types::error::Error::Custom(format!("{:?}", e)))?
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
			#visibility async fn #f_name (client: &impl jsonrpsee_types::traits::Client #(, #params_list)*) -> core::result::Result<#ret_ty, jsonrpsee_types::error::Error>
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
