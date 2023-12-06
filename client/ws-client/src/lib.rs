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

//! # jsonrpsee-ws-client
//!
//! `jsonrpsee-ws-client` is a [JSON RPC](https://www.jsonrpc.org/specification) WebSocket client library that's is built for `async/await`.
//!
//! ## Async runtime support
//!
//! This library uses `tokio` as the runtime and does not support other runtimes.

#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(test)]
mod tests;

pub use http::{HeaderMap, HeaderValue};
pub use jsonrpsee_core::client::Client as WsClient;
pub use jsonrpsee_types as types;

use jsonrpsee_client_transport::ws::{AsyncRead, AsyncWrite, WsTransportClientBuilder};
use jsonrpsee_core::client::{
	CertificateStore, ClientBuilder, Error, IdKind, MaybeSend, TransportReceiverT, TransportSenderT,
};
use jsonrpsee_core::TEN_MB_SIZE_BYTES;
use std::time::Duration;
use url::Url;

/// Builder for [`WsClient`].
///
/// # Examples
///
/// ```no_run
///
/// use jsonrpsee_ws_client::{WsClientBuilder, HeaderMap, HeaderValue};
///
/// #[tokio::main]
/// async fn main() {
///     // Build custom headers used during the handshake process.
///     let mut headers = HeaderMap::new();
///     headers.insert("Any-Header-You-Like", HeaderValue::from_static("42"));
///
///     // Build client
///     let client = WsClientBuilder::default()
///          .set_headers(headers)
///          .build("wss://localhost:443")
///          .await
///          .unwrap();
///
///     // use client....
/// }
///
/// ```
#[derive(Clone, Debug)]
pub struct WsClientBuilder {
	certificate_store: CertificateStore,
	max_request_size: u32,
	max_response_size: u32,
	request_timeout: Duration,
	connection_timeout: Duration,
	ping_interval: Option<Duration>,
	headers: http::HeaderMap,
	max_concurrent_requests: usize,
	subscription_buf_cap: usize,
	max_redirections: usize,
	id_kind: IdKind,
	max_log_length: u32,
}

impl Default for WsClientBuilder {
	fn default() -> Self {
		Self {
			certificate_store: CertificateStore::Native,
			max_request_size: TEN_MB_SIZE_BYTES,
			max_response_size: TEN_MB_SIZE_BYTES,
			request_timeout: Duration::from_secs(60),
			connection_timeout: Duration::from_secs(10),
			ping_interval: None,
			headers: HeaderMap::new(),
			max_concurrent_requests: 256,
			subscription_buf_cap: 16,
			max_redirections: 5,
			id_kind: IdKind::Number,
			max_log_length: 4096,
		}
	}
}

impl WsClientBuilder {
	/// Create a new WebSocket client builder.
	pub fn new() -> WsClientBuilder {
		WsClientBuilder::default()
	}

	/// Force to use the rustls native certificate store.
	///
	/// Since multiple certificate stores can be optionally enabled, this option will
	/// force the `native certificate store` to be used.
	///
	/// This is enabled with the default settings and features.
	///
	/// # Optional
	///
	/// This requires the optional `native-tls` feature.
	#[cfg(feature = "native-tls")]
	pub fn use_native_rustls(mut self) -> Self {
		self.certificate_store = CertificateStore::Native;
		self
	}

	/// Force to use the rustls webpki certificate store.
	///
	/// Since multiple certificate stores can be optionally enabled, this option will
	/// force the `webpki certificate store` to be used.
	///
	/// # Optional
	///
	/// This requires the optional `webpki-tls` feature.
	#[cfg(feature = "webpki-tls")]
	pub fn use_webpki_rustls(mut self) -> Self {
		self.certificate_store = CertificateStore::WebPki;
		self
	}

	/// See documentation [`WsTransportClientBuilder::max_request_size`] (default is 10 MB).
	pub fn max_request_size(mut self, size: u32) -> Self {
		self.max_request_size = size;
		self
	}

	/// See documentation [`WsTransportClientBuilder::max_response_size`] (default is 10 MB).
	pub fn max_response_size(mut self, size: u32) -> Self {
		self.max_response_size = size;
		self
	}

	/// See documentation [`ClientBuilder::request_timeout`] (default is 60 seconds).
	pub fn request_timeout(mut self, timeout: Duration) -> Self {
		self.request_timeout = timeout;
		self
	}

	/// See documentation [`WsTransportClientBuilder::connection_timeout`] (default is 10 seconds).
	pub fn connection_timeout(mut self, timeout: Duration) -> Self {
		self.connection_timeout = timeout;
		self
	}

	/// See documentation [`ClientBuilder::ping_interval`] (disabled by default).
	pub fn ping_interval(mut self, interval: Duration) -> Self {
		self.ping_interval = Some(interval);
		self
	}

	/// See documentation [`WsTransportClientBuilder::set_headers`] (default is none).
	pub fn set_headers(mut self, headers: http::HeaderMap) -> Self {
		self.headers = headers;
		self
	}

	/// See documentation [`ClientBuilder::max_concurrent_requests`] (default is 256).
	pub fn max_concurrent_requests(mut self, max: usize) -> Self {
		self.max_concurrent_requests = max;
		self
	}

	/// See documentation [`ClientBuilder::with_buf_capacity_per_subscription`] (default is 16).
	pub fn with_buf_capacity_per_subscription(mut self, capacity: usize) -> Self {
		// https://docs.rs/tokio/latest/src/tokio/sync/broadcast.rs.html#501-506
		assert!(capacity > 0, "subscription buffer capacity cannot be zero");
		assert!(capacity <= usize::MAX >> 1, "subscription buffer capacity exceeded `usize::MAX / 2`");

		self.subscription_buf_cap = capacity;
		self
	}

	/// See documentation [`WsTransportClientBuilder::max_redirections`] (default is 5).
	pub fn max_redirections(mut self, redirect: usize) -> Self {
		self.max_redirections = redirect;
		self
	}

	/// See documentation for [`ClientBuilder::id_format`] (default is Number).
	pub fn id_format(mut self, kind: IdKind) -> Self {
		self.id_kind = kind;
		self
	}

	/// Set maximum length for logging calls and responses.
	///
	/// Logs bigger than this limit will be truncated.
	pub fn set_max_logging_length(mut self, max: u32) -> Self {
		self.max_log_length = max;
		self
	}

	/// Build the [`WsClient`] with specified [`TransportSenderT`] [`TransportReceiverT`] parameters
	///
	/// ## Panics
	///
	/// Panics if being called outside of `tokio` runtime context.
	pub fn build_with_transport<S, R>(self, sender: S, receiver: R) -> WsClient
	where
		S: TransportSenderT + Send,
		R: TransportReceiverT + Send,
	{
		let Self {
			max_concurrent_requests,
			request_timeout,
			ping_interval,
			subscription_buf_cap,
			id_kind,
			max_log_length,
			..
		} = self;

		let mut client = ClientBuilder::default()
			.with_buf_capacity_per_subscription(subscription_buf_cap)
			.request_timeout(request_timeout)
			.max_concurrent_requests(max_concurrent_requests)
			.id_format(id_kind)
			.set_max_logging_length(max_log_length);

		if let Some(interval) = ping_interval {
			client = client.ping_interval(interval);
		}

		client.build_with_tokio(sender, receiver)
	}

	/// Build the [`WsClient`] with specified data stream, using [`WsTransportClientBuilder::build_with_stream`].
	///
	/// ## Panics
	///
	/// Panics if being called outside of `tokio` runtime context.
	pub async fn build_with_stream<T>(self, url: impl AsRef<str>, data_stream: T) -> Result<WsClient, Error>
	where
		T: AsyncRead + AsyncWrite + Unpin + MaybeSend + 'static,
	{
		let transport_builder = WsTransportClientBuilder {
			certificate_store: self.certificate_store,
			connection_timeout: self.connection_timeout,
			headers: self.headers.clone(),
			max_request_size: self.max_request_size,
			max_response_size: self.max_response_size,
			max_redirections: self.max_redirections,
		};

		let uri = Url::parse(url.as_ref()).map_err(|e| Error::Transport(e.into()))?;
		let (sender, receiver) =
			transport_builder.build_with_stream(uri, data_stream).await.map_err(|e| Error::Transport(e.into()))?;

		let ws_client = self.build_with_transport(sender, receiver);
		Ok(ws_client)
	}

	/// Build the [`WsClient`] with specified URL to connect to, using the default
	/// [`WsTransportClientBuilder::build_with_stream`], therefore with the default TCP as transport layer.
	///
	/// ## Panics
	///
	/// Panics if being called outside of `tokio` runtime context.
	pub async fn build(self, url: impl AsRef<str>) -> Result<WsClient, Error> {
		let transport_builder = WsTransportClientBuilder {
			certificate_store: self.certificate_store,
			connection_timeout: self.connection_timeout,
			headers: self.headers.clone(),
			max_request_size: self.max_request_size,
			max_response_size: self.max_response_size,
			max_redirections: self.max_redirections,
		};

		let uri = Url::parse(url.as_ref()).map_err(|e| Error::Transport(e.into()))?;
		let (sender, receiver) = transport_builder.build(uri).await.map_err(|e| Error::Transport(e.into()))?;

		let ws_client = self.build_with_transport(sender, receiver);
		Ok(ws_client)
	}
}
