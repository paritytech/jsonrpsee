//! Middleware that proxies requests at a specified URI to internal
//! RPC method calls.

use crate::transport::http;
use hyper::header::{ACCEPT, CONTENT_TYPE};
use hyper::http::HeaderValue;
use hyper::{Body, Method, Request, Response, Uri};
use jsonrpsee_core::error::Error as RpcError;
use jsonrpsee_types::{Id, RequestSer};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// Layer that applies [`ProxyGetRequest`] which proxies the `GET /path` requests to
/// specific RPC method calls and that strips the response.
///
/// See [`ProxyGetRequest`] for more details.
#[derive(Debug, Clone)]
pub struct ProxyGetRequestLayer {
	path: String,
	method: String,
}

impl ProxyGetRequestLayer {
	/// Creates a new [`ProxyGetRequestLayer`].
	///
	/// See [`ProxyGetRequest`] for more details.
	pub fn new(path: impl Into<String>, method: impl Into<String>) -> Result<Self, RpcError> {
		let path = path.into();
		if !path.starts_with('/') {
			return Err(RpcError::Custom("ProxyGetRequestLayer path must start with `/`".to_string()));
		}

		Ok(Self { path, method: method.into() })
	}
}
impl<S> Layer<S> for ProxyGetRequestLayer {
	type Service = ProxyGetRequest<S>;

	fn layer(&self, inner: S) -> Self::Service {
		ProxyGetRequest::new(inner, &self.path, &self.method)
			.expect("Path already validated in ProxyGetRequestLayer; qed")
	}
}

/// Proxy `GET /path` requests to the specified RPC method calls.
///
/// # Request
///
/// The `GET /path` requests are modified into valid `POST` requests for
/// calling the RPC method. This middleware adds appropriate headers to the
/// request, and completely modifies the request `BODY`.
///
/// # Response
///
/// The response of the RPC method is stripped down to contain only the method's
/// response, removing any RPC 2.0 spec logic regarding the response' body.
#[derive(Debug, Clone)]
pub struct ProxyGetRequest<S> {
	inner: S,
	path: Arc<str>,
	method: Arc<str>,
}

impl<S> ProxyGetRequest<S> {
	/// Creates a new [`ProxyGetRequest`].
	///
	/// The request `GET /path` is redirected to the provided method.
	/// Fails if the path does not start with `/`.
	pub fn new(inner: S, path: &str, method: &str) -> Result<Self, RpcError> {
		if !path.starts_with('/') {
			return Err(RpcError::Custom(format!("ProxyGetRequest path must start with `/`, got: {}", path)));
		}

		Ok(Self { inner, path: Arc::from(path), method: Arc::from(method) })
	}
}

impl<S> Service<Request<Body>> for ProxyGetRequest<S>
where
	S: Service<Request<Body>, Response = Response<Body>>,
	S::Response: 'static,
	S::Error: Into<Box<dyn Error + Send + Sync>> + 'static,
	S::Future: Send + 'static,
{
	type Response = S::Response;
	type Error = Box<dyn Error + Send + Sync + 'static>;
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

	#[inline]
	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.inner.poll_ready(cx).map_err(Into::into)
	}

	fn call(&mut self, mut req: Request<Body>) -> Self::Future {
		let modify = self.path.as_ref() == req.uri() && req.method() == Method::GET;

		// Proxy the request to the appropriate method call.
		if modify {
			// RPC methods are accessed with `POST`.
			*req.method_mut() = Method::POST;
			// Precautionary remove the URI.
			*req.uri_mut() = Uri::from_static("/");

			// Requests must have the following headers:
			req.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
			req.headers_mut().insert(ACCEPT, HeaderValue::from_static("application/json"));

			// Adjust the body to reflect the method call.
			let body = Body::from(
				serde_json::to_string(&RequestSer::borrowed(&Id::Number(0), &self.method, None))
					.expect("Valid request; qed"),
			);
			req = req.map(|_| body);
		}

		// Call the inner service and get a future that resolves to the response.
		let fut = self.inner.call(req);

		// Adjust the response if needed.
		let res_fut = async move {
			let res = fut.await.map_err(|err| err.into())?;

			// Nothing to modify: return the response as is.
			if !modify {
				return Ok(res);
			}

			let body = res.into_body();
			let bytes = hyper::body::to_bytes(body).await?;

			#[derive(serde::Deserialize, Debug)]
			struct RpcPayload<'a> {
				#[serde(borrow)]
				result: &'a serde_json::value::RawValue,
			}

			let response = if let Ok(payload) = serde_json::from_slice::<RpcPayload>(&bytes) {
				http::response::ok_response(payload.result.to_string())
			} else {
				http::response::internal_error()
			};

			Ok(response)
		};

		Box::pin(res_fut)
	}
}
