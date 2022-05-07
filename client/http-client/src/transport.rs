// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// Additionally, despite the fact that hyper is capable of performing requests to multiple different
// servers through the same `hyper::Client`, we don't use that feature on purpose. The reason is
// that we need to be guaranteed that hyper doesn't re-use an existing connection if we ever reset
// the JSON-RPC request id to a value that might have already been used.

use hyper::client::{Client, HttpConnector};
use hyper::{HeaderMap, Uri};
use jsonrpsee_core::client::CertificateStore;
use jsonrpsee_core::error::GenericTransportError;
use jsonrpsee_core::http_helpers;
use thiserror::Error;

const CONTENT_TYPE_JSON: &str = "application/json";

#[derive(Debug, Clone)]
enum HyperClient {
	/// Hyper client with https connector.
	#[cfg(feature = "tls")]
	Https(Client<hyper_rustls::HttpsConnector<HttpConnector>>),
	/// Hyper client with http connector.
	Http(Client<HttpConnector>),
}

impl HyperClient {
	fn request(&self, req: hyper::Request<hyper::Body>) -> hyper::client::ResponseFuture {
		match self {
			Self::Http(client) => client.request(req),
			#[cfg(feature = "tls")]
			Self::Https(client) => client.request(req),
		}
	}
}

/// HTTP Transport Client.
#[derive(Debug, Clone)]
pub struct HttpTransportClient {
	/// Target to connect to.
	target: Uri,
	/// HTTP client
	client: HyperClient,
	/// Configurable max request body size
	max_request_body_size: u32,
	/// Custom headers sent with every request
	custom_headers: HeaderMap,
}

impl HttpTransportClient {
	/// Initializes a new HTTP client.
	pub(crate) fn new(
		target: impl AsRef<str>,
		max_request_body_size: u32,
		cert_store: CertificateStore,
		custom_headers: hyper::HeaderMap,
	) -> Result<Self, Error> {
		let target: Uri = target.as_ref().parse().map_err(|e| Error::Url(format!("Invalid URL: {}", e)))?;
		if target.port_u16().is_none() {
			return Err(Error::Url("Port number is missing in the URL".into()));
		}

		let client = match target.scheme_str() {
			Some("http") => HyperClient::Http(Client::new()),
			#[cfg(feature = "tls")]
			Some("https") => {
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
				HyperClient::Https(Client::builder().build::<_, hyper::Body>(connector))
			}
			_ => {
				#[cfg(feature = "tls")]
				let err = "URL scheme not supported, expects 'http' or 'https'";
				#[cfg(not(feature = "tls"))]
				let err = "URL scheme not supported, expects 'http'";
				return Err(Error::Url(err.into()));
			}
		};
		Ok(Self { target, client, max_request_body_size, custom_headers })
	}

	async fn inner_send(&self, body: String) -> Result<hyper::Response<hyper::Body>, Error> {
		tracing::debug!("send: {}", body);

		if body.len() > self.max_request_body_size as usize {
			return Err(Error::RequestTooLarge);
		}

		let mut builder = hyper::Request::post(&self.target)
			.header(hyper::header::CONTENT_TYPE, hyper::header::HeaderValue::from_static(CONTENT_TYPE_JSON))
			.header(hyper::header::ACCEPT, hyper::header::HeaderValue::from_static(CONTENT_TYPE_JSON));

		for (h, v) in self.custom_headers.iter() {
			builder = builder.header(h, v);
		}

		let req = builder.body(From::from(body)).expect("URI and request headers are valid; qed");

		let response = self.client.request(req).await.map_err(|e| Error::Http(Box::new(e)))?;
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
		let (body, _) = http_helpers::read_body(&parts.headers, body, self.max_request_body_size).await?;
		Ok(body)
	}

	/// Send serialized message without reading the HTTP message body.
	pub(crate) async fn send(&self, body: String) -> Result<(), Error> {
		let _ = self.inner_send(body).await?;
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

impl<T> From<GenericTransportError<T>> for Error
where
	T: std::error::Error + Send + Sync + 'static,
{
	fn from(err: GenericTransportError<T>) -> Self {
		match err {
			GenericTransportError::<T>::TooLarge => Self::RequestTooLarge,
			GenericTransportError::<T>::Malformed => Self::Malformed,
			GenericTransportError::<T>::Inner(e) => Self::Http(Box::new(e)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{CertificateStore, Error, HttpTransportClient};

	fn assert_target(
		client: &HttpTransportClient,
		host: &str,
		scheme: &str,
		path_and_query: &str,
		port: u16,
		max_request_size: u32,
	) {
		assert_eq!(client.target.scheme_str(), Some(scheme));
		assert_eq!(client.target.path_and_query().map(|pq| pq.as_str()), Some(path_and_query));
		assert_eq!(client.target.host(), Some(host));
		assert_eq!(client.target.port_u16(), Some(port));
		assert_eq!(client.max_request_body_size, max_request_size);
	}

	#[test]
	fn invalid_http_url_rejected() {
		let err = HttpTransportClient::new("ws://localhost:9933", 80, CertificateStore::Native).unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[cfg(feature = "tls")]
	#[test]
	fn https_works() {
		let client = HttpTransportClient::new("https://localhost:9933", 80, CertificateStore::Native).unwrap();
		assert_target(&client, "localhost", "https", "/", 9933, 80);
	}

	#[cfg(not(feature = "tls"))]
	#[test]
	fn https_fails_without_tls_feature() {
		let err = HttpTransportClient::new("https://localhost:9933", 80, CertificateStore::Native).unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[test]
	fn faulty_port() {
		let err = HttpTransportClient::new("http://localhost:-43", 80, CertificateStore::Native).unwrap_err();
		assert!(matches!(err, Error::Url(_)));
		let err = HttpTransportClient::new("http://localhost:-99999", 80, CertificateStore::Native).unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[test]
	fn url_with_path_works() {
		let client =
			HttpTransportClient::new("http://localhost:9944/my-special-path", 1337, CertificateStore::Native).unwrap();
		assert_target(&client, "localhost", "http", "/my-special-path", 9944, 1337);
	}

	#[test]
	fn url_with_query_works() {
		let client = HttpTransportClient::new(
			"http://127.0.0.1:9999/my?name1=value1&name2=value2",
			u32::MAX,
			CertificateStore::WebPki,
		)
		.unwrap();
		assert_target(&client, "127.0.0.1", "http", "/my?name1=value1&name2=value2", 9999, u32::MAX);
	}

	#[test]
	fn url_with_fragment_is_ignored() {
		let client =
			HttpTransportClient::new("http://127.0.0.1:9944/my.htm#ignore", 999, CertificateStore::Native).unwrap();
		assert_target(&client, "127.0.0.1", "http", "/my.htm", 9944, 999);
	}

	#[tokio::test]
	async fn request_limit_works() {
		let eighty_bytes_limit = 80;
		let client = HttpTransportClient::new("http://localhost:9933", 80, CertificateStore::WebPki).unwrap();
		assert_eq!(client.max_request_body_size, eighty_bytes_limit);

		let body = "a".repeat(81);
		assert_eq!(body.len(), 81);
		let response = client.send(body).await.unwrap_err();
		assert!(matches!(response, Error::RequestTooLarge));
	}
}
