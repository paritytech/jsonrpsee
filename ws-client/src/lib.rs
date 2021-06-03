#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]

//! # jsonrpsee-ws-client
//!
//! `jsonrpsee-ws-client` is a [JSON RPC](https://www.jsonrpc.org/specification) WebSocket client library that's is built for `async/await`.
//!
//! ## Runtime support
//!
//! This library uses `tokio` as the runtime and does not support other kinds of runtimes.
//! Tokio versions v1 and v0.2 are supported behind `tokioV1` and `tokioV02` feature flags correspondingly.

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

/// Compatibility layer to support both `tokio` 0.2 and 1.x versions.
mod tokio;

#[cfg(test)]
mod tests;

pub use client::{WsClient, WsClientBuilder};
pub use jsonrpsee_types::*;
