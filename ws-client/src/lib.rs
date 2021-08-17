#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]

//! # jsonrpsee-ws-client
//!
//! `jsonrpsee-ws-client` is a [JSON RPC](https://www.jsonrpc.org/specification) WebSocket client library that's is built for `async/await`.
//!
//! ## Runtime support
//!
//! This library uses `tokio` as the runtime and does not support other kinds of runtimes.

/// WebSocket Client.
pub mod client;
/// Helpers.
pub mod helpers;
/// Request manager.
pub mod manager;
/// Stream.
pub mod stream;
/// WebSocket transport.
pub mod transport;

#[cfg(test)]
mod tests;

pub use client::{WsClient, WsClientBuilder};
pub use jsonrpsee_types as types;
