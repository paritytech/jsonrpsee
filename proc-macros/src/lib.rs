// Copyright 2019 Parity Technologies (UK) Ltd.
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

extern crate proc_macro;

use new::RpcDescription;
use proc_macro::TokenStream;
use quote::quote;

mod api_def;
mod client_builder;
mod helpers;
mod new;

/// Wraps around one or more API definitions and generates an enum.
///
/// The format within this macro must be:
///
/// ```ignore
/// jsonrpsee_proc_macros::rpc_client_api! {
///     Foo { ... }
///     pub(crate) Bar { ... }
/// }
/// ```
///
/// The `Foo` and `Bar` are identifiers, optionally prefixed with a visibility modifier
/// (e.g. `pub`).
///
/// The content of the blocks is the same as the content of a trait definition, except that
/// default implementations for methods are forbidden.
///
/// For each identifier (such as `Foo` and `Bar` in the example above), this macro will generate
/// an enum where each variant corresponds to a function of the definition. Function names are
/// turned into PascalCase to conform to the Rust style guide.
///
/// Additionally, each generated enum has one method per function definition that lets you perform
/// the method has a client.
///
// TODO(niklasad1): Generic type params for individual methods doesn't work
// because how the enum is generated, so for now type params must be declared on the entire enum.
// The reason is that all type params on the enum is bound as a separate variant but
// not generic params i.e, either params or return type.
// To handle that properly, all generic types has to be collected and applied to the enum, see example:
//
// ```rust
// jsonrpsee_rpc_client_api! {
//     Api {
//       // Doesn't work.
//       fn generic_notif<T>(t: T);
// }
// ```
//
// Expands to which doesn't compile:
// ```rust
// enum Api {
//    GenericNotif {
//        t: T,
//    },
// }
// ```
// The code should be expanded to (to compile):
// ```rust
// enum Api<T> {
//    GenericNotif {
//        t: T,
//    },
// }
// ```
#[proc_macro]
pub fn rpc_client_api(input_token_stream: TokenStream) -> TokenStream {
	// Start by parsing the input into what we expect.
	let defs: api_def::ApiDefinitions = match syn::parse(input_token_stream) {
		Ok(d) => d,
		Err(err) => return err.to_compile_error().into(),
	};

	let mut out = Vec::with_capacity(defs.apis.len());
	for api in defs.apis {
		match client_builder::build_client_api(api) {
			Ok(a) => out.push(a),
			Err(err) => return err.to_compile_error().into(),
		};
	}

	TokenStream::from(quote! {
		#(#out)*
	})
}

// New implementation starts here.

/// Main RPC macro.
///
/// ## Description
///
/// This macro is capable of generating both server and client implementations on demand.
/// Based on the attributes provided to the `rpc` macro, either one or both of implementations
/// will be generated.
///
/// For clients, it will be an extension trait that will add all the required methods to any
/// type that implements `Client` or `SubscriptionClient` (depending on whether trait has
/// subscriptions methods or not), namely `HttpClient` and `WsClient`.
///
/// For servers, it will generate a trait mostly equivalent to the initial one, with two main
/// differences:
///
/// - This trait will have one additional (already implemented) method `into_rpc`, which
///   will turn any object that implements the server trait into an `RpcModule`.
/// - For subscription methods, there will be one additional argument inserted right
///   after `&self`: `subscription_sink: SubscriptionSink`. It should be used to
///   actually maintain the subscription.
///
/// Since this macro can generate up to two traits, both server and client traits will have
/// a new name. For the `Foo` trait, server trait will be named `FooServer`, and client,
/// correspondingly, `FooClient`.
///
/// `FooClient` in that case will only have to be imported in the context and will be ready to
/// use, while `FooServer` must be implemented for some type first.
///
/// ## Prerequisites
///
/// - Implementors of the server trait must be `Sync`, `Send`, `Sized` and `'static`.
///   If you want to implement this trait to some type that is not thread-safe, consider
///   using `Arc<RwLock<..>>`.
///
/// ## Examples
///
/// Below you can find the example of the macro usage along with the code
/// that will be generated for it.
///
/// ```ignore
/// #[rpc(client, server, namespace = "foo")]
/// pub trait Rpc {
///     #[method(name = "foo")]
///     async fn async_method(&self, param_a: u8, param_b: String) -> u16;
///     #[method(name = "bar")]
///     fn sync_method(&self) -> String;
///     
///     #[subscription(name = "sub", unsub = "unsub", item = "String")]
///     fn sub(&self);
/// }
/// ```
///
/// Server code that will be generated:
///
/// ```ignore
/// #[async_trait]
/// pub trait RpcServer {
///     // RPC methods are usual methods and can be either sync or async.
///     async fn async_method(&self, param_a: u8, param_b: String) -> u16;
///     fn sync_method(&self) -> String;
///
///     // Note that `subscription_sink` was added automatically.
///     fn sub(&self, subscription_sink: SubscriptionSink);
///
///     fn into_rpc(self) -> Result<Self, jsonrpsee::types::error::Error> {
///         // Actual implementation stripped, but inside we will create
///         // a module with one method and one subscription
///     }
/// }
/// ```
///
/// Client code that will be generated:
///
/// ```ignore
/// #[async_trait]
/// pub trait RpcClient: SubscriptionClient {
///     // In client implementation all the methods are (obviously) async.
///     async fn async_method(&self, param_a: u8, param_b: String) -> Result<u16, Error> {
///         // Actual implementations are stripped, but inside a corresponding `Client` or
///         // `SubscriptionClient` method is called.
///     }
///     async fn sync_method(&self) -> Result<String, Error> {
///         // ...
///     }
///     
///     // Subscription method returns `Subscription` object in case of success.
///     async fn sub(&self, subscription_sink: SubscriptionSink) -> Result<Subscription<String>, Error> {
///         // ...
///     }
/// }
///
/// impl<T> RpcClient for T where T: SubscriptionClient {}
/// ```
///
/// ## Attributes
///
/// To be done...
#[proc_macro_attribute]
pub fn rpc(attr: TokenStream, item: TokenStream) -> TokenStream {
	let attr = proc_macro2::TokenStream::from(attr);

	let rebuilt_rpc_attribute = syn::Attribute {
		pound_token: syn::token::Pound::default(),
		style: syn::AttrStyle::Outer,
		bracket_token: syn::token::Bracket::default(),
		path: syn::Ident::new("rpc", proc_macro2::Span::call_site()).into(),
		tokens: quote! { (#attr) },
	};

	match rpc_impl(rebuilt_rpc_attribute, item) {
		Ok(tokens) => tokens,
		Err(err) => err.to_compile_error(),
	}
	.into()
}

/// Convenience form of `rpc` that may use `?` for error handling to avoid boilerplate.
fn rpc_impl(attr: syn::Attribute, item: TokenStream) -> Result<proc_macro2::TokenStream, syn::Error> {
	let trait_data: syn::ItemTrait = syn::parse(item)?;
	let rpc = RpcDescription::from_item(attr, trait_data)?;
	rpc.render()
}
