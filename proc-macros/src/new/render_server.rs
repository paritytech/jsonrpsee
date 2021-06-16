use super::RpcDescription;
use proc_macro2::TokenStream as TokenStream2;

impl RpcDescription {
	pub(super) fn render_server(&self) -> Result<TokenStream2, syn::Error> {
		todo!()
	}
}
