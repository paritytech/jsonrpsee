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

use crate::middleware::rpc::RpcServiceT;
use jsonrpsee_types::Request;

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
	type Future = futures_util::future::Either<A::Future, B::Future>;

	fn call(&self, request: Request<'a>) -> Self::Future {
		match self {
			Either::Left(service) => futures_util::future::Either::Left(service.call(request)),
			Either::Right(service) => futures_util::future::Either::Right(service.call(request)),
		}
	}
}
