use super::RpcDescription;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl RpcDescription {
	pub(super) fn render_server(&self) -> Result<TokenStream2, syn::Error> {
		let trait_name = quote::format_ident!("{}Server", &self.trait_def.ident);

		let method_impls = self.render_methods()?;
		let into_rpc_impl = self.render_into_rpc()?;

		let async_trait = self.jrps_server_item(quote! { __reexports::async_trait });
		// panic!("Parsing is {}", async_trait.to_string());

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
		let subscriptions = self.subscriptions.iter().map(|sub| &sub.signature);

		Ok(quote! {
			#(#methods)*
			#(#subscriptions)*
		})
	}

	fn render_into_rpc(&self) -> Result<TokenStream2, syn::Error> {
		let jsonrpsee = &self.jsonrpsee_server_path.as_ref().unwrap();
		let jrps_error = self.jrps_server_item(quote! { error::CallError });
		let rpc_module = self.jrps_server_item(quote! { RpcModule });

		let methods = self.methods.iter().map(|method| {
			// Rust method to invoke (e.g. `self.<foo>(...)`).
			let rust_method_name = &method.signature.sig.ident;
			// Name of the RPC method (e.g. `foo_makeSpam`).
			let rpc_method_name = self.rpc_identifier(&method.name);
			// `parsing` is the code associated with parsing structure from the
			// provided `RpcParams` object.
			// `params_seq` is the comma-delimited sequence of parametsrs.
			let (parsing, params_seq) = self.render_params_decoding(&rpc_method_name, &method.params);

			if method.signature.sig.asyncness.is_some() {
				quote! {
					rpc.register_async_method(#rpc_method_name, |params, context| {
						let fut = async move {
							#parsing
							context.as_ref().#rust_method_name(#params_seq).await
						};
						Box::pin(fut)
					})?;
				}
			} else {
				quote! {
					rpc.register_method(#rpc_method_name, |params, context| {
						#parsing
						context.#rust_method_name(#params_seq)
					})?;
				}
			}
		});

		Ok(quote! {
			fn into_rpc(self) -> Result<#rpc_module<Self>, #jrps_error> {
				let mut rpc = #rpc_module::new(self);

				#(#methods)*

				Ok(rpc)
			}
		})
	}

	fn render_params_decoding(
		&self,
		method: &str,
		params: &[(syn::PatIdent, syn::Type)],
	) -> (TokenStream2, TokenStream2) {
		if params.is_empty() {
			return (TokenStream2::default(), TokenStream2::default());
		}

		let jrps_error = self.jrps_server_item(quote! { error::CallError });
		let serde_json = self.jrps_server_item(quote! { __reexports::serde_json });

		// Parameters encoded as a tuple (to be parsed from array).
		let (params_fields_seq, params_types_seq): (Vec<_>, Vec<_>) = params.iter().cloned().unzip();
		let params_types = quote! { (#(#params_types_seq),*) };
		let params_fields = quote! { (#(#params_fields_seq),*) };

		let decode_array = quote! {};
		let decode_map = quote! {};
		let decode_single = if params.len() == 1 {
			quote! {}
		} else {
			quote! { return Err(#jrps_error::InvalidParams);}
		};

		// TODO: Will fail if params are represented by the array
		let parsing = quote! {
			let json: #serde_json::Value = params.parse()?;
			let #params_fields: #params_types = match json {
				#serde_json::Value::Null => return Err(#jrps_error::InvalidParams),
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
