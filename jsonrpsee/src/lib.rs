//! jsonrpsee wrapper crate.

/// JSON RPC HTTP client.
#[cfg(feature = "client")]
pub use http_client;

/// JSON RPC WebSocket client.
#[cfg(feature = "client")]
pub use ws_client;

/// JSON RPC HTTP server.
#[cfg(feature = "server")]
pub use http_server;

/// JSON RPC WebSocket server.
#[cfg(feature = "server")]
pub use ws_server;

/// Set of RPC methods that can be mounted to the server.
#[cfg(feature = "server")]
pub use utils::server::rpc_module::RpcModule;

/// Procedural macros for JSON RPC implementations.
#[cfg(feature = "macros")]
pub use proc_macros;

/// Common types used to implement JSON RPC server and client.
#[cfg(feature = "macros")]
pub use types;
