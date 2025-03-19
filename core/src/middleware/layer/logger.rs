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

//! RPC Logger layer.

use std::{
	pin::Pin,
	task::{Context, Poll},
};

use crate::{
	middleware::{Notification, RpcServiceT},
	tracing::server::{rx_log_from_json, tx_log_from_str},
};

use futures_util::Future;
use jsonrpsee_types::Request;
use pin_project::pin_project;
use tracing::{Instrument, instrument::Instrumented};

/// RPC logger layer.
#[derive(Copy, Clone, Debug)]
pub struct RpcLoggerLayer(u32);

impl RpcLoggerLayer {
	/// Create a new logging layer.
	pub fn new(max: u32) -> Self {
		Self(max)
	}
}

impl<S> tower::Layer<S> for RpcLoggerLayer {
	type Service = RpcLogger<S>;

	fn layer(&self, service: S) -> Self::Service {
		RpcLogger { service, max: self.0 }
	}
}

/// A middleware that logs each RPC call and response.
#[derive(Debug)]
pub struct RpcLogger<S> {
	max: u32,
	service: S,
}

impl<'a, S> RpcServiceT<'a> for RpcLogger<S>
where
	S: RpcServiceT<'a>,
	S::Error: std::fmt::Debug + Send,
	S::Response: AsRef<str>,
{
	type Future = Instrumented<ResponseFuture<S::Future>>;
	type Error = S::Error;
	type Response = S::Response;

	#[tracing::instrument(name = "method_call", skip_all, fields(method = request.method_name()), level = "trace")]
	fn call(&self, request: Request<'a>) -> Self::Future {
		rx_log_from_json(&request, self.max);

		ResponseFuture::new(self.service.call(request), self.max).in_current_span()
	}

	#[tracing::instrument(name = "batch", skip_all, fields(method = "batch"), level = "trace")]
	fn batch(&self, requests: Vec<Request<'a>>) -> Self::Future {
		rx_log_from_json(&requests, self.max);

		ResponseFuture::new(self.service.batch(requests), self.max).in_current_span()
	}

	#[tracing::instrument(name = "notification", skip_all, fields(method = &*n.method), level = "trace")]
	fn notification(&self, n: Notification<'a>) -> Self::Future {
		rx_log_from_json(&n, self.max);

		ResponseFuture::new(self.service.notification(n), self.max).in_current_span()
	}
}

/// Response future to log the response for a method call.
#[pin_project]
pub struct ResponseFuture<F> {
	#[pin]
	fut: F,
	max: u32,
}

impl<F> ResponseFuture<F> {
	/// Create a new response future.
	fn new(fut: F, max: u32) -> Self {
		Self { fut, max }
	}
}

impl<F> std::fmt::Debug for ResponseFuture<F> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str("ResponseFuture")
	}
}

impl<F, R, E> Future for ResponseFuture<F>
where
	F: Future<Output = Result<R, E>>,
	R: AsRef<str>,
	E: std::fmt::Debug,
{
	type Output = F::Output;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let max = self.max;
		let fut = self.project().fut;

		match fut.poll(cx) {
			Poll::Ready(Ok(rp)) => {
				tx_log_from_str(&rp, max);
				Poll::Ready(Ok(rp))
			}
			Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
			Poll::Pending => Poll::Pending,
		}
	}
}
