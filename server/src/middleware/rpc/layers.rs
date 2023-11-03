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

use std::sync::Arc;

use jsonrpsee_core::server::{
	BoundedSubscriptions, MethodCallback, MethodResponse, MethodSink, Methods, SubscriptionState,
};
use jsonrpsee_core::tracing::{rx_log_from_json, tx_log_from_str};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_types::error::{reject_too_many_subscriptions, ErrorCode};
use jsonrpsee_types::{ErrorObject, Request};

use super::{RpcServiceT, TransportProtocol};

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

#[async_trait::async_trait]
impl<'a> RpcServiceT<'a> for RpcService {
	async fn call(&self, req: Request<'a>, _transport: TransportProtocol) -> MethodResponse {
		let params = req.params();
		let name = req.method_name();
		let id = req.id().clone();

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

#[async_trait::async_trait]
impl<'a, S> RpcServiceT<'a> for RpcLogger<S>
where
	S: RpcServiceT<'a> + Send + Sync,
{
	#[tracing::instrument(name = "method_call", skip(self, request, transport), level = "trace")]
	async fn call(&self, request: Request<'a>, transport: TransportProtocol) -> MethodResponse {
		rx_log_from_json(&request, self.max);
		let rp = self.service.call(request, transport).await;
		tx_log_from_str(&rp.result, self.max);
		rp
	}
}
