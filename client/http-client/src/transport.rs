// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// Additionally, despite the fact that hyper is capable of performing requests to multiple different
// servers through the same `hyper::Client`, we don't use that feature on purpose. The reason is
// that we need to be guaranteed that hyper doesn't re-use an existing connection if we ever reset
// the JSON-RPC request id to a value that might have already been used.

use hyper::body::{Body, HttpBody};
use hyper::client::{Client, HttpConnector};
use hyper::http::{HeaderMap, HeaderValue};
use hyper::Uri;
use jsonrpsee_core::client::CertificateStore;
use jsonrpsee_core::error::GenericTransportError;
use jsonrpsee_core::http_helpers;
use jsonrpsee_core::tracing::{rx_log_from_bytes, tx_log_from_str};
use std::error::Error as StdError;
use thiserror::Error;
use tower::{Layer, Service, ServiceExt};

const CONTENT_TYPE_JSON: &str = "application/json";

#[cfg(feature = "tls")]
pub(crate) type TlsService = Client<hyper_rustls::HttpsConnector<HttpConnector>>;
#[cfg(not(feature = "tls"))]
pub(crate) type PlainService = Client<HttpConnector>;

/// HTTP Transport Client.
#[derive(Debug, Clone)]
pub struct HttpTransportClient<S> {
	/// Target to connect to.
	target: ParsedUri,
	/// HTTP client
	client: S,
	/// Configurable max request body size
	max_request_size: u32,
	/// Configurable max response body size
	max_response_size: u32,
	/// Max length for logging for requests and responses
	///
	/// Logs bigger than this limit will be truncated.
	max_log_length: u32,
	/// Custom headers to pass with every request.
	headers: HeaderMap,
}

impl<B, S> HttpTransportClient<S>
where
	S: Service<hyper::Request<Body>, Response = hyper::Response<B>, Error = hyper::Error> + Clone,
	B: HttpBody + Send + 'static,
	B::Data: Send,
	B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
	/// Initializes a new HTTP client.
	#[cfg(feature = "tls")]
	pub(crate) fn new<L: Layer<TlsService, Service = S>>(
		max_request_size: u32,
		target: impl AsRef<str>,
		max_response_size: u32,
		cert_store: CertificateStore,
		max_log_length: u32,
		headers: HeaderMap,
		service_builder: tower::ServiceBuilder<L>,
	) -> Result<Self, Error> {
		let uri = ParsedUri::try_from(target.as_ref())?;

		let client = match uri.0.scheme_str() {
			Some("http") | Some("https") => {
				let connector = match cert_store {
					CertificateStore::Native => hyper_rustls::HttpsConnectorBuilder::new()
						.with_native_roots()
						.https_or_http()
						.enable_http1()
						.build(),
					CertificateStore::WebPki => hyper_rustls::HttpsConnectorBuilder::new()
						.with_webpki_roots()
						.https_or_http()
						.enable_http1()
						.build(),
					_ => return Err(Error::InvalidCertficateStore),
				};
				Client::builder().build::<_, hyper::Body>(connector)
			}
			_ => {
				#[cfg(feature = "tls")]
				let err = "URL scheme not supported, expects 'http' or 'https'";
				#[cfg(not(feature = "tls"))]
				let err = "URL scheme not supported, expects 'http'";
				return Err(Error::Url(err.into()));
			}
		};

		Ok(Self {
			target: uri,
			client: service_builder.service(client),
			max_request_size,
			max_response_size,
			max_log_length,
			headers: build_header_cache(headers),
		})
	}

	/// Initializes a new HTTP client.
	#[cfg(not(feature = "tls"))]
	pub(crate) fn new<L: Layer<PlainService, Service = S>>(
		max_request_size: u32,
		target: impl AsRef<str>,
		max_response_size: u32,
		_cert_store: CertificateStore,
		max_log_length: u32,
		headers: HeaderMap,
		service_builder: tower::ServiceBuilder<L>,
	) -> Result<Self, Error> {
		let uri = ParsedUri::try_from(target.as_ref())?;

		let client = match uri.0.scheme_str() {
			Some("http") => Client::new(),
			_ => {
				#[cfg(feature = "tls")]
				let err = "URL scheme not supported, expects 'http' or 'https'";
				#[cfg(not(feature = "tls"))]
				let err = "URL scheme not supported, expects 'http'";
				return Err(Error::Url(err.into()));
			}
		};

		Ok(Self {
			target: uri,
			client: service_builder.service(client),
			max_request_size,
			max_response_size,
			max_log_length,
			headers: build_header_cache(headers),
		})
	}

	async fn inner_send(&self, body: String) -> Result<hyper::Response<B>, Error> {
		tx_log_from_str(&body, self.max_log_length);

		if body.len() > self.max_request_size as usize {
			return Err(Error::RequestTooLarge);
		}

		let mut req = hyper::Request::post(&self.target.0);
		if let Some(headers) = req.headers_mut() {
			*headers = self.headers.clone();
		}
		let req = req.body(From::from(body)).expect("URI and request headers are valid; qed");
		let response = self.client.clone().ready().await?.call(req).await?;

		if response.status().is_success() {
			Ok(response)
		} else {
			Err(Error::RequestFailure { status_code: response.status().into() })
		}
	}

	/// Send serialized message and wait until all bytes from the HTTP message body have been read.
	pub(crate) async fn send_and_read_body(&self, body: String) -> Result<Vec<u8>, Error> {
		let response = self.inner_send(body).await?;
		let (parts, body) = response.into_parts();
		let (body, _) = http_helpers::read_body(&parts.headers, body, self.max_response_size).await?;

		rx_log_from_bytes(&body, self.max_log_length);

		Ok(body)
	}

	/// Send serialized message without reading the HTTP message body.
	pub(crate) async fn send(&self, body: String) -> Result<(), Error> {
		let _ = self.inner_send(body).await?;

		Ok(())
	}
}

#[derive(Debug, Clone)]
struct ParsedUri(Uri);

impl TryFrom<&str> for ParsedUri {
	type Error = Error;

	fn try_from(target: &str) -> Result<Self, Self::Error> {
		let uri: Uri = target.parse().map_err(|e| Error::Url(format!("Invalid URL: {}", e)))?;
		if uri.port_u16().is_none() {
			return Err(Error::Url("Port number is missing in the URL".into()));
		} else {
			Ok(ParsedUri(uri))
		}
	}
}

fn build_header_cache(headers: HeaderMap) -> HeaderMap {
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

	cached_headers
}

/// Error that can happen during a request.
#[derive(Debug, Error)]
pub enum Error {
	/// Invalid URL.
	#[error("Invalid Url: {0}")]
	Url(String),

	/// Error during the HTTP request, including networking errors and HTTP protocol errors.
	#[error("HTTP error: {0}")]
	Http(Box<dyn std::error::Error + Send + Sync>),

	/// Server returned a non-success status code.
	#[error("Server returned an error status code: {:?}", status_code)]
	RequestFailure {
		/// Status code returned by the server.
		status_code: u16,
	},

	/// Request body too large.
	#[error("The request body was too large")]
	RequestTooLarge,

	/// Malformed request.
	#[error("Malformed request")]
	Malformed,

	/// Invalid certificate store.
	#[error("Invalid certificate store")]
	InvalidCertficateStore,
}

impl From<GenericTransportError> for Error {
	fn from(err: GenericTransportError) -> Self {
		match err {
			GenericTransportError::TooLarge => Self::RequestTooLarge,
			GenericTransportError::Malformed => Self::Malformed,
			GenericTransportError::Inner(e) => Self::Http(e.into()),
		}
	}
}

impl From<hyper::Error> for Error {
	fn from(err: hyper::Error) -> Self {
		Self::Http(Box::new(err))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use jsonrpsee_core::client::CertificateStore;

	#[cfg(feature = "tls")]
	type Client = HttpTransportClient<TlsService>;
	#[cfg(not(feature = "tls"))]
	type Client = HttpTransportClient<PlainService>;

	fn assert_target(
		client: &Client,
		host: &str,
		scheme: &str,
		path_and_query: &str,
		port: u16,
		max_request_size: u32,
	) {
		assert_eq!(client.target.0.scheme_str(), Some(scheme));
		assert_eq!(client.target.0.path_and_query().map(|pq| pq.as_str()), Some(path_and_query));
		assert_eq!(client.target.0.host(), Some(host));
		assert_eq!(client.target.0.port_u16(), Some(port));
		assert_eq!(client.max_request_size, max_request_size);
	}

	#[test]
	fn invalid_http_url_rejected() {
		let err = HttpTransportClient::new(
			80,
			"ws://localhost:9933",
			80,
			CertificateStore::Native,
			80,
			HeaderMap::new(),
			tower::ServiceBuilder::new(),
		)
		.unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[cfg(feature = "tls")]
	#[test]
	fn https_works() {
		let client = HttpTransportClient::new(
			80,
			"https://localhost:9933",
			80,
			CertificateStore::Native,
			80,
			HeaderMap::new(),
			tower::ServiceBuilder::new(),
		)
		.unwrap();
		assert_target(&client, "localhost", "https", "/", 9933, 80);
	}

	#[cfg(not(feature = "tls"))]
	#[test]
	fn https_fails_without_tls_feature() {
		let err = HttpTransportClient::new(
			80,
			"https://localhost:9933",
			80,
			CertificateStore::Native,
			80,
			HeaderMap::new(),
			tower::ServiceBuilder::new(),
		)
		.unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[test]
	fn faulty_port() {
		let err = HttpTransportClient::new(
			80,
			"http://localhost:-43",
			80,
			CertificateStore::Native,
			80,
			HeaderMap::new(),
			tower::ServiceBuilder::new(),
		)
		.unwrap_err();
		assert!(matches!(err, Error::Url(_)));
		let err = HttpTransportClient::new(
			80,
			"http://localhost:-99999",
			80,
			CertificateStore::Native,
			80,
			HeaderMap::new(),
			tower::ServiceBuilder::new(),
		)
		.unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[test]
	fn url_with_path_works() {
		let client = HttpTransportClient::new(
			1337,
			"http://localhost:9944/my-special-path",
			1337,
			CertificateStore::Native,
			80,
			HeaderMap::new(),
			tower::ServiceBuilder::new(),
		)
		.unwrap();
		assert_target(&client, "localhost", "http", "/my-special-path", 9944, 1337);
	}

	#[test]
	fn url_with_query_works() {
		let client = HttpTransportClient::new(
			u32::MAX,
			"http://127.0.0.1:9999/my?name1=value1&name2=value2",
			u32::MAX,
			CertificateStore::WebPki,
			80,
			HeaderMap::new(),
			tower::ServiceBuilder::new(),
		)
		.unwrap();
		assert_target(&client, "127.0.0.1", "http", "/my?name1=value1&name2=value2", 9999, u32::MAX);
	}

	#[test]
	fn url_with_fragment_is_ignored() {
		let client = HttpTransportClient::new(
			999,
			"http://127.0.0.1:9944/my.htm#ignore",
			999,
			CertificateStore::Native,
			80,
			HeaderMap::new(),
			tower::ServiceBuilder::new(),
		)
		.unwrap();
		assert_target(&client, "127.0.0.1", "http", "/my.htm", 9944, 999);
	}

	#[tokio::test]
	async fn request_limit_works() {
		let eighty_bytes_limit = 80;
		let fifty_bytes_limit = 50;

		let client = HttpTransportClient::new(
			eighty_bytes_limit,
			"http://localhost:9933",
			fifty_bytes_limit,
			CertificateStore::WebPki,
			99,
			HeaderMap::new(),
			tower::ServiceBuilder::new(),
		)
		.unwrap();
		assert_eq!(client.max_request_size, eighty_bytes_limit);
		assert_eq!(client.max_response_size, fifty_bytes_limit);

		let body = "a".repeat(81);
		assert_eq!(body.len(), 81);
		let response = client.send(body).await.unwrap_err();
		assert!(matches!(response, Error::RequestTooLarge));
	}
}
