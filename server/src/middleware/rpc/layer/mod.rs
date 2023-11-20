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

//! Specific middleware layer implementation provided by jsonrpsee.

pub mod either;
pub mod logger;
pub mod rpc_service;

pub use logger::*;
pub use rpc_service::*;

use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::Future;
use jsonrpsee_core::server::MethodResponse;
use pin_project::pin_project;

/// Response which may be ready or a future that needs to be
/// polled.
#[pin_project(project = ResponseStateProj)]
pub enum ResponseFuture<F> {
	/// The response is ready.
	Ready(Option<MethodResponse>),
	/// The response has to be polled.
	Poll(#[pin] F),
}

impl<F> std::fmt::Debug for ResponseFuture<F> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::Poll { .. } => "ResponseFuture::poll",
			Self::Ready(_) => "ResponseFuture::ready",
		};
		f.write_str(s)
	}
}

impl<F> ResponseFuture<F> {
	/// The response is ready.
	pub fn ready(rp: MethodResponse) -> Self {
		Self::Ready(Some(rp))
	}

	/// The response needs to be polled.
	pub fn future(fut: F) -> Self {
		Self::Poll(fut)
	}
}

impl<F: Future<Output = MethodResponse>> Future for ResponseFuture<F> {
	type Output = MethodResponse;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let this = self.project();

		match this {
			ResponseStateProj::Poll(fut) => fut.poll(cx),
			ResponseStateProj::Ready(rp) => Poll::Ready(rp.take().expect("Future not polled after Ready; qed")),
		}
	}
}
