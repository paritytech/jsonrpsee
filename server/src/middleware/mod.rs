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

use std::{net::SocketAddr, sync::Arc};

pub use authority::*;
pub use host_filter::*;
pub use proxy_get_request::*;

use http::{HeaderMap, Uri};
use tower::layer::util::{Identity, Stack};
use tower::layer::LayerFn;
use tower::util::Either;

use jsonrpsee_core::server::{
	BoundedSubscriptions, MethodCallback, MethodResponse, MethodSink, Methods, SubscriptionState,
};
use jsonrpsee_core::tracing::{rx_log_from_json, tx_log_from_str};
use jsonrpsee_core::traits::IdProvider;
use jsonrpsee_types::error::{reject_too_many_subscriptions, ErrorCode};
use jsonrpsee_types::{ErrorObject, Params, Request};

use tracing::instrument;

/// The transport protocol used to send or receive a call or request.
#[derive(Debug, Copy, Clone)]
pub enum TransportProtocol {
	/// HTTP transport.
	Http,
	/// WebSocket transport.
	WebSocket,
}

impl std::fmt::Display for TransportProtocol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::Http => "http",
			Self::WebSocket => "websocket",
		};

		write!(f, "{s}")
	}
}

/// Metadata to a JSON-RPC call.
#[derive(Debug, Clone)]
pub struct Meta {
	/// Transport protocol.
	pub transport: TransportProtocol,
	/// Remote addr.
	pub remote_addr: SocketAddr,
	/// Connection id.
	pub conn_id: usize,
	/// HTTP Headers.
	pub headers: HeaderMap,
	/// URI.
	pub uri: Uri,
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

/// JSON-RPC service middleware.
#[derive(Clone, Debug)]
pub struct RpcService {
	conn_id: usize,
	methods: Methods,
	max_response_body_size: usize,
	cfg: RpcServiceCfg,
	max_log_length: u32,
}

impl RpcService {
	/// Create a new service with doesn't support subscriptions.
	pub(crate) fn new(
		methods: Methods,
		max_response_body_size: usize,
		conn_id: usize,
		max_log_length: u32,
		cfg: RpcServiceCfg,
	) -> Self {
		Self { methods, max_response_body_size, conn_id, cfg, max_log_length }
	}
}

/// Similar to `tower::Service` but specific for jsonrpsee and
/// doesn't requires `&mut self` for performance reasons.
#[async_trait::async_trait]
pub trait RpcServiceT<'a> {
	/// Process a single JSON-RPC call it may be a subscription or regular call.
	/// In this interface they are treated in the same way but it's possible to
	/// distinguish those based on the `MethodResponse`.
	async fn call(&self, request: Request<'a>, meta: &Meta) -> MethodResponse;
}

#[async_trait::async_trait]
impl<'a> RpcServiceT<'a> for RpcService {
	#[instrument(name = "method_call", fields(method = req.method.as_ref()), skip(_meta, req, self), level = "TRACE")]
	async fn call(&self, req: Request<'a>, _meta: &Meta) -> MethodResponse {
		rx_log_from_json(&req, self.max_log_length);

		let params = Params::new(req.params.map(|params| params.get()));
		let name = &req.method;
		let id = req.id;

		let rp = match self.methods.method_with_name(name) {
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
		};

		tx_log_from_str(&rp.result, self.max_log_length);
		rp
	}
}

/// Similar to [`tower::ServiceBuilder`] but doesn't
/// support any middleware implementations.
#[derive(Debug, Clone)]
pub struct RpcServiceBuilder<L>(tower::ServiceBuilder<L>);

impl Default for RpcServiceBuilder<Identity> {
	fn default() -> Self {
		RpcServiceBuilder(tower::ServiceBuilder::new())
	}
}

impl RpcServiceBuilder<Identity> {
	/// Create a new [`RpcServiceBuilder`].
	pub fn new() -> Self {
		Self(tower::ServiceBuilder::new())
	}
}

impl<L> RpcServiceBuilder<L> {
	/// Optionally add a new layer `T` to the [`RpcServiceBuilder`].
	///
	/// See the documentation for [`tower::ServiceBuilder::option_layer`] for more details.
	pub fn option_layer<T>(self, layer: Option<T>) -> RpcServiceBuilder<Stack<Either<T, Identity>, L>> {
		RpcServiceBuilder(self.0.option_layer(layer))
	}

	/// Add a new layer `T` to the [`RpcServiceBuilder`].
	///
	/// See the documentation for [`tower::ServiceBuilder::layer`] for more details.
	pub fn layer<T>(self, layer: T) -> RpcServiceBuilder<Stack<T, L>> {
		RpcServiceBuilder(self.0.layer(layer))
	}

	/// Add a [`tower::Layer`] built from a function that accepts a service and returns another service.
	///
	/// See the documentation for [`tower::ServiceBuilder::layer_fn`] for more details.
	pub fn layer_fn<F>(self, f: F) -> RpcServiceBuilder<Stack<LayerFn<F>, L>> {
		RpcServiceBuilder(self.0.layer_fn(f))
	}

	/// Wrap the service `S` with the middleware.
	pub(crate) fn service<S>(&self, service: S) -> L::Service
	where
		L: tower::Layer<S>,
	{
		self.0.service(service)
	}
}
