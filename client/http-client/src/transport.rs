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
use jsonrpsee_core::client::CertificateStore;
use jsonrpsee_core::tracing::client::{rx_log_from_bytes, tx_log_from_str};
use jsonrpsee_core::{
	http_helpers::{self, HttpError},
	TEN_MB_SIZE_BYTES,
};
use std::error::Error as StdError;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use thiserror::Error;
use tower::layer::util::Identity;
use tower::{Layer, Service, ServiceExt};
use url::Url;

const CONTENT_TYPE_JSON: &str = "application/json";

/// Wrapper over HTTP transport and connector.
#[derive(Debug)]
pub enum HttpBackend<B = Body> {
	/// Hyper client with https connector.
	#[cfg(feature = "__tls")]
	Https(Client<hyper_rustls::HttpsConnector<HttpConnector>, B>),
	/// Hyper client with http connector.
	Http(Client<HttpConnector, B>),
}

impl Clone for HttpBackend {
	fn clone(&self) -> Self {
		match self {
			Self::Http(inner) => Self::Http(inner.clone()),
			#[cfg(feature = "__tls")]
			Self::Https(inner) => Self::Https(inner.clone()),
		}
	}
}

impl<B> tower::Service<hyper::Request<B>> for HttpBackend<B>
where
	B: HttpBody<Error = hyper::Error> + Send + 'static,
	B::Data: Send,
	B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
	type Response = hyper::Response<Body>;
	type Error = Error;
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

	fn poll_ready(&mut self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		match self {
			Self::Http(inner) => inner.poll_ready(ctx),
			#[cfg(feature = "__tls")]
			Self::Https(inner) => inner.poll_ready(ctx),
		}
		.map_err(|e| Error::Http(e.into()))
	}

	fn call(&mut self, req: hyper::Request<B>) -> Self::Future {
		let resp = match self {
			Self::Http(inner) => inner.call(req),
			#[cfg(feature = "__tls")]
			Self::Https(inner) => inner.call(req),
		};

		Box::pin(async move { resp.await.map_err(|e| Error::Http(e.into())) })
	}
}

/// Builder for [`HttpTransportClient`].
#[derive(Debug)]
pub struct HttpTransportClientBuilder<L> {
	/// Certificate store.
	certificate_store: CertificateStore,
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
	/// Service builder
	service_builder: tower::ServiceBuilder<L>,
	/// TCP_NODELAY
	tcp_no_delay: bool,
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
			certificate_store: CertificateStore::Native,
			max_request_size: TEN_MB_SIZE_BYTES,
			max_response_size: TEN_MB_SIZE_BYTES,
			max_log_length: 1024,
			headers: HeaderMap::new(),
			service_builder: tower::ServiceBuilder::new(),
			tcp_no_delay: true,
		}
	}
}

impl<L> HttpTransportClientBuilder<L> {
	/// Set the certificate store.
	pub fn set_certification_store(mut self, cert_store: CertificateStore) -> Self {
		self.certificate_store = cert_store;
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

	/// Max length for logging for requests and responses in number characters.
	///
	/// Logs bigger than this limit will be truncated.
	pub fn set_max_logging_length(mut self, max: u32) -> Self {
		self.max_log_length = max;
		self
	}

	/// Configure a tower service.
	pub fn set_service<T>(self, service: tower::ServiceBuilder<T>) -> HttpTransportClientBuilder<T> {
		HttpTransportClientBuilder {
			certificate_store: self.certificate_store,
			headers: self.headers,
			max_log_length: self.max_log_length,
			max_request_size: self.max_request_size,
			max_response_size: self.max_response_size,
			service_builder: service,
			tcp_no_delay: self.tcp_no_delay,
		}
	}

	/// Build a [`HttpTransportClient`].
	pub fn build<S, B>(self, target: impl AsRef<str>) -> Result<HttpTransportClient<S>, Error>
	where
		L: Layer<HttpBackend<Body>, Service = S>,
		S: Service<hyper::Request<Body>, Response = hyper::Response<B>, Error = Error> + Clone,
		B: HttpBody + Send + 'static,
		B::Data: Send,
		B::Error: Into<Box<dyn StdError + Send + Sync>>,
	{
		let Self {
			certificate_store,
			max_request_size,
			max_response_size,
			max_log_length,
			headers,
			service_builder,
			tcp_no_delay,
		} = self;
		let mut url = Url::parse(target.as_ref()).map_err(|e| Error::Url(format!("Invalid URL: {e}")))?;
		if url.host_str().is_none() {
			return Err(Error::Url("Invalid host".into()));
		}
		url.set_fragment(None);

		let client = match url.scheme() {
			"http" => {
				let mut connector = HttpConnector::new();
				connector.set_nodelay(tcp_no_delay);
				HttpBackend::Http(Client::builder().build(connector))
			}
			#[cfg(feature = "__tls")]
			"https" => {
				let mut http_conn = HttpConnector::new();
				http_conn.set_nodelay(tcp_no_delay);
				http_conn.enforce_http(false);

				let https_conn = match certificate_store {
					#[cfg(feature = "native-tls")]
					CertificateStore::Native => hyper_rustls::HttpsConnectorBuilder::new()
						.with_native_roots()
						.https_or_http()
						.enable_all_versions()
						.wrap_connector(http_conn),
					#[cfg(feature = "webpki-tls")]
					CertificateStore::WebPki => hyper_rustls::HttpsConnectorBuilder::new()
						.with_webpki_roots()
						.https_or_http()
						.enable_all_versions()
						.wrap_connector(http_conn),
					_ => return Err(Error::InvalidCertficateStore),
				};

				HttpBackend::Https(Client::builder().build::<_, hyper::Body>(https_conn))
			}
			_ => {
				#[cfg(feature = "__tls")]
				let err = "URL scheme not supported, expects 'http' or 'https'";
				#[cfg(not(feature = "__tls"))]
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

		Ok(HttpTransportClient {
			target: url.as_str().to_owned(),
			client: service_builder.service(client),
			max_request_size,
			max_response_size,
			max_log_length,
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
	/// Max length for logging for requests and responses
	///
	/// Logs bigger than this limit will be truncated.
	max_log_length: u32,
	/// Custom headers to pass with every request.
	headers: HeaderMap,
}

impl<B, S> HttpTransportClient<S>
where
	S: Service<hyper::Request<Body>, Response = hyper::Response<B>, Error = Error> + Clone,
	B: HttpBody<Error = hyper::Error> + Send + 'static,
	B::Data: Send,
{
	async fn inner_send(&self, body: String) -> Result<hyper::Response<B>, Error> {
		if body.len() > self.max_request_size as usize {
			return Err(Error::RequestTooLarge);
		}

		let mut req = hyper::Request::post(&self.target);
		if let Some(headers) = req.headers_mut() {
			*headers = self.headers.clone();
		}
		let req = req.body(From::from(body)).expect("URI and request headers are valid; qed");
		let response = self.client.clone().ready().await?.call(req).await?;

		if response.status().is_success() {
			Ok(response)
		} else {
			Err(Error::Rejected { status_code: response.status().into() })
		}
	}

	/// Send serialized message and wait until all bytes from the HTTP message body have been read.
	pub(crate) async fn send_and_read_body(&self, body: String) -> Result<Vec<u8>, Error> {
		tx_log_from_str(&body, self.max_log_length);

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

/// Error that can happen during a request.
#[derive(Debug, Error)]
pub enum Error {
	/// Invalid URL.
	#[error("Invalid Url: {0}")]
	Url(String),

	/// Error during the HTTP request, including networking errors and HTTP protocol errors.
	#[error("{0}")]
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

	#[cfg(feature = "__tls")]
	#[test]
	fn https_works() {
		let client = HttpTransportClientBuilder::new().build("https://localhost").unwrap();
		assert_eq!(&client.target, "https://localhost/");
	}

	#[cfg(not(feature = "__tls"))]
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

	#[cfg(feature = "__tls")]
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
