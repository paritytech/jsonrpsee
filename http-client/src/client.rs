use crate::transport::HttpTransportClient;
use async_trait::async_trait;
use jsonrpc::DeserializeOwned;
use jsonrpsee_types::{error::Error, jsonrpc, traits::Client};

use std::convert::TryInto;
use std::sync::atomic::{AtomicU64, Ordering};

/// Http Client Builder.
#[derive(Debug)]
pub struct HttpClientBuilder {
	max_request_body_size: u32,
}

impl HttpClientBuilder {
	/// Sets the maximum size of a request body in bytes (default is 10 MiB).
	pub fn max_request_body_size(mut self, size: u32) -> Self {
		self.max_request_body_size = size;
		self
	}

	/// Build the HTTP client with target to connect to.
	pub fn build(self, target: impl AsRef<str>) -> Result<HttpClient, Error> {
		let transport = HttpTransportClient::new(target, self.max_request_body_size)
			.map_err(|e| Error::TransportError(Box::new(e)))?;
		Ok(HttpClient { transport, request_id: AtomicU64::new(0) })
	}
}

impl Default for HttpClientBuilder {
	fn default() -> Self {
		Self { max_request_body_size: 10 * 1024 * 1024 }
	}
}

/// JSON-RPC HTTP Client that provides functionality to perform method calls and notifications.
#[derive(Debug)]
pub struct HttpClient {
	/// HTTP transport client.
	transport: HttpTransportClient,
	/// Request ID that wraps around when overflowing.
	request_id: AtomicU64,
}

#[async_trait]
impl Client for HttpClient {
	async fn notification<M, P>(&self, method: M, params: P) -> Result<(), Error>
	where
		M: Into<String> + Send,
		P: Into<jsonrpc::Params> + Send,
	{
		let request = jsonrpc::Request::Single(jsonrpc::Call::Notification(jsonrpc::Notification {
			jsonrpc: jsonrpc::Version::V2,
			method: method.into(),
			params: params.into(),
		}));

		self.transport.send_notification(request).await.map_err(|e| Error::TransportError(Box::new(e)))
	}

	/// Perform a request towards the server.
	async fn request<T, M, P>(&self, method: M, params: P) -> Result<T, Error>
	where
		T: DeserializeOwned,
		M: Into<String> + Send,
		P: Into<jsonrpc::Params> + Send,
	{
		// NOTE: `fetch_add` wraps on overflow which is intended.
		let id = self.request_id.fetch_add(1, Ordering::SeqCst);
		let request = jsonrpc::Request::Single(jsonrpc::Call::MethodCall(jsonrpc::MethodCall {
			jsonrpc: jsonrpc::Version::V2,
			method: method.into(),
			params: params.into(),
			id: jsonrpc::Id::Num(id),
		}));

		let response = self
			.transport
			.send_request_and_wait_for_response(request)
			.await
			.map_err(|e| Error::TransportError(Box::new(e)))?;

		let json_value = match response {
			jsonrpc::Response::Single(response) => match response.id() {
				jsonrpc::Id::Num(n) if n == &id => response.try_into().map_err(Error::Request),
				_ => Err(Error::InvalidRequestId),
			},
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
	}
}
