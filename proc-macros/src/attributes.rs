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
	/// Alias for the method.
	pub alias: Option<syn::LitStr>,
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
	/// Alias for the subscribe method.
	pub sub_alias: Option<syn::LitStr>,
	/// Alias for the unsubscribe method.
	pub unsub_alias: Option<syn::LitStr>,
}
