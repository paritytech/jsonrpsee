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

//! # jsonrpsee-server
//!
//! `jsonrpsee-server` is a [JSON RPC](https://www.jsonrpc.org/specification) server that supports both HTTP and WebSocket transport.

#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod future;
mod server;
mod transport;

pub mod logger;
pub mod middleware;

#[cfg(test)]
mod tests;

pub use future::ServerHandle;
pub use jsonrpsee_core::server::*;
pub use jsonrpsee_core::{id_providers::*, traits::IdProvider};
pub use jsonrpsee_types as types;
pub use server::{Builder as ServerBuilder, Server};
pub use tracing;
