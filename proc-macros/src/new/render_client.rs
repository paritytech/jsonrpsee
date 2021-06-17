use super::{RpcDescription, RpcMethod, RpcSubscription};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl RpcDescription {
	pub(super) fn render_client(&self) -> Result<TokenStream2, syn::Error> {
		let jsonrpsee = self.jsonrpsee_client_path.as_ref().unwrap();

		let trait_name = quote::format_ident!("{}Client", &self.trait_def.ident);

		let super_trait = if self.subscriptions.is_empty() {
			quote! { #jsonrpsee::traits::Client }
		} else {
			quote! { #jsonrpsee::traits::SubscriptionClient }
		};

		let method_impls =
			self.methods.iter().map(|method| self.render_method(method)).collect::<Result<Vec<_>, _>>()?;
		let sub_impls = self.subscriptions.iter().map(|sub| self.render_sub(sub)).collect::<Result<Vec<_>, _>>()?;

		let async_trait = self.jrps_item(quote! { __reexports::async_trait });

		// Doc-comment to be associated with the client.
		let doc_comment = format!("Client implementation for the `{}` RPC API.", &self.trait_def.ident);

		let trait_impl = quote! {
			#[#async_trait]
			#[doc = #doc_comment]
			pub trait #trait_name: #super_trait {
				#(#method_impls)*
				#(#sub_impls)*
			}
		};

		Ok(trait_impl)
	}

	fn render_method(&self, method: &RpcMethod) -> Result<TokenStream2, syn::Error> {
		// `jsonrpsee::Error`
		let jrps_error = self.jrps_item(quote::format_ident!("Error"));
		// Rust method to invoke (e.g. `self.<foo>(...)`).
		let rust_method_name = &method.signature.sig.ident;
		// List of inputs to put into `JsonRpcParams` (e.g. `self.foo(<12, "baz">)`).
		// Includes `&self` receiver.
		let rust_method_params = &method.signature.sig.inputs;
		// Name of the RPC method (e.g. `foo_makeSpam`).
		let rpc_method_name = self.rpc_identifier(&method.name);

		// Called method is either `request` or `notification`.
		// `returns` represent the return type of the *rust method* (`Result< <..>, jsonrpsee::Error`).
		let (called_method, returns) = if let Some(returns) = &method.returns {
			let called_method = quote::format_ident!("request");
			let returns = quote! { Result<#returns, #jrps_error> };

			(called_method, returns)
		} else {
			let called_method = quote::format_ident!("notification");
			let returns = quote! { Result<(), #jrps_error> };

			(called_method, returns)
		};

		// Encoded parameters for the request.
		let parameters = if !method.params.is_empty() {
			let serde_json = self.jrps_item(quote! { __reexports::serde_json });
			let params = method.params.iter().map(|(param, _param_type)| {
				quote! { #serde_json::to_value(&#param)? }
			});

			quote! {
				vec![ #(#params),* ].into()
			}
		} else {
			self.jrps_item(quote::format_ident!("JsonRpcParams::NoParams"))
		};

		// Doc-comment to be associated with the method.
		let doc_comment = format!("Invokes the RPC method `{}`.", rpc_method_name);

		let method = quote! {
			#[doc = #doc_comment]
			async fn #rust_method_name(#rust_method_params) -> #returns {
				self.#called_method(#rpc_method_name, #parameters).await
			}
		};
		Ok(method)
	}

	fn render_sub(&self, sub: &RpcSubscription) -> Result<TokenStream2, syn::Error> {
		// `jsonrpsee::Error`
		let jrps_error = self.jrps_item(quote::format_ident!("Error"));
		// Rust method to invoke (e.g. `self.<foo>(...)`).
		let rust_method_name = &sub.signature.sig.ident;
		// List of inputs to put into `JsonRpcParams` (e.g. `self.foo(<12, "baz">)`).
		let rust_method_params = &sub.signature.sig.inputs;
		// Name of the RPC subscription (e.g. `foo_sub`).
		let rpc_sub_name = self.rpc_identifier(&sub.name);
		// Name of the RPC method to unsubscribe (e.g. `foo_unsub`).
		let rpc_unsub_name = self.rpc_identifier(&sub.unsub_method);

		// `returns` represent the return type of the *rust method*, which is wrapped
		// into the `Subscription` object.
		let sub_type = self.jrps_item(quote::format_ident!("Subscription"));
		let item = &sub.item;
		let returns = quote! { Result<#sub_type<#item>, #jrps_error> };

		// Encoded parameters for the request.
		let parameters = if !sub.params.is_empty() {
			let serde_json = self.jrps_item(quote! { __reexports::serde_json });
			let params = sub.params.iter().map(|(param, _param_type)| {
				quote! { #serde_json::to_value(&#param)? }
			});

			quote! {
				vec![ #(#params),* ].into()
			}
		} else {
			self.jrps_item("JsonRpcParams::NoParams")
		};

		// Doc-comment to be associated with the method.
		let doc_comment = format!("Subscribes to the RPC method `{}`.", rpc_sub_name);

		let method = quote! {
			#[doc = #doc_comment]
			async fn #rust_method_name(&self, #rust_method_params) -> #returns {
				self.subscribe(#rpc_sub_name, #parameters, #rpc_unsub_name).await
			}
		};
		Ok(method)
	}
}
