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

//! [`tower::util::Either`] but
//! adjusted to satisfy the trait bound [`RpcServiceT].
//!
//! NOTE: This is introduced because it doesn't
//! work to implement tower::Layer for
//! external types such as future::Either.

use std::{
	pin::Pin,
	task::{Context, Poll},
};

use crate::middleware::rpc::RpcServiceT;
use futures_util::Future;
use jsonrpsee_core::server::MethodResponse;
use jsonrpsee_types::Request;
use pin_project::pin_project;

/// [`tower::util::Either`] but
/// adjusted to satisfy the trait bound [`RpcServiceT].
#[derive(Clone, Debug)]
pub enum Either<A, B> {
	/// One type of backing [`RpcServiceT`].
	Left(A),
	/// The other type of backing [`RpcServiceT`].
	Right(B),
}

impl<S, A, B> tower::Layer<S> for Either<A, B>
where
	A: tower::Layer<S>,
	B: tower::Layer<S>,
{
	type Service = Either<A::Service, B::Service>;

	fn layer(&self, inner: S) -> Self::Service {
		match self {
			Either::Left(layer) => Either::Left(layer.layer(inner)),
			Either::Right(layer) => Either::Right(layer.layer(inner)),
		}
	}
}

impl<'a, A, B> RpcServiceT<'a> for Either<A, B>
where
	A: RpcServiceT<'a> + Send + 'a,
	B: RpcServiceT<'a> + Send + 'a,
{
	fn call(&self, request: Request<'a>) -> impl Future<Output = MethodResponse> {
		match self {
			Either::Left(service) => ResponseFuture::Left(service.call(request)),
			Either::Right(service) => ResponseFuture::Right(service.call(request)),
		}
	}
}

/// Response future for the either layer.
#[pin_project(project = Fut)]
pub enum ResponseFuture<A, B> {
	/// Left future
	Left(#[pin] A),
	/// Right future.
	Right(#[pin] B),
}

impl<A, B> std::fmt::Debug for ResponseFuture<A, B> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::Left { .. } => "ResponseFuture::left",
			Self::Right { .. } => "ResponseFuture::right",
		};
		f.write_str(s)
	}
}

impl<A, B> Future for ResponseFuture<A, B>
where
	A: Future,
	B: Future<Output = A::Output>,
{
	type Output = A::Output;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		match self.project() {
			Fut::Left(inner) => inner.poll(cx),
			Fut::Right(inner) => inner.poll(cx),
		}
	}
}
