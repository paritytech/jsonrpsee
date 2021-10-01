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

//! jsonrpsee wrapper crate.

/// JSON RPC HTTP client.
#[cfg(feature = "client")]
pub use http_client;

/// JSON RPC WebSocket client.
#[cfg(feature = "client")]
pub use ws_client;

/// JSON RPC WebSocket client convenience macro to build params.
#[cfg(feature = "client")]
pub use utils::rpc_params;

/// JSON RPC HTTP server.
#[cfg(feature = "server")]
pub use http_server;

/// JSON RPC WebSocket server.
#[cfg(feature = "server")]
pub use ws_server;

/// Set of RPC methods that can be mounted to the server.
#[cfg(feature = "server")]
pub use utils::server::rpc_module::{RpcModule, SubscriptionSink};

/// Procedural macros for JSON RPC implementations.
#[cfg(feature = "macros")]
pub use proc_macros;

/// Common types used to implement JSON RPC server and client.
#[cfg(any(feature = "types", feature = "macros"))]
pub mod types {
	pub use ::types::*;

	/// Set of RPC methods that can be mounted to the server.
	#[cfg(feature = "server")]
	pub use utils::server::rpc_module::{RpcModule, SubscriptionSink};
}
