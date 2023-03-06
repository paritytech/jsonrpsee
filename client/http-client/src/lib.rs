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

//! # jsonrpsee-http-client
//!
//! `jsonrpsee-http-client` is [JSON RPC](https://www.jsonrpc.org/specification) HTTP client library that's is built for `async/await`.
//!
//! It is tightly-coupled to [`tokio`](https://docs.rs/tokio) because [`hyper`](https://docs.rs/hyper) is used as transport client,
//! which is not compatible with other async runtimes such as
//! [`async-std`](https://docs.rs/async-std/), [`smol`](https://docs.rs/smol) and similar.

#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, unreachable_pub)]
#![deny(unused_crate_dependencies)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod client;

/// HTTP transport.
pub mod transport;

#[cfg(test)]
mod tests;

pub use client::{HttpClient, HttpClientBuilder};
pub use hyper::http::{HeaderMap, HeaderValue};
pub use jsonrpsee_types as types;
