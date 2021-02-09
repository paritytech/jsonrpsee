//! Shared types in `jsonrpsee` for clients, servers and utilities.

#![deny(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

/// JSON-RPC 2.0 specification related types.
pub mod jsonrpc;

/// Shared error type.
pub mod error;

/// Shared types for HTTP
pub mod http;

/// traits.
pub mod traits;

/// core client
pub mod client;

/// request manager
pub mod manager;
