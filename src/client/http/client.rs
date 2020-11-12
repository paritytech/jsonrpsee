use crate::client::http::transport::HttpTransportClient;
use crate::types::client::{Error, Mismatch};
use crate::types::jsonrpc::{self, JsonValue};
use std::sync::atomic::{AtomicU64, Ordering};

/// Default maximum request body size (10 MB).
const DEFAULT_MAX_BODY_SIZE_TEN_MB: u32 = 10 * 1024 * 1024;

/// HTTP configuration.
#[derive(Copy, Clone)]
pub struct HttpConfig {
	/// Maximum request body size in bytes.
	pub max_request_body_size: u32,
}

/// JSON-RPC HTTP Client that provides functionality to perform method calls and notifications.
///
/// WARNING: The async methods must be executed on [Tokio 0.2](https://docs.rs/tokio/0.2.22/tokio).
pub struct HttpClient {
	/// HTTP transport client.
	transport: HttpTransportClient,
	/// Request ID that wraps around when overflowing.
	request_id: AtomicU64,
}

impl Default for HttpConfig {
	fn default() -> Self {
		Self { max_request_body_size: DEFAULT_MAX_BODY_SIZE_TEN_MB }
	}
}

impl HttpClient {
	/// Initializes a new HTTP client.
	///
	/// Fails when the URL is invalid.
	pub fn new(target: impl AsRef<str>, config: HttpConfig) -> Result<Self, Error> {
		let transport = HttpTransportClient::new(target, config.max_request_body_size)
			.map_err(|e| Error::TransportError(Box::new(e)))?;
		Ok(Self { transport, request_id: AtomicU64::new(0) })
	}

	/// Send a notification to the server.
	///
	/// WARNING: This method must be executed on [Tokio 0.2](https://docs.rs/tokio/0.2.22/tokio).
	pub async fn notification(
		&self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<(), Error> {
		let request = jsonrpc::Request::Single(jsonrpc::Call::Notification(jsonrpc::Notification {
			jsonrpc: jsonrpc::Version::V2,
			method: method.into(),
			params: params.into(),
		}));

		self.transport.send_notification(request).await.map_err(|e| Error::TransportError(Box::new(e)))
	}

	/// Perform a request towards the server.
	///
	/// WARNING: This method must be executed on [Tokio 0.2](https://docs.rs/tokio/0.2.22/tokio).
	pub async fn request(
		&self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<JsonValue, Error> {
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

		match response {
			jsonrpc::Response::Single(rp) => Self::process_response(rp, id),
			// Server should not send batch response to a single request.
			jsonrpc::Response::Batch(_rps) => {
				Err(Error::Custom("Server replied with batch response to a single request".to_string()))
			}
			// Server should not reply to a Notification.
			jsonrpc::Response::Notif(_notif) => {
				Err(Error::Custom(format!("Server replied with notification response to request ID: {}", id)))
			}
		}
	}

	fn process_response(response: jsonrpc::Output, expected_id: u64) -> Result<JsonValue, Error> {
		match response.id() {
			jsonrpc::Id::Num(n) if n == &expected_id => {
				let ret: Result<JsonValue, _> = response.into();
				ret.map_err(Error::Request)
			}
			jsonrpc::Id::Num(n) => {
				Err(Error::InvalidRequestId(Mismatch { expected: expected_id.into(), got: (*n).into() }))
			}
			jsonrpc::Id::Str(s) => {
				Err(Error::InvalidRequestId(Mismatch { expected: expected_id.into(), got: s.to_string().into() }))
			}
			jsonrpc::Id::Null => {
				Err(Error::InvalidRequestId(Mismatch { expected: expected_id.into(), got: JsonValue::Null }))
			}
		}
	}
}
