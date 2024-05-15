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

use std::error::Error as StdError;
use std::pin::Pin;
use std::task::{Context, Poll};

use http_body::Frame;
use http_body_util::BodyExt;
use hyper::body::Bytes;
use pin_project::pin_project;
use tower::util::Oneshot;
use tower::ServiceExt;

/// HTTP request type.
pub type HttpRequest<T = HttpBody> = hyper::Request<T>;

/// HTTP response body.
pub type HttpResponseBody = http_body_util::Full<hyper::body::Bytes>;

/// HTTP response type.
pub type HttpResponse<T = HttpResponseBody> = hyper::Response<T>;

/// A HTTP request body.
#[derive(Debug, Default)]
pub struct HttpBody(http_body_util::combinators::BoxBody<Bytes, Box<dyn StdError + Send + Sync + 'static>>);

impl HttpBody {
	/// Create an empty body.
	pub fn empty() -> Self {
		Self::default()
	}

	/// Create a new body.
	pub fn new<B>(body: B) -> Self
	where
		B: http_body::Body<Data = Bytes> + Send + Sync + 'static,
		B::Data: Send + 'static,
		B::Error: Into<Box<dyn StdError + Send + Sync + 'static>>,
	{
		Self(body.map_err(|e| e.into()).boxed())
	}
}

impl http_body::Body for HttpBody {
	type Data = Bytes;
	type Error = Box<dyn StdError + Send + Sync + 'static>;

	#[inline]
	fn poll_frame(
		mut self: Pin<&mut Self>,
		cx: &mut Context<'_>,
	) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
		Pin::new(&mut self.0).poll_frame(cx)
	}

	#[inline]
	fn size_hint(&self) -> http_body::SizeHint {
		self.0.size_hint()
	}

	#[inline]
	fn is_end_stream(&self) -> bool {
		self.0.is_end_stream()
	}
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct TowerToHyperService<S> {
	service: S,
}

impl<S> TowerToHyperService<S> {
	pub(crate) fn new(service: S) -> Self {
		Self { service }
	}
}

impl<S> hyper::service::Service<hyper::Request<hyper::body::Incoming>> for TowerToHyperService<S>
where
	S: tower::Service<HttpRequest> + Clone,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = TowerToHyperServiceFuture<S, HttpRequest>;

	fn call(&self, req: hyper::Request<hyper::body::Incoming>) -> Self::Future {
		let req = req.map(HttpBody::new);
		TowerToHyperServiceFuture { future: self.service.clone().oneshot(req) }
	}
}

#[pin_project]
pub(crate) struct TowerToHyperServiceFuture<S, R>
where
	S: tower::Service<R>,
{
	#[pin]
	future: Oneshot<S, R>,
}

impl<S, R> std::future::Future for TowerToHyperServiceFuture<S, R>
where
	S: tower::Service<R>,
{
	type Output = Result<S::Response, S::Error>;

	#[inline]
	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		self.project().future.poll(cx)
	}
}
