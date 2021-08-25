// Copyright 2019-2020 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::transport::HttpTransportClient;
use crate::types::{
	traits::Client,
	v2::{
		error::JsonRpcError,
		params::{Id, JsonRpcParams},
		request::{JsonRpcCallSer, JsonRpcNotificationSer},
		response::JsonRpcResponse,
	},
	Error, TEN_MB_SIZE_BYTES,
};
use async_trait::async_trait;
use fnv::FnvHashMap;
use serde::de::DeserializeOwned;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

/// Http Client Builder.
#[derive(Debug)]
pub struct HttpClientBuilder {
	max_request_body_size: u32,
	request_timeout: Duration,
}

impl HttpClientBuilder {
	/// Sets the maximum size of a request body in bytes (default is 10 MiB).
	pub fn max_request_body_size(mut self, size: u32) -> Self {
		self.max_request_body_size = size;
		self
	}

	/// Set request timeout (default is 60 seconds).
	pub fn request_timeout(mut self, timeout: Duration) -> Self {
		self.request_timeout = timeout;
		self
	}

	/// Build the HTTP client with target to connect to.
	pub fn build(self, target: impl AsRef<str>) -> Result<HttpClient, Error> {
		let transport =
			HttpTransportClient::new(target, self.max_request_body_size).map_err(|e| Error::Transport(e.into()))?;
		Ok(HttpClient { transport, request_id: AtomicU64::new(0), request_timeout: self.request_timeout })
	}
}

impl Default for HttpClientBuilder {
	fn default() -> Self {
		Self { max_request_body_size: TEN_MB_SIZE_BYTES, request_timeout: Duration::from_secs(60) }
	}
}

/// JSON-RPC HTTP Client that provides functionality to perform method calls and notifications.
#[derive(Debug)]
pub struct HttpClient {
	/// HTTP transport client.
	transport: HttpTransportClient,
	/// Request ID that wraps around when overflowing.
	request_id: AtomicU64,
	/// Request timeout. Defaults to 60sec.
	request_timeout: Duration,
}

#[async_trait]
impl Client for HttpClient {
	async fn notification<'a>(&self, method: &'a str, params: JsonRpcParams<'a>) -> Result<(), Error> {
		let notif = JsonRpcNotificationSer::new(method, params);
		let fut = self.transport.send(serde_json::to_string(&notif).map_err(Error::ParseError)?);
		match tokio::time::timeout(self.request_timeout, fut).await {
			Ok(Ok(ok)) => Ok(ok),
			Err(_) => Err(Error::RequestTimeout),
			Ok(Err(e)) => Err(Error::Transport(e.into())),
		}
	}

	/// Perform a request towards the server.
	async fn request<'a, R>(&self, method: &'a str, params: JsonRpcParams<'a>) -> Result<R, Error>
	where
		R: DeserializeOwned,
	{
		// NOTE: `fetch_add` wraps on overflow which is intended.
		let id = self.request_id.fetch_add(1, Ordering::SeqCst);
		let request = JsonRpcCallSer::new(Id::Number(id), method, params);

		let fut = self.transport.send_and_read_body(serde_json::to_string(&request).map_err(Error::ParseError)?);
		let body = match tokio::time::timeout(self.request_timeout, fut).await {
			Ok(Ok(body)) => body,
			Err(_e) => return Err(Error::RequestTimeout),
			Ok(Err(e)) => return Err(Error::Transport(e.into())),
		};

		let response: JsonRpcResponse<_> = match serde_json::from_slice(&body) {
			Ok(response) => response,
			Err(_) => {
				let err: JsonRpcError = serde_json::from_slice(&body).map_err(Error::ParseError)?;
				return Err(Error::Request(err.to_string()));
			}
		};

		let response_id = response.id.as_number().copied().ok_or(Error::InvalidRequestId)?;

		if response_id == id {
			Ok(response.result)
		} else {
			Err(Error::InvalidRequestId)
		}
	}

	async fn batch_request<'a, R>(&self, batch: Vec<(&'a str, JsonRpcParams<'a>)>) -> Result<Vec<R>, Error>
	where
		R: DeserializeOwned + Default + Clone,
	{
		let mut batch_request = Vec::with_capacity(batch.len());
		// NOTE(niklasad1): `ID` is not necessarily monotonically increasing.
		let mut ordered_requests = Vec::with_capacity(batch.len());
		let mut request_set = FnvHashMap::with_capacity_and_hasher(batch.len(), Default::default());

		for (pos, (method, params)) in batch.into_iter().enumerate() {
			let id = self.request_id.fetch_add(1, Ordering::SeqCst);
			batch_request.push(JsonRpcCallSer::new(Id::Number(id), method, params));
			ordered_requests.push(id);
			request_set.insert(id, pos);
		}

		let fut = self.transport.send_and_read_body(serde_json::to_string(&batch_request).map_err(Error::ParseError)?);

		let body = match tokio::time::timeout(self.request_timeout, fut).await {
			Ok(Ok(body)) => body,
			Err(_e) => return Err(Error::RequestTimeout),
			Ok(Err(e)) => return Err(Error::Transport(e.into())),
		};

		let rps: Vec<JsonRpcResponse<_>> = match serde_json::from_slice(&body) {
			Ok(response) => response,
			Err(_) => {
				let err: JsonRpcError = serde_json::from_slice(&body).map_err(Error::ParseError)?;
				return Err(Error::Request(err.to_string()));
			}
		};

		// NOTE: `R::default` is placeholder and will be replaced in loop below.
		let mut responses = vec![R::default(); ordered_requests.len()];
		for rp in rps {
			let response_id = rp.id.as_number().copied().ok_or(Error::InvalidRequestId)?;
			let pos = match request_set.get(&response_id) {
				Some(pos) => *pos,
				None => return Err(Error::InvalidRequestId),
			};
			responses[pos] = rp.result
		}
		Ok(responses)
	}
}
