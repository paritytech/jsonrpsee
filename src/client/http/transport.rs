// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// In particular, hyper can only be polled by tokio, but we don't want users to have to suffer
// from this restriction. We therefore spawn a background thread dedicated to running the tokio
// runtime.
//
// In order to perform a request, we send this request to the background thread through a channel
// and wait for an answer to come back.
//
// Additionally, despite the fact that hyper is capable of performing requests to multiple different
// servers through the same `hyper::Client`, we don't use that feature on purpose. The reason is
// that we need to be guaranteed that hyper doesn't re-use an existing connection if we ever reset
// the JSON-RPC request id to a value that might have already been used.

use crate::types::jsonrpc;
use std::fmt;
use thiserror::Error;

/// Implementation of a raw client for HTTP requests.
pub struct HttpTransportClient {
	/// URI to connect to.
	uri: String,
	/// Hyper to client,
	client: hyper::Client<hyper::client::HttpConnector>,
}

impl HttpTransportClient {
	/// Initializes a new HTTP client.
	// TODO: better type for target
	pub fn new(uri: &str) -> Self {
		HttpTransportClient { client: hyper::Client::new(), uri: uri.to_owned() }
	}

	/// Send request.
	pub async fn send_request(&self, request: jsonrpc::Request) -> Result<hyper::Response<hyper::Body>, RequestError> {
		let body = jsonrpc::to_vec(&request).map_err(|e| RequestError::Serialization(e))?;

		let req = hyper::Request::post(&self.uri)
			.header(hyper::header::CONTENT_TYPE, hyper::header::HeaderValue::from_static("application/json"))
			.body(From::from(body))
			.expect("Uri and request headers are valid; qed");

		let response = match self.client.request(req).await {
			Ok(r) => r,
			Err(err) => return Err(RequestError::Http(Box::new(err))),
		};

		if !response.status().is_success() {
			return Err(RequestError::RequestFailure { status_code: response.status().into() });
		}
		Ok(response)
	}

	/// Send notification.
	pub async fn send_notification(&self, request: jsonrpc::Request) -> Result<(), RequestError> {
		let _response = self.send_request(request).await?;
		Ok(())
	}

	/// Send request and wait for response.
	pub async fn send_request_and_wait_for_response(
		&self,
		request: jsonrpc::Request,
	) -> Result<jsonrpc::Response, RequestError> {
		let response = self.send_request(request).await?;

		// TODO: enforce a maximum size here
		let body =
			hyper::body::to_bytes(response.into_body()).await.map_err(|err| RequestError::Http(Box::new(err)))?;

		// Note that we don't check the Content-Type of the request. This is deemed
		// unnecessary, as a parsing error while happen anyway.
		// TODO: use Response::from_json
		let as_json: jsonrpc::Response = jsonrpc::from_slice(&body).map_err(RequestError::ParseError)?;
		log::debug!("recv: {}", jsonrpc::to_string(&as_json).expect("request valid JSON; qed"));
		Ok(as_json)
	}
}

impl fmt::Debug for HttpTransportClient {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_tuple("HttpTransportClient").finish()
	}
}

/// Error that can happen during a request.
#[derive(Debug, Error)]
pub enum RequestError {
	/// Error while serializing the request.
	// TODO: can that happen?
	#[error("error while serializing the request")]
	Serialization(#[source] serde_json::error::Error),

	/// Response given by the server failed to decode as UTF-8.
	#[error("response body is not UTF-8")]
	Utf8(#[source] std::string::FromUtf8Error),

	/// Error during the HTTP request, including networking errors and HTTP protocol errors.
	#[error("error while performing the HTTP request")]
	Http(Box<dyn std::error::Error + Send + Sync>),

	/// Server returned a non-success status code.
	#[error("server returned an error status code: {:?}", status_code)]
	RequestFailure {
		/// Status code returned by the server.
		status_code: u16,
	},

	/// Failed to parse the JSON returned by the server into a JSON-RPC response.
	#[error("error while parsing the response body")]
	ParseError(#[source] serde_json::error::Error),
}

#[cfg(test)]
mod tests {
	use super::*;
	use futures::channel::oneshot;
}
