//! Shared utilities for `jsonrpsee`.

#![warn(missing_docs, missing_debug_implementations, unreachable_pub)]

/// Shared hyper helpers.
#[cfg(feature = "http-helpers")]
pub mod http_helpers;

/// Shared code for JSON-RPC servers.
#[cfg(feature = "server")]
pub mod server;
