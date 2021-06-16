use super::{RpcDescription, RpcMethod, RpcSubscription};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

impl RpcDescription {
	pub(super) fn render_client(&self) -> Result<TokenStream2, syn::Error> {
		let jsonrpsee = &self.jsonrpsee_path;

		let trait_name = format!("{}Client", &self.trait_def.ident);

		let super_trait = if self.subscriptions.is_empty() {
			quote! { #jsonrpsee::Client }
		} else {
			quote! { #jsonrpsee::SubscriptionClient }
		};

		let method_impls =
			self.methods.iter().map(|method| self.render_method(method)).collect::<Result<Vec<_>, _>>()?;
		let sub_impls = self.subscriptions.iter().map(|sub| self.render_sub(sub)).collect::<Result<Vec<_>, _>>()?;

		let trait_impl = quote! {
			pub trait #trait_name: #super_trait {
				#(#method_impls)*
				#(#sub_impls)*
			}
		};

		Ok(trait_impl)
	}

	fn render_method(&self, _method: &RpcMethod) -> Result<TokenStream2, syn::Error> {
		todo!()
	}

	fn render_sub(&self, _sub: &RpcSubscription) -> Result<TokenStream2, syn::Error> {
		todo!()
	}
}
