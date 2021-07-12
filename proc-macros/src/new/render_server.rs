use super::RpcDescription;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl RpcDescription {
	pub(super) fn render_server(&self) -> Result<TokenStream2, syn::Error> {
		let trait_name = quote::format_ident!("{}Server", &self.trait_def.ident);

		let method_impls = self.render_methods()?;
		let into_rpc_impl = self.render_into_rpc()?;

		let async_trait = self.jrps_server_item(quote! { types::__reexports::async_trait });

		// Doc-comment to be associated with the server.
		let doc_comment = format!("Server trait implementation for the `{}` RPC API.", &self.trait_def.ident);

		let trait_impl = quote! {
			#[#async_trait]
			#[doc = #doc_comment]
			pub trait #trait_name: Sized + Send + Sync + 'static {
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
		let jrps_error = self.jrps_server_item(quote! { types::Error });
		let rpc_module = self.jrps_server_item(quote! { RpcModule });

		let methods = self.methods.iter().map(|method| {
			// Rust method to invoke (e.g. `self.<foo>(...)`).
			let rust_method_name = &method.signature.sig.ident;
			// Name of the RPC method (e.g. `foo_makeSpam`).
			let rpc_method_name = self.rpc_identifier(&method.name);
			// `parsing` is the code associated with parsing structure from the
			// provided `RpcParams` object.
			// `params_seq` is the comma-delimited sequence of parametsrs.
			let is_method = true;
			let (parsing, params_seq) = self.render_params_decoding(&method.params, is_method);

			if method.signature.sig.asyncness.is_some() {
				quote! {
					rpc.register_async_method(#rpc_method_name, |params, context| {
						let fut = async move {
							#parsing
							Ok(context.as_ref().#rust_method_name(#params_seq).await)
						};
						Box::pin(fut)
					})?;
				}
			} else {
				quote! {
					rpc.register_method(#rpc_method_name, |params, context| {
						#parsing
						Ok(context.#rust_method_name(#params_seq))
					})?;
				}
			}
		});

		let subscriptions = self.subscriptions.iter().map(|sub| {
			// Rust method to invoke (e.g. `self.<foo>(...)`).
			let rust_method_name = &sub.signature.sig.ident;
			// Name of the RPC method to subscribe (e.g. `foo_sub`).
			let rpc_sub_name = self.rpc_identifier(&sub.name);
			// Name of the RPC method to unsubscribe (e.g. `foo_sub`).
			let rpc_unsub_name = self.rpc_identifier(&sub.unsub_method);
			// `parsing` is the code associated with parsing structure from the
			// provided `RpcParams` object.
			// `params_seq` is the comma-delimited sequence of parametsrs.
			let is_method = false;
			let (parsing, params_seq) = self.render_params_decoding(&sub.params, is_method);

			quote! {
				rpc.register_subscription(#rpc_sub_name, #rpc_unsub_name, |params, sink, context| {
					#parsing
					Ok(context.as_ref().#rust_method_name(sink, #params_seq))
				})?;
			}
		});

		let doc_comment = "Collects all the methods and subscriptions defined in the trait \
								and adds them into a single `RpcModule`.";

		Ok(quote! {
			#[doc = #doc_comment]
			fn into_rpc(self) -> Result<#rpc_module<Self>, #jrps_error> {
				let mut rpc = #rpc_module::new(self);

				#(#methods)*
				#(#subscriptions)*

				Ok(rpc)
			}
		})
	}

	fn render_params_decoding(
		&self,
		params: &[(syn::PatIdent, syn::Type)],
		is_method: bool,
	) -> (TokenStream2, TokenStream2) {
		if params.is_empty() {
			return (TokenStream2::default(), TokenStream2::default());
		}

		// Implementations for `.map_err(...)?` and `.ok_or(...)?` with respect to the expected
		// error return type.
		let (err, map_err_impl, ok_or_impl) = if is_method {
			// For methods, we return `CallError`.
			let jrps_call_error = self.jrps_server_item(quote! { types::CallError });
			let err = quote! { #jrps_call_error::InvalidParams };
			let map_err = quote! { .map_err(|_| #jrps_call_error::InvalidParams)? };
			let ok_or = quote! { .ok_or(#jrps_call_error::InvalidParams)? };
			(err, map_err, ok_or)
		} else {
			// For subscriptions, we return `Error`.
			// Note that while `Error` can be constructed from `CallError`, we should not do it,
			// because it would be an abuse of the error type semantics.
			// Instead, we use suitable top-level error variants.
			let jrps_error = self.jrps_server_item(quote! { types::Error });
			let err = quote! { #jrps_error::Request("Required paramater missing".into()) };
			let map_err = quote! { .map_err(|err| #jrps_error::ParseError(err))? };
			let ok_or = quote! { .ok_or(#jrps_error::Request("Required paramater missing".into()))? };
			(err, map_err, ok_or)
		};

		let serde_json = self.jrps_server_item(quote! { types::__reexports::serde_json });

		// Parameters encoded as a tuple (to be parsed from array).
		let (params_fields_seq, params_types_seq): (Vec<_>, Vec<_>) = params.iter().cloned().unzip();
		let params_types = quote! { (#(#params_types_seq),*) };
		let params_fields = quote! { (#(#params_fields_seq),*) };

		// Code to decode sequence of parameters from a JSON array.
		let decode_array = {
			let decode_fields = params.iter().enumerate().map(|(id, (name, ty))| {
				if is_option(ty) {
					quote! {
						let #name = arr
							.get(#id)
							.cloned()
							.map(#serde_json::from_value)
							.transpose()
							#map_err_impl;
					}
				} else {
					quote! {
						let #name = arr
							.get(#id)
							.cloned()
							.map(#serde_json::from_value)
							#ok_or_impl
							#map_err_impl;
					}
				}
			});

			quote! {
				#(#decode_fields);*
				#params_fields
			}
		};

		// Code to decode sequence of parameters from a JSON object (aka map).
		let decode_map = {
			let decode_fields = params.iter().map(|(name, ty)| {
				let name_str = name.ident.to_string();
				if is_option(ty) {
					quote! {
						let #name = obj
							.get(#name_str)
							.cloned()
							.map(#serde_json::from_value)
							.transpose()
							#map_err_impl;
					}
				} else {
					quote! {
						let #name = obj
							.get(#name_str)
							.cloned()
							.map(#serde_json::from_value)
							#ok_or_impl
							#map_err_impl;
					}
				}
			});

			quote! {
				#(#decode_fields);*
				#params_fields
			}
		};

		// Code to decode single parameter from a JSON primitive.
		let decode_single = if params.len() == 1 {
			quote! {
				#serde_json::from_value(json)
				#map_err_impl
			}
		} else {
			quote! { return Err(#err);}
		};

		// Parsing of `serde_json::Value`.
		let parsing = quote! {
			let json: #serde_json::Value = params.parse()?;
			let #params_fields: #params_types = match json {
				#serde_json::Value::Null => return Err(#err),
				#serde_json::Value::Array(arr) => {
					#decode_array
				}
				#serde_json::Value::Object(obj) => {
					#decode_map
				}
				_ => {
					#decode_single
				}
			};
		};

		let seq = quote! {
			#(#params_fields_seq),*
		};

		(parsing, seq)
	}
}

/// Checks whether provided type is an `Option<...>`.
fn is_option(ty: &syn::Type) -> bool {
	if let syn::Type::Path(path) = ty {
		// TODO: Probably not the best way to check whether type is an `Option`.
		if path.path.segments.iter().any(|seg| seg.ident == "Option") {
			return true;
		}
	}

	false
}
