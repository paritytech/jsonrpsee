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

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use rpc_macro::RpcDescription;

mod attributes;
mod helpers;
mod render_client;
mod render_server;
mod rpc_macro;
pub(crate) mod visitor;

/// Main RPC macro.
///
/// ## Description
///
/// This macro is capable of generating both server and client implementations on demand.
/// Based on the attributes provided to the `rpc` macro, either one or both of implementations
/// will be generated.
///
/// For clients, it will be an extension trait that adds all the required methods to a
/// type that implements `Client` or `SubscriptionClient` (depending on whether trait has
/// subscriptions methods or not), namely `HttpClient` and `WsClient`.
///
/// For servers, it will generate a trait mostly equivalent to the input, with the following differences:
///
/// - The trait will have one additional (already implemented) method, `into_rpc`, which turns any object that
///   implements the server trait into an `RpcModule`.
/// - For subscription methods:
///   - There will be one additional argument inserted right after `&self`: `subscription_sink: SubscriptionSink`.
///   It should be used to accept or reject a subscription and send data back to subscribers.
///   - The return type of the subscription method is `SubscriptionResult` for improved ergonomics.
///
/// Since this macro can generate up to two traits, both server and client traits will have
/// a new name. For the `Foo` trait, server trait will be named `FooServer`, and client,
/// correspondingly, `FooClient`.
///
/// To use the `FooClient`, just import it in the context. To use the server, the `FooServer` trait must be implemented
/// on your type first.
///
/// Note: you need to import the `jsonrpsee` façade crate in your code for the macro to work properly.
///
/// ## Prerequisites
///
/// - Implementors of the server trait must be `Sync`, `Send`, `Sized` and `'static`. If you want to implement this
///   trait on some type that is not thread-safe, consider using `Arc<RwLock<..>>`.
///
/// ## Examples
///
/// Below you can find examples of the macro usage along with the code
/// that generated for it by the macro.
///
/// ```ignore
/// #[rpc(client, server, namespace = "foo")]
/// pub trait Rpc {
///     #[method(name = "foo")]
///     async fn async_method(&self, param_a: u8, param_b: String) -> u16;
///     #[method(name = "bar")]
///     fn sync_method(&self) -> String;
///
///     #[subscription(name = "subscribe", item = "String")]
///     fn sub(&self);
/// }
/// ```
///
/// Server code that will be generated:
///
/// ```ignore
/// #[async_trait]
/// pub trait RpcServer {
///     // RPC methods are normal methods and can be either sync or async.
///     async fn async_method(&self, param_a: u8, param_b: String) -> u16;
///     fn sync_method(&self) -> String;
///
///     // Note that `subscription_sink` and `SubscriptionResult` were added automatically.
///     fn sub(&self, subscription_sink: SubscriptionResult) -> SubscriptionResult;
///
///     fn into_rpc(self) -> Result<Self, jsonrpsee::core::Error> {
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
///     async fn sub(&self) -> Result<Subscription<String>, Error> {
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
/// - `client`: generate `<Trait>Client` extension trait that builds RPC clients to invoke a concrete RPC
///   implementation's methods conveniently.
/// - `namespace`: add a prefix to all the methods and subscriptions in this RPC. For example, with namespace `foo` and
///   method `spam`, the resulting method name will be `foo_spam`.
/// - `server_bounds`: replace *all* auto-generated trait bounds with the user-defined ones for the server
///   implementation.
/// - `client_bounds`: replace *all* auto-generated trait bounds with the user-defined ones for the client
///   implementation.
///
/// **Trait requirements:**
///
/// A trait wrapped with the `rpc` attribute **must not**:
///
/// - have associated types or constants;
/// - have Rust methods not marked with either the `method` or `subscription` attribute;
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
/// - `aliases`: list of name aliases for the RPC method as a comma separated string.
///              Aliases are processed ignoring the namespace, so add the complete name, including the
///              namespace.
/// - `blocking`: when set method execution will always spawn on a dedicated thread. Only usable with non-`async` methods.
/// - `param_kind`: kind of structure to use for parameter passing. Can be "array" or "map", defaults to "array".
///
/// **Method requirements:**
///
/// A Rust method marked with the `method` attribute, **may**:
///
/// - be either `async` or not;
/// - have input parameters or not;
/// - have a return value or not (in the latter case, it will be considered a notification method).
///
/// ### `subscription` attribute
///
/// `subscription` attribute is used to define a publish/subscribe interface according to the [ethereum pubsub specification](https://geth.ethereum.org/docs/rpc/pubsub)
///
/// **Arguments:**
///
/// - `name` (mandatory): name of the RPC method. Does not have to be the same as the Rust method name.
/// - `unsubscribe` (optional): name of the RPC method to unsubscribe from the subscription. Must not be the same as `name`.
///                             This is generated for you if the subscription name starts with `subscribe`.
/// - `aliases` (optional): aliases for `name`. Aliases are processed ignoring the namespace,
///                         so add the complete name, including the namespace.
/// - `unsubscribe_aliases` (optional): Similar to `aliases` but for `unsubscribe`.
/// - `item` (mandatory): type of items yielded by the subscription. Note that it must be the type, not string.
/// - `param_kind`: kind of structure to use for parameter passing. Can be "array" or "map", defaults to "array".
///
/// **Method requirements:**
///
/// Rust method marked with the `subscription` attribute **must**:
///
/// - be synchronous;
/// - return `RpcResult<()>`
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
/// // RPC is put into a separate module to clearly show names of generated entities.
/// mod rpc_impl {
///     use jsonrpsee::{proc_macros::rpc, core::async_trait, core::RpcResult, ws_server::SubscriptionSink};
///     use jsonrpsee::types::SubscriptionResult;
///
///     // Generate both server and client implementations, prepend all the methods with `foo_` prefix.
///     #[rpc(client, server, namespace = "foo")]
///     pub trait MyRpc {
///         #[method(name = "foo")]
///         async fn async_method(&self, param_a: u8, param_b: String) -> RpcResult<u16>;
///
///         #[method(name = "bar")]
///         fn sync_method(&self) -> RpcResult<u16>;
///
///         #[method(name = "baz", blocking)]
///         fn blocking_method(&self) -> RpcResult<u16>;
///
///         /// Override the `foo_sub` and use `foo_subNotif` for the notifications.
///         ///
///         /// The item field indicates which type goes into result field below.
///         ///
///         /// The notification format:
///         ///
///         /// ```
///         /// {
///         ///     "jsonrpc":"2.0",
///         ///     "method":"foo_subNotif",
///         ///     "params":["subscription":"someID", "result":"some string"]
///         /// }
///         /// ```
///         #[subscription(name = "sub" => "subNotif", unsubscribe = "unsub", item = String)]
///         fn sub_override_notif_method(&self);
///
///         /// Use the same method name for both the `subscribe call` and `notifications`
///         ///
///         /// The unsubscribe method name is generated here `foo_unsubscribe`
///         /// Thus the `unsubscribe attribute` is not needed unless a custom unsubscribe method name is wanted.
///         ///
///         /// The notification format:
///         ///
///         /// ```
///         /// {
///         ///     "jsonrpc":"2.0",
///         ///     "method":"foo_subscribe",
///         ///     "params":["subscription":"someID", "result":"some string"]
///         /// }
///         /// ```
///         #[subscription(name = "subscribe", item = String)]
///         fn sub(&self);
///     }
///
///     // Structure that will implement the `MyRpcServer` trait.
///     // It can have fields, if required, as long as it's still `Send + Sync + 'static`.
///     pub struct RpcServerImpl;
///
///     // Note that the trait name we use is `MyRpcServer`, not `MyRpc`!
///     #[async_trait]
///     impl MyRpcServer for RpcServerImpl {
///         async fn async_method(&self, _param_a: u8, _param_b: String) -> RpcResult<u16> {
///             Ok(42)
///         }
///
///         fn sync_method(&self) -> RpcResult<u16> {
///             Ok(10)
///         }
///
///         fn blocking_method(&self) -> RpcResult<u16> {
///             // This will block current thread for 1 second, which is fine since we marked
///             // this method as `blocking` above.
///             std::thread::sleep(std::time::Duration::from_millis(1000));
///             Ok(11)
///         }
///
///         // The stream API can be used to pipe items from the underlying stream
///         // as subscription responses.
///         fn sub_override_notif_method(&self, mut sink: SubscriptionSink) -> SubscriptionResult {
///             tokio::spawn(async move {
///                 let stream = futures_util::stream::iter(["one", "two", "three"]);
///                 sink.pipe_from_stream(stream).await;
///             });
///             Ok(())
///         }
///
///         // We could've spawned a `tokio` future that yields values while our program works,
///         // but for simplicity of the example we will only send two values and then close
///         // the subscription.
///         fn sub(&self, mut sink: SubscriptionSink) -> SubscriptionResult {
///             let _ = sink.send(&"Response_A");
///             let _ = sink.send(&"Response_B");
///             Ok(())
///         }
///     }
/// }
///
/// // Use the generated implementations of server and client.
/// use rpc_impl::{MyRpcClient, MyRpcServer, RpcServerImpl};
///
/// pub async fn websocket_server() -> SocketAddr {
///     let server = WsServerBuilder::default().build("127.0.0.1:0").await.unwrap();
///     let addr = server.local_addr().unwrap();
///
///     // `into_rpc()` method was generated inside of the `RpcServer` trait under the hood.
///     server.start(RpcServerImpl.into_rpc()).unwrap();
///
///     addr
/// }
///
/// // In the main function, we start the server, create a client connected to this server,
/// // and call the available methods.
/// #[tokio::main]
/// async fn main() {
///     let server_addr = websocket_server().await;
///     let server_url = format!("ws://{}", server_addr);
///     // Note that we create the client as usual, but thanks to the `use rpc_impl::MyRpcClient`,
///     // the client object will have all the methods to interact with the server.
///     let client = WsClientBuilder::default().build(&server_url).await.unwrap();
///
///     // Invoke RPC methods.
///     assert_eq!(client.async_method(10, "a".into()).await.unwrap(), 42);
///     assert_eq!(client.sync_method().await.unwrap(), 10);
///
///     // Subscribe and receive messages from the subscription.
///     let mut sub = client.sub().await.unwrap();
///     let first_recv = sub.next().await.unwrap().unwrap();
///     assert_eq!(first_recv, "Response_A".to_string());
///     let second_recv = sub.next().await.unwrap().unwrap();
///     assert_eq!(second_recv, "Response_B".to_string());
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
