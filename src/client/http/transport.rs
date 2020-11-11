// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// Additionally, despite the fact that hyper is capable of performing requests to multiple different
// servers through the same `hyper::Client`, we don't use that feature on purpose. The reason is
// that we need to be guaranteed that hyper doesn't re-use an existing connection if we ever reset
// the JSON-RPC request id to a value that might have already been used.

use crate::types::jsonrpc;
use futures::StreamExt;
use thiserror::Error;

/// HTTP Transport Client.
#[derive(Debug, Clone)]
pub struct HttpTransportClient {
	/// Target to connect to.
	target: url::Url,
	/// HTTP client,
	client: hyper::Client<hyper::client::HttpConnector>,
	/// Configurable max request body size
	max_request_body_size: usize,
}

impl HttpTransportClient {
	/// Initializes a new HTTP client.
	pub fn new(target: impl AsRef<str>, max_request_body_size: usize) -> Result<Self, Error> {
		let target = url::Url::parse(target.as_ref()).map_err(|e| Error::Url(format!("Invalid URL: {}", e).into()))?;
		if target.scheme() != "http" {
			return Err(Error::Url("URL scheme not supported, expects 'http'".into()));
		};
		Ok(HttpTransportClient { client: hyper::Client::new(), target, max_request_body_size })
	}

	/// Send request.
	async fn send_request(&self, request: jsonrpc::Request) -> Result<hyper::Response<hyper::Body>, Error> {
		log::debug!("send: {}", jsonrpc::to_string(&request).expect("request valid JSON; qed"));
		let body = jsonrpc::to_vec(&request).map_err(|e| Error::Serialization(e))?;

		if body.len() > self.max_request_body_size {
			return Err(Error::RequestTooLarge);
		}

		let req = hyper::Request::post(self.target.as_str())
			.header(hyper::header::CONTENT_TYPE, hyper::header::HeaderValue::from_static("application/json"))
			.body(From::from(body))
			.expect("Uri and request headers are valid; qed");

		let response = match self.client.request(req).await {
			Ok(r) => r,
			Err(err) => return Err(Error::Http(Box::new(err))),
		};

		if !response.status().is_success() {
			return Err(Error::RequestFailure { status_code: response.status().into() });
		}

		Ok(response)
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
		let mut body_fut: hyper::Body = response.into_body();

		let mut body = Vec::new();

		while let Some(chunk) = body_fut.next().await {
			let chunk = chunk.map_err(|e| Error::Http(Box::new(e)))?;
			if chunk.len() + body.len() > self.max_request_body_size {
				return Err(Error::RequestTooLarge);
			}
			body.extend_from_slice(&chunk);
		}

		// Note that we don't check the Content-Type of the request. This is deemed
		// unnecessary, as a parsing error while happen anyway.
		// TODO: use Response::from_json
		let as_json: jsonrpc::Response = jsonrpc::from_slice(&body).map_err(Error::ParseError)?;
		log::debug!("recv: {}", jsonrpc::to_string(&as_json).expect("request valid JSON; qed"));
		Ok(as_json)
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
	#[error("The request body was to large")]
	RequestTooLarge,
}

#[cfg(test)]
mod tests {
	use super::{Error, HttpTransportClient};
	use crate::types::jsonrpc::{Call, Id, MethodCall, Params, Request, Version};

	#[test]
	fn invalid_http_url_rejected() {
		let err = HttpTransportClient::new("ws://localhost:9933", 1337).unwrap_err();
		assert!(matches!(err, Error::Url(_)));
	}

	#[tokio::test]
	async fn request_limit_works() {
		let eighty_bytes_limit = 80;
		let client = HttpTransportClient::new("http://localhost:9933", eighty_bytes_limit).unwrap();
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
