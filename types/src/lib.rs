//! Shared types in `jsonrpsee` for clients, servers and utilities.

#![deny(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

/// JSON-RPC 2.0 specification related types.
pub mod jsonrpc;

/// JSON-RPC 2.0 specification related types v2.
pub mod v2;

/// Shared error type.
pub mod error;

/// Shared client types.
pub mod client;

/// Traits
pub mod traits;
