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

use super::lifetimes::replace_lifetimes;
use super::RpcDescription;
use crate::helpers::{generate_where_clause, is_option};
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use std::collections::HashSet;

impl RpcDescription {
	pub(super) fn render_server(&self) -> Result<TokenStream2, syn::Error> {
		let trait_name = quote::format_ident!("{}Server", &self.trait_def.ident);
		let generics = self.trait_def.generics.clone();
		let (impl_generics, _, where_clause) = generics.split_for_impl();

		let method_impls = self.render_methods()?;
		let into_rpc_impl = self.render_into_rpc()?;

		let async_trait = self.jrps_server_item(quote! { types::__reexports::async_trait });

		// Doc-comment to be associated with the server.
		let doc_comment = format!("Server trait implementation for the `{}` RPC API.", &self.trait_def.ident);

		let trait_impl = quote! {
			#[#async_trait]
			#[doc = #doc_comment]
			pub trait #trait_name #impl_generics: Sized + Send + Sync + 'static #where_clause {
				#method_impls
				#into_rpc_impl
			}
		};

		Ok(trait_impl)
	}

	fn render_methods(&self) -> Result<TokenStream2, syn::Error> {
		let methods = self.methods.iter().map(|method| &method.signature);

		let subscription_sink_ty = self.jrps_server_item(quote! { SubscriptionSink });
		let subscriptions = self.subscriptions.iter().cloned().map(|mut sub| {
			// Add `SubscriptionSink` as the second input parameter to the signature.
			let subscription_sink: syn::FnArg = syn::parse_quote!(subscription_sink: #subscription_sink_ty);
			sub.signature.sig.inputs.insert(1, subscription_sink);
			sub.signature
		});

		Ok(quote! {
			#(#methods)*
			#(#subscriptions)*
		})
	}

	fn render_into_rpc(&self) -> Result<TokenStream2, syn::Error> {
		let rpc_module = self.jrps_server_item(quote! { RpcModule });

		let mut registered = HashSet::new();
		let mut errors = Vec::new();
		let mut check_name = |name: String, span: Span| {
			if registered.contains(&name) {
				let message = format!("{:?} is already defined", name);
				errors.push(quote_spanned!(span => compile_error!(#message);));
			} else {
				registered.insert(name);
			}
		};

		/// Helper that will ignore results of `register_*` method calls, and panic
		/// if there have been any errors in debug builds.
		///
		/// The debug assert is a safeguard should the contract that guarantees the method
		/// names to never conflict in the macro be broken in the future.
		fn handle_register_result(tokens: TokenStream2) -> TokenStream2 {
			quote! {{
				let res = #tokens;
				debug_assert!(res.is_ok(), "RPC macro method names should never conflict, this is a bug, please report it.");
			}}
		}

		let methods = self
			.methods
			.iter()
			.map(|method| {
				// Rust method to invoke (e.g. `self.<foo>(...)`).
				let rust_method_name = &method.signature.sig.ident;
				// Name of the RPC method (e.g. `foo_makeSpam`).
				let rpc_method_name = self.rpc_identifier(&method.name);
				// `parsing` is the code associated with parsing structure from the
				// provided `RpcParams` object.
				// `params_seq` is the comma-delimited sequence of parameters.
				let (parsing, params_seq) = self.render_params_decoding(&method.params);

				check_name(rpc_method_name.clone(), rust_method_name.span());

				if method.signature.sig.asyncness.is_some() {
					handle_register_result(quote! {
						rpc.register_async_method(#rpc_method_name, |params, context| {
							let fut = async move {
								#parsing
								context.as_ref().#rust_method_name(#params_seq).await
							};
							Box::pin(fut)
						})
					})
				} else {
					handle_register_result(quote! {
						rpc.register_method(#rpc_method_name, |params, context| {
							#parsing
							context.#rust_method_name(#params_seq)
						})
					})
				}
			})
			.collect::<Vec<_>>();

		let subscriptions = self
			.subscriptions
			.iter()
			.map(|sub| {
				// Rust method to invoke (e.g. `self.<foo>(...)`).
				let rust_method_name = &sub.signature.sig.ident;
				// Name of the RPC method to subscribe to (e.g. `foo_sub`).
				let rpc_sub_name = self.rpc_identifier(&sub.name);
				// Name of the RPC method to unsubscribe (e.g. `foo_sub`).
				let rpc_unsub_name = self.rpc_identifier(&sub.unsub_method);
				// `parsing` is the code associated with parsing structure from the
				// provided `RpcParams` object.
				// `params_seq` is the comma-delimited sequence of parameters.
				let (parsing, params_seq) = self.render_params_decoding(&sub.params);

				check_name(rpc_sub_name.clone(), rust_method_name.span());
				check_name(rpc_unsub_name.clone(), rust_method_name.span());

				handle_register_result(quote! {
					rpc.register_subscription(#rpc_sub_name, #rpc_unsub_name, |params, sink, context| {
						#parsing
						Ok(context.as_ref().#rust_method_name(sink, #params_seq))
					})
				})
			})
			.collect::<Vec<_>>();

		let doc_comment = "Collects all the methods and subscriptions defined in the trait \
								and adds them into a single `RpcModule`.";

		let sub_tys: Vec<syn::Type> = self.subscriptions.clone().into_iter().map(|s| s.item).collect();
		let where_clause = generate_where_clause(&self.trait_def, &sub_tys, false);

		// NOTE(niklasad1): empty where clause is valid rust syntax.
		Ok(quote! {
			#[doc = #doc_comment]
			fn into_rpc(self) -> #rpc_module<Self> where #(#where_clause,)* {
				let mut rpc = #rpc_module::new(self);

				#(#errors)*
				#(#methods)*
				#(#subscriptions)*

				rpc
			}
		})
	}

	fn render_params_decoding(&self, params: &[(syn::PatIdent, syn::Type)]) -> (TokenStream2, TokenStream2) {
		if params.is_empty() {
			return (TokenStream2::default(), TokenStream2::default());
		}

		let params_fields_seq = params.iter().map(|(name, _)| name);
		let params_fields = quote! { #(#params_fields_seq),* };

		// Code to decode sequence of parameters from a JSON array.
		let decode_array = {
			let decode_fields = params.iter().map(|(name, ty)| {
				if is_option(ty) {
					quote! {
						let #name: #ty = seq.optional_next()?;
					}
				} else {
					quote! {
						let #name: #ty = seq.next()?;
					}
				}
			});

			quote! {
				let mut seq = params.sequence();
				#(#decode_fields);*
			}
		};

		// Code to decode sequence of parameters from a JSON object (aka map).
		let _decode_map = {
			let mut generics = None;

			let serde = self.jrps_server_item(quote! { types::__reexports::serde });
			let serde_crate = serde.to_string();
			let fields = params
				.iter()
				.map(|(name, ty)| {
					let mut ty = ty.clone();

					if replace_lifetimes(&mut ty) {
						generics = Some(());
						quote! {
							#[serde(borrow)]
							#name: #ty,
						}
					} else {
						quote! { #name: #ty, }
					}
				})
				.collect::<Vec<_>>();
			let destruct = params.iter().map(|(name, _)| quote! { parsed.#name });
			let generics = generics.map(|()| quote! { <'a> });

			quote! {
				#[derive(#serde::Deserialize)]
				#[serde(crate = #serde_crate)]
				struct ParamsObject#generics {
					#(#fields)*
				}

				let parsed: ParamsObject = params.parse()?;

				(#(#destruct),*)
			}
		};

		// Parsing of `serde_json::Value`.
		let parsing = quote! {
			// TODO: https://github.com/paritytech/jsonrpsee/issues/445
			/*let (#params_fields) = if params.is_object() {
				#decode_map
			} else {
				#decode_array
			};*/
			#decode_array;
		};

		(parsing, params_fields)
	}
}
