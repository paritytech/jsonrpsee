//! Middleware that proxies requests at a specified URI to internal
//! RPC method calls.

use crate::response;
use futures_util::ready;
use hyper::body::HttpBody;
use hyper::header::{ACCEPT, CONTENT_TYPE};
use hyper::http::HeaderValue;
use hyper::{Body, Method, Request, Response};
use pin_project_lite::pin_project;
use std::future::Future;
use std::pin::Pin;
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
	S: Service<Request<Body>, Response = Response<Body>>,
	<S as Service<Request<Body>>>::Error: From<hyper::Error>,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = ResponseFuture<S::Future>;

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

		// Depending on `modify` adjust the response.
		ResponseFuture::PollFuture { future: self.inner.call(req), modify }
	}
}

pin_project! {
	/// Response future for [`ProxyRequest`].
	#[project = ResponseFutureState]
	#[allow(missing_docs)]
	pub enum ResponseFuture<F> {
		/// Poll the response out of the future.
		PollFuture {
			#[pin]
			future: F,
			modify: bool,
		},
		/// Poll the [`hyper::Body`] response and modify it.
		PollBodyData {
			body: Body,
			body_bytes: Vec<u8>,
		},
	}
}

impl<F, E> Future for ResponseFuture<F>
where
	F: Future<Output = Result<Response<Body>, E>>,
	E: From<hyper::Error>,
{
	type Output = F::Output;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		// The purpose of this loop is to optimise the transition from
		// `PollFuture` -> `PollBodyData` state, that would otherwise
		// require a `cx.wake().wake_by_ref and return Poll::Pending`.
		loop {
			match self.as_mut().project() {
				ResponseFutureState::PollFuture { future, modify } => {
					let res: Response<Body> = ready!(future.poll(cx)?);

					// Nothing to modify: return the response as is.
					if !*modify {
						return Poll::Ready(Ok(res));
					}

					let inner = ResponseFuture::PollBodyData { body: res.into_body(), body_bytes: Vec::new() };
					self.set(inner);
				}
				ResponseFutureState::PollBodyData { body, body_bytes } => {
					while let Some(chunk) = ready!(Pin::new(&mut *body).poll_data(cx)?) {
						body_bytes.extend_from_slice(chunk.as_ref());
					}

					#[derive(serde::Deserialize, Debug)]
					struct RpcPayload<'a> {
						#[serde(borrow)]
						result: &'a serde_json::value::RawValue,
					}

					let response = if let Ok(payload) = serde_json::from_slice::<RpcPayload>(body_bytes) {
						response::ok_response(payload.result.to_string())
					} else {
						response::internal_error()
					};

					return Poll::Ready(Ok(response));
				}
			}
		}
	}
}
