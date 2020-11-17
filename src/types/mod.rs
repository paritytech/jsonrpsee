/// JSON-RPC 2.0 specification related types.
pub mod jsonrpc;

/// Shared error type.
pub mod error;

/// Shared types for HTTP
#[cfg(feature = "http")]
pub mod http;
