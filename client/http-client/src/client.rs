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

use std::borrow::Cow as StdCow;
use std::error::Error as StdError;
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use crate::transport::{self, HttpTransportClient};
use crate::types::{ErrorResponse, NotificationSer, RequestSer, Response};
use async_trait::async_trait;
use hyper::body::HttpBody;
use hyper::http::HeaderMap;
use hyper::Body;
use jsonrpsee_core::client::{
	generate_batch_id_range, BatchResponse, CertificateStore, ClientT, IdKind, RequestIdManager, Subscription,
	SubscriptionClientT,
};
use jsonrpsee_core::params::BatchRequestBuilder;
use jsonrpsee_core::traits::ToRpcParams;
use jsonrpsee_core::{Error, JsonRawValue, TEN_MB_SIZE_BYTES};
use jsonrpsee_types::error::CallError;
use jsonrpsee_types::{ErrorObject, TwoPointZero};
use serde::de::DeserializeOwned;
use tower::layer::util::Identity;
use tower::{Layer, Service};
use tracing::instrument;

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
///          .build("http://localhost")
///          .unwrap();
///
///     // use client....
/// }
///
/// ```
#[derive(Debug)]
pub struct HttpClientBuilder<L = Identity> {
	max_request_size: u32,
	max_response_size: u32,
	request_timeout: Duration,
	max_concurrent_requests: usize,
	certificate_store: CertificateStore,
	id_kind: IdKind,
	max_log_length: u32,
	headers: HeaderMap,
	service_builder: tower::ServiceBuilder<L>,
}

impl<L> HttpClientBuilder<L> {
	/// Set the maximum size of a request body in bytes. Default is 10 MiB.
	pub fn max_request_size(mut self, size: u32) -> Self {
		self.max_request_size = size;
		self
	}

	/// Set the maximum size of a response in bytes. Default is 10 MiB.
	pub fn max_response_size(mut self, size: u32) -> Self {
		self.max_response_size = size;
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

	/// Set custom tower middleware.
	pub fn set_middleware<T>(self, service_builder: tower::ServiceBuilder<T>) -> HttpClientBuilder<T> {
		HttpClientBuilder {
			certificate_store: self.certificate_store,
			id_kind: self.id_kind,
			headers: self.headers,
			max_log_length: self.max_log_length,
			max_concurrent_requests: self.max_concurrent_requests,
			max_request_size: self.max_request_size,
			max_response_size: self.max_response_size,
			service_builder,
			request_timeout: self.request_timeout,
		}
	}
}

#[cfg(feature = "tls")]
impl<B, S, L> HttpClientBuilder<L>
where
	L: Layer<transport::TlsService, Service = S>,
	S: Service<hyper::Request<Body>, Response = hyper::Response<B>, Error = hyper::Error> + Clone,
	B: HttpBody + Send + 'static,
	B::Data: Send,
	B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
	/// Build the HTTP client with target to connect to.
	pub fn build(self, target: impl AsRef<str>) -> Result<HttpClient<S>, Error> {
		let Self {
			max_request_size,
			max_response_size,
			max_concurrent_requests,
			request_timeout,
			certificate_store,
			id_kind,
			headers,
			max_log_length,
			service_builder,
			..
		} = self;

		let transport = HttpTransportClient::new(
			max_request_size,
			target,
			max_response_size,
			certificate_store,
			max_log_length,
			headers,
			service_builder,
		)
		.map_err(|e| Error::Transport(e.into()))?;
		Ok(HttpClient {
			transport,
			id_manager: Arc::new(RequestIdManager::new(max_concurrent_requests, id_kind)),
			request_timeout,
		})
	}
}

#[cfg(not(feature = "tls"))]
impl<B, S, L> HttpClientBuilder<L>
where
	L: Layer<transport::PlainService, Service = S>,
	S: Service<hyper::Request<Body>, Response = hyper::Response<B>, Error = hyper::Error> + Clone,
	B: HttpBody + Send + 'static,
	B::Data: Send,
	B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
	/// Build the HTTP client with target to connect to.
	pub fn build(self, target: impl AsRef<str>) -> Result<HttpClient<S>, Error> {
		let Self {
			max_request_size,
			max_response_size,
			max_concurrent_requests,
			request_timeout,
			certificate_store,
			id_kind,
			headers,
			max_log_length,
			service_builder,
			..
		} = self;

		let transport = HttpTransportClient::new(
			max_request_size,
			target,
			max_response_size,
			certificate_store,
			max_log_length,
			headers,
			service_builder,
		)
		.map_err(|e| Error::Transport(e.into()))?;
		Ok(HttpClient {
			transport,
			id_manager: Arc::new(RequestIdManager::new(max_concurrent_requests, id_kind)),
			request_timeout,
		})
	}
}

impl Default for HttpClientBuilder<Identity> {
	fn default() -> Self {
		Self {
			max_request_size: TEN_MB_SIZE_BYTES,
			max_response_size: TEN_MB_SIZE_BYTES,
			request_timeout: Duration::from_secs(60),
			max_concurrent_requests: 256,
			certificate_store: CertificateStore::Native,
			id_kind: IdKind::Number,
			max_log_length: 4096,
			headers: HeaderMap::new(),
			service_builder: tower::ServiceBuilder::new(),
		}
	}
}

/// JSON-RPC HTTP Client that provides functionality to perform method calls and notifications.
#[derive(Debug, Clone)]
pub struct HttpClient<S> {
	/// HTTP transport client.
	transport: HttpTransportClient<S>,
	/// Request timeout. Defaults to 60sec.
	request_timeout: Duration,
	/// Request ID manager.
	id_manager: Arc<RequestIdManager>,
}

#[async_trait]
impl<B, S> ClientT for HttpClient<S>
where
	S: Service<hyper::Request<Body>, Response = hyper::Response<B>, Error = hyper::Error> + Send + Sync + Clone,
	<S as Service<hyper::Request<Body>>>::Future: Send,
	B: HttpBody + Send + 'static,
	B::Data: Send,
	B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
	#[instrument(name = "notification", skip(self, params), level = "trace")]
	async fn notification<Params>(&self, method: &str, params: Params) -> Result<(), Error>
	where
		Params: ToRpcParams + Send,
	{
		let params = params.to_rpc_params()?;
		let notif =
			serde_json::to_string(&NotificationSer::borrowed(&method, params.as_deref())).map_err(Error::ParseError)?;

		let fut = self.transport.send(notif);

		match tokio::time::timeout(self.request_timeout, fut).await {
			Ok(Ok(ok)) => Ok(ok),
			Err(_) => Err(Error::RequestTimeout),
			Ok(Err(e)) => Err(Error::Transport(e.into())),
		}
	}

	/// Perform a request towards the server.

	#[instrument(name = "method_call", skip(self, params), level = "trace")]
	async fn request<R, Params>(&self, method: &str, params: Params) -> Result<R, Error>
	where
		R: DeserializeOwned,
		Params: ToRpcParams + Send,
	{
		let guard = self.id_manager.next_request_id()?;
		let id = guard.inner();
		let params = params.to_rpc_params()?;

		let request = RequestSer::borrowed(&id, &method, params.as_deref());
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

		// NOTE: it's decoded first to `JsonRawValue` and then to `R` below to get
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

	#[instrument(name = "batch", skip(self, batch), level = "trace")]
	async fn batch_request<'a, R>(&self, batch: BatchRequestBuilder<'a>) -> Result<BatchResponse<'a, R>, Error>
	where
		R: DeserializeOwned + fmt::Debug + 'a,
	{
		let batch = batch.build()?;
		let guard = self.id_manager.next_request_id()?;
		let id_range = generate_batch_id_range(&guard, batch.len() as u64)?;

		let mut batch_request = Vec::with_capacity(batch.len());
		for ((method, params), id) in batch.into_iter().zip(id_range.clone()) {
			let id = self.id_manager.as_id_kind().into_id(id);
			batch_request.push(RequestSer {
				jsonrpc: TwoPointZero,
				id,
				method: method.into(),
				params: params.map(StdCow::Owned),
			});
		}

		let fut = self.transport.send_and_read_body(serde_json::to_string(&batch_request).map_err(Error::ParseError)?);

		let body = match tokio::time::timeout(self.request_timeout, fut).await {
			Ok(Ok(body)) => body,
			Err(_e) => return Err(Error::RequestTimeout),
			Ok(Err(e)) => return Err(Error::Transport(e.into())),
		};

		let json_rps: Vec<&JsonRawValue> = serde_json::from_slice(&body).map_err(Error::ParseError)?;

		let mut responses = Vec::with_capacity(json_rps.len());
		let mut successful_calls = 0;
		let mut failed_calls = 0;

		for _ in 0..json_rps.len() {
			responses.push(Err(ErrorObject::borrowed(0, &"", None)));
		}

		for rp in json_rps {
			let (id, res) = match serde_json::from_str::<Response<R>>(rp.get()).map_err(Error::ParseError) {
				Ok(r) => {
					let id = r.id.try_parse_inner_as_number().ok_or(Error::InvalidRequestId)?;
					successful_calls += 1;
					(id, Ok(r.result))
				}
				Err(err) => match serde_json::from_str::<ErrorResponse>(rp.get()).map_err(Error::ParseError) {
					Ok(err) => {
						let id = err.id().try_parse_inner_as_number().ok_or(Error::InvalidRequestId)?;
						failed_calls += 1;
						(id, Err(err.error_object().clone().into_owned()))
					}
					Err(_) => {
						return Err(err);
					}
				},
			};

			let maybe_elem = id
				.checked_sub(id_range.start)
				.and_then(|p| p.try_into().ok())
				.and_then(|p: usize| responses.get_mut(p));

			if let Some(elem) = maybe_elem {
				*elem = res;
			} else {
				return Err(Error::InvalidRequestId);
			}
		}

		Ok(BatchResponse::new(successful_calls, responses, failed_calls))
	}
}

#[async_trait]
impl<B, S> SubscriptionClientT for HttpClient<S>
where
	S: Service<hyper::Request<Body>, Response = hyper::Response<B>, Error = hyper::Error> + Send + Sync + Clone,
	<S as Service<hyper::Request<Body>>>::Future: Send,
	B: HttpBody + Send + 'static,
	B::Data: Send,
	B::Error: Into<Box<dyn StdError + Send + Sync>>,
{
	/// Send a subscription request to the server. Not implemented for HTTP; will always return [`Error::HttpNotImplemented`].
	#[instrument(name = "subscription", fields(method = _subscribe_method), skip(self, _params, _subscribe_method, _unsubscribe_method), level = "trace")]
	async fn subscribe<'a, N, Params>(
		&self,
		_subscribe_method: &'a str,
		_params: Params,
		_unsubscribe_method: &'a str,
	) -> Result<Subscription<N>, Error>
	where
		Params: ToRpcParams + Send,
		N: DeserializeOwned,
	{
		Err(Error::HttpNotImplemented)
	}

	/// Subscribe to a specific method. Not implemented for HTTP; will always return [`Error::HttpNotImplemented`].
	#[instrument(name = "subscribe_method", fields(method = _method), skip(self, _method), level = "trace")]
	async fn subscribe_to_method<'a, N>(&self, _method: &'a str) -> Result<Subscription<N>, Error>
	where
		N: DeserializeOwned,
	{
		Err(Error::HttpNotImplemented)
	}
}
