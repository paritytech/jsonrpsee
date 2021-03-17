use crate::transport::HttpTransportClient;
use async_trait::async_trait;
use fnv::FnvHashMap;
use jsonrpc::DeserializeOwned;
use jsonrpsee_types::{error::Error, http::HttpConfig, jsonrpc, traits::Client};
use std::convert::TryInto;
use std::sync::atomic::{AtomicU64, Ordering};

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

	async fn batch_request<T, M, P>(&self, batch: Vec<(M, P)>) -> Result<Vec<T>, Error>
	where
		T: DeserializeOwned + Default + Clone,
		M: Into<String> + Send,
		P: Into<jsonrpc::Params> + Send,
	{
		let mut calls = Vec::with_capacity(batch.len());
		// NOTE(niklasad1): `ID` is not necessarily monotonically increasing.
		let mut ordered_requests = Vec::with_capacity(batch.len());
		let mut request_set = FnvHashMap::with_capacity_and_hasher(batch.len(), Default::default());

		for (pos, (method, params)) in batch.into_iter().enumerate() {
			let id = self.request_id.fetch_add(1, Ordering::SeqCst);
			calls.push(jsonrpc::Call::MethodCall(jsonrpc::MethodCall {
				jsonrpc: jsonrpc::Version::V2,
				method: method.into(),
				params: params.into(),
				id: jsonrpc::Id::Num(id),
			}));
			ordered_requests.push(id);
			request_set.insert(id, pos);
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
				Err(Error::Custom("Server replied with notification to with a batch request".to_string()))
			}
			jsonrpc::Response::Batch(rps) => {
				// NOTE: `T::default` is placeholder and will be replaced in loop below.
				let mut responses = vec![T::default(); ordered_requests.len()];
				for rp in rps {
					let id = match rp.id().as_number() {
						Some(n) => *n,
						_ => return Err(Error::InvalidRequestId),
					};
					let pos = match request_set.get(&id) {
						Some(pos) => *pos,
						None => return Err(Error::InvalidRequestId),
					};
					let json_val: jsonrpc::JsonValue = rp.try_into().map_err(Error::Request)?;
					let response = jsonrpc::from_value(json_val).map_err(Error::ParseError)?;
					responses[pos] = response;
				}
				Ok(responses)
			}
		}
	}
}
