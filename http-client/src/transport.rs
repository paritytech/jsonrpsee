// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// Additionally, despite the fact that hyper is capable of performing requests to multiple different
// servers through the same `hyper::Client`, we don't use that feature on purpose. The reason is
// that we need to be guaranteed that hyper doesn't re-use an existing connection if we ever reset
// the JSON-RPC request id to a value that might have already been used.

use hyper::client::{Client, HttpConnector};
use hyper_rustls::HttpsConnector;
use jsonrpsee_types::{
	error::GenericTransportError,
	v2::dummy::{JsonRpcNotification, JsonRpcRequest, JsonRpcResponse},
};
use jsonrpsee_utils::hyper_helpers;
use serde::{de::DeserializeOwned, Serialize};
use thiserror::Error;

const CONTENT_TYPE_JSON: &str = "application/json";

/// HTTP Transport Client.
#[derive(Debug, Clone)]
pub struct HttpTransportClient {
	/// Target to connect to.
	target: url::Url,
	/// HTTP client
	client: Client<HttpsConnector<HttpConnector>>,
	/// Configurable max request body size
	max_request_body_size: u32,
}

impl HttpTransportClient {
	/// Initializes a new HTTP client.
	pub fn new(target: impl AsRef<str>, max_request_body_size: u32) -> Result<Self, Error> {
		let target = url::Url::parse(target.as_ref()).map_err(|e| Error::Url(format!("Invalid URL: {}", e)))?;
		if target.scheme() == "http" || target.scheme() == "https" {
			#[cfg(feature = "tokio1")]
			let connector = HttpsConnector::with_native_roots();
			#[cfg(feature = "tokio02")]
			let connector = HttpsConnector::new();
			let client = Client::builder().build::<_, hyper::Body>(connector);
			Ok(HttpTransportClient { target, client, max_request_body_size })
		} else {
			Err(Error::Url("URL scheme not supported, expects 'http' or 'https'".into()))
		}
	}

	/// Send serialized message.
	pub async fn send(&self, body: String) -> Result<hyper::Response<hyper::Body>, Error> {
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

	/// Send serialized message and wait until all bytes from the body is read.
	pub async fn send_and_wait_for_response(&self, body: String) -> Result<Vec<u8>, Error> {
		let response = self.send(body).await?;
		let (parts, body) = response.into_parts();
		let body = hyper_helpers::read_response_to_body(&parts.headers, body, self.max_request_body_size).await?;
		Ok(body)
	}
}

/// Error that can happen during a request.
#[derive(Debug, Error)]
pub enum Error {
	/// Invalid URL.
	#[error("Invalid Url: {0}")]
	Url(String),

	/// Error while serializing the request.
	// TODO: can that happen?
	#[error("Error while serializing the request")]
	Serialization(#[source] serde_json::error::Error),

	/// Response given by the server failed to decode as UTF-8.
	#[error("Response body is not UTF-8")]
	Utf8(#[source] std::string::FromUtf8Error),

	/// Error during the HTTP request, including networking errors and HTTP protocol errors.
	#[error("Error while performing the HTTP request")]
	Http(Box<dyn std::error::Error + Send + Sync>),

	/// Server returned a non-success status code.
	#[error("Server returned an error status code: {:?}", status_code)]
	RequestFailure {
		/// Status code returned by the server.
		status_code: u16,
	},

	/// Failed to parse the JSON returned by the server into a JSON-RPC response.
	#[error("Error while parsing the response body")]
	ParseError(#[source] serde_json::error::Error),

	/// Request body too large.
	#[error("The request body was too large")]
	RequestTooLarge,
}

impl<T> From<GenericTransportError<T>> for Error
where
	T: std::error::Error + Send + Sync + 'static,
{
	fn from(err: GenericTransportError<T>) -> Self {
		match err {
			GenericTransportError::<T>::TooLarge => Self::RequestTooLarge,
			GenericTransportError::<T>::Inner(e) => Self::Http(Box::new(e)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{Error, HttpTransportClient};
	use jsonrpsee_types::v2::dummy::{JsonRpcCall, JsonRpcParams};

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

		let body = serde_json::to_string(&JsonRpcCall::new(
			1,
			"request_larger_than_eightybytes",
			JsonRpcParams::NoParams::<u64>,
		))
		.unwrap();
		assert_eq!(body.len(), 81);
		let response = client.send(body).await.unwrap_err();
		assert!(matches!(response, Error::RequestTooLarge));
	}
}
