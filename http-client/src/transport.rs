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

const CONTENT_TYPE_JSON: &str = "application/json";

pub fn http_transport(target: impl AsRef<str>, config: HttpConfig) -> Result<(Sender, Receiver), Error> {
	let target = url::Url::parse(target.as_ref()).map_err(|e| Error::Url(format!("Invalid URL: {}", e)))?;
	let (to_back, from_send) = mpsc::channel::<()>(4);
	let (to_recv, from_back) = mpsc::channel::<()>(4);
	let sender = Sender { to_back, target, client: hyper::Client::new(), config };
	background_thread(to_recv, from_send);
	let receiver = Receiver { responses: from_back };
	Ok((sender, receiver))
}

/// Message transmitted from the foreground task to the background.
struct FrontToBack {
	/// Request that the background task should perform.
	request: hyper::Request<hyper::Body>,
	/// Channel to send back to the response.
	send_back: oneshot::Sender<Result<hyper::Response<hyper::Body>, hyper::Error>>,
}

/// HTTP Transport Sender.
pub struct Sender {
	/// to back
	to_back: mpsc::Sender<()>,
	/// Target to connect to.
	target: url::Url,
	/// HTTP client,
	client: hyper::Client<hyper::client::HttpConnector>,
	/// Configurable max request body size
	config: HttpConfig,
}

/// HTTP Transport Receiver.
pub struct Receiver {
	/// Receives responses in any order.
	responses: mpsc::Receiver<()>,
}

#[async_trait]
impl TransportSender for Sender {
	async fn send(&mut self, _request: jsonrpc::Request) -> Result<(), JsonRpcError> {
		todo!();
	}
}

#[async_trait]
impl TransportReceiver for Receiver {
	async fn receive(&mut self) -> Result<jsonrpc::Response, JsonRpcError> {
		todo!();
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

/// Function that runs in a background thread.
fn background_thread(mut from_send: mpsc::Sender<()>, mut to_receiver: mpsc::Receiver<()>) {
	std::thread::spawn(move || {
		let mut runtime = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
			Ok(r) => r,
			Err(err) => {
				// Ideally, we would try to initialize the tokio runtime in the main thread then move
				// it here. That however isn't possible. If we fail to initialize the runtime, the only
				// thing we can do is print an error and shut down the background thread.
				// Initialization failures should be almost non-existant anyway, so this isn't a big
				// deal.
				log::error!("Failed to initialize tokio runtime: {:?}", err);
				return;
			}
		};

		// Running until the channel has been closed, and all requests have been completed.
		runtime.block_on(async move {
			// Collection of futures that process ongoing requests.
			//let mut pending_requests = stream::FuturesUnordered::new();

			loop {
				// recv from channel.
			}
		});
	});
}

#[cfg(test)]
mod tests {
	use super::{Error, HttpTransportClient};
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
		//assert!(matches!(response, Error::RequestTooLarge));
	}
}
