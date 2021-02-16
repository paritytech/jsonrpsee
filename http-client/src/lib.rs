#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms, unreachable_pub)]

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
//! - `tokio1`: Enable to work with [`tokio 1.0`](https://docs.rs/tokio/1.2.0/tokio/) (mutual exclusive to `tokio02`)
//! - `tokio0.2`: Enable to work with [`tokio 0.2`](https://docs.rs/tokio/0.2.25/tokio/index.html) (mutual exclusive to `tokio1`)
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section

#[cfg(all(feature = "tokio1", feature = "tokio02"))]
compile_error!("feature `tokio1` and `tokio02` are mutably exclusive");

#[cfg(not(any(feature = "tokio1", feature = "tokio02")))]
compile_error!("feature `tokio1` or `tokio02` must be enabled for this crate");

#[cfg(all(feature = "tokio1", not(feature = "tokio02")))]
extern crate hyper14 as hyper;

#[cfg(all(feature = "tokio02", not(feature = "tokio1")))]
extern crate hyper13 as hyper;

mod client;
mod transport;

#[cfg(test)]
mod tests;

pub use client::HttpClient;
pub use jsonrpsee_types::http::HttpConfig;
pub use transport::HttpTransportClient;
