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
use jsonrpsee_utils::http::hyper_helpers;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
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
			Ok(HttpTransportClient { client, target, max_request_body_size })
		} else {
			Err(Error::Url("URL scheme not supported, expects 'http' or 'https'".into()))
		}
	}

	/// Send request.
	async fn send<'a>(&self, body: String) -> Result<hyper::Response<hyper::Body>, Error> {
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

	/// Send notification.
	pub async fn send_notification<'a, T>(&self, notif: JsonRpcNotification<'a, T>) -> Result<(), Error>
	where
		T: Serialize + std::fmt::Debug + PartialEq,
	{
		let body = serde_json::to_string(&notif).map_err(Error::Serialization)?;
		let _response = self.send(body).await?;
		Ok(())
	}

	/// Send request and wait for response.
	pub async fn send_request_and_wait_for_response<'a, T, R>(
		&self,
		request: impl Into<JsonRpcRequest<'a, T>>,
	) -> Result<JsonRpcResponse<R>, Error>
	where
		T: Serialize + std::fmt::Debug + PartialEq + 'a,
		R: DeserializeOwned,
	{
		let body = serde_json::to_string(&request.into()).map_err(Error::Serialization)?;
		let response = self.send(body).await?;
		let (parts, body) = response.into_parts();
		let body = hyper_helpers::read_response_to_body(&parts.headers, body, self.max_request_body_size).await?;

		// Note that we don't check the Content-Type of the request. This is deemed
		// unnecessary, as a parsing error while happen anyway.
		let response = serde_json::from_slice(&body).map_err(Error::ParseError)?;
		//log::debug!("recv: {}", serde_json::to_string(&response).expect("valid JSON; qed"));
		Ok(response)
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
	use jsonrpsee_types::jsonrpc::{Call, Id, MethodCall, Params, Request, Version};

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

		let request = Request::Single(Call::MethodCall(MethodCall {
			jsonrpc: Version::V2,
			method: "request_larger_than_eightybytes".to_string(),
			params: Params::None,
			id: Id::Num(1),
		}));
		let bytes = serde_json::to_vec(&request).unwrap();
		assert_eq!(bytes.len(), 81);
		let response = client.send_request(request).await.unwrap_err();
		assert!(matches!(response, Error::RequestTooLarge));
	}
}
