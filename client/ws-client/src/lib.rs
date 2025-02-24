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
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(test)]
mod tests;

pub use http::{HeaderMap, HeaderValue};
pub use jsonrpsee_core::client::Client as WsClient;
pub use jsonrpsee_core::client::async_client::PingConfig;
pub use jsonrpsee_types as types;

use jsonrpsee_client_transport::ws::{AsyncRead, AsyncWrite, WsTransportClientBuilder};
use jsonrpsee_core::TEN_MB_SIZE_BYTES;
use jsonrpsee_core::client::{ClientBuilder, Error, IdKind, MaybeSend, TransportReceiverT, TransportSenderT};
use std::time::Duration;
use url::Url;

#[cfg(feature = "tls")]
pub use jsonrpsee_client_transport::ws::CustomCertStore;

#[cfg(feature = "tls")]
use jsonrpsee_client_transport::ws::CertificateStore;

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
	#[cfg(feature = "tls")]
	certificate_store: CertificateStore,
	max_request_size: u32,
	max_response_size: u32,
	request_timeout: Duration,
	connection_timeout: Duration,
	ping_config: Option<PingConfig>,
	headers: http::HeaderMap,
	max_concurrent_requests: usize,
	max_buffer_capacity_per_subscription: usize,
	max_redirections: usize,
	id_kind: IdKind,
	max_log_length: u32,
	tcp_no_delay: bool,
}

impl Default for WsClientBuilder {
	fn default() -> Self {
		Self {
			#[cfg(feature = "tls")]
			certificate_store: CertificateStore::Native,
			max_request_size: TEN_MB_SIZE_BYTES,
			max_response_size: TEN_MB_SIZE_BYTES,
			request_timeout: Duration::from_secs(60),
			connection_timeout: Duration::from_secs(10),
			ping_config: None,
			headers: HeaderMap::new(),
			max_concurrent_requests: 256,
			max_buffer_capacity_per_subscription: 1024,
			max_redirections: 5,
			id_kind: IdKind::Number,
			max_log_length: 4096,
			tcp_no_delay: true,
		}
	}
}

impl WsClientBuilder {
	/// Create a new WebSocket client builder.
	pub fn new() -> WsClientBuilder {
		WsClientBuilder::default()
	}

	/// Force to use a custom certificate store.
	///
	/// # Optional
	///
	/// This requires the optional `tls` feature.
	///
	/// # Example
	///
	/// ```no_run
	/// use jsonrpsee_ws_client::{WsClientBuilder, CustomCertStore};
	/// use rustls::{
	///     client::danger::{self, HandshakeSignatureValid, ServerCertVerified},
	///     pki_types::{CertificateDer, ServerName, UnixTime},
	///     Error,
	/// };
	///
	/// #[derive(Debug)]
	/// struct NoCertificateVerification;
	///
	/// impl rustls::client::danger::ServerCertVerifier for NoCertificateVerification {
	///     fn verify_server_cert(
	///         &self,
	///         _: &CertificateDer<'_>,
	///         _: &[CertificateDer<'_>],
	///         _: &ServerName<'_>,
	///         _: &[u8],
	///         _: UnixTime,
	///     ) -> Result<ServerCertVerified, Error> {
	///         Ok(ServerCertVerified::assertion())
	///     }
	///
	///     fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
	///         vec![rustls::SignatureScheme::ECDSA_NISTP256_SHA256]
	///     }
	///
	///     fn verify_tls12_signature(
	///         &self,
	///         _: &[u8],
	///         _: &CertificateDer<'_>,
	///         _: &rustls::DigitallySignedStruct,
	///     ) -> Result<rustls::client::danger::HandshakeSignatureValid, Error> {
	///         Ok(HandshakeSignatureValid::assertion())
	///     }
	///
	///     fn verify_tls13_signature(
	///         &self,
	///         _: &[u8],
	///         _: &CertificateDer<'_>,
	///         _: &rustls::DigitallySignedStruct,
	///     ) -> Result<HandshakeSignatureValid, Error> {
	///         Ok(HandshakeSignatureValid::assertion())
	///     }
	/// }
	///
	/// let tls_cfg = CustomCertStore::builder()
	///    .dangerous()
	///    .with_custom_certificate_verifier(std::sync::Arc::new(NoCertificateVerification))
	///    .with_no_client_auth();
	///
	/// // client builder with disabled certificate verification.
	/// let client_builder = WsClientBuilder::new().with_custom_cert_store(tls_cfg);
	/// ```
	#[cfg(feature = "tls")]
	pub fn with_custom_cert_store(mut self, cfg: CustomCertStore) -> Self {
		self.certificate_store = CertificateStore::Custom(cfg);
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

	/// See documentation [`ClientBuilder::enable_ws_ping`] (disabled by default).
	pub fn enable_ws_ping(mut self, cfg: PingConfig) -> Self {
		self.ping_config = Some(cfg);
		self
	}

	/// See documentation [`ClientBuilder::disable_ws_ping`]
	pub fn disable_ws_ping(mut self) -> Self {
		self.ping_config = None;
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

	/// See documentation [`ClientBuilder::max_buffer_capacity_per_subscription`] (default is 1024).
	pub fn max_buffer_capacity_per_subscription(mut self, max: usize) -> Self {
		self.max_buffer_capacity_per_subscription = max;
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

	/// See documentation [`ClientBuilder::set_tcp_no_delay`] (default is true).
	pub fn set_tcp_no_delay(mut self, no_delay: bool) -> Self {
		self.tcp_no_delay = no_delay;
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
			ping_config,
			max_buffer_capacity_per_subscription,
			id_kind,
			max_log_length,
			tcp_no_delay,
			..
		} = self;

		let mut client = ClientBuilder::default()
			.max_buffer_capacity_per_subscription(max_buffer_capacity_per_subscription)
			.request_timeout(request_timeout)
			.max_concurrent_requests(max_concurrent_requests)
			.id_format(id_kind)
			.set_max_logging_length(max_log_length)
			.set_tcp_no_delay(tcp_no_delay);

		if let Some(cfg) = ping_config {
			client = client.enable_ws_ping(cfg);
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
			#[cfg(feature = "tls")]
			certificate_store: self.certificate_store.clone(),
			connection_timeout: self.connection_timeout,
			headers: self.headers.clone(),
			max_request_size: self.max_request_size,
			max_response_size: self.max_response_size,
			max_redirections: self.max_redirections,
			tcp_no_delay: self.tcp_no_delay,
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
			#[cfg(feature = "tls")]
			certificate_store: self.certificate_store.clone(),
			connection_timeout: self.connection_timeout,
			headers: self.headers.clone(),
			max_request_size: self.max_request_size,
			max_response_size: self.max_response_size,
			max_redirections: self.max_redirections,
			tcp_no_delay: self.tcp_no_delay,
		};

		let uri = Url::parse(url.as_ref()).map_err(|e| Error::Transport(e.into()))?;
		let (sender, receiver) = transport_builder.build(uri).await.map_err(|e| Error::Transport(e.into()))?;

		let ws_client = self.build_with_transport(sender, receiver);
		Ok(ws_client)
	}
}
