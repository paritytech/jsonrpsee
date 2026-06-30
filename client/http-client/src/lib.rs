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
//!
//! ## Building for `wasm32-wasip2`
//!
//! On `wasm32-wasip2` this crate uses `tokio::net::TcpStream`, which tokio gates behind the
//! unstable `--cfg tokio_unstable` rustc flag. Downstream consumers must set this flag themselves
//! when targeting `wasm32-wasip2`. The recommended way is a `.cargo/config.toml` in the consumer
//! crate:
//!
//! ```toml
//! [target.wasm32-wasip2]
//! rustflags = ["--cfg", "tokio_unstable"]
//! ```
//!
//! Without it, the build fails inside `tokio` with
//! `compile_error!("Only features sync,macros,io-util,rt,time are supported on wasm.")`.
//!
//! The default `tls` feature uses [`ring`](https://docs.rs/ring) for crypto, which has a C build
//! step that requires `clang` on the host. For wasip2 builds without a C toolchain, use the
//! pure-Rust `tls-rustcrypto` feature instead:
//!
//! ```bash
//! cargo build --target wasm32-wasip2 --no-default-features --features tls-rustcrypto
//! ```

#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]

// These crates are only used behind wasip2 cfg gates, suppress unused warnings on other targets.
#[cfg(not(wasip2))]
use log as _;
#[cfg(all(feature = "tls-rustcrypto", not(wasip2)))]
use webpki_roots as _;

// rustls_platform_verifier is used on native targets (ConfigVerifierExt) but not on wasip2,
// where we use webpki_roots instead. Suppress the unused dependency warning for wasip2.
#[cfg(all(wasip2, any(feature = "tls", feature = "tls-rustcrypto")))]
use rustls_platform_verifier as _;

mod client;
mod rpc_service;

/// HTTP transport.
pub mod transport;

#[cfg(test)]
mod tests;

pub use client::{HttpClient, HttpClientBuilder};
pub use hyper::http::{HeaderMap, HeaderValue};
pub use jsonrpsee_types as types;

/// This is the default implementation of the [`jsonrpsee_core::middleware::RpcServiceT`] trait used in the [`HttpClient`].
pub use rpc_service::RpcService;
/// Default HTTP body for the client.
pub type HttpBody = jsonrpsee_core::http_helpers::Body;
/// HTTP request with default body.
pub type HttpRequest<T = HttpBody> = jsonrpsee_core::http_helpers::Request<T>;
/// HTTP response with default body.
pub type HttpResponse<T = HttpBody> = jsonrpsee_core::http_helpers::Response<T>;

pub use jsonrpsee_core::middleware::layer::{RpcLogger, RpcLoggerLayer};
pub use jsonrpsee_core::middleware::{RpcServiceBuilder, RpcServiceT};
pub use transport::{HttpBackend, HttpTransportClient};

/// Custom TLS configuration.
#[cfg(any(feature = "tls", feature = "tls-rustcrypto"))]
pub type CustomCertStore = rustls::ClientConfig;

#[cfg(any(feature = "tls", feature = "tls-rustcrypto"))]
// rustls needs the concrete `ClientConfig` type so we can't Box it here.
#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug)]
pub(crate) enum CertificateStore {
	Native,
	Custom(CustomCertStore),
}
