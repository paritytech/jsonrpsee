use crate::transport::HttpTransportClient;
use async_trait::async_trait;
use fnv::FnvHashMap;
use jsonrpsee_types::{
	error::{Error, Mismatch},
	traits::Client,
	v2::dummy::{JsonRpcCall, JsonRpcMethod, JsonRpcNotification, JsonRpcParams, JsonRpcResponse},
};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};

const SINGLE_RESPONSE: &str = "Single Response";
const BATCH_RESPONSE: &str = "Batch response";
const SUBSCRIPTION_RESPONSE: &str = "Subscription response";

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
	async fn notification<'a, T>(&self, method: &'a str, params: JsonRpcParams<'a, T>) -> Result<(), Error>
	where
		T: Serialize + std::fmt::Debug + PartialEq + Send + Sync,
	{
		let notif = JsonRpcNotification::new(method, params);
		self.transport.send_notification(notif).await.map_err(|e| Error::TransportError(Box::new(e)))
	}

	/// Perform a request towards the server.
	async fn request<'a, T, R>(&self, method: &'a str, params: JsonRpcParams<'a, T>) -> Result<R, Error>
	where
		T: Serialize + std::fmt::Debug + PartialEq + Send + Sync,
		R: DeserializeOwned,
	{
		// NOTE: `fetch_add` wraps on overflow which is intended.
		let id = self.request_id.fetch_add(1, Ordering::Relaxed);
		let request = JsonRpcCall::new(id, method, params);

		let response = self
			.transport
			.send_request_and_wait_for_response(request)
			.await
			.map_err(|e| Error::TransportError(Box::new(e)))?;

		match response {
			JsonRpcResponse::Single(response) if response.id == id => Ok(response.result),
			JsonRpcResponse::Single(_) => Err(Error::InvalidRequestId),
			JsonRpcResponse::Batch(_rps) => Err(invalid_response(SINGLE_RESPONSE, BATCH_RESPONSE)),
			JsonRpcResponse::Subscription(_notif) => Err(invalid_response(SINGLE_RESPONSE, SUBSCRIPTION_RESPONSE)),
		}
	}

	async fn batch_request<'a, T, R>(&self, batch: Vec<(&'a str, JsonRpcParams<'a, T>)>) -> Result<Vec<R>, Error>
	where
		T: Serialize + std::fmt::Debug + PartialEq + Send + Sync,
		R: DeserializeOwned + Default + Clone,
	{
		let mut batch_request = Vec::with_capacity(batch.len());
		// NOTE(niklasad1): `ID` is not necessarily monotonically increasing.
		let mut ordered_requests = Vec::with_capacity(batch.len());
		let mut request_set = FnvHashMap::with_capacity_and_hasher(batch.len(), Default::default());

		for (pos, (method, params)) in batch.into_iter().enumerate() {
			let id = self.request_id.fetch_add(1, Ordering::SeqCst);
			batch_request.push(JsonRpcCall::new(id, method, params));
			ordered_requests.push(id);
			request_set.insert(id, pos);
		}

		let response = self
			.transport
			.send_request_and_wait_for_response(batch_request)
			.await
			.map_err(|e| Error::TransportError(Box::new(e)))?;

		match response {
			JsonRpcResponse::Single(_response) => Err(invalid_response(BATCH_RESPONSE, SINGLE_RESPONSE)),
			JsonRpcResponse::Subscription(_notif) => Err(invalid_response(BATCH_RESPONSE, SUBSCRIPTION_RESPONSE)),
			JsonRpcResponse::Batch(rps) => {
				// NOTE: `T::default` is placeholder and will be replaced in loop below.
				let mut responses = vec![R::default(); ordered_requests.len()];
				for rp in rps {
					let pos = match request_set.get(&rp.id) {
						Some(pos) => *pos,
						None => return Err(Error::InvalidRequestId),
					};
					responses[pos] = rp.result
				}
				Ok(responses)
			}
		}
	}
}

fn invalid_response(expected: impl Into<String>, got: impl Into<String>) -> Error {
	Error::InvalidResponse(Mismatch { expected: expected.into(), got: got.into() })
}
