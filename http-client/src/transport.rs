// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// Additionally, despite the fact that hyper is capable of performing requests to multiple different
// servers through the same `hyper::Client`, we don't use that feature on purpose. The reason is
// that we need to be guaranteed that hyper doesn't re-use an existing connection if we ever reset
// the JSON-RPC request id to a value that might have already been used.

use hyper::{
	client::{Client, HttpConnector},
	Body, Request, Response,
};
use jsonrpsee_types::{error::GenericTransportError, http::HttpConfig, jsonrpc};
use jsonrpsee_utils::http::hyper_helpers;
use thiserror::Error;

const CONTENT_TYPE_JSON: &str = "application/json";

/// Wrapper enum around [`hyper::Client`] to support `HTTP` or `HTTPS` connector.
#[derive(Clone, Debug)]
pub enum HyperClient {
	/// Plain mode (`http://` URL).
	Http(Client<HttpConnector>),
	/// HTTPs mode (`https://` URL).
	#[cfg(feature = "tls")]
	Https(Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>),
}

impl HyperClient {
	async fn request(&self, req: Request<Body>) -> Result<Response<Body>, Error> {
		match self {
			Self::Http(inner) => inner.request(req).await.map_err(|e| Error::Http(Box::new(e))),
			Self::Https(inner) => inner.request(req).await.map_err(|e| Error::Http(Box::new(e))),
		}
	}
}

/// HTTP Transport Client.
#[derive(Debug, Clone)]
pub struct HttpTransportClient {
	/// Target to connect to.
	target: url::Url,
	/// HTTP client,
	client: HyperClient,
	/// Configurable max request body size
	config: HttpConfig,
}

impl HttpTransportClient {
	/// Initializes a new HTTP client.
	pub fn new(target: impl AsRef<str>, config: HttpConfig) -> Result<Self, Error> {
		let target = url::Url::parse(target.as_ref()).map_err(|e| Error::Url(format!("Invalid URL: {}", e)))?;
		if target.scheme() == "http" || target.scheme() == "https" {
			let client = if cfg!(feature = "tls") {
				let connector = hyper_rustls::HttpsConnector::with_native_roots();
				let client = hyper::Client::builder().build::<_, hyper::Body>(connector);
				HyperClient::Https(client)
			} else {
				HyperClient::Http(hyper::Client::new())
			};
			Ok(HttpTransportClient { client, target, config })
		} else {
			Err(Error::Url("URL scheme not supported, expects 'http or https'".into()))
		}
	}

	/// Send request.
	async fn send_request(&self, request: jsonrpc::Request) -> Result<hyper::Response<hyper::Body>, Error> {
		let body = jsonrpc::to_vec(&request).map_err(Error::Serialization)?;
		log::debug!("send: {}", request);

		if body.len() > self.config.max_request_body_size as usize {
			return Err(Error::RequestTooLarge);
		}

		let req = hyper::Request::post(self.target.as_str())
			.header(hyper::header::CONTENT_TYPE, hyper::header::HeaderValue::from_static(CONTENT_TYPE_JSON))
			.header(hyper::header::ACCEPT, hyper::header::HeaderValue::from_static(CONTENT_TYPE_JSON))
			.body(From::from(body))
			.expect("URI and request headers are valid; qed");

		let response = self.client.request(req).await?;
		if response.status().is_success() {
			Ok(response)
		} else {
			Err(Error::RequestFailure { status_code: response.status().into() })
		}
	}

	/// Send notification.
	pub async fn send_notification(&self, request: jsonrpc::Request) -> Result<(), Error> {
		let _response = self.send_request(request).await?;
		Ok(())
	}

	/// Send request and wait for response.
	pub async fn send_request_and_wait_for_response(
		&self,
		request: jsonrpc::Request,
	) -> Result<jsonrpc::Response, Error> {
		let response = self.send_request(request).await?;
		let (parts, body) = response.into_parts();
		let body = hyper_helpers::read_response_to_body(&parts.headers, body, self.config).await?;

		// Note that we don't check the Content-Type of the request. This is deemed
		// unnecessary, as a parsing error while happen anyway.
		let response: jsonrpc::Response = jsonrpc::from_slice(&body).map_err(Error::ParseError)?;
		log::debug!("recv: {}", jsonrpc::to_string(&response).expect("request valid JSON; qed"));
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
	use jsonrpsee_types::{
		http::HttpConfig,
		jsonrpc::{Call, Id, MethodCall, Params, Request, Version},
	};

	#[test]
	fn invalid_http_url_rejected() {
		let err = HttpTransportClient::new("ws://localhost:9933", HttpConfig::default()).unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[tokio::test]
	async fn request_limit_works() {
		let eighty_bytes_limit = 80;
		let client =
			HttpTransportClient::new("http://localhost:9933", HttpConfig { max_request_body_size: 80 }).unwrap();
		assert_eq!(client.config.max_request_body_size, eighty_bytes_limit);

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
