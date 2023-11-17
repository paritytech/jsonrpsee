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

use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use futures_util::future::BoxFuture;
use futures_util::{ready, Future, FutureExt};
use jsonrpsee_core::server::{
	BoundedSubscriptions, MethodCallback, MethodResponse, MethodSink, Methods, SubscriptionState,
};
use jsonrpsee_core::tracing::{rx_log_from_json, tx_log_from_str};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_types::error::{reject_too_many_subscriptions, ErrorCode};
use jsonrpsee_types::{ErrorObject, Request};
use pin_project::pin_project;
use tower::BoxError;

use super::RpcServiceT;

/// JSON-RPC service middleware.
#[derive(Clone, Debug)]
pub struct RpcService {
	conn_id: usize,
	methods: Methods,
	max_response_body_size: usize,
	cfg: RpcServiceCfg,
}

/// Configuration of the RpcService.
#[derive(Clone, Debug)]
pub(crate) enum RpcServiceCfg {
	/// The server supports only calls.
	OnlyCalls,
	/// The server supports both method calls and subscriptions.
	CallsAndSubscriptions {
		bounded_subscriptions: BoundedSubscriptions,
		sink: MethodSink,
		id_provider: Arc<dyn IdProvider>,
		_pending_calls: tokio::sync::mpsc::Sender<()>,
	},
}

impl RpcService {
	/// Create a new service with doesn't support subscriptions.
	pub(crate) fn new(methods: Methods, max_response_body_size: usize, conn_id: usize, cfg: RpcServiceCfg) -> Self {
		Self { methods, max_response_body_size, conn_id, cfg }
	}
}

impl<'a> RpcServiceT<'a> for RpcService {
	type Future = ResponseFuture<BoxFuture<'a, MethodResponse>>;

	fn call(&self, req: Request<'a>) -> Self::Future {
		let conn_id = self.conn_id;
		let max_response_body_size = self.max_response_body_size;

		let params = req.params();
		let name = req.method_name();
		let id = req.id().clone();

		match self.methods.method_with_name(name) {
			None => {
				let rp = MethodResponse::error(id, ErrorObject::from(ErrorCode::MethodNotFound));
				ResponseFuture::ready(rp)
			}
			Some((_name, method)) => match method {
				MethodCallback::Async(callback) => {
					let params = params.into_owned();
					let id = id.into_owned();

					let fut = (callback)(id, params, conn_id, max_response_body_size);
					ResponseFuture::future(fut)
				}
				MethodCallback::Sync(callback) => {
					let rp = (callback)(id, params, max_response_body_size);
					ResponseFuture::ready(rp)
				}
				MethodCallback::Subscription(callback) => {
					let RpcServiceCfg::CallsAndSubscriptions {
						bounded_subscriptions,
						sink,
						id_provider,
						_pending_calls,
					} = self.cfg.clone()
					else {
						tracing::warn!("Subscriptions not supported");
						let rp = MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError));
						return ResponseFuture::ready(rp);
					};

					if let Some(p) = bounded_subscriptions.acquire() {
						let conn_state =
							SubscriptionState { conn_id, id_provider: &*id_provider.clone(), subscription_permit: p };

						let fut = callback(id.clone(), params, sink, conn_state);
						ResponseFuture::future(fut)
					} else {
						let max = bounded_subscriptions.max();
						let rp = MethodResponse::error(id, reject_too_many_subscriptions(max));
						return ResponseFuture::ready(rp);
					}
				}
				MethodCallback::Unsubscription(callback) => {
					// Don't adhere to any resource or subscription limits; always let unsubscribing happen!

					let RpcServiceCfg::CallsAndSubscriptions { .. } = self.cfg else {
						tracing::warn!("Subscriptions not supported");
						let rp = MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError));
						return ResponseFuture::ready(rp);
					};

					let rp = callback(id, params, conn_id, max_response_body_size);
					ResponseFuture::ready(rp)
				}
			},
		}
	}
}

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
	S: RpcServiceT<'a> + Send + Sync + Clone + 'static,
{
	type Future = BoxFuture<'a, MethodResponse>;

	//#[tracing::instrument(name = "method_call", skip(self, request), level = "trace")]
	fn call(&self, request: Request<'a>) -> Self::Future {
		let service = self.service.clone();
		let max = self.max;

		async move {
			rx_log_from_json(&request, max);
			let rp = service.call(request).await;
			tx_log_from_str(&rp.result, max);
			rp
		}
		.boxed()
	}
}

/// Similar to [`tower::util::Either`] but
/// adjusted to satisfy the trait bound [`RpcServiceT].
//
// NOTE: This is introduced because it doesn't
// work to implement tower::Layer for
// external types such as future::Either.
#[pin_project(project = EitherProj)]
#[derive(Clone, Debug)]
pub enum Either<A, B> {
	/// One type of backing [`RpcServiceT`].
	A(#[pin] A),
	/// The other type of backing [`RpcServiceT`].
	B(#[pin] B),
}

impl<A, B, T, AE, BE> Future for Either<A, B>
where
	A: Future<Output = Result<T, AE>>,
	AE: Into<BoxError>,
	B: Future<Output = Result<T, BE>>,
	BE: Into<BoxError>,
{
	type Output = Result<T, BoxError>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		match self.project() {
			EitherProj::A(fut) => Poll::Ready(Ok(ready!(fut.poll(cx)).map_err(Into::into)?)),
			EitherProj::B(fut) => Poll::Ready(Ok(ready!(fut.poll(cx)).map_err(Into::into)?)),
		}
	}
}

impl<S, A, B> tower::Layer<S> for Either<A, B>
where
	A: tower::Layer<S>,
	B: tower::Layer<S>,
{
	type Service = Either<A::Service, B::Service>;

	fn layer(&self, inner: S) -> Self::Service {
		match self {
			Either::A(layer) => Either::A(layer.layer(inner)),
			Either::B(layer) => Either::B(layer.layer(inner)),
		}
	}
}

impl<'a, A, B> RpcServiceT<'a> for Either<A, B>
where
	A: RpcServiceT<'a> + Send + 'a,
	B: RpcServiceT<'a> + Send + 'a,
{
	type Future = BoxFuture<'a, MethodResponse>;

	fn call(&self, request: Request<'a>) -> Self::Future {
		match self {
			Either::A(service) => Box::pin(service.call(request)),
			Either::B(service) => Box::pin(service.call(request)),
		}
	}
}

/// Response which may be ready or a future that needs to be
/// polled.
#[pin_project(project = ResponseStateProj)]
pub enum ResponseFuture<F> {
	/// The response is ready.
	Ready(Option<MethodResponse>),
	/// The response has to be polled.
	Poll {
		#[pin]
		/// Future.
		fut: F,
	},
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
		Self::Poll { fut }
	}
}

impl<F: Future<Output = MethodResponse>> Future for ResponseFuture<F> {
	type Output = MethodResponse;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let this = self.project();

		match this {
			ResponseStateProj::Poll { fut } => fut.poll(cx),
			ResponseStateProj::Ready(rp) => Poll::Ready(rp.take().expect("Future not polled after Ready; qed")),
		}
	}
}
