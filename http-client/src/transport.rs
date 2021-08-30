// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// Additionally, despite the fact that hyper is capable of performing requests to multiple different
// servers through the same `hyper::Client`, we don't use that feature on purpose. The reason is
// that we need to be guaranteed that hyper doesn't re-use an existing connection if we ever reset
// the JSON-RPC request id to a value that might have already been used.

use crate::types::error::GenericTransportError;
use hyper::client::{Client, HttpConnector};
use hyper_rustls::HttpsConnector;
use jsonrpsee_utils::http_helpers;
use thiserror::Error;

const CONTENT_TYPE_JSON: &str = "application/json";

/// HTTP Transport Client.
#[derive(Debug, Clone)]
pub(crate) struct HttpTransportClient {
	/// Target to connect to.
	target: url::Url,
	/// HTTP client
	client: Client<HttpsConnector<HttpConnector>>,
	/// Configurable max request body size
	max_request_body_size: u32,
}

impl HttpTransportClient {
	/// Initializes a new HTTP client.
	pub(crate) fn new(target: impl AsRef<str>, max_request_body_size: u32) -> Result<Self, Error> {
		let target = url::Url::parse(target.as_ref()).map_err(|e| Error::Url(format!("Invalid URL: {}", e)))?;
		if target.scheme() == "http" || target.scheme() == "https" {
			let connector = HttpsConnector::with_native_roots();
			let client = Client::builder().build::<_, hyper::Body>(connector);
			Ok(HttpTransportClient { target, client, max_request_body_size })
		} else {
			Err(Error::Url("URL scheme not supported, expects 'http' or 'https'".into()))
		}
	}

	async fn inner_send(&self, body: String) -> Result<hyper::Response<hyper::Body>, Error> {
		log::debug!("send: {}", body);

		if body.len() > self.max_request_body_size as usize {
			return Err(Error::RequestTooLarge);
		}

		let req = hyper::Request::post(self.target.as_str())
			.header(hyper::header::CONTENT_TYPE, hyper::header::HeaderValue::from_static(CONTENT_TYPE_JSON))
			.header(hyper::header::ACCEPT, hyper::header::HeaderValue::from_static(CONTENT_TYPE_JSON))
			.body(From::from(body))
			.expect("URI and request headers are valid; qed");

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
pub(crate) enum Error {
	/// Invalid URL.
	#[error("Invalid Url: {0}")]
	Url(String),

	/// Error during the HTTP request, including networking errors and HTTP protocol errors.
	#[error("Error while performing the HTTP request")]
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
	use super::{Error, HttpTransportClient};

	#[test]
	fn invalid_http_url_rejected() {
		let err = HttpTransportClient::new("ws://localhost:9933", 80).unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[tokio::test]
	async fn request_limit_works() {
		let eighty_bytes_limit = 80;
		let client = HttpTransportClient::new("http://localhost:9933", 80).unwrap();
		assert_eq!(client.max_request_body_size, eighty_bytes_limit);

		let body = "a".repeat(81);
		assert_eq!(body.len(), 81);
		let response = client.send(body).await.unwrap_err();
		assert!(matches!(response, Error::RequestTooLarge));
	}
}
