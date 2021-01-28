use crate::transport::HttpTransportClient;
use futures::prelude::*;
use jsonrpc::DeserializeOwned;
use jsonrpsee_types::{
	error::Error,
	http::HttpConfig,
	jsonrpc::{self, JsonValue},
	traits::Client,
};

use std::convert::TryInto;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};

/// JSON-RPC HTTP Client that provides functionality to perform method calls and notifications.
///
/// WARNING: The async methods must be executed on [Tokio 1.0](https://docs.rs/tokio/1.0.1/tokio).
pub struct HttpClient {
	/// HTTP transport client.
	transport: HttpTransportClient,
	/// Request ID that wraps around when overflowing.
	request_id: AtomicU64,
}

impl HttpClient {
	/// Initializes a new HTTP client.
	///
	/// Fails when the URL is invalid.
	pub fn new(target: impl AsRef<str>, config: HttpConfig) -> Result<Self, Error> {
		let transport = HttpTransportClient::new(target, config).map_err(|e| Error::TransportError(Box::new(e)))?;
		Ok(Self { transport, request_id: AtomicU64::new(0) })
	}
}

impl Client for HttpClient {
	type Error = Error;
	type Subscription = ();

	fn notification<'a>(
		&'a self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>> {
		let request = jsonrpc::Request::Single(jsonrpc::Call::Notification(jsonrpc::Notification {
			jsonrpc: jsonrpc::Version::V2,
			method: method.into(),
			params: params.into(),
		}));

		Box::pin(async move {
			self.transport.send_notification(request).await.map_err(|e| Error::TransportError(Box::new(e)))
		})
	}

	fn request<'a, T: DeserializeOwned>(
		&'a self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Pin<Box<dyn Future<Output = Result<T, Self::Error>> + Send + 'a>> {
		// NOTE: `fetch_add` wraps on overflow which is intended.
		let id = self.request_id.fetch_add(1, Ordering::SeqCst);
		let request = jsonrpc::Request::Single(jsonrpc::Call::MethodCall(jsonrpc::MethodCall {
			jsonrpc: jsonrpc::Version::V2,
			method: method.into(),
			params: params.into(),
			id: jsonrpc::Id::Num(id),
		}));

		Box::pin(async move {
			let response = self
				.transport
				.send_request_and_wait_for_response(request)
				.await
				.map_err(|e| Error::TransportError(Box::new(e)))?;

			let json_value = match response {
				jsonrpc::Response::Single(rp) => process_response(rp, id),
				// Server should not send batch response to a single request.
				jsonrpc::Response::Batch(_rps) => {
					Err(Error::Custom("Server replied with batch response to a single request".to_string()))
				}
				// Server should not reply to a Notification.
				jsonrpc::Response::Notif(_notif) => {
					Err(Error::Custom(format!("Server replied with notification response to request ID: {}", id)))
				}
			}?;
			jsonrpc::from_value(json_value).map_err(Error::ParseError)
		})
	}

	fn subscribe<'a>(
		&'a self,
		_subscribe_method: impl Into<String>,
		_params: impl Into<jsonrpc::Params>,
		_unsubscribe_method: impl Into<String>,
	) -> Pin<Box<dyn Future<Output = Result<Self::Subscription, Self::Error>> + Send + 'a>> {
		Box::pin(async { Err(Error::Custom("Subscription not supported on HTTP transport".into())) })
	}
}

fn process_response(response: jsonrpc::Output, expected_id: u64) -> Result<JsonValue, Error> {
	match response.id() {
		jsonrpc::Id::Num(n) if n == &expected_id => response.try_into().map_err(Error::Request),
		_ => Err(Error::InvalidRequestId),
	}
}
