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

//! Various middleware implementations for RPC specific purposes.

/// Utility and types related to the authority of an URI.
mod authority;
/// HTTP Host filtering middleware.
mod host_filter;
/// Proxy `GET /path` to internal RPC methods.
mod proxy_get_request;

use std::sync::Arc;

pub use authority::*;
pub use host_filter::*;
use jsonrpsee_core::{
	server::{BoundedSubscriptions, MethodCallback, MethodResponse, MethodSink, Methods, SubscriptionState},
	traits::IdProvider,
};
use jsonrpsee_types::{
	error::{reject_too_many_subscriptions, ErrorCode},
	ErrorObject, Params, Request,
};
pub use proxy_get_request::*;

#[derive(Clone, Debug)]
enum RpcServiceCfg {
	// The server supports only subscriptions.
	OnlyCalls,
	// The server supports both method calls and subscriptions.
	CallsAndSubscriptions {
		bounded_subscriptions: BoundedSubscriptions,
		sink: MethodSink,
		id_provider: Arc<dyn IdProvider>,
		_pending_calls: tokio::sync::mpsc::Sender<()>,
	},
}

/// JSON-RPC service middleware.
#[derive(Clone, Debug)]
pub struct RpcService {
	conn_id: usize,
	methods: Methods,
	max_response_body_size: usize,
	cfg: RpcServiceCfg,
}

impl RpcService {
	/// Create a new service with doesn't support subscriptions.
	pub fn only_calls(methods: Methods, max_response_body_size: usize, conn_id: usize) -> Self {
		Self { methods, max_response_body_size, conn_id, cfg: RpcServiceCfg::OnlyCalls }
	}

	/// Create a new service that supports both calls and subscriptions.
	pub fn full(
		methods: Methods,
		max_response_body_size: usize,
		bounded_subscriptions: BoundedSubscriptions,
		sink: MethodSink,
		id_provider: Arc<dyn IdProvider>,
		conn_id: usize,
		pending_calls: tokio::sync::mpsc::Sender<()>,
	) -> Self {
		Self {
			conn_id,
			methods,
			max_response_body_size,
			cfg: RpcServiceCfg::CallsAndSubscriptions {
				bounded_subscriptions,
				sink,
				id_provider,
				_pending_calls: pending_calls,
			},
		}
	}
}

/// Layer for the RpcService middleware.
#[derive(Clone, Copy, Debug)]
pub struct RpcServiceLayer;

impl tower::Layer<RpcService> for RpcServiceLayer {
	type Service = RpcService;

	fn layer(&self, inner: Self::Service) -> Self::Service {
		inner
	}
}

/// Similar to `tower::Service` but specific for jsonrpsee and
/// doesn't requires `&mut self` for performance reasons.
///
/// Because &mut self will cause every to call to by guarded by Arc<Mutex>
/// and each RPC can only be processed sequentially which is very bad.
#[async_trait::async_trait]
pub trait RpcServiceT<'a>: Send {
	/// Process a single JSON-RPC call it may be a subscription or regular call.
	/// In this interface they are treated in the same way but it's possible to
	/// distinguish those based on the `MethodResponse`.
	async fn call(&self, request: Request<'a>) -> MethodResponse;
}

#[async_trait::async_trait]
impl<'a> RpcServiceT<'a> for RpcService {
	async fn call(&self, req: Request<'a>) -> MethodResponse {
		let params = Params::new(req.params.map(|params| params.get()));
		let name = &req.method;
		let id = req.id;

		match self.methods.method_with_name(name) {
			None => MethodResponse::error(id, ErrorObject::from(ErrorCode::MethodNotFound)),
			Some((_name, method)) => match method {
				MethodCallback::Async(callback) => {
					let id = id.into_owned();
					let params = params.into_owned();
					let conn_id = self.conn_id;
					let max_response_body_size = self.max_response_body_size;

					(callback)(id, params, conn_id, max_response_body_size).await
				}
				MethodCallback::Sync(callback) => {
					let max_response_body_size = self.max_response_body_size;
					(callback)(id, params, max_response_body_size)
				}
				MethodCallback::Subscription(callback) => {
					let RpcServiceCfg::CallsAndSubscriptions {
						bounded_subscriptions,
						sink,
						id_provider,
						_pending_calls,
					} = &self.cfg
					else {
						tracing::warn!("Subscriptions not supported");
						return MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError));
					};

					if let Some(p) = bounded_subscriptions.acquire() {
						let conn_state = SubscriptionState {
							conn_id: self.conn_id,
							id_provider: &*id_provider.clone(),
							subscription_permit: p,
						};

						match callback(id, params, sink.clone(), conn_state).await {
							Ok(r) => r,
							Err(id) => MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError)),
						}
					} else {
						let max = bounded_subscriptions.max();
						MethodResponse::error(id, reject_too_many_subscriptions(max))
					}
				}
				MethodCallback::Unsubscription(callback) => {
					// Don't adhere to any resource or subscription limits; always let unsubscribing happen!

					let RpcServiceCfg::CallsAndSubscriptions { .. } = self.cfg else {
						tracing::warn!("Subscriptions not supported");
						return MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError));
					};

					let conn_id = self.conn_id;
					let max_response_body_size = self.max_response_body_size;
					callback(id, params, conn_id, max_response_body_size)
				}
			},
		}
	}
}
