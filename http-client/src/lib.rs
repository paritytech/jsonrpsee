//! # jsonrpsee-http-client
//!
//! jsonrpsee-http-client is a small JSONRPC HTTP client built directly for `async/await`.
//!
//! - Swappable HTTP backends.
//! - Supports multiple executors (depends on which backend that is configured, see [Optional features](#optional-features))
//!
//! # Optional Features
//!
//! jsonrpsee-http-client uses a set of [feature flags](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section) to support
//! different configuration depending in which environment you are using the client.
//!
//! The following optional features are available:
//!
//! - `hyper-client`: Enables [hyper](https://docs.rs/hyper) as HTTP transport backend which is the fastest option.
//! This feature works only on [tokio 0.2](https://docs.rs/tokio/0.2.25/tokio/) or [async-std](https://docs.rs/async-std).
//! - `curl-client`: Enables [curl](https://docs.rs/curl) as HTTP transport backend.
//! - `wasm-client`: Enables JavaScript bindings as HTTP transport backend.
//! - `middleware-logger`: Enables middleware logging from [surf](https://docs.rs/surf)
//!

mod client;
mod transport;

#[cfg(test)]
mod tests;

pub use client::HttpClient;
pub use jsonrpsee_types::http::HttpConfig;
pub use transport::HttpTransportClient;
