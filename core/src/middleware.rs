//! Middleware for the RPC service.

use futures_util::future::{Either, Future};
use jsonrpsee_types::{Notification, Request};
use serde_json::value::RawValue;
use tower::layer::util::{Identity, Stack};
use tower::layer::LayerFn;

use crate::server::MethodResponse;

/// Similar to the [`tower::Service`] but specific for jsonrpsee and
/// doesn't requires `&mut self` for performance reasons.
pub trait RpcServiceT<'a> {
	/// The future response value.
	type Future: Future<Output = MethodResponse> + Send;

	/// Process a single JSON-RPC call it may be a subscription or regular call.
	///
	/// In this interface both are treated in the same way but it's possible to
	/// distinguish those based on the `MethodResponse`.
	fn call(&self, request: Request<'a>) -> Self::Future;

	/// Similar to `RpcServiceT::call` but process multiple JSON-RPC calls at once.
	///
	/// This method is optional because it's generally not by the server however
	/// it may be useful for batch processing on the client side.
	fn batch(&self, _requests: Vec<Request<'a>>) -> Self::Future {
		todo!();
	}

	/// Similar to `RpcServiceT::call` but process a JSON-RPC notification.
	fn notification(&self, _request: Notification<'a, Option<Box<RawValue>>>) -> Self::Future {
		todo!();
	}
}

/// Similar to [`tower::ServiceBuilder`] but doesn't
/// support any tower middleware implementations.
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
		let layer = if let Some(layer) = layer { Either::Left(layer) } else { Either::Right(Identity::new()) };
		self.layer(layer)
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

	/// Add a logging layer to [`RpcServiceBuilder`]
	///
	/// This logs each request and response for every call.
	///
	/*pub fn rpc_logger(self, max_log_len: u32) -> RpcServiceBuilder<Stack<RpcLoggerLayer, L>> {
		RpcServiceBuilder(self.0.layer(RpcLoggerLayer::new(max_log_len)))
	}*/

	/// Wrap the service `S` with the middleware.
	pub fn service<S>(&self, service: S) -> L::Service
	where
		L: tower::Layer<S>,
	{
		self.0.service(service)
	}
}
