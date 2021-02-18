// Implementation note: hyper's API is not adapted to async/await at all, and there's
// unfortunately a lot of boilerplate here that could be removed once/if it gets reworked.
//
// Additionally, despite the fact that hyper is capable of performing requests to multiple different
// servers through the same `hyper::Client`, we don't use that feature on purpose. The reason is
// that we need to be guaranteed that hyper doesn't re-use an existing connection if we ever reset
// the JSON-RPC request id to a value that might have already been used.

use async_trait::async_trait;
use futures::{channel::mpsc, channel::oneshot, prelude::*};
use jsonrpsee_types::{
	error::GenericTransportError,
	http::HttpConfig,
	jsonrpc::{self, Error as JsonRpcError},
	traits::{TransportReceiver, TransportSender},
};
use jsonrpsee_utils::http::hyper_helpers;
use thiserror::Error;
type HyperResult = Result<hyper::Response<hyper::Body>, hyper::Error>;

const CONTENT_TYPE_JSON: &str = "application/json";

pub fn http_transport(target: impl AsRef<str>, config: HttpConfig) -> Result<(Sender, Receiver), Error> {
	let url = url::Url::parse(target.as_ref()).map_err(|e| Error::Url(format!("Invalid URL: {}", e)))?;
	let (tx, rx) = mpsc::channel(4);
	let sender = Sender { to_back: tx, url, client: hyper::Client::new(), config };
	let receiver = Receiver { responses: rx };
	Ok((sender, receiver))
}

/// HTTP Transport Sender.
pub struct Sender {
	/// to back
	to_back: mpsc::Sender<HyperResult>,
	/// Target to connect to.
	url: url::Url,
	/// HTTP client,
	client: hyper::Client<hyper::client::HttpConnector>,
	/// Configurable max request body size
	config: HttpConfig,
}

/// HTTP Transport Receiver.
pub struct Receiver {
	/// Receives responses in any order.
	responses: mpsc::Receiver<HyperResult>,
}

#[async_trait]
impl TransportSender for Sender {
	async fn send(&mut self, request: jsonrpc::Request) -> Result<(), JsonRpcError> {
		let req: hyper::Request<hyper::Body> = jsonrpc::to_vec(&request)
			.map(|body| {
				hyper::Request::post(self.url.as_str())
					.header(hyper::header::CONTENT_TYPE, hyper::header::HeaderValue::from_static(CONTENT_TYPE_JSON))
					.body(From::from(body))
					.expect("Uri and request headers are valid; qed") // TODO: not necessarily true for URL here
			})
			.unwrap();

		let response = self.client.request(req).await;
		self.to_back.send(response).await.unwrap();
		Ok(())
	}
}

#[async_trait]
impl TransportReceiver for Receiver {
	async fn receive(&mut self) -> Result<jsonrpc::Response, JsonRpcError> {
		let response = match self.responses.next().await {
			Some(Ok(r)) => r,
			_ => todo!(),
		};
		let (parts, body) = response.into_parts();
		let body = hyper_helpers::read_response_to_body(&parts.headers, body, HttpConfig::default()).await.unwrap();

		// Note that we don't check the Content-Type of the request. This is deemed
		// unnecessary, as a parsing error while happen anyway.
		let response: jsonrpc::Response = jsonrpc::from_slice(&body).map_err(Error::ParseError).unwrap();
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
	/*use super::{Error, HttpTransportClient};
	use jsonrpsee_types::{
		http::HttpConfig,
		jsonrpc::{Call, Id, MethodCall, Params, Request, Version},
		traits::TransportSender,
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
		let response = client.send(request).await.unwrap_err();
		assert!(matches!(response, Error::RequestTooLarge));
	}*/
}
