use crate::client::http::transport::HttpTransportClient;
use crate::types::error::Error;
use crate::types::http::HttpConfig;
use crate::types::jsonrpc::{self, JsonValue};
use std::convert::TryInto;
use std::sync::atomic::{AtomicU64, Ordering};

/// JSON-RPC HTTP Client that provides functionality to perform method calls and notifications.
///
/// WARNING: The async methods must be executed on [Tokio 0.2](https://docs.rs/tokio/0.2.22/tokio).
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
			jsonrpc::Response::Single(rp) => {
				let (val, received_id) = Self::process_response(rp)?;
				if id == received_id {
					Ok(val)
				} else {
					Err(Error::InvalidRequestId)
				}
			}
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

	/// Perform a batch request towards the server.
	///
	/// Returns `Ok` if all requests were answered successfully.
	/// Returns `Error` if any of the requests fails.
	///
	/// If more than `u64::MAX` requests are performed in the `batch` then duplicate IDs are used
	/// which we don't support because ID is used to uniquely identify a given request.
	///
	// TODO(niklasad1): maybe simplify generic `requests`, it's quite unreadable.
	pub async fn batch_request<'a>(
		&self,
		requests: impl IntoIterator<Item = (impl Into<String>, impl Into<jsonrpc::Params>)>,
	) -> Result<Vec<JsonValue>, Error> {
		let mut calls = Vec::new();
		// NOTE: `request_id` is not necessilary increasing, it might wrap around at `u64::MAX`
		// Thus, we can't use `request_id` as ordering.
		let mut id_lookup_table = fnv::FnvHashMap::default();

		for (order_id, (method, params)) in requests.into_iter().enumerate() {
			let id = self.request_id.fetch_add(1, Ordering::SeqCst);
			calls.push(jsonrpc::Call::MethodCall(jsonrpc::MethodCall {
				jsonrpc: jsonrpc::Version::V2,
				method: method.into(),
				params: params.into(),
				id: jsonrpc::Id::Num(id),
			}));
			id_lookup_table.insert(id, order_id);
		}

		let batch_request = jsonrpc::Request::Batch(calls);
		let response = self
			.transport
			.send_request_and_wait_for_response(batch_request)
			.await
			.map_err(|e| Error::TransportError(Box::new(e)))?;

		match response {
			jsonrpc::Response::Single(_) => {
				Err(Error::Custom("Server replied with single response to a batch request".to_string()))
			}
			jsonrpc::Response::Notif(_notif) => {
				Err(Error::Custom("Server replied with notification to a a batch request".to_string()))
			}
			jsonrpc::Response::Batch(rps) => {
				if rps.len() != id_lookup_table.len() {
					return Err(Error::InvalidRequestId);
				}

				// TODO(niklasad1): this could be replaced by `smallvec`.
				let mut responses = vec![JsonValue::Null; rps.len()];
				for rp in rps {
					let (val, id) = Self::process_response(rp)?;
					if let Some(pos) = id_lookup_table.remove(&id) {
						responses[pos] = val;
					} else {
						return Err(Error::InvalidRequestId);
					}
				}
				Ok(responses)
			}
		}
	}

	fn process_response(response: jsonrpc::Output) -> Result<(JsonValue, u64), Error> {
		match response.id().as_number().cloned() {
			Some(n) => Ok((response.try_into().map_err(Error::Request)?, n)),
			_ => Err(Error::InvalidRequestId),
		}
	}
}
