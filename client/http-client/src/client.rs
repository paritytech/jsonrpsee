// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

use std::sync::Arc;
use std::time::Duration;

use crate::transport::HttpTransportClient;
use crate::types::{ErrorResponse, Id, NotificationSer, ParamsSer, RequestSer, Response};
use async_trait::async_trait;
use hyper::http::HeaderMap;
use jsonrpsee_core::client::{CertificateStore, ClientT, IdKind, RequestIdManager, Subscription, SubscriptionClientT};
use jsonrpsee_core::tracing::RpcTracing;
use jsonrpsee_core::{Error, JsonRawValue, TEN_MB_SIZE_BYTES};
use jsonrpsee_types::error::CallError;
use rustc_hash::FxHashMap;
use serde::de::DeserializeOwned;
use tracing_futures::Instrument;

/// Http Client Builder.
///
/// # Examples
///
/// ```no_run
///
/// use jsonrpsee_http_client::{HttpClientBuilder, HeaderMap, HeaderValue};
///
/// #[tokio::main]
/// async fn main() {
///     // Build custom headers used for every submitted request.
///     let mut headers = HeaderMap::new();
///     headers.insert("Any-Header-You-Like", HeaderValue::from_static("42"));
///
///     // Build client
///     let client = HttpClientBuilder::default()
///          .set_headers(headers)
///          .build("wss://localhost:443")
///          .unwrap();
///
///     // use client....
/// }
///
/// ```
#[derive(Debug)]
pub struct HttpClientBuilder {
	max_request_body_size: u32,
	request_timeout: Duration,
	max_concurrent_requests: usize,
	certificate_store: CertificateStore,
	id_kind: IdKind,
	max_log_length: u32,
	headers: HeaderMap,
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

	/// Set max concurrent requests.
	pub fn max_concurrent_requests(mut self, max: usize) -> Self {
		self.max_concurrent_requests = max;
		self
	}

	/// Set which certificate store to use.
	pub fn certificate_store(mut self, certificate_store: CertificateStore) -> Self {
		self.certificate_store = certificate_store;
		self
	}

	/// Configure the data type of the request object ID (default is number).
	pub fn id_format(mut self, id_kind: IdKind) -> Self {
		self.id_kind = id_kind;
		self
	}

	/// Max length for logging for requests and responses in number characters.
	///
	/// Logs bigger than this limit will be truncated.
	pub fn set_max_logging_length(mut self, max: u32) -> Self {
		self.max_log_length = max;
		self
	}

	/// Set a custom header passed to the server with every request (default is none).
	///
	/// The caller is responsible for checking that the headers do not conflict or are duplicated.
	pub fn set_headers(mut self, headers: HeaderMap) -> Self {
		self.headers = headers;
		self
	}

	/// Build the HTTP client with target to connect to.
	pub fn build(self, target: impl AsRef<str>) -> Result<HttpClient, Error> {
		let transport = HttpTransportClient::new(
			target,
			self.max_request_body_size,
			self.certificate_store,
			self.max_log_length,
			self.headers,
		)
		.map_err(|e| Error::Transport(e.into()))?;
		Ok(HttpClient {
			transport,
			id_manager: Arc::new(RequestIdManager::new(self.max_concurrent_requests, self.id_kind)),
			request_timeout: self.request_timeout,
		})
	}
}

impl Default for HttpClientBuilder {
	fn default() -> Self {
		Self {
			max_request_body_size: TEN_MB_SIZE_BYTES,
			request_timeout: Duration::from_secs(60),
			max_concurrent_requests: 256,
			certificate_store: CertificateStore::Native,
			id_kind: IdKind::Number,
			max_log_length: 4096,
			headers: HeaderMap::new(),
		}
	}
}

/// JSON-RPC HTTP Client that provides functionality to perform method calls and notifications.
#[derive(Debug, Clone)]
pub struct HttpClient {
	/// HTTP transport client.
	transport: HttpTransportClient,
	/// Request timeout. Defaults to 60sec.
	request_timeout: Duration,
	/// Request ID manager.
	id_manager: Arc<RequestIdManager>,
}

#[async_trait]
impl ClientT for HttpClient {
	async fn notification<'a>(&self, method: &'a str, params: Option<ParamsSer<'a>>) -> Result<(), Error> {
		let trace = RpcTracing::notification(method);
		async {
			let notif = serde_json::to_string(&NotificationSer::new(method, params)).map_err(Error::ParseError)?;

			let fut = self.transport.send(notif);

			match tokio::time::timeout(self.request_timeout, fut).await {
				Ok(Ok(ok)) => Ok(ok),
				Err(_) => Err(Error::RequestTimeout),
				Ok(Err(e)) => Err(Error::Transport(e.into())),
			}
		}
		.instrument(trace.into_span())
		.await
	}

	/// Perform a request towards the server.
	async fn request<'a, R>(&self, method: &'a str, params: Option<ParamsSer<'a>>) -> Result<R, Error>
	where
		R: DeserializeOwned,
	{
		let guard = self.id_manager.next_request_id()?;
		let id = guard.inner();
		let request = RequestSer::new(&id, method, params);
		let trace = RpcTracing::method_call(method);

		async {
			let raw = serde_json::to_string(&request).map_err(Error::ParseError)?;

			let fut = self.transport.send_and_read_body(raw);
			let body = match tokio::time::timeout(self.request_timeout, fut).await {
				Ok(Ok(body)) => body,
				Err(_e) => {
					return Err(Error::RequestTimeout);
				}
				Ok(Err(e)) => {
					return Err(Error::Transport(e.into()));
				}
			};

			// NOTE: it's decoded first `JsonRawValue` and then to `R` below to get
			// a better error message if `R` couldn't be decoded.
			let response: Response<&JsonRawValue> = match serde_json::from_slice(&body) {
				Ok(response) => response,
				Err(_) => {
					let err: ErrorResponse = serde_json::from_slice(&body).map_err(Error::ParseError)?;
					return Err(Error::Call(CallError::Custom(err.error_object().clone().into_owned())));
				}
			};

			let result = serde_json::from_str(response.result.get()).map_err(Error::ParseError)?;

			if response.id == id {
				Ok(result)
			} else {
				Err(Error::InvalidRequestId)
			}
		}
		.instrument(trace.into_span())
		.await
	}

	async fn batch_request<'a, R>(&self, batch: Vec<(&'a str, Option<ParamsSer<'a>>)>) -> Result<Vec<R>, Error>
	where
		R: DeserializeOwned + Default + Clone,
	{
		let guard = self.id_manager.next_request_ids(batch.len())?;
		let ids: Vec<Id> = guard.inner();
		let trace = RpcTracing::batch();

		async {
			let mut batch_request = Vec::with_capacity(batch.len());
			// NOTE(niklasad1): `ID` is not necessarily monotonically increasing.
			let mut ordered_requests = Vec::with_capacity(batch.len());
			let mut request_set = FxHashMap::with_capacity_and_hasher(batch.len(), Default::default());

			for (pos, (method, params)) in batch.into_iter().enumerate() {
				batch_request.push(RequestSer::new(&ids[pos], method, params));
				ordered_requests.push(&ids[pos]);
				request_set.insert(&ids[pos], pos);
			}

			let fut =
				self.transport.send_and_read_body(serde_json::to_string(&batch_request).map_err(Error::ParseError)?);

			let body = match tokio::time::timeout(self.request_timeout, fut).await {
				Ok(Ok(body)) => body,
				Err(_e) => return Err(Error::RequestTimeout),
				Ok(Err(e)) => return Err(Error::Transport(e.into())),
			};

			// NOTE: it's decoded first `JsonRawValue` and then to `R` below to get
			// a better error message if `R` couldn't be decoded.
			let rps: Vec<Response<&JsonRawValue>> =
				serde_json::from_slice(&body).map_err(|_| match serde_json::from_slice::<ErrorResponse>(&body) {
					Ok(e) => Error::Call(CallError::Custom(e.error_object().clone().into_owned())),
					Err(e) => Error::ParseError(e),
				})?;

			// NOTE: `R::default` is placeholder and will be replaced in loop below.
			let mut responses = vec![R::default(); ordered_requests.len()];
			for rp in rps {
				let pos = match request_set.get(&rp.id) {
					Some(pos) => *pos,
					None => return Err(Error::InvalidRequestId),
				};
				let result = serde_json::from_str(rp.result.get()).map_err(Error::ParseError)?;
				responses[pos] = result;
			}
			Ok(responses)
		}
		.instrument(trace.into_span())
		.await
	}
}

#[async_trait]
impl SubscriptionClientT for HttpClient {
	/// Send a subscription request to the server. Not implemented for HTTP; will always return [`Error::HttpNotImplemented`].
	async fn subscribe<'a, N>(
		&self,
		_subscribe_method: &'a str,
		_params: Option<ParamsSer<'a>>,
		_unsubscribe_method: &'a str,
	) -> Result<Subscription<N>, Error>
	where
		N: DeserializeOwned,
	{
		Err(Error::HttpNotImplemented)
	}

	/// Subscribe to a specific method. Not implemented for HTTP; will always return [`Error::HttpNotImplemented`].
	async fn subscribe_to_method<'a, N>(&self, _method: &'a str) -> Result<Subscription<N>, Error>
	where
		N: DeserializeOwned,
	{
		Err(Error::HttpNotImplemented)
	}
}
