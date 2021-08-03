// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]

//! # jsonrpsee-http-client
//!
//! `jsonrpsee-http-client` is [JSON RPC](https://www.jsonrpc.org/specification) HTTP client library that's is built for `async/await`.
//!
//! It is tightly-coupled to [`tokio`](https://docs.rs/tokio) because [`hyper`](https://docs.rs/hyper) is used as transport client,
//! which is not compatible with other async runtimes such as
//! [`async-std`](https://docs.rs/async-std/), [`smol`](https://docs.rs/smol) and similar.
//!
//! It supports both [`tokio 1.0`](https://docs.rs/tokio/1.2.0/tokio/) and [`tokio 0.2`](https://docs.rs/tokio/0.2.25/tokio/index.html)
//! via [Optional features](#optional-features).
//!
//! # Optional Features
//!
//! `jsonrpsee-http-client` uses the following [feature flags]:
//!
//! - `tokio1`: Enable to use the library with [`tokio 1.0`](https://docs.rs/tokio/1.2.0/tokio/) (mutually exclusive with `tokio02`)
//! - `tokio0.2`: Enable to use the library with [`tokio 0.2`](https://docs.rs/tokio/0.2.25/tokio/index.html) (mutually exclusive with `tokio1`)
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

#[cfg(all(feature = "tokio1", feature = "tokio02"))]
compile_error!("feature `tokio1` and `tokio02` are mutably exclusive");

#[cfg(not(any(feature = "tokio1", feature = "tokio02")))]
compile_error!("feature `tokio1` or `tokio02` must be enabled for this crate");

#[cfg(all(feature = "tokio1", not(feature = "tokio02")))]
extern crate hyper14 as hyper;
#[cfg(all(feature = "tokio1", not(feature = "tokio02")))]
extern crate hyper14_rustls as hyper_rustls;

#[cfg(all(feature = "tokio02", not(feature = "tokio1")))]
extern crate hyper13 as hyper;
#[cfg(all(feature = "tokio02", not(feature = "tokio1")))]
extern crate hyper13_rustls as hyper_rustls;

mod client;
mod transport;

#[cfg(all(feature = "tokio1", not(feature = "tokio02")))]
mod tokio {
	pub(crate) use tokioV1::time::timeout;
	#[cfg(test)]
	pub(crate) use tokioV1::{runtime, test};
}

#[cfg(all(feature = "tokio02", not(feature = "tokio1")))]
mod tokio {
	pub(crate) use tokioV02::time::timeout;
	pub(crate) use tokioV02::time::Elapsed;
}

#[cfg(test)]
mod tests;

pub use client::{HttpClient, HttpClientBuilder};
pub use jsonrpsee_types as types;
