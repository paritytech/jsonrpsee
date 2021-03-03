#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms, unreachable_pub)]

//! # jsonrpsee-ws-client
//!
//! `jsonrpsee-ws-client` is a [JSON RPC](https://www.jsonrpc.org/specification) WebSocket client library that's is built for `async/await`.

/// WebSocket Client.
pub mod client;
/// JSONRPC WebSocket transport.
pub mod jsonrpc_transport;
/// Request manager.
pub mod manager;
/// Stream.
pub mod stream;
/// WebSocket transport.
pub mod transport;

#[cfg(test)]
mod tests;

pub use client::{WsClient, WsConfig};
pub use jsonrpsee_types::client::Subscription as WsSubscription;
