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

//! JSON-RPC clients, servers, and utilities.
//!
//! This crate allows you to perform outgoing JSON-RPC requests and creating servers accepting
//! JSON-RPC requests. Only [JSON-RPC version 2](https://www.jsonrpc.org/specification) is
//! supported.
//!
//! In addition to the core JSON-RPC specifications this crate also supports the non-standard
//! "JSON-RPC pub sub" extension, which allows the server to push notifications the client
//! subscribes to. This extension is most notably used in the Ethereum ecosystem, but it is very
//! generic and can be used for any purpose related or not to Ethereum.
//!
//! # Writing an API definition (optional)
//!
//! Before starting to perform or answer queries, one optional step is to define your JSON-RPC API
//! using the `rpc_api!` macro.
//!
//! ```
//! jsonrpsee::rpc_api! {
//!     Health {
//!         /// Returns true if the server is healthy.
//!         fn healthy() -> bool;
//!     }
//!
//!     System {
//!         /// Returns the name of the server.
//!         fn system_name() -> String;
//!     }
//! }
//!
//! # fn main() {}
//! ```
//!
//! # Clients
//!
//! In order to perform outgoing requests, you first have to create a
//! [`RawClient`](core::client::RawClient). There exist several shortcuts such as the [`http_raw_client`]
//! method.
//!
//! Once a client is created, you can use the
//! [`start_request`](core::client::RawClient::start_request) method to perform requests.
//!
//! ```no_run
//! let result: String = async_std::task::block_on(async {
//!     let mut client = jsonrpsee::http_raw_client("http://localhost:8000");
//!     let request_id = client.start_request("system_name", jsonrpsee::core::common::Params::None).await.unwrap();
//!     jsonrpsee::core::common::from_value(client.request_by_id(request_id).unwrap().await.unwrap()).unwrap()
//! });
//!
//! println!("system_name = {:?}", result);
//! ```
//!
//! If you defined an API using the `rpc_api!` macro, the generated type allows you to perform
//! requests as well:
//!
//! ```no_run
//! # jsonrpsee::rpc_api! { System { fn system_name() -> String; } }
//! # fn main() {
//! let result = async_std::task::block_on(async {
//!     let mut client = jsonrpsee::http_raw_client("http://localhost:8000");
//!     System::system_name(&mut client).await
//! });
//!
//! println!("system_name = {:?}", result);
//! # }
//! ```
//!
//! # Servers
//!
//! In order to server JSON-RPC requests, you have to create a [`RawServer`](core::server::RawServer).
//! Just like for the client, there exists shortcuts for creating a server.
//!
//! Once a server is created, use the [`next_event`](core::server::RawServer::next_event) asynchronous
//! function to wait for a request to arrive. The generated
//! [`RawServerEvent`](core::server::RawServerEvent) can be either a "notification", in other words a
//! message from the client that doesn't expect any answer, or a "request" which you should answer.
//!
//! ```no_run
//! // Should run forever
//! async_std::task::block_on(async {
//!     let mut server = jsonrpsee::http_raw_server(&"localhost:8000".parse().unwrap()).await.unwrap();
//!     loop {
//!         match server.next_event().await {
//!             jsonrpsee::core::server::RawServerEvent::Notification(notif) => {
//!                 println!("received notification: {:?}", notif);
//!             }
//!             jsonrpsee::core::server::RawServerEvent::SubscriptionsClosed(_) => {}
//!             jsonrpsee::core::server::RawServerEvent::SubscriptionsReady(_) => {}
//!             jsonrpsee::core::server::RawServerEvent::Request(rq) => {
//!                 // Note that `rq` borrows `server`. If you want to store the request for later,
//!                 // you should get its id by calling `let id = rq.id();`, then later call
//!                 // `server.request_by_id(id)`.
//!                 println!("received request: {:?}", rq);
//!                 rq.respond(Err(jsonrpsee::core::common::Error::method_not_found()));
//!             }
//!         }
//!     }
//! });
//! ```
//!
//! Similarly, if you defined an API using the `rpc_api!` macro, a utility function is provided:
//!
//! ```no_run
//! # jsonrpsee::rpc_api! { System { fn system_name() -> String; } }
//! # fn main() {
//! // Should run forever
//! async_std::task::block_on(async {
//!     let mut server = jsonrpsee::http_raw_server(&"localhost:8000".parse().unwrap()).await.unwrap();
//!     while let Ok(request) = System::next_request(&mut server).await {
//!         match request {
//!             System::SystemName { respond } => {
//!                 respond.ok("my name").await;
//!             }
//!         }
//!     }
//! });
//! # }
//! ```
//!

#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

extern crate alloc;

#[cfg(feature = "http")]
pub use http::{http_raw_client, http_raw_server};
pub use jsonrpsee_proc_macros::rpc_api;

/*#[cfg(feature = "ws")]
pub use jsonrpsee_ws::ws_raw_client;

#[doc(inline)]
pub use jsonrpsee_core as core;
#[doc(inline)]
#[cfg(feature = "ws")]
pub use jsonrpsee_ws as ws;*/

pub use client::Client;
pub use server::Server;

use std::{error, net::SocketAddr};

pub mod client;
pub mod core;

mod server;

#[cfg(feature = "http")]
mod server_utils;

#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub mod http;
#[cfg(feature = "ws")]
#[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
pub mod ws;

/// Builds a new client and a new server that are connected to each other.
pub fn local() -> (Client, Server) {
    let (client, server) = local_raw();
    let client = Client::from(client);
    let server = Server::from(server);
    (client, server)
}

/// Builds a new client and a new server that are connected to each other.
pub fn local_raw() -> (
    crate::core::RawClient<crate::core::local::LocalTransportClient>,
    crate::core::RawServer<
        crate::core::local::LocalTransportServer,
        <crate::core::local::LocalTransportServer as crate::core::TransportServer>::RequestId,
    >,
) {
    let (client, server) = core::local_transport();
    let client = crate::core::RawClient::new(client);
    let server = crate::core::RawServer::new(server);
    (client, server)
}

/// Builds a new HTTP server.
#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub async fn http_server(addr: &SocketAddr) -> Result<Server, Box<dyn error::Error + Send + Sync>> {
    http::http_raw_server(addr).await.map(From::from)
}

/// Builds a new HTTP client.
#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub fn http_client(addr: &str) -> Client {
    Client::from(http::http_raw_client(addr))
}

/// Builds a new WebSockets client.
#[cfg(feature = "ws")]
#[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
pub async fn ws_client(target: &str) -> Result<Client, ws::WsNewDnsError> {
    ws::ws_raw_client(target).await.map(From::from)
}
