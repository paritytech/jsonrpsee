//! Middleware for the RPC service.

pub mod layer;

use futures_util::future::{Either, Future};
use jsonrpsee_types::{ErrorCode, ErrorObject, Id, InvalidRequestId, Response};
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

#[derive(Debug)]
/// A JSON-RPC error response indicating an invalid request.
pub struct InvalidRequest<'a>(Response<'a, ()>);

impl<'a> InvalidRequest<'a> {
	/// Create a new `InvalidRequest` error response.
	pub fn new(id: Id<'a>) -> Self {
		let payload = jsonrpsee_types::ResponsePayload::error(ErrorObject::from(ErrorCode::InvalidRequest));
		let response = Response::new(payload, id);
		Self(response)
	}

	/// Consume the `InvalidRequest` to get the error object and id.
	pub fn into_parts(self) -> (ErrorObject<'a>, Id<'a>) {
		match self.0.payload {
			jsonrpsee_types::ResponsePayload::Error(err) => (err, self.0.id),
			_ => unreachable!("InvalidRequest::new should only create an error response; qed"),
		}
	}
}

impl Serialize for InvalidRequest<'_> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.serialize(serializer)
	}
}

/// A batch of JSON-RPC calls and notifications.
#[derive(Debug, Default)]
pub struct Batch<'a> {
	inner: Vec<Result<BatchEntry<'a>, InvalidRequest<'a>>>,
	id_range: Option<std::ops::Range<u64>>,
}

impl Serialize for Batch<'_> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		// Err(InvalidRequest) is just an internal error type and should not be serialized.
		//
		// It is used to indicate the RpcServiceT::batch method that the request is invalid
		// and should replied with an error entry.
		//
		// This clippy lint is faulty because `Result::ok` consumes the value
		// and we don't want to consume the value here.
		#[allow(clippy::manual_ok_err)]
		let only_ok: Vec<&BatchEntry> = self
			.inner
			.iter()
			.filter_map(|entry| match entry {
				Ok(entry) => Some(entry),
				Err(_) => None,
			})
			.collect();

		serde::Serialize::serialize(&only_ok, serializer)
	}
}

impl std::fmt::Display for Batch<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = serde_json::to_string(&self.inner).expect("Batch serialization failed");
		f.write_str(&s)
	}
}

impl<'a> Batch<'a> {
	/// Create a new empty batch.
	pub fn new() -> Self {
		Self::default()
	}

	/// Create a new batch from a list of batch entries without an id range.
	pub fn from_batch_entries(inner: Vec<Result<BatchEntry<'a>, InvalidRequest<'a>>>) -> Self {
		Self { inner, id_range: None }
	}

	/// Insert a new batch entry into the batch.
	///
	/// Fails if the request id is not a number or the id range overflows.
	pub fn push(&mut self, req: Request<'a>) -> Result<(), InvalidRequestId> {
		let id = req.id().try_parse_inner_as_number()?;

		match self.id_range {
			Some(ref mut range) => {
				debug_assert!(id + 1 > range.end);
				range.end =
					id.checked_add(1).ok_or_else(|| InvalidRequestId::Invalid("Id range overflow".to_string()))?;
			}
			None => {
				self.id_range = Some(id..id);
			}
		}
		self.inner.push(Ok(BatchEntry::Call(req)));

		Ok(())
	}

	/// Get an iterator over the batch entries.
	pub fn as_batch_entries(&self) -> &[Result<BatchEntry<'a>, InvalidRequest<'a>>] {
		&self.inner
	}

	/// Get a mutable iterator over the batch entries.
	pub fn as_mut_batch_entries(&mut self) -> &mut [Result<BatchEntry<'a>, InvalidRequest<'a>>] {
		&mut self.inner
	}

	/// Consume the batch and return the inner entries.
	pub fn into_batch_entries(self) -> Vec<Result<BatchEntry<'a>, InvalidRequest<'a>>> {
		self.inner
	}

	/// Get the id range of the batch.
	///
	/// This is only available if the batch has been constructed using `Batch::push`.
	pub fn id_range(&self) -> Option<std::ops::Range<u64>> {
		self.id_range.clone()
	}

	/// Get the length of the batch.
	pub fn len(&self) -> usize {
		self.inner.len()
	}

	/// Check if the batch is empty.
	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}
}

#[derive(Debug, Clone)]
/// A marker type to indicate that the request is a subscription for the [`RpcServiceT::call`] method.
pub struct IsSubscription {
	sub_id: Id<'static>,
	unsub_id: Id<'static>,
	unsub_method: String,
}

impl IsSubscription {
	/// Create a new [`IsSubscription`] instance.
	pub fn new(sub_id: Id<'static>, unsub_id: Id<'static>, unsub_method: String) -> Self {
		Self { sub_id, unsub_id, unsub_method }
	}

	/// Get the request id of the subscription calls.
	pub fn sub_req_id(&self) -> Id<'static> {
		self.sub_id.clone()
	}

	/// Get the request id of the unsubscription call.
	pub fn unsub_req_id(&self) -> Id<'static> {
		self.unsub_id.clone()
	}

	/// Get the unsubscription method name.
	pub fn unsubscribe_method(&self) -> &str {
		&self.unsub_method
	}
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
}

impl<'a> BatchEntry<'a> {
	/// Get a reference to extensions of the batch entry.
	pub fn extensions(&self) -> &Extensions {
		match self {
			BatchEntry::Call(req) => req.extensions(),
			BatchEntry::Notification(n) => n.extensions(),
		}
	}

	/// Get a mut reference to extensions of the batch entry.
	pub fn extensions_mut(&mut self) -> &mut Extensions {
		match self {
			BatchEntry::Call(req) => req.extensions_mut(),
			BatchEntry::Notification(n) => n.extensions_mut(),
		}
	}

	/// Get the method name of the batch entry.
	pub fn method_name(&self) -> &str {
		match self {
			BatchEntry::Call(req) => req.method_name(),
			BatchEntry::Notification(n) => n.method_name(),
		}
	}

	/// Set the method name of the batch entry.
	pub fn set_method_name(&mut self, name: impl Into<Cow<'a, str>>) {
		match self {
			BatchEntry::Call(req) => {
				req.method = name.into();
			}
			BatchEntry::Notification(n) => {
				n.method = name.into();
			}
		}
	}

	/// Get the params of the batch entry (may be empty).
	pub fn params(&self) -> Option<&Cow<'a, RawValue>> {
		match self {
			BatchEntry::Call(req) => req.params.as_ref(),
			BatchEntry::Notification(n) => n.params.as_ref(),
		}
	}

	/// Set the params of the batch entry.
	pub fn set_params(&mut self, params: Option<Box<RawValue>>) {
		match self {
			BatchEntry::Call(req) => {
				req.params = params.map(Cow::Owned);
			}
			BatchEntry::Notification(n) => {
				n.params = params.map(Cow::Owned);
			}
		}
	}

	/// Consume the batch entry and extract the extensions.
	pub fn into_extensions(self) -> Extensions {
		match self {
			BatchEntry::Call(req) => req.extensions,
			BatchEntry::Notification(n) => n.extensions,
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
pub trait RpcServiceT {
	/// The error type.
	type Error: std::fmt::Debug;

	/// The response type
	type Response: ToJson;

	/// Processes a single JSON-RPC call, which may be a subscription or regular call.
	fn call<'a>(&self, request: Request<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a;

	/// Processes multiple JSON-RPC calls at once, similar to `RpcServiceT::call`.
	///
	/// This method wraps `RpcServiceT::call` and `RpcServiceT::notification`,
	/// but the root RPC service does not inherently recognize custom implementations
	/// of these methods.
	///
	/// As a result, if you have custom logic for individual calls or notifications,
	/// you must duplicate that implementation in this method or no middleware will be applied
	/// for calls inside the batch.
	fn batch<'a>(&self, requests: Batch<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a;

	/// Similar to `RpcServiceT::call` but processes a JSON-RPC notification.
	fn notification<'a>(
		&self,
		n: Notification<'a>,
	) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a;
}

/// Interface for types that can be serialized into JSON.
pub trait ToJson {
	/// Convert the type into a JSON value.
	fn to_json(&self) -> Result<Box<RawValue>, serde_json::Error>;
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

		let req = Request::borrowed("say_hello", None, Id::Number(1));
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

	#[test]
	fn serialize_batch_works() {
		use super::{Batch, BatchEntry, InvalidRequest, Notification, Request};
		use jsonrpsee_types::Id;

		let req = Request::borrowed("say_hello", None, Id::Number(1));
		let notification = Notification::new("say_hello".into(), None);
		let batch = Batch::from_batch_entries(vec![
			Ok(BatchEntry::Call(req)),
			Ok(BatchEntry::Notification(notification)),
			Err(InvalidRequest::new(Id::Null)),
		]);
		assert_eq!(
			serde_json::to_string(&batch).unwrap(),
			r#"[{"jsonrpc":"2.0","id":1,"method":"say_hello"},{"jsonrpc":"2.0","method":"say_hello","params":null}]"#,
		);
	}
}
