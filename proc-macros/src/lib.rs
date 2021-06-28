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
/// ### `rpc` attribute
///
/// `rpc` attribute is applied to a trait in order to turn it into an RPC implementation.
///
/// **Arguments:**
///
/// - `server`: generate `<Trait>Server` trait for the server implementation.
/// - `client`: generate `<Trait>Client` extension trait that makes RPC clients to invoke a concrete RPC
///   implementation methods conveniently.
/// - `namespace`: add a prefix to all the methods and subscriptions in this RPC. For example, with namespace
///   `foo` and method `spam`, the resulting method name will be `foo_spam`.
///
/// **Trait requirements:**
///
/// Trait wrapped with an `rpc` attribute **must not**:
///
/// - have associated types or constants;
/// - have Rust methods not marked with either `method` or `subscription` attribute;
/// - be empty.
///
/// At least one of the `server` or `client` flags must be provided, otherwise the compilation will err.
///
/// ### `method` attribute
///
/// `method` attribute is used to define an RPC method.
///
/// **Arguments:**
///
/// - `name` (mandatory): name of the RPC method. Does not have to be the same as the Rust method name.
///
/// **Method requirements:**
///
/// Rust method marked with `method` attribute, **may**:
///
/// - be either `async` or not;
/// - have input parameters or not;
/// - have return value or not (in the latter case, it will be considered a notification method).
///
/// ### `subscription` attribute
///
/// **Arguments:**
///
/// - `name` (mandatory): name of the RPC method. Does not have to be the same as the Rust method name.
/// - `unsub` (mandatory): name of the RPC method to unsubscribe from the subscription. Must not be the same as `name`.
/// - `item` (mandatory): type of items yielded by the subscription. Note that it must be the type, not string.
///
/// **Method requirements:**
///
/// Rust method marked with `subscription` attribute **must**:
///
/// - be synchronous;
/// - not have return value.
///
/// Rust method marked with `subscription` attribute **may**:
///
/// - have input parameters or not.
///
/// ## Full workflow example
///
/// ```rust
/// //! Example of using proc macro to generate working client and server.
///
/// use std::net::SocketAddr;
///
/// use futures_channel::oneshot;
/// use jsonrpsee::{ws_client::*, ws_server::WsServerBuilder};
///
/// // RPC is moved into a separate module to clearly show names of generated entities.
/// mod rpc_impl {
///     use jsonrpsee::{proc_macros::rpc, types::async_trait, ws_server::SubscriptionSink};
///
///     // Generate both server and client implementations, prepend all the methods with `foo_` prefix.
///     #[rpc(client, server, namespace = "foo")]
///     pub trait Rpc {
///         #[method(name = "foo")]
///         async fn async_method(&self, param_a: u8, param_b: String) -> u16;
///
///         #[method(name = "bar")]
///         fn sync_method(&self) -> u16;
///
///         #[subscription(name = "sub", unsub = "unsub", item = String)]
///         fn sub(&self);
///     }
///
///     // Structure that will implement `RpcServer` trait.
///     // In can have fields, if required, as long as it's still `Send + Sync + 'static`.
///     pub struct RpcServerImpl;
///
///     #[async_trait]
///     impl RpcServer for RpcServerImpl {
///         async fn async_method(&self, _param_a: u8, _param_b: String) -> u16 {
///             42u16
///         }
///
///         fn sync_method(&self) -> u16 {
///             10u16
///         }
///
///         // We could've spawned a `tokio` future that yields values while our program works,
///         // but for simplicity of the example we will only send two values and then close
///         // the subscription.
///         fn sub(&self, mut sink: SubscriptionSink) {
///             sink.send(&"Response_A").unwrap();
///             sink.send(&"Response_B").unwrap();
///         }
///     }
/// }
///
/// // Use generated implementations of server and client.
/// use rpc_impl::{RpcClient, RpcServer, RpcServerImpl};
///
/// pub async fn websocket_server() -> SocketAddr {
///     let (server_started_tx, server_started_rx) = oneshot::channel();
///
///     std::thread::spawn(move || {
///         let rt = tokio::runtime::Runtime::new().unwrap();
///         let mut server = rt.block_on(WsServerBuilder::default().build("127.0.0.1:0")).unwrap();
///         // `into_rpc()` method was generated inside of the `RpcServer` trait under the hood.
///         server.register_module(RpcServerImpl.into_rpc().unwrap()).unwrap();
///
///         rt.block_on(async move {
///             server_started_tx.send(server.local_addr().unwrap()).unwrap();
///
///             server.start().await
///         });
///     });
///
///     server_started_rx.await.unwrap()
/// }
///
/// // In the main function, we will spawn the server, create a client connected to this server,
/// // and call all the available methods.
/// #[tokio::main]
/// async fn main() {
///     let server_addr = websocket_server().await;
///     let server_url = format!("ws://{}", server_addr);
///     // Note that we create the client as usual, but thanks to the `use rpc_impl::RpcClient`,
///     // the client object will have all the methods to interact with the server.
///     let client = WsClientBuilder::default().build(&server_url).await.unwrap();
///
///     // Invoke RPC methods.
///     assert_eq!(client.async_method(10, "a".into()).await.unwrap(), 42);
///     assert_eq!(client.sync_method().await.unwrap(), 10);
///
///     // Subscribe and receive messages from the subscription.
///     let mut sub = client.sub().await.unwrap();
///     let first_recv = sub.next().await.unwrap();
///     assert_eq!(first_recv, Some("Response_A".to_string()));
///     let second_recv = sub.next().await.unwrap();
///     assert_eq!(second_recv, Some("Response_B".to_string()));
/// }
/// ```
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
