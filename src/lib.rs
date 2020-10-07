// Copyright 2019 Parity Technologies (UK) Ltd.
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

//! JSON-RPC clients, servers, and utilities.
//!
//! This crate allows you to perform outgoing JSON-RPC requests and creating servers accepting
//! JSON-RPC requests. Only [JSON-RPC version 2](https://www.jsonrpc.org/specification) is
//! supported.
//!
//! In addition to the core JSON-RPC specifications this crate also supports the non-standard
//! "JSON-RPC pub sub" extension, which allows the server to push notifications the client
//! subscribes to. This extension is most notably used in the Ethereum ecosystem, but it is very
//! generic and can be used for any purpose related or not to Ethereum.
//!
//! # Writing an API definition (optional)
//!
//! Before starting to perform or answer queries, one optional step is to define your JSON-RPC API
//! using the `rpc_api!` macro.

#![deny(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

pub use jsonrpsee_proc_macros::rpc_api;

/// Client implementations.
pub mod client;
/// Common types.
pub mod common;
/// JSONRPC 2.0 HTTP server.
#[cfg(feature = "http")]
pub mod http;
/// JSONRPC 2.0 WebSocket server.
#[cfg(feature = "ws")]
pub mod ws;
