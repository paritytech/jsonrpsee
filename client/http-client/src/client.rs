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
use std::fmt;
use std::sync::Arc;
use std::time::Duration;

use crate::rpc_service::RpcService;
use crate::transport::{self, Error as TransportError, HttpBackend, HttpTransportClientBuilder};
use crate::{HttpRequest, HttpResponse};
use async_trait::async_trait;
use hyper::body::Bytes;
use hyper::http::{Extensions, HeaderMap};
use jsonrpsee_core::client::{
	BatchResponse, ClientT, Error, IdKind, MethodResponse, RequestIdManager, Subscription, SubscriptionClientT,
	generate_batch_id_range,
};
use jsonrpsee_core::middleware::{Batch, RpcServiceBuilder, RpcServiceT};
use jsonrpsee_core::params::BatchRequestBuilder;
use jsonrpsee_core::traits::ToRpcParams;
use jsonrpsee_core::{BoxError, TEN_MB_SIZE_BYTES};
use jsonrpsee_types::{ErrorObject, InvalidRequestId, Notification, Request, ResponseSuccess, TwoPointZero};
use serde::de::DeserializeOwned;
use tokio::sync::Semaphore;
use tower::layer::util::Identity;
use tower::{Layer, Service};

#[cfg(feature = "tls")]
use crate::{CertificateStore, CustomCertStore};

/// HTTP client builder.
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
/// ```
#[derive(Clone, Debug)]
pub struct HttpClientBuilder<HttpMiddleware = Identity, RpcMiddleware = Identity> {
	max_request_size: u32,
	max_response_size: u32,
	request_timeout: Duration,
	#[cfg(feature = "tls")]
	certificate_store: CertificateStore,
	id_kind: IdKind,
	headers: HeaderMap,
	service_builder: tower::ServiceBuilder<HttpMiddleware>,
	rpc_middleware: RpcServiceBuilder<RpcMiddleware>,
	tcp_no_delay: bool,
	max_concurrent_requests: Option<usize>,
}

impl<HttpMiddleware, RpcMiddleware> HttpClientBuilder<HttpMiddleware, RpcMiddleware> {
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

	/// Set the maximum number of concurrent requests. Default disabled.
	pub fn max_concurrent_requests(mut self, max_concurrent_requests: usize) -> Self {
		self.max_concurrent_requests = Some(max_concurrent_requests);
		self
	}

	/// Force to use the rustls native certificate store.
	///
	/// Since multiple certificate stores can be optionally enabled, this option will
	/// force the `native certificate store` to be used.
	///
	/// # Optional
	///
	/// This requires the optional `tls` feature.
	///
	/// # Example
	///
	/// ```no_run
	/// use jsonrpsee_http_client::{HttpClientBuilder, CustomCertStore};
	/// use rustls::{
	///     client::danger::{self, HandshakeSignatureValid, ServerCertVerified},
	///     pki_types::{CertificateDer, ServerName, UnixTime},
	///     Error,
	/// };
	///
	/// #[derive(Debug)]
	/// struct NoCertificateVerification;
	///
	/// impl rustls::client::danger::ServerCertVerifier for NoCertificateVerification {
	///     fn verify_server_cert(
	///         &self,
	///         _: &CertificateDer<'_>,
	///         _: &[CertificateDer<'_>],
	///         _: &ServerName<'_>,
	///         _: &[u8],
	///         _: UnixTime,
	///     ) -> Result<ServerCertVerified, Error> {
	///         Ok(ServerCertVerified::assertion())
	///     }
	///
	///     fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
	///         vec![rustls::SignatureScheme::ECDSA_NISTP256_SHA256]
	///     }
	///
	///     fn verify_tls12_signature(
	///         &self,
	///         _: &[u8],
	///         _: &CertificateDer<'_>,
	///         _: &rustls::DigitallySignedStruct,
	///     ) -> Result<rustls::client::danger::HandshakeSignatureValid, Error> {
	///         Ok(HandshakeSignatureValid::assertion())
	///     }
	///
	///     fn verify_tls13_signature(
	///         &self,
	///         _: &[u8],
	///         _: &CertificateDer<'_>,
	///         _: &rustls::DigitallySignedStruct,
	///     ) -> Result<HandshakeSignatureValid, Error> {
	///         Ok(HandshakeSignatureValid::assertion())
	///     }
	/// }
	///
	/// let tls_cfg = CustomCertStore::builder()
	///    .dangerous()
	///    .with_custom_certificate_verifier(std::sync::Arc::new(NoCertificateVerification))
	///    .with_no_client_auth();
	///
	/// // client builder with disabled certificate verification.
	/// let client_builder = HttpClientBuilder::new().with_custom_cert_store(tls_cfg);
	/// ```
	#[cfg(feature = "tls")]
	pub fn with_custom_cert_store(mut self, cfg: CustomCertStore) -> Self {
		self.certificate_store = CertificateStore::Custom(cfg);
		self
	}

	/// Configure the data type of the request object ID (default is number).
	pub fn id_format(mut self, id_kind: IdKind) -> Self {
		self.id_kind = id_kind;
		self
	}

	/// Set a custom header passed to the server with every request (default is none).
	///
	/// The caller is responsible for checking that the headers do not conflict or are duplicated.
	pub fn set_headers(mut self, headers: HeaderMap) -> Self {
		self.headers = headers;
		self
	}

	/// Configure `TCP_NODELAY` on the socket to the supplied value `nodelay`.
	///
	/// Default is `true`.
	pub fn set_tcp_no_delay(mut self, no_delay: bool) -> Self {
		self.tcp_no_delay = no_delay;
		self
	}

	/// Set the RPC middleware.
	pub fn set_rpc_middleware<T>(self, rpc_builder: RpcServiceBuilder<T>) -> HttpClientBuilder<HttpMiddleware, T> {
		HttpClientBuilder {
			#[cfg(feature = "tls")]
			certificate_store: self.certificate_store,
			id_kind: self.id_kind,
			headers: self.headers,
			max_request_size: self.max_request_size,
			max_response_size: self.max_response_size,
			service_builder: self.service_builder,
			rpc_middleware: rpc_builder,
			request_timeout: self.request_timeout,
			tcp_no_delay: self.tcp_no_delay,
			max_concurrent_requests: self.max_concurrent_requests,
		}
	}

	/// Set custom tower middleware.
	pub fn set_http_middleware<T>(
		self,
		service_builder: tower::ServiceBuilder<T>,
	) -> HttpClientBuilder<T, RpcMiddleware> {
		HttpClientBuilder {
			#[cfg(feature = "tls")]
			certificate_store: self.certificate_store,
			id_kind: self.id_kind,
			headers: self.headers,
			max_request_size: self.max_request_size,
			max_response_size: self.max_response_size,
			service_builder,
			rpc_middleware: self.rpc_middleware,
			request_timeout: self.request_timeout,
			tcp_no_delay: self.tcp_no_delay,
			max_concurrent_requests: self.max_concurrent_requests,
		}
	}
}

impl<B, S, S2, HttpMiddleware, RpcMiddleware> HttpClientBuilder<HttpMiddleware, RpcMiddleware>
where
	RpcMiddleware: Layer<RpcService<S>, Service = S2>,
	for<'a> <RpcMiddleware as Layer<RpcService<S>>>::Service: RpcServiceT<'a>,
	HttpMiddleware: Layer<transport::HttpBackend, Service = S>,
	S: Service<HttpRequest, Response = HttpResponse<B>, Error = TransportError> + Clone,
	B: http_body::Body<Data = Bytes> + Send + Unpin + 'static,
	B::Data: Send,
	B::Error: Into<BoxError>,
{
	/// Build the HTTP client with target to connect to.
	pub fn build(self, target: impl AsRef<str>) -> Result<HttpClient<S2>, Error> {
		let Self {
			max_request_size,
			max_response_size,
			request_timeout,
			#[cfg(feature = "tls")]
			certificate_store,
			id_kind,
			headers,
			service_builder,
			tcp_no_delay,
			rpc_middleware,
			..
		} = self;

		let http = HttpTransportClientBuilder {
			max_request_size,
			max_response_size,
			headers,
			tcp_no_delay,
			service_builder,
			#[cfg(feature = "tls")]
			certificate_store,
			request_timeout,
		}
		.build(target)
		.map_err(|e| Error::Transport(e.into()))?;

		let request_guard = self
			.max_concurrent_requests
			.map(|max_concurrent_requests| Arc::new(Semaphore::new(max_concurrent_requests)));

		Ok(HttpClient {
			transport: rpc_middleware.service(RpcService::new(http)),
			id_manager: Arc::new(RequestIdManager::new(id_kind)),
			request_guard,
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
			#[cfg(feature = "tls")]
			certificate_store: CertificateStore::Native,
			id_kind: IdKind::Number,
			headers: HeaderMap::new(),
			service_builder: tower::ServiceBuilder::new(),
			rpc_middleware: RpcServiceBuilder::default(),
			tcp_no_delay: true,
			max_concurrent_requests: None,
		}
	}
}

impl HttpClientBuilder<Identity> {
	/// Create a new builder.
	pub fn new() -> HttpClientBuilder<Identity> {
		HttpClientBuilder::default()
	}
}

/// JSON-RPC HTTP Client that provides functionality to perform method calls and notifications.
#[derive(Debug, Clone)]
pub struct HttpClient<S> {
	/// HTTP transport client.
	transport: S,
	/// Request ID manager.
	id_manager: Arc<RequestIdManager>,
	/// Concurrent requests limit guard.
	request_guard: Option<Arc<Semaphore>>,
	/// Request timeout.
	request_timeout: Duration,
}

impl HttpClient<HttpBackend> {
	/// Create a builder for the HttpClient.
	pub fn builder() -> HttpClientBuilder<Identity> {
		HttpClientBuilder::new()
	}

	/// Returns configured request timeout.
	pub fn request_timeout(&self) -> Duration {
		self.request_timeout
	}
}

#[async_trait]
impl<S> ClientT for HttpClient<S>
where
	for<'a> S: RpcServiceT<'a, Error = Error, Response = MethodResponse> + Send + Sync,
{
	async fn notification<Params>(&self, method: &str, params: Params) -> Result<(), Error>
	where
		Params: ToRpcParams + Send,
	{
		let _permit = match self.request_guard.as_ref() {
			Some(permit) => permit.acquire().await.ok(),
			None => None,
		};
		let params = params.to_rpc_params()?.map(StdCow::Owned);
		self.transport.notification(Notification::new(method.into(), params)).await?;
		Ok(())
	}

	async fn request<R, Params>(&self, method: &str, params: Params) -> Result<R, Error>
	where
		R: DeserializeOwned,
		Params: ToRpcParams + Send,
	{
		let _permit = match self.request_guard.as_ref() {
			Some(permit) => permit.acquire().await.ok(),
			None => None,
		};
		let id = self.id_manager.next_request_id();
		let params = params.to_rpc_params()?;

		let request = Request::borrowed(method, params.as_deref(), id.clone());
		let rp =
			self.transport.call(request).await?.into_method_call().expect("Transport::call must return a method call");
		let rp = ResponseSuccess::try_from(rp)?;

		let result = serde_json::from_str(rp.result.get()).map_err(Error::ParseError)?;
		if rp.id == id { Ok(result) } else { Err(InvalidRequestId::NotPendingRequest(rp.id.to_string()).into()) }
	}

	async fn batch_request<'a, R>(&self, batch: BatchRequestBuilder<'a>) -> Result<BatchResponse<'a, R>, Error>
	where
		R: DeserializeOwned + fmt::Debug + 'a,
	{
		let _permit = match self.request_guard.as_ref() {
			Some(permit) => permit.acquire().await.ok(),
			None => None,
		};
		let batch = batch.build()?;
		let id = self.id_manager.next_request_id();
		let id_range = generate_batch_id_range(id, batch.len() as u64)?;

		let mut batch_request = Batch::new();
		for ((method, params), id) in batch.into_iter().zip(id_range.clone()) {
			let id = self.id_manager.as_id_kind().into_id(id);
			let req = Request {
				jsonrpc: TwoPointZero,
				method: method.into(),
				params: params.map(StdCow::Owned),
				id,
				extensions: Extensions::new(),
			};
			batch_request.push(req)?;
		}

		let json_rps =
			self.transport.batch(batch_request).await?.into_batch().expect("Transport::batch must return a batch");

		let mut batch_response = Vec::new();
		let mut success = 0;
		let mut failed = 0;

		// Fill the batch response with placeholder values.
		for _ in 0..json_rps.len() {
			batch_response.push(Err(ErrorObject::borrowed(0, "", None)));
		}

		for rp in json_rps.into_iter() {
			let id = rp.id.try_parse_inner_as_number()?;

			let res = match ResponseSuccess::try_from(rp) {
				Ok(r) => {
					let v = serde_json::from_str(r.result.get()).map_err(Error::ParseError)?;
					success += 1;
					Ok(v)
				}
				Err(err) => {
					failed += 1;
					Err(err)
				}
			};

			let maybe_elem = id
				.checked_sub(id_range.start)
				.and_then(|p| p.try_into().ok())
				.and_then(|p: usize| batch_response.get_mut(p));

			if let Some(elem) = maybe_elem {
				*elem = res;
			} else {
				return Err(InvalidRequestId::NotPendingRequest(id.to_string()).into());
			}
		}

		Ok(BatchResponse::new(success, batch_response, failed))
	}
}

#[async_trait]
impl<S> SubscriptionClientT for HttpClient<S>
where
	for<'a> S: RpcServiceT<'a, Error = Error, Response = MethodResponse> + Send + Sync,
{
	/// Send a subscription request to the server. Not implemented for HTTP; will always return
	/// [`Error::HttpNotImplemented`].
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
	async fn subscribe_to_method<'a, N>(&self, _method: &'a str) -> Result<Subscription<N>, Error>
	where
		N: DeserializeOwned,
	{
		Err(Error::HttpNotImplemented)
	}
}
