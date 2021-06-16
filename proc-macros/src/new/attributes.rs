use bae::FromAttributes;

/// Input for the `#[rpc(...)]` attribute macro.
#[derive(Debug, Clone, FromAttributes)]
pub(crate) struct Rpc {
	/// Switch denoting that server trait must be generated.
	/// Assuming that trait to which attribute is applied is named `Foo`, the generated
	/// server trait will have `FooServer` name.
	pub server: Option<()>,
	/// Switch denoting that client extension trait must be generated.
	/// Assuming that trait to which attribute is applied is named `Foo`, the generated
	/// client trait will have `FooClient` name.
	pub client: Option<()>,
	/// Optional prefix for RPC namespace.
	pub namespace: Option<syn::LitStr>,
}

impl Rpc {
	/// Returns `true` if at least one of `server` or `client` attributes is present.
	pub(crate) fn is_correct(&self) -> bool {
		self.server.is_some() || self.client.is_some()
	}

	/// Returns `true` if server implementation was requested.
	pub(crate) fn needs_server(&self) -> bool {
		self.server.is_some()
	}

	/// Returns `true` if client implementation was requested.
	pub(crate) fn needs_client(&self) -> bool {
		self.client.is_some()
	}
}

/// Input for the `#[method(...)]` attribute.
#[derive(Debug, Clone, FromAttributes)]
pub(crate) struct Method {
	/// Method name
	pub name: syn::LitStr,
}

/// Input for the `#[subscription(...)]` attribute.
#[derive(Debug, Clone, FromAttributes)]
pub(crate) struct Subscription {
	/// Subscription name
	pub name: syn::LitStr,
	/// Name of the method to unsubscribe.
	pub unsub: syn::LitStr,
	/// Type yielded by the subscription.
	pub item: syn::Type,
}
