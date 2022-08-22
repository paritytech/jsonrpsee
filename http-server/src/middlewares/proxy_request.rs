//! Middleware that proxies requests at a specified URI to internal
//! RPC method calls.

use hyper::header::{ACCEPT, CONTENT_TYPE};
use hyper::http::HeaderValue;
use hyper::{Body, Method, Request, Response};
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// Layer that applies [`ProxyRequest`] which proxies the `GET /path` requests to
/// specific RPC method calls and that strips the response.
///
/// See [`ProxyRequest`] for more details.
#[derive(Debug, Clone)]
pub struct ProxyRequestLayer {
	path: String,
	method: String,
}

impl ProxyRequestLayer {
	/// Creates a new [`ProxyRequestLayer`].
	///
	/// See [`ProxyRequest`] for more details.
	pub fn new(path: impl Into<String>, method: impl Into<String>) -> Self {
		Self { path: path.into(), method: method.into() }
	}
}
impl<S> Layer<S> for ProxyRequestLayer {
	type Service = ProxyRequest<S>;

	fn layer(&self, inner: S) -> Self::Service {
		ProxyRequest::new(inner, self.path.clone(), self.method.clone())
	}
}

/// Proxy `GET/path` requests to the specified RPC method calls.
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
pub struct ProxyRequest<S> {
	inner: S,
	path: String,
	method: String,
}

impl<S> ProxyRequest<S> {
	/// Creates a new [`ProxyRequest`].
	///
	/// The request `GET /path` is redirected to the provided method.
	pub fn new(inner: S, path: impl Into<String>, method: impl Into<String>) -> Self {
		Self { inner, path: path.into(), method: method.into() }
	}
}

impl<S> Service<Request<Body>> for ProxyRequest<S>
where
	S: Service<Request<Body>, Response = Response<Body>, Error = hyper::Error>,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = S::Future;

	#[inline]
	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.inner.poll_ready(cx)
	}

	fn call(&mut self, mut req: Request<Body>) -> Self::Future {
		let modify = self.path.as_str() == req.uri() && req.method() == Method::GET;

		// Proxy the request to the appropriate method call.
		if modify {
			// RPC methods are accessed with `POST`.
			*req.method_mut() = Method::POST;
			// Precautionary remove the URI.
			*req.uri_mut() = "/".parse().unwrap();

			// Requests must have the following headers:
			req.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
			req.headers_mut().insert(ACCEPT, HeaderValue::from_static("application/json"));

			// Adjust the body to reflect the method call.
			let body =
				Body::from(format!("{{\"jsonrpc\":\"2.0\",\"method\":\"{}\",\"params\":null,\"id\":1}}", self.method));
			req = req.map(|_| body);
		}

		self.inner.call(req)
	}
}
