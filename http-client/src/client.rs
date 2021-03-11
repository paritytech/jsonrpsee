use crate::transport::HttpTransportClient;
use async_trait::async_trait;
use jsonrpc::DeserializeOwned;
use jsonrpsee_types::{error::Error, http::HttpConfig, jsonrpc, traits::Client, jsonrpc::JsonValue};
use std::convert::TryInto;
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashSet;

/// JSON-RPC HTTP Client that provides functionality to perform method calls and notifications.
#[derive(Debug)]
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

	/// Perform a batch request towards the server.
	///
	/// Returns `Ok` if all requests were answered successfully.
	/// Returns `Error` if any of the requests fails.
	//
	// TODO(niklasad1): maybe simplify generic `requests`, it's quite unreadable.
	pub async fn batch_request<'a, T>(
		&self,
		requests: impl IntoIterator<Item = (impl Into<String>, impl Into<jsonrpc::Params>)>,
	) -> Result<Vec<T>, Error>
	where
		T: DeserializeOwned,
	{
		let mut calls = Vec::new();
		// NOTE(niklasad1): If more than `u64::MAX` requests are performed in the `batch` then duplicate IDs are used
		// which we don't support because ID is used to uniquely identify a given request.
		let mut ids = HashSet::new();

		for (method, params) in requests.into_iter() {
			let id = self.request_id.fetch_add(1, Ordering::SeqCst);
			calls.push(jsonrpc::Call::MethodCall(jsonrpc::MethodCall {
				jsonrpc: jsonrpc::Version::V2,
				method: method.into(),
				params: params.into(),
				id: jsonrpc::Id::Num(id),
			}));
			ids.insert(id);
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
				let mut responses = Vec::with_capacity(ids.len());
				for rp in rps {
					let id = match rp.id().as_number() {
						Some(n) => *n,
						_ => return Err(Error::InvalidRequestId),
					};
					if !ids.remove(&id) {
						return Err(Error::InvalidRequestId);
					}
					let json_val: JsonValue = rp.try_into().map_err(Error::Request)?;
					let val = jsonrpc::from_value(json_val).map_err(Error::ParseError)?;
					responses.push(val);
				}
				Ok(responses)
			}
		}
	}

	fn process_response(response: jsonrpc::Output) -> Result<(JsonValue, u64), Error> {
		match response.id().as_number().copied() {
			Some(n) => Ok((response.try_into().map_err(Error::Request)?, n)),
			_ => Err(Error::InvalidRequestId),
		}
	}
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
		}?;
		jsonrpc::from_value(json_value).map_err(Error::ParseError)
	}
}
