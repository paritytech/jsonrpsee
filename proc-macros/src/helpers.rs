use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use std::collections::HashSet;
use syn::{
	parse_quote,
	punctuated::Punctuated,
	visit::{self, Visit},
	GenericParam, Generics, Token,
};

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

pub(crate) fn client_add_trait_bounds(item_trait: &syn::ItemTrait) -> Generics {
	let mut visitor = FindTyParams::default();
	visitor.visit_item_trait(item_trait);
	let mut generics = item_trait.generics.clone();

	for param in &mut generics.params {
		if let GenericParam::Type(ty) = param {
			ty.bounds.push(parse_quote!(Send));
			ty.bounds.push(parse_quote!(Sync));
			ty.bounds.push(parse_quote!('static));

			if visitor.input_params.contains(&ty.ident) {
				ty.bounds.push(parse_quote!(jsonrpsee::types::Serialize))
			}
			if visitor.ret_params.contains(&ty.ident) {
				ty.bounds.push(parse_quote!(jsonrpsee::types::DeserializeOwned))
			}
		}
	}
	generics
}

pub(crate) fn server_generate_where_clause(item_trait: &syn::ItemTrait) -> Vec<syn::WherePredicate> {
	let mut visitor = FindTyParams::default();
	visitor.visit_item_trait(item_trait);

	let additional_where_clause = item_trait.generics.where_clause.clone();

	item_trait
		.generics
		.type_params()
		.map(|ty| {
			let ty_path = syn::TypePath { qself: None, path: ty.ident.clone().into() };
			let mut bounds: Punctuated<syn::TypeParamBound, Token![+]> = parse_quote!(Send + Sync + 'static);

			if visitor.input_params.contains(&ty.ident) {
				bounds.push(parse_quote!(jsonrpsee::types::DeserializeOwned))
			}
			if visitor.ret_params.contains(&ty.ident) {
				bounds.push(parse_quote!(jsonrpsee::types::Serialize))
			}

			// Add the trait bounds specified in the trait.
			if let Some(ref where_clause) = additional_where_clause {
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

#[derive(Default)]
struct FindTyParams {
	trait_generics: HashSet<syn::Ident>,
	input_params: HashSet<syn::Ident>,
	ret_params: HashSet<syn::Ident>,
	visiting_return_type: bool,
	visiting_fn_arg: bool,
}
impl<'ast> Visit<'ast> for FindTyParams {
	fn visit_type_param(&mut self, ty_param: &'ast syn::TypeParam) {
		self.trait_generics.insert(ty_param.ident.clone());
	}

	fn visit_return_type(&mut self, return_type: &'ast syn::ReturnType) {
		self.visiting_return_type = true;
		visit::visit_return_type(self, return_type);
		self.visiting_return_type = false
	}

	fn visit_ident(&mut self, ident: &'ast syn::Ident) {
		if self.trait_generics.contains(ident) {
			if self.visiting_return_type {
				self.ret_params.insert(ident.clone());
			}
			if self.visiting_fn_arg {
				self.input_params.insert(ident.clone());
			}
		}
	}

	fn visit_fn_arg(&mut self, arg: &'ast syn::FnArg) {
		self.visiting_fn_arg = true;
		visit::visit_fn_arg(self, arg);
		self.visiting_fn_arg = false;
	}
}
