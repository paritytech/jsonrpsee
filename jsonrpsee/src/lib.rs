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

//! Jsonrpsee wrapper crate.
//!
//! <br>
//!
//! # Optional features
//!
//! The `jsonrpsee` crate composes JSON-RPC functionality behind optional feature
//! flags to provide for client and server communication over specific protocols.
//! There are no default features, all functionality must be opted in to accordingly.
//! The following features are avaliable.
//!
//! - **`http-client`** - JSON-RPC client functionality over HTTP protocol.
//! - **`http-server`** - JSON-RPC server functionality over HTTP protocol.
//! - **`ws-client`** - JSON-RPC client functionality over WebSocket protocol.
//! - **`ws-server`** - JSON-RPC server functionality over WebSocket protocol.
//! - **`macros`** - JSON-RPC API generation convenience by derive macros.
//! - **`client`** - Enables `http-client` and `ws-client` features.
//! - **`server`** - Enables `http-server` and `ws-server` features.
//! - **`full`** - Enables `client`, `server` and `macros` features.

/// JSON-RPC HTTP client.
#[cfg(feature = "jsonrpsee-http-client")]
pub use jsonrpsee_http_client as http_client;

/// JSON-RPC WebSocket client.
#[cfg(feature = "jsonrpsee-ws-client")]
pub use jsonrpsee_ws_client as ws_client;

/// JSON-RPC core client.
#[cfg(feature = "jsonrpsee-core-client")]
pub use jsonrpsee_core_client as core_client;

/// JSON-RPC client convenience macro to build params.
#[cfg(any(feature = "http-client", feature = "ws-client"))]
pub use jsonrpsee_utils::rpc_params;

/// JSON-RPC HTTP server.
#[cfg(feature = "jsonrpsee-http-server")]
pub use jsonrpsee_http_server as http_server;

/// JSON-RPC WebSocket server.
#[cfg(feature = "jsonrpsee-ws-server")]
pub use jsonrpsee_ws_server as ws_server;

/// Procedural macros for JSON-RPC implementations.
#[cfg(feature = "jsonrpsee-proc-macros")]
pub use jsonrpsee_proc_macros as proc_macros;

/// Common types used to implement JSON-RPC server and client.
#[cfg(feature = "jsonrpsee-types")]
pub use jsonrpsee_types as types;

/// Set of RPC methods that can be mounted to the server.
#[cfg(any(feature = "http-server", feature = "ws-server"))]
pub use jsonrpsee_utils::server::rpc_module::{RpcModule, SubscriptionSink};

#[cfg(feature = "http-server")]
pub use http_server::tracing;

#[cfg(all(feature = "ws-server", not(feature = "http-server")))]
pub use ws_server::tracing;
