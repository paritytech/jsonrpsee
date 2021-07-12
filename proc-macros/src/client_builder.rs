use quote::{format_ident, quote, quote_spanned};
use std::collections::HashSet;
use syn::spanned::Spanned as _;

use crate::helpers::*;

/// Generates the macro output token stream corresponding to a single API.
pub fn build_client_api(api: crate::api_def::ApiDefinition) -> Result<proc_macro2::TokenStream, syn::Error> {
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
			 #(#[allow(unused)] #variants,)* #(#[allow(unused)] #ret_variants,)*
		}

		#client_impl_block
	))
}

/// Builds the impl block that allow performing outbound JSON-RPC queries.
///
/// Generates the `impl <enum> { }` block containing functions that perform RPC client calls.
fn build_client_impl(api: &crate::api_def::ApiDefinition) -> Result<proc_macro2::TokenStream, syn::Error> {
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
fn build_client_functions(api: &crate::api_def::ApiDefinition) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
	let visibility = &api.visibility;

	let _crate = find_jsonrpsee_client_crate()?;

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
					#rpc_param_name,
					#_crate::types::to_json_value(#generated_param_name.into()).map_err(#_crate::types::Error::ParseError)?
				);
			));
			params_to_array.push(quote_spanned!(pat_span=>
				#_crate::types::to_json_value(#generated_param_name.into()).map_err(#_crate::types::Error::ParseError)?
			));
		}

		let params_building = if params_list.is_empty() {
			quote_spanned!(function.signature.span()=> #_crate::types::v2::params::JsonRpcParams::NoParams)
		} else if function.attributes.positional_params {
			quote_spanned!(function.signature.span()=> vec![#(#params_to_array),*].into())
		} else {
			quote_spanned!(function.signature.span()=>
				{
					let mut map = std::collections::BTreeMap::new();
					#(#params_to_json)*
					map.into()
				}
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
			#visibility async fn #f_name (client: &impl #_crate::types::traits::Client #(, #params_list)*) -> core::result::Result<#ret_ty, #_crate::types::Error>
			where
				#ret_ty: #_crate::types::DeserializeOwned
				#(, #params_tys: #_crate::types::Serialize)*
			{
				#function_body
			}
		));
	}

	Ok(client_functions)
}
