// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// Additionally, despite the fact that hyper is capable of performing requests to multiple different
// servers through the same `hyper::Client`, we don't use that feature on purpose. The reason is
// that we need to be guaranteed that hyper doesn't re-use an existing connection if we ever reset
// the JSON-RPC request id to a value that might have already been used.

#[cfg(all(feature = "tls", feature = "tls-rustcrypto"))]
compile_error!("Features `tls` and `tls-rustcrypto` are mutually exclusive");

use base64::Engine;
use hyper::body::Bytes;
use hyper::http::{HeaderMap, HeaderValue};
use hyper_util::client::legacy::Client;
#[cfg(not(all(target_os = "wasi", target_env = "p2")))]
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use jsonrpsee_core::BoxError;
use jsonrpsee_core::{
	TEN_MB_SIZE_BYTES,
	http_helpers::{self, HttpError},
};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use thiserror::Error;
use tower::layer::util::Identity;
use tower::{Layer, Service, ServiceExt};
use url::Url;

use crate::{HttpBody, HttpRequest, HttpResponse};

#[cfg(any(feature = "tls", feature = "tls-rustcrypto"))]
use crate::{CertificateStore, CustomCertStore};

/// TCP connector for wasip2 that bypasses hyper-util's `HttpConnector`.
///
/// `HttpConnector` creates sockets via socket2 and calls `set_nonblocking()`,
/// which fails on wasip2. This connector uses `tokio::net::TcpStream::connect()`
/// instead, which goes through mio's wasip2-compatible path.
///
/// DNS resolution uses `std::net::ToSocketAddrs` directly (not `spawn_blocking`)
/// since wasip2 is single-threaded but wasi-libc provides working `getaddrinfo`.
#[cfg(all(target_os = "wasi", target_env = "p2"))]
#[derive(Clone, Debug)]
struct WasiConnector;

#[cfg(all(target_os = "wasi", target_env = "p2"))]
impl Service<hyper::Uri> for WasiConnector {
	type Response = hyper_util::rt::TokioIo<tokio::net::TcpStream>;
	type Error = Box<dyn std::error::Error + Send + Sync>;
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

	fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		Poll::Ready(Ok(()))
	}

	fn call(&mut self, uri: hyper::Uri) -> Self::Future {
		Box::pin(async move {
			let host = uri.host().ok_or("missing host")?;
			let port = uri.port_u16().unwrap_or(match uri.scheme_str() {
				Some("https") => 443,
				_ => 80,
			});

			// Resolve DNS synchronously via std::net (wasi-libc getaddrinfo)
			let addr = std::net::ToSocketAddrs::to_socket_addrs(&(host, port))?
				.next()
				.ok_or("DNS resolved no addresses")?;

			// Connect via tokio (goes through mio's wasip2-compatible path)
			let stream = tokio::net::TcpStream::connect(addr).await?;
			Ok(hyper_util::rt::TokioIo::new(stream))
		})
	}
}

const CONTENT_TYPE_JSON: &str = "application/json";

/// Wrapper over HTTP transport and connector.
#[derive(Debug)]
pub enum HttpBackend<B = HttpBody> {
	/// Hyper client with https connector.
	#[cfg(all(feature = "tls", not(all(target_os = "wasi", target_env = "p2"))))]
	Https(Client<hyper_rustls::HttpsConnector<HttpConnector>, B>),
	/// Hyper client with https connector using pure-Rust crypto (non-wasi).
	#[cfg(all(feature = "tls-rustcrypto", not(feature = "tls"), not(all(target_os = "wasi", target_env = "p2"))))]
	Https(Client<hyper_rustls::HttpsConnector<HttpConnector>, B>),
	/// Hyper client with https connector using wasip2-compatible transport.
	#[cfg(all(feature = "tls-rustcrypto", target_os = "wasi", target_env = "p2"))]
	Https(Client<hyper_rustls::HttpsConnector<WasiConnector>, B>),
	/// Hyper client with http connector.
	#[cfg(not(all(target_os = "wasi", target_env = "p2")))]
	Http(Client<HttpConnector, B>),
	/// Hyper client with wasip2 connector.
	#[cfg(all(target_os = "wasi", target_env = "p2"))]
	Http(Client<WasiConnector, B>),
}

impl<B> Clone for HttpBackend<B> {
	fn clone(&self) -> Self {
		match self {
			Self::Http(inner) => Self::Http(inner.clone()),
			#[cfg(all(feature = "tls", not(all(target_os = "wasi", target_env = "p2"))))]
			Self::Https(inner) => Self::Https(inner.clone()),
			#[cfg(all(feature = "tls-rustcrypto", not(feature = "tls"), not(all(target_os = "wasi", target_env = "p2"))))]
			Self::Https(inner) => Self::Https(inner.clone()),
			#[cfg(all(feature = "tls-rustcrypto", target_os = "wasi", target_env = "p2"))]
			Self::Https(inner) => Self::Https(inner.clone()),
		}
	}
}

impl<B> tower::Service<HttpRequest<B>> for HttpBackend<B>
where
	B: http_body::Body<Data = Bytes> + Send + Unpin + 'static,
	B::Data: Send,
	B::Error: Into<BoxError>,
{
	type Response = HttpResponse<hyper::body::Incoming>;
	type Error = Error;
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

	fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		match self {
			Self::Http(inner) => inner.poll_ready(ctx),
			#[cfg(all(feature = "tls", not(all(target_os = "wasi", target_env = "p2"))))]
			Self::Https(inner) => inner.poll_ready(ctx),
			#[cfg(all(feature = "tls-rustcrypto", not(feature = "tls"), not(all(target_os = "wasi", target_env = "p2"))))]
			Self::Https(inner) => inner.poll_ready(ctx),
			#[cfg(all(feature = "tls-rustcrypto", target_os = "wasi", target_env = "p2"))]
			Self::Https(inner) => inner.poll_ready(ctx),
		}
		.map_err(|e| Error::Http(HttpError::Stream(e.into())))
	}

	fn call(&mut self, req: HttpRequest<B>) -> Self::Future {
		let resp = match self {
			Self::Http(inner) => inner.call(req),
			#[cfg(all(feature = "tls", not(all(target_os = "wasi", target_env = "p2"))))]
			Self::Https(inner) => inner.call(req),
			#[cfg(all(feature = "tls-rustcrypto", not(feature = "tls"), not(all(target_os = "wasi", target_env = "p2"))))]
			Self::Https(inner) => inner.call(req),
			#[cfg(all(feature = "tls-rustcrypto", target_os = "wasi", target_env = "p2"))]
			Self::Https(inner) => inner.call(req),
		};

		Box::pin(async move { resp.await.map_err(|e| Error::Http(HttpError::Stream(e.into()))) })
	}
}

/// Builder for [`HttpTransportClient`].
#[derive(Debug)]
pub struct HttpTransportClientBuilder<L> {
	/// Certificate store.
	#[cfg(any(feature = "tls", feature = "tls-rustcrypto"))]
	pub(crate) certificate_store: CertificateStore,
	/// Configurable max request body size
	pub(crate) max_request_size: u32,
	/// Configurable max response body size
	pub(crate) max_response_size: u32,
	/// Custom headers to pass with every request.
	pub(crate) headers: HeaderMap,
	/// Service builder
	pub(crate) service_builder: tower::ServiceBuilder<L>,
	/// TCP_NODELAY
	pub(crate) tcp_no_delay: bool,
	/// KEEP_ALIVE duration
	pub(crate) keep_alive_duration: Option<std::time::Duration>,
	/// KEEP_ALIVE interval
	pub(crate) keep_alive_interval: Option<std::time::Duration>,
	/// KEEP_ALIVE retries
	pub(crate) keep_alive_retries: Option<u32>,
}

impl Default for HttpTransportClientBuilder<Identity> {
	fn default() -> Self {
		Self::new()
	}
}

impl HttpTransportClientBuilder<Identity> {
	/// Create a new [`HttpTransportClientBuilder`].
	pub fn new() -> Self {
		Self {
			#[cfg(any(feature = "tls", feature = "tls-rustcrypto"))]
			certificate_store: CertificateStore::Native,
			max_request_size: TEN_MB_SIZE_BYTES,
			max_response_size: TEN_MB_SIZE_BYTES,
			headers: HeaderMap::new(),
			service_builder: tower::ServiceBuilder::new(),
			tcp_no_delay: true,
			keep_alive_duration: None,
			keep_alive_interval: None,
			keep_alive_retries: None,
		}
	}
}

impl<L> HttpTransportClientBuilder<L> {
	/// See docs [`crate::HttpClientBuilder::with_custom_cert_store`] for more information.
	#[cfg(any(feature = "tls", feature = "tls-rustcrypto"))]
	pub fn with_custom_cert_store(mut self, cfg: CustomCertStore) -> Self {
		self.certificate_store = CertificateStore::Custom(cfg);
		self
	}

	/// Set the maximum size of a request body in bytes. Default is 10 MiB.
	pub fn max_request_size(mut self, size: u32) -> Self {
		self.max_request_size = size;
		self
	}

	/// Set the maximum size of a response in bytes. Default is 10 MiB.
	pub fn max_response_size(mut self, size: u32) -> Self {
		self.max_response_size = size;
		self
	}

	/// Set a custom header passed to the server with every request (default is none).
	///
	/// The caller is responsible for checking that the headers do not conflict or are duplicated.
	pub fn set_headers(mut self, headers: HeaderMap) -> Self {
		self.headers = headers;
		self
	}

	/// Configure `TCP_NODELAY` on the socket to the supplied value `nodelay`.
	///
	/// Default is `true`.
	pub fn set_tcp_no_delay(mut self, no_delay: bool) -> Self {
		self.tcp_no_delay = no_delay;
		self
	}

	/// Configure the keep-alive duration for the connection.
	pub fn set_keep_alive(mut self, duration: Option<std::time::Duration>) -> Self {
		self.keep_alive_duration = duration;
		self
	}

	/// Configure the keep-alive interval for the connection.
	pub fn set_keep_alive_interval(mut self, interval: Option<std::time::Duration>) -> Self {
		self.keep_alive_interval = interval;
		self
	}

	/// Configure the number of keep-alive retries for the connection.
	pub fn set_keep_alive_retries(mut self, retries: Option<u32>) -> Self {
		self.keep_alive_retries = retries;
		self
	}

	/// Configure a tower service.
	pub fn set_service<T>(self, service: tower::ServiceBuilder<T>) -> HttpTransportClientBuilder<T> {
		HttpTransportClientBuilder {
			#[cfg(any(feature = "tls", feature = "tls-rustcrypto"))]
			certificate_store: self.certificate_store,
			headers: self.headers,
			max_request_size: self.max_request_size,
			max_response_size: self.max_response_size,
			service_builder: service,
			tcp_no_delay: self.tcp_no_delay,
			keep_alive_duration: self.keep_alive_duration,
			keep_alive_retries: self.keep_alive_retries,
			keep_alive_interval: self.keep_alive_interval,
		}
	}

	/// Build a [`HttpTransportClient`].
	pub fn build<S, B>(self, target: impl AsRef<str>) -> Result<HttpTransportClient<S>, Error>
	where
		L: Layer<HttpBackend, Service = S>,
		S: Service<HttpRequest, Response = HttpResponse<B>, Error = Error> + Clone,
		B: http_body::Body<Data = Bytes> + Send + 'static,
		B::Data: Send,
		B::Error: Into<BoxError>,
	{
		let Self {
			#[cfg(any(feature = "tls", feature = "tls-rustcrypto"))]
			certificate_store,
			max_request_size,
			max_response_size,
			headers,
			service_builder,
			tcp_no_delay,
			keep_alive_duration,
			keep_alive_interval,
			keep_alive_retries,
		} = self;
		let mut url = Url::parse(target.as_ref()).map_err(|e| Error::Url(format!("Invalid URL: {e}")))?;

		if url.host_str().is_none() {
			return Err(Error::Url("Invalid host".into()));
		}
		url.set_fragment(None);

		let client = match url.scheme() {
			"http" => {
				#[cfg(all(target_os = "wasi", target_env = "p2"))]
				{
					// wasip2 sockets do not expose TCP-level options (nodelay, keepalive).
					// WasiConnector bypasses HttpConnector entirely, so these settings are ignored.
					if !tcp_no_delay || keep_alive_duration.is_some() || keep_alive_interval.is_some() || keep_alive_retries.is_some() {
						log::warn!("TCP socket options (tcp_no_delay, keep_alive_*) are not supported on wasip2 and will be ignored");
					}
					HttpBackend::Http(Client::builder(TokioExecutor::new()).build(WasiConnector))
				}
				#[cfg(not(all(target_os = "wasi", target_env = "p2")))]
				{
					let mut connector = HttpConnector::new();
					connector.set_nodelay(tcp_no_delay);
					connector.set_keepalive(keep_alive_duration);
					connector.set_keepalive_interval(keep_alive_interval);
					connector.set_keepalive_retries(keep_alive_retries);
					HttpBackend::Http(Client::builder(TokioExecutor::new()).build(connector))
				}
			}
			#[cfg(all(feature = "tls", not(all(target_os = "wasi", target_env = "p2"))))]
			"https" => {
				// Make sure that the TLS provider is set. If not, set a default one.
				// Otherwise, creating `tls` configuration may panic if there are multiple
				// providers available due to `rustls` features (e.g. both `ring` and `aws-lc-rs`).
				// Function returns an error if the provider is already installed, and we're fine with it.
				let _ = rustls::crypto::ring::default_provider().install_default();

				let mut http_conn = HttpConnector::new();
				http_conn.set_nodelay(tcp_no_delay);
				http_conn.enforce_http(false);
				http_conn.set_keepalive(keep_alive_duration);
				http_conn.set_keepalive_interval(keep_alive_interval);
				http_conn.set_keepalive_retries(keep_alive_retries);

				let https_conn = match certificate_store {
					CertificateStore::Native => {
						use rustls_platform_verifier::ConfigVerifierExt;

						hyper_rustls::HttpsConnectorBuilder::new()
							.with_tls_config(rustls::ClientConfig::with_platform_verifier())
							.https_or_http()
							.enable_all_versions()
							.wrap_connector(http_conn)
					}

					CertificateStore::Custom(tls_config) => hyper_rustls::HttpsConnectorBuilder::new()
						.with_tls_config(tls_config)
						.https_or_http()
						.enable_all_versions()
						.wrap_connector(http_conn),
				};

				HttpBackend::Https(Client::builder(TokioExecutor::new()).build(https_conn))
			}
			#[cfg(all(feature = "tls-rustcrypto", not(feature = "tls"), not(all(target_os = "wasi", target_env = "p2"))))]
			"https" => {
				let _ = rustls_rustcrypto::provider().install_default();

				let mut http_conn = HttpConnector::new();
				http_conn.set_nodelay(tcp_no_delay);
				http_conn.enforce_http(false);
				http_conn.set_keepalive(keep_alive_duration);
				http_conn.set_keepalive_interval(keep_alive_interval);
				http_conn.set_keepalive_retries(keep_alive_retries);

				let https_conn = match certificate_store {
					CertificateStore::Native => {
						use rustls_platform_verifier::ConfigVerifierExt;

						hyper_rustls::HttpsConnectorBuilder::new()
							.with_tls_config(rustls::ClientConfig::with_platform_verifier())
							.https_or_http()
							.enable_all_versions()
							.wrap_connector(http_conn)
					}
					CertificateStore::Custom(tls_config) => hyper_rustls::HttpsConnectorBuilder::new()
						.with_tls_config(tls_config)
						.https_or_http()
						.enable_all_versions()
						.wrap_connector(http_conn),
				};

				HttpBackend::Https(Client::builder(TokioExecutor::new()).build(https_conn))
			}
			#[cfg(all(feature = "tls-rustcrypto", target_os = "wasi", target_env = "p2"))]
			"https" => {
				// Use pure-Rust crypto provider (rustls-rustcrypto) since ring/aws-lc-rs
				// require C/ASM compilation which is not available on wasm32.
				let _ = rustls_rustcrypto::provider().install_default();

				// wasip2 sockets do not expose TCP-level options (nodelay, keepalive).
				// WasiConnector bypasses HttpConnector entirely, so these settings are ignored.
				if !tcp_no_delay || keep_alive_duration.is_some() || keep_alive_interval.is_some() || keep_alive_retries.is_some() {
					log::warn!("TCP socket options (tcp_no_delay, keep_alive_*) are not supported on wasip2 and will be ignored");
				}

				let tls_config = match certificate_store {
					CertificateStore::Native => {
						// wasip2 has no OS cert store; use bundled Mozilla root certs.
						let mut root_store = rustls::RootCertStore::empty();
						root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
						rustls::ClientConfig::builder()
							.with_root_certificates(root_store)
							.with_no_client_auth()
					}
					CertificateStore::Custom(tls_config) => tls_config,
				};

				let https_conn = hyper_rustls::HttpsConnectorBuilder::new()
					.with_tls_config(tls_config)
					.https_or_http()
					.enable_all_versions()
					.wrap_connector(WasiConnector);

				HttpBackend::Https(Client::builder(TokioExecutor::new()).build(https_conn))
			}
			_ => {
				#[cfg(any(feature = "tls", feature = "tls-rustcrypto"))]
				let err = "URL scheme not supported, expects 'http' or 'https'";
				#[cfg(not(any(feature = "tls", feature = "tls-rustcrypto")))]
				let err = "URL scheme not supported, expects 'http'";
				return Err(Error::Url(err.into()));
			}
		};

		// Cache request headers: 2 default headers, followed by user custom headers.
		// Maintain order for headers in case of duplicate keys:
		// https://datatracker.ietf.org/doc/html/rfc7230#section-3.2.2
		let mut cached_headers = HeaderMap::with_capacity(2 + headers.len());
		cached_headers.insert(hyper::header::CONTENT_TYPE, HeaderValue::from_static(CONTENT_TYPE_JSON));
		cached_headers.insert(hyper::header::ACCEPT, HeaderValue::from_static(CONTENT_TYPE_JSON));
		for (key, value) in headers.into_iter() {
			if let Some(key) = key {
				cached_headers.insert(key, value);
			}
		}

		if let Some(pwd) = url.password() {
			if !cached_headers.contains_key(hyper::header::AUTHORIZATION) {
				let digest = base64::engine::general_purpose::STANDARD.encode(format!("{}:{pwd}", url.username()));
				cached_headers.insert(
					hyper::header::AUTHORIZATION,
					HeaderValue::from_str(&format!("Basic {digest}"))
						.map_err(|_| Error::Url("Header value `authorization basic user:pwd` invalid".into()))?,
				);
			}
		}
		let _ = url.set_password(None);
		let _ = url.set_username("");

		Ok(HttpTransportClient {
			target: url.as_str().to_owned(),
			client: service_builder.service(client),
			max_request_size,
			max_response_size,
			headers: cached_headers,
		})
	}
}

/// HTTP Transport Client.
#[derive(Debug, Clone)]
pub struct HttpTransportClient<S> {
	/// Target to connect to.
	target: String,
	/// HTTP client
	client: S,
	/// Configurable max request body size
	max_request_size: u32,
	/// Configurable max response body size
	max_response_size: u32,
	/// Custom headers to pass with every request.
	headers: HeaderMap,
}

impl<B, S> HttpTransportClient<S>
where
	S: Service<HttpRequest, Response = HttpResponse<B>, Error = Error> + Clone,
	B: http_body::Body<Data = Bytes> + Send + 'static,
	B::Data: Send,
	B::Error: Into<BoxError>,
{
	async fn inner_send(&self, body: String) -> Result<HttpResponse<B>, Error> {
		if body.len() > self.max_request_size as usize {
			return Err(Error::RequestTooLarge);
		}

		let mut req = HttpRequest::post(&self.target);
		if let Some(headers) = req.headers_mut() {
			*headers = self.headers.clone();
		}

		let req = req.body(body.into()).expect("URI and request headers are valid; qed");
		let response = self.client.clone().ready().await?.call(req).await?;

		if response.status().is_success() {
			Ok(response)
		} else {
			Err(Error::Rejected { status_code: response.status().into() })
		}
	}

	/// Send serialized message and wait until all bytes from the HTTP message body have been read.
	pub(crate) async fn send_and_read_body(&self, body: String) -> Result<Vec<u8>, Error> {
		let response = self.inner_send(body).await?;

		let (parts, body) = response.into_parts();
		let (body, _is_single) = http_helpers::read_body(&parts.headers, body, self.max_response_size).await?;

		Ok(body)
	}

	/// Send serialized message without reading the HTTP message body.
	pub(crate) async fn send(&self, body: String) -> Result<(), Error> {
		self.inner_send(body).await?;
		Ok(())
	}
}

/// Error that can happen during a request.
#[derive(Debug, Error)]
pub enum Error {
	/// Invalid URL.
	#[error("Invalid Url: {0}")]
	Url(String),

	/// Error during the HTTP request, including networking errors and HTTP protocol errors.
	#[error(transparent)]
	Http(#[from] HttpError),

	/// Server returned a non-success status code.
	#[error("Request rejected `{status_code}`")]
	Rejected {
		/// HTTP Status code returned by the server.
		status_code: u16,
	},

	/// Request body too large.
	#[error("The request body was too large")]
	RequestTooLarge,

	/// Invalid certificate store.
	#[error("Invalid certificate store")]
	InvalidCertficateStore,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn invalid_http_url_rejected() {
		let err = HttpTransportClientBuilder::new().build("ws://localhost:9933").unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[cfg(any(feature = "tls", feature = "tls-rustcrypto"))]
	#[test]
	fn https_works() {
		let client = HttpTransportClientBuilder::new().build("https://localhost").unwrap();
		assert_eq!(&client.target, "https://localhost/");
	}

	#[cfg(not(any(feature = "tls", feature = "tls-rustcrypto")))]
	#[test]
	fn https_fails_without_tls_feature() {
		let err = HttpTransportClientBuilder::new().build("https://localhost").unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[test]
	fn faulty_port() {
		let err = HttpTransportClientBuilder::new().build("http://localhost:-43").unwrap_err();
		assert!(matches!(err, Error::Url(_)));

		let err = HttpTransportClientBuilder::new().build("http://localhost:-99999").unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[test]
	fn url_with_path_works() {
		let client = HttpTransportClientBuilder::new().build("http://localhost/my-special-path").unwrap();
		assert_eq!(&client.target, "http://localhost/my-special-path");
	}

	#[test]
	fn url_with_query_works() {
		let client = HttpTransportClientBuilder::new().build("http://127.0.0.1/my?name1=value1&name2=value2").unwrap();
		assert_eq!(&client.target, "http://127.0.0.1/my?name1=value1&name2=value2");
	}

	#[test]
	fn url_with_fragment_is_ignored() {
		let client = HttpTransportClientBuilder::new().build("http://127.0.0.1/my.htm#ignore").unwrap();
		assert_eq!(&client.target, "http://127.0.0.1/my.htm");
	}

	#[test]
	fn url_default_port_is_omitted() {
		let client = HttpTransportClientBuilder::new().build("http://127.0.0.1:80").unwrap();
		assert_eq!(&client.target, "http://127.0.0.1/");
	}

	#[cfg(any(feature = "tls", feature = "tls-rustcrypto"))]
	#[test]
	fn https_custom_port_works() {
		let client = HttpTransportClientBuilder::new().build("https://localhost:9999").unwrap();
		assert_eq!(&client.target, "https://localhost:9999/");
	}

	#[test]
	fn http_custom_port_works() {
		let client = HttpTransportClientBuilder::new().build("http://localhost:9999").unwrap();
		assert_eq!(&client.target, "http://localhost:9999/");
	}

	#[tokio::test]
	async fn request_limit_works() {
		let eighty_bytes_limit = 80;
		let fifty_bytes_limit = 50;

		let client = HttpTransportClientBuilder::new()
			.max_request_size(eighty_bytes_limit)
			.max_response_size(fifty_bytes_limit)
			.build("http://localhost:9933")
			.unwrap();

		assert_eq!(client.max_request_size, eighty_bytes_limit);
		assert_eq!(client.max_response_size, fifty_bytes_limit);

		let body = "a".repeat(81);
		assert_eq!(body.len(), 81);
		let response = client.send(body).await.unwrap_err();
		assert!(matches!(response, Error::RequestTooLarge));
	}
}
