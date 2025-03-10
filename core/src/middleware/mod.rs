//! Middleware for the RPC service.

pub mod layer;

use futures_util::future::{Either, Future};
use pin_project::pin_project;
use serde_json::value::RawValue;
use tower::layer::LayerFn;
use tower::layer::util::{Identity, Stack};

use std::borrow::Cow;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::server::MethodResponse;

/// Re-export types from `jsonrpsee_types` crate for convenience
pub type Notification<'a> = jsonrpsee_types::Notification<'a, Option<Cow<'a, RawValue>>>;
/// Re-export types from `jsonrpsee_types` crate for convenience
pub use jsonrpsee_types::Request;

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
	fn batch(&self, requests: Vec<Request<'a>>) -> Self::Future;

	/// Similar to `RpcServiceT::call` but process a JSON-RPC notification.
	fn notification(&self, n: Notification<'a>) -> Self::Future;
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
	pub fn rpc_logger(self, max_log_len: u32) -> RpcServiceBuilder<Stack<layer::RpcLoggerLayer, L>> {
		RpcServiceBuilder(self.0.layer(layer::RpcLoggerLayer::new(max_log_len)))
	}

	/// Wrap the service `S` with the middleware.
	pub fn service<S>(&self, service: S) -> L::Service
	where
		L: tower::Layer<S>,
	{
		self.0.service(service)
	}
}

/// Response which may be ready or a future.
#[derive(Debug)]
#[pin_project]
pub struct ResponseFuture<F>(#[pin] futures_util::future::Either<F, std::future::Ready<MethodResponse>>);

impl<F> ResponseFuture<F> {
	/// Returns a future that resolves to a response.
	pub fn future(f: F) -> ResponseFuture<F> {
		ResponseFuture(Either::Left(f))
	}

	/// Return a response which is already computed.
	pub fn ready(response: MethodResponse) -> ResponseFuture<F> {
		ResponseFuture(Either::Right(std::future::ready(response)))
	}
}

impl<F: Future<Output = MethodResponse>> Future for ResponseFuture<F> {
	type Output = MethodResponse;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		self.project().0.poll(cx)
	}
}
