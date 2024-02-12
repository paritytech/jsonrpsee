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

//! Shared utilities for `jsonrpsee` clients.

cfg_async_client! {
	pub mod async_client;
	pub use async_client::{Client, ClientBuilder};
}

pub mod error;
pub use error::Error;

use std::fmt;
use std::ops::Range;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::task;

use crate::params::BatchRequestBuilder;
use crate::traits::ToRpcParams;
use async_trait::async_trait;
use core::marker::PhantomData;
use futures_util::stream::{Stream, StreamExt};
use jsonrpsee_types::{ErrorObject, Id, SubscriptionId};
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;
use tokio::sync::{mpsc, oneshot};

// Re-exports for the `rpc_params` macro.
#[doc(hidden)]
pub mod __reexports {
	// Needs to be in scope for `ArrayParams` to implement it.
	pub use crate::traits::ToRpcParams;
	// Main builder object for constructing the rpc parameters.
	pub use crate::params::ArrayParams;
}

/// [JSON-RPC](https://www.jsonrpc.org/specification) client interface that can make requests and notifications.
#[async_trait]
pub trait ClientT {
	/// Send a [notification request](https://www.jsonrpc.org/specification#notification)
	async fn notification<Params>(&self, method: &str, params: Params) -> Result<(), Error>
	where
		Params: ToRpcParams + Send;

	/// Send a [method call request](https://www.jsonrpc.org/specification#request_object).
	async fn request<R, Params>(&self, method: &str, params: Params) -> Result<R, Error>
	where
		R: DeserializeOwned,
		Params: ToRpcParams + Send;

	/// Send a [batch request](https://www.jsonrpc.org/specification#batch).
	///
	/// The response to batch are returned in the same order as it was inserted in the batch.
	///
	///
	/// Returns `Ok` if all requests in the batch were answered.
	/// Returns `Error` if the network failed or any of the responses could be parsed a valid JSON-RPC response.
	async fn batch_request<'a, R>(&self, batch: BatchRequestBuilder<'a>) -> Result<BatchResponse<'a, R>, Error>
	where
		R: DeserializeOwned + fmt::Debug + 'a;
}

/// [JSON-RPC](https://www.jsonrpc.org/specification) client interface that can make requests, notifications and subscriptions.
#[async_trait]
pub trait SubscriptionClientT: ClientT {
	/// Initiate a subscription by performing a JSON-RPC method call where the server responds with
	/// a `Subscription ID` that is used to fetch messages on that subscription,
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server.
	///
	/// The params may be used as input for the subscription for the server to process.
	///
	/// The `unsubscribe_method` is used to close the subscription
	///
	/// The `Notif` param is a generic type to receive generic subscriptions, see [`Subscription`] for further
	/// documentation.
	async fn subscribe<'a, Notif, Params>(
		&self,
		subscribe_method: &'a str,
		params: Params,
		unsubscribe_method: &'a str,
	) -> Result<Subscription<Notif>, Error>
	where
		Params: ToRpcParams + Send,
		Notif: DeserializeOwned;

	/// Register a method subscription, this is used to filter only server notifications that a user is interested in.
	///
	/// The `Notif` param is a generic type to receive generic subscriptions, see [`Subscription`] for further
	/// documentation.
	async fn subscribe_to_method<'a, Notif>(&self, method: &'a str) -> Result<Subscription<Notif>, Error>
	where
		Notif: DeserializeOwned;
}

/// Marker trait to determine whether a type implements `Send` or not.
#[cfg(target_arch = "wasm32")]
pub trait MaybeSend {}

/// Marker trait to determine whether a type implements `Send` or not.
#[cfg(not(target_arch = "wasm32"))]
pub trait MaybeSend: Send {}

#[cfg(not(target_arch = "wasm32"))]
impl<T: Send> MaybeSend for T {}

#[cfg(target_arch = "wasm32")]
impl<T> MaybeSend for T {}

/// Transport interface to send data asynchronous.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait TransportSenderT: MaybeSend + 'static {
	/// Error that may occur during sending a message.
	type Error: std::error::Error + Send + Sync;

	/// Send.
	async fn send(&mut self, msg: String) -> Result<(), Self::Error>;

	/// This is optional because it's most likely relevant for WebSocket transports only.
	/// You should only implement this is your transport supports sending periodic pings.
	///
	/// Send ping frame (opcode of 0x9).
	async fn send_ping(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}

	/// This is optional because it's most likely relevant for WebSocket transports only.
	/// You should only implement this is your transport supports being closed.
	///
	/// Send customized close message.
	async fn close(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}
}

/// Message type received from the RPC server.
/// It can either be plain text data, bytes, or `Pong` messages.
#[derive(Debug, Clone)]
pub enum ReceivedMessage {
	/// Incoming packet contains plain `String` data.
	Text(String),
	/// Incoming packet contains bytes.
	Bytes(Vec<u8>),
	/// Incoming `Pong` frame as a reply to a previously submitted `Ping` frame.
	Pong,
}

/// Transport interface to receive data asynchronous.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait TransportReceiverT: 'static {
	/// Error that may occur during receiving a message.
	type Error: std::error::Error + Send + Sync;

	/// Receive.
	async fn receive(&mut self) -> Result<ReceivedMessage, Self::Error>;
}

/// Convert the given values to a [`crate::params::ArrayParams`] as expected by a
/// jsonrpsee Client (http or websocket).
///
/// # Panics
///
/// Panics if the serialization of parameters fails.
#[macro_export]
macro_rules! rpc_params {
	($($param:expr),*) => {
		{
			let mut params = $crate::client::__reexports::ArrayParams::new();
			$(
				if let Err(err) = params.insert($param) {
					panic!("Parameter `{}` cannot be serialized: {:?}", stringify!($param), err);
				}
			)*
			params
		}
	};
}

/// Subscription kind
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SubscriptionKind {
	/// Get notifications based on Subscription ID.
	Subscription(SubscriptionId<'static>),
	/// Get notifications based on method name.
	Method(String),
}

/// Active subscription on the client.
///
/// It will try to `unsubscribe` in the drop implementation
/// but it may fail if the underlying buffer is full.
/// Thus, if you want to ensure it's actually unsubscribed then
/// [`Subscription::unsubscribe`] is recommended to use.
#[derive(Debug)]
pub struct Subscription<Notif> {
	/// Channel to send requests to the background task.
	to_back: mpsc::Sender<FrontToBack>,
	/// Channel from which we receive notifications from the server, as encoded `JsonValue`s.
	notifs_rx: mpsc::Receiver<JsonValue>,
	/// Callback kind.
	kind: Option<SubscriptionKind>,
	/// Marker in order to pin the `Notif` parameter.
	marker: PhantomData<Notif>,
}

// `Subscription` does not automatically implement this due to `PhantomData<Notif>`,
// but type type has no need to be pinned.
impl<Notif> std::marker::Unpin for Subscription<Notif> {}

impl<Notif> Subscription<Notif> {
	/// Create a new subscription.
	pub fn new(
		to_back: mpsc::Sender<FrontToBack>,
		notifs_rx: mpsc::Receiver<JsonValue>,
		kind: SubscriptionKind,
	) -> Self {
		Self { to_back, notifs_rx, kind: Some(kind), marker: PhantomData }
	}

	/// Return the subscription type and, if applicable, ID.
	pub fn kind(&self) -> &SubscriptionKind {
		self.kind.as_ref().expect("only None after unsubscribe; qed")
	}

	/// Unsubscribe and consume the subscription.
	pub async fn unsubscribe(mut self) -> Result<(), Error> {
		let msg = match self.kind.take().expect("only None after unsubscribe; qed") {
			SubscriptionKind::Method(notif) => FrontToBack::UnregisterNotification(notif),
			SubscriptionKind::Subscription(sub_id) => FrontToBack::SubscriptionClosed(sub_id),
		};
		// If this fails the connection was already closed i.e, already "unsubscribed".
		let _ = self.to_back.send(msg).await;

		// wait until notif channel is closed then the subscription was closed.
		while self.notifs_rx.recv().await.is_some() {}
		Ok(())
	}
}

/// Batch request message.
#[derive(Debug)]
pub struct BatchMessage {
	/// Serialized batch request.
	pub raw: String,
	/// Request IDs.
	pub ids: Range<u64>,
	/// One-shot channel over which we send back the result of this request.
	pub send_back: oneshot::Sender<Result<Vec<BatchEntry<'static, JsonValue>>, Error>>,
}

/// Request message.
#[derive(Debug)]
pub struct RequestMessage {
	/// Serialized message.
	pub raw: String,
	/// Request ID.
	pub id: Id<'static>,
	/// One-shot channel over which we send back the result of this request.
	pub send_back: Option<oneshot::Sender<Result<JsonValue, Error>>>,
}

/// Subscription message.
#[derive(Debug)]
pub struct SubscriptionMessage {
	/// Serialized message.
	pub raw: String,
	/// Request ID of the subscribe message.
	pub subscribe_id: Id<'static>,
	/// Request ID of the unsubscribe message.
	pub unsubscribe_id: Id<'static>,
	/// Method to use to unsubscribe later. Used if the channel unexpectedly closes.
	pub unsubscribe_method: String,
	/// If the subscription succeeds, we return a [`mpsc::Receiver`] that will receive notifications.
	/// When we get a response from the server about that subscription, we send the result over
	/// this channel.
	pub send_back: oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, SubscriptionId<'static>), Error>>,
}

/// RegisterNotification message.
#[derive(Debug)]
pub struct RegisterNotificationMessage {
	/// Method name this notification handler is attached to
	pub method: String,
	/// We return a [`mpsc::Receiver`] that will receive notifications.
	/// When we get a response from the server about that subscription, we send the result over
	/// this channel.
	pub send_back: oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, String), Error>>,
}

/// Message that the Client can send to the background task.
#[derive(Debug)]
pub enum FrontToBack {
	/// Send a batch request to the server.
	Batch(BatchMessage),
	/// Send a notification to the server.
	Notification(String),
	/// Send a request to the server.
	Request(RequestMessage),
	/// Send a subscription request to the server.
	Subscribe(SubscriptionMessage),
	/// Register a notification handler
	RegisterNotification(RegisterNotificationMessage),
	/// Unregister a notification handler
	UnregisterNotification(String),
	/// When a subscription channel is closed, we send this message to the background
	/// task to mark it ready for garbage collection.
	// NOTE: It is not possible to cancel pending subscriptions or pending requests.
	// Such operations will be blocked until a response is received or the background
	// thread has been terminated.
	SubscriptionClosed(SubscriptionId<'static>),
}

impl<Notif> Subscription<Notif>
where
	Notif: DeserializeOwned,
{
	/// Returns the next notification from the stream.
	/// This may return `None` if the subscription has been terminated,
	/// which may happen if the channel becomes full or is dropped.
	///
	/// **Note:** This has an identical signature to the [`StreamExt::next`]
	/// method (and delegates to that). Import [`StreamExt`] if you'd like
	/// access to other stream combinator methods.
	#[allow(clippy::should_implement_trait)]
	pub async fn next(&mut self) -> Option<Result<Notif, Error>> {
		StreamExt::next(self).await
	}
}

impl<Notif> Stream for Subscription<Notif>
where
	Notif: DeserializeOwned,
{
	type Item = Result<Notif, Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Option<Self::Item>> {
		let n = futures_util::ready!(self.notifs_rx.poll_recv(cx));
		let res = n.map(|n| match serde_json::from_value::<Notif>(n) {
			Ok(parsed) => Ok(parsed),
			Err(e) => Err(Error::ParseError(e)),
		});
		task::Poll::Ready(res)
	}
}

impl<Notif> Drop for Subscription<Notif> {
	fn drop(&mut self) {
		// We can't actually guarantee that this goes through. If the background task is busy, then
		// the channel's buffer will be full.
		// However, when a notification arrives, the background task will realize that the channel
		// to the `Callback` has been closed.

		let msg = match self.kind.take() {
			Some(SubscriptionKind::Method(notif)) => FrontToBack::UnregisterNotification(notif),
			Some(SubscriptionKind::Subscription(sub_id)) => FrontToBack::SubscriptionClosed(sub_id),
			None => return,
		};
		let _ = self.to_back.try_send(msg);
	}
}

#[derive(Debug)]
/// Keep track of request IDs.
pub struct RequestIdManager {
	// Current pending requests.
	current_pending: Arc<()>,
	/// Max concurrent pending requests allowed.
	max_concurrent_requests: usize,
	/// Get the next request ID.
	current_id: CurrentId,
	/// Request ID type.
	id_kind: IdKind,
}

impl RequestIdManager {
	/// Create a new `RequestIdGuard` with the provided concurrency limit.
	pub fn new(limit: usize, id_kind: IdKind) -> Self {
		Self { current_pending: Arc::new(()), max_concurrent_requests: limit, current_id: CurrentId::new(), id_kind }
	}

	fn get_slot(&self) -> Result<Arc<()>, Error> {
		// Strong count is 1 at start, so that's why we use `>` and not `>=`.
		if Arc::strong_count(&self.current_pending) > self.max_concurrent_requests {
			Err(Error::MaxSlotsExceeded)
		} else {
			Ok(self.current_pending.clone())
		}
	}

	/// Attempts to get the next request ID.
	///
	/// Fails if request limit has been exceeded.
	pub fn next_request_id(&self) -> Result<RequestIdGuard<Id<'static>>, Error> {
		let rc = self.get_slot()?;
		let id = self.id_kind.into_id(self.current_id.next());

		Ok(RequestIdGuard { _rc: rc, id })
	}

	/// Attempts to get fetch two ids (used for subscriptions) but only
	/// occupy one slot in the request guard.
	///
	/// Fails if request limit has been exceeded.
	pub fn next_request_two_ids(&self) -> Result<RequestIdGuard<(Id<'static>, Id<'static>)>, Error> {
		let rc = self.get_slot()?;
		let id1 = self.id_kind.into_id(self.current_id.next());
		let id2 = self.id_kind.into_id(self.current_id.next());
		Ok(RequestIdGuard { _rc: rc, id: (id1, id2) })
	}

	/// Get a handle to the `IdKind`.
	pub fn as_id_kind(&self) -> IdKind {
		self.id_kind
	}
}

/// Reference counted request ID.
#[derive(Debug)]
pub struct RequestIdGuard<T: Clone> {
	id: T,
	/// Reference count decreased when dropped.
	_rc: Arc<()>,
}

impl<T: Clone> RequestIdGuard<T> {
	/// Get the actual ID or IDs.
	pub fn inner(&self) -> T {
		self.id.clone()
	}
}

/// What certificate store to use
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum CertificateStore {
	/// Use the native system certificate store
	Native,
	/// Use WebPKI's certificate store
	WebPki,
}

/// JSON-RPC request object id data type.
#[derive(Debug, Copy, Clone)]
pub enum IdKind {
	/// String.
	String,
	/// Number.
	Number,
}

impl IdKind {
	/// Generate an `Id` from number.
	pub fn into_id(self, id: u64) -> Id<'static> {
		match self {
			IdKind::Number => Id::Number(id),
			IdKind::String => Id::Str(format!("{id}").into()),
		}
	}
}

#[derive(Debug)]
struct CurrentId(AtomicUsize);

impl CurrentId {
	fn new() -> Self {
		CurrentId(AtomicUsize::new(0))
	}

	fn next(&self) -> u64 {
		self.0
			.fetch_add(1, Ordering::Relaxed)
			.try_into()
			.expect("usize -> u64 infallible, there are no CPUs > 64 bits; qed")
	}
}

/// Generate a range of IDs to be used in a batch request.
pub fn generate_batch_id_range(guard: &RequestIdGuard<Id>, len: u64) -> Result<Range<u64>, Error> {
	let id_start = guard.inner().try_parse_inner_as_number()?;
	let id_end = id_start
		.checked_add(len)
		.ok_or_else(|| Error::Custom("BatchID range wrapped; restart the client or try again later".to_string()))?;

	Ok(id_start..id_end)
}

/// Represent a single entry in a batch response.
pub type BatchEntry<'a, R> = Result<R, ErrorObject<'a>>;

/// Batch response.
#[derive(Debug, Clone)]
pub struct BatchResponse<'a, R> {
	successful_calls: usize,
	failed_calls: usize,
	responses: Vec<BatchEntry<'a, R>>,
}

impl<'a, R: fmt::Debug + 'a> BatchResponse<'a, R> {
	/// Create a new [`BatchResponse`].
	pub fn new(successful_calls: usize, responses: Vec<BatchEntry<'a, R>>, failed_calls: usize) -> Self {
		Self { successful_calls, responses, failed_calls }
	}

	/// Get the length of the batch response.
	pub fn len(&self) -> usize {
		self.responses.len()
	}

	/// Is empty.
	pub fn is_empty(&self) -> bool {
		self.responses.len() == 0
	}

	/// Get the number of successful calls in the batch.
	pub fn num_successful_calls(&self) -> usize {
		self.successful_calls
	}

	/// Get the number of failed calls in the batch.
	pub fn num_failed_calls(&self) -> usize {
		self.failed_calls
	}

	/// Returns `Ok(iterator)` if all responses were successful
	/// otherwise `Err(iterator)` is returned.
	///
	/// If you want get all responses if an error responses occurs use [`BatchResponse::into_iter`]
	/// instead where it's possible to implement customized logic.
	pub fn into_ok(
		self,
	) -> Result<impl Iterator<Item = R> + 'a + std::fmt::Debug, impl Iterator<Item = ErrorObject<'a>> + std::fmt::Debug>
	{
		if self.failed_calls > 0 {
			Err(self.into_iter().filter_map(|err| err.err()))
		} else {
			Ok(self.into_iter().filter_map(|r| r.ok()))
		}
	}

	/// Similar to [`BatchResponse::into_ok`] but takes the responses by reference instead.
	pub fn ok(
		&self,
	) -> Result<impl Iterator<Item = &R> + std::fmt::Debug, impl Iterator<Item = &ErrorObject<'a>> + std::fmt::Debug> {
		if self.failed_calls > 0 {
			Err(self.responses.iter().filter_map(|err| err.as_ref().err()))
		} else {
			Ok(self.responses.iter().filter_map(|r| r.as_ref().ok()))
		}
	}

	/// Returns an iterator over all responses.
	pub fn iter(&self) -> impl Iterator<Item = &BatchEntry<'_, R>> {
		self.responses.iter()
	}
}

impl<'a, R> IntoIterator for BatchResponse<'a, R> {
	type Item = BatchEntry<'a, R>;
	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.responses.into_iter()
	}
}

#[cfg(test)]
mod tests {
	use super::{IdKind, RequestIdManager};

	#[test]
	fn request_id_guard_works() {
		let manager = RequestIdManager::new(2, IdKind::Number);
		let _first = manager.next_request_id().unwrap();

		{
			let _second = manager.next_request_two_ids().unwrap();
			assert!(manager.next_request_id().is_err());
			// second dropped here.
		}

		assert!(manager.next_request_id().is_ok());
	}
}
