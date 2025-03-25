//! Middleware for the RPC service.

pub mod layer;

use futures_util::future::{BoxFuture, Either, Future};
use jsonrpsee_types::Id;
use pin_project::pin_project;
use serde::Serialize;
use serde_json::value::RawValue;
use tower::layer::LayerFn;
use tower::layer::util::{Identity, Stack};

use std::borrow::Cow;
use std::pin::Pin;
use std::task::{Context, Poll};

/// Re-export types from `jsonrpsee_types` crate for convenience.
pub type Notification<'a> = jsonrpsee_types::Notification<'a, Option<Cow<'a, RawValue>>>;
/// Re-export types from `jsonrpsee_types` crate for convenience.
pub use jsonrpsee_types::{Extensions, Request};
/// Type alias for a boxed future that resolves to Result<R, E>.
pub type ResponseBoxFuture<'a, R, E> = BoxFuture<'a, Result<R, E>>;
/// Type alias for a batch of JSON-RPC calls and notifications.
pub type Batch<'a> = Vec<BatchEntry<'a>>;

#[derive(Debug, Clone)]
/// A marker type to indicate that the request is a subscription for the [`RpcServiceT::call`] method.
pub struct IsSubscription {
	pub sub_id: Id<'static>,
	pub unsub_id: Id<'static>,
	pub unsub_method: String,
}

/// A batch entry specific for the [`RpcServiceT::batch`] method to support both
/// method calls and notifications.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BatchEntry<'a> {
	/// A regular JSON-RPC call.
	Call(Request<'a>),
	/// A JSON-RPC notification.
	Notification(Notification<'a>),
	/// The representation of an invalid request and kept to maintain the batch order.
	///
	/// WARNING: This is an internal state and should be NOT be used by external users
	///
	#[serde(skip)]
	#[doc(hidden)]
	InvalidRequest(Id<'a>),
}

/// Internal InvalidRequest that can't be instantiated by the user.
#[derive(Debug, Clone)]
pub struct InvalidRequest<'a>(Id<'a>);

impl<'a> InvalidRequest<'a> {
	/// Consume the invalid request and extract the id.
	pub fn into_id(self) -> Id<'a> {
		self.0
	}
}

impl<'a> BatchEntry<'a> {
	/// Get a reference to extensions of the batch entry.
	pub fn extensions(&self) -> &Extensions {
		match self {
			BatchEntry::Call(req) => req.extensions(),
			BatchEntry::Notification(n) => n.extensions(),
			BatchEntry::InvalidRequest(_) => panic!("BatchEntry::InvalidRequest should not be used"),
		}
	}

	/// Get a mut reference to extensions of the batch entry.
	pub fn extensions_mut(&mut self) -> &mut Extensions {
		match self {
			BatchEntry::Call(req) => req.extensions_mut(),
			BatchEntry::Notification(n) => n.extensions_mut(),
			BatchEntry::InvalidRequest(_) => panic!("BatchEntry::InvalidRequest should not be used"),
		}
	}

	/// Get the method name of the batch entry.
	pub fn method_name(&self) -> &str {
		match self {
			BatchEntry::Call(req) => req.method_name(),
			BatchEntry::Notification(n) => n.method_name(),
			BatchEntry::InvalidRequest(_) => panic!("BatchEntry::InvalidRequest should not be used"),
		}
	}

	/// Consume the batch entry and extract the extensions.
	pub fn into_extensions(self) -> Extensions {
		match self {
			BatchEntry::Call(req) => req.extensions,
			BatchEntry::Notification(n) => n.extensions,
			BatchEntry::InvalidRequest(_) => panic!("BatchEntry::InvalidRequest should not be used"),
		}
	}
}

/// Present a JSON-RPC service that can process JSON-RPC calls, notifications, and batch requests.
///
/// This trait is similar to [`tower::Service`] but it's specialized for JSON-RPC operations.
///
/// The response type is a future that resolves to a `Result<R, E>` mainly because this trait is
/// intended to by used by both client and server implementations.
///
/// In the server implementation, the error is infallible but in the client implementation, the error
/// can occur due to I/O errors or JSON-RPC protocol errors.
pub trait RpcServiceT<'a> {
	/// The future response value.
	type Future: Future<Output = Result<Self::Response, Self::Error>> + Send;

	/// The error type.
	type Error: std::fmt::Debug;

	/// The response type
	type Response;

	/// Processes a single JSON-RPC call, which may be a subscription or regular call.
	fn call(&self, request: Request<'a>) -> Self::Future;

	/// Processes multiple JSON-RPC calls at once, similar to `RpcServiceT::call`.
	///
	/// This method wraps `RpcServiceT::call` and `RpcServiceT::notification`,
	/// but the root RPC service does not inherently recognize custom implementations
	/// of these methods.
	///
	/// As a result, if you have custom logic for individual calls or notifications,
	/// you must duplicate that logic here.
	///
	// TODO: Investigate if the complete service can be invoked inside `RpcService`.
	fn batch(&self, requests: Batch<'a>) -> Self::Future;

	/// Similar to `RpcServiceT::call` but processes a JSON-RPC notification.
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
pub struct ResponseFuture<F, R, E>(#[pin] futures_util::future::Either<F, std::future::Ready<Result<R, E>>>);

impl<F, R, E> ResponseFuture<F, R, E> {
	/// Returns a future that resolves to a response.
	pub fn future(f: F) -> ResponseFuture<F, R, E> {
		ResponseFuture(Either::Left(f))
	}

	/// Return a response which is already computed.
	pub fn ready(response: R) -> ResponseFuture<F, R, E> {
		ResponseFuture(Either::Right(std::future::ready(Ok(response))))
	}
}

impl<F, R, E> Future for ResponseFuture<F, R, E>
where
	F: Future<Output = Result<R, E>>,
{
	type Output = F::Output;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		match self.project().0.poll(cx) {
			Poll::Ready(rp) => Poll::Ready(rp),
			Poll::Pending => Poll::Pending,
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn serialize_batch_entry() {
		use super::{BatchEntry, Notification, Request};
		use jsonrpsee_types::Id;

		let req = Request::new("say_hello".into(), None, Id::Number(1));
		let batch_entry = BatchEntry::Call(req.clone());
		assert_eq!(
			serde_json::to_string(&batch_entry).unwrap(),
			"{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"say_hello\"}",
		);

		let notification = Notification::new("say_hello".into(), None);
		let batch_entry = BatchEntry::Notification(notification.clone());
		assert_eq!(
			serde_json::to_string(&batch_entry).unwrap(),
			"{\"jsonrpc\":\"2.0\",\"method\":\"say_hello\",\"params\":null}",
		);
	}
}
