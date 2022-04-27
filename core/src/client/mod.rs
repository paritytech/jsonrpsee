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

use std::pin::Pin;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::task;

use crate::error::Error;
use async_trait::async_trait;
use core::marker::PhantomData;
use futures_channel::{mpsc, oneshot};
use futures_util::future::FutureExt;
use futures_util::sink::SinkExt;
use futures_util::stream::{Stream, StreamExt};
use jsonrpsee_types::{Id, ParamsSer, SubscriptionId};
use serde::de::DeserializeOwned;
use serde_json::Value as JsonValue;

#[doc(hidden)]
pub mod __reexports {
	pub use crate::to_json_value;
	pub use jsonrpsee_types::ParamsSer;
}

cfg_async_client! {
	pub mod async_client;
	pub use async_client::{Client, ClientBuilder};
}

/// [JSON-RPC](https://www.jsonrpc.org/specification) client interface that can make requests and notifications.
#[async_trait]
pub trait ClientT {
	/// Send a [notification request](https://www.jsonrpc.org/specification#notification)
	async fn notification<'a>(&self, method: &'a str, params: Option<ParamsSer<'a>>) -> Result<(), Error>;

	/// Send a [method call request](https://www.jsonrpc.org/specification#request_object).
	async fn request<'a, R>(&self, method: &'a str, params: Option<ParamsSer<'a>>) -> Result<R, Error>
	where
		R: DeserializeOwned;

	/// Send a [batch request](https://www.jsonrpc.org/specification#batch).
	///
	/// The response to batch are returned in the same order as it was inserted in the batch.
	///
	/// Returns `Ok` if all requests in the batch were answered successfully.
	/// Returns `Error` if any of the requests in batch fails.
	async fn batch_request<'a, R>(&self, batch: Vec<(&'a str, Option<ParamsSer<'a>>)>) -> Result<Vec<R>, Error>
	where
		R: DeserializeOwned + Default + Clone;
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
	async fn subscribe<'a, Notif>(
		&self,
		subscribe_method: &'a str,
		params: Option<ParamsSer<'a>>,
		unsubscribe_method: &'a str,
	) -> Result<Subscription<Notif>, Error>
	where
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

	/// If the transport supports sending customized close messages.
	async fn close(&mut self) -> Result<(), Self::Error> {
		Ok(())
	}
}

/// Transport interface to receive data asynchronous.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait TransportReceiverT: 'static {
	/// Error that may occur during receiving a message.
	type Error: std::error::Error + Send + Sync;

	/// Receive.
	async fn receive(&mut self) -> Result<String, Self::Error>;
}

#[macro_export]
/// Convert the given values to a [`jsonrpsee_types::ParamsSer`] as expected by a jsonrpsee Client (http or websocket).
macro_rules! rpc_params {
	($($param:expr),*) => {
		{
			let mut __params = vec![];
			$(
				__params.push($crate::client::__reexports::to_json_value($param).expect("json serialization is infallible; qed."));
			)*
			Some($crate::client::__reexports::ParamsSer::Array(__params))
		}
	};
	() => {
		None
	}
}

/// Subscription kind
#[derive(Debug)]
#[non_exhaustive]
pub enum SubscriptionKind {
	/// Get notifications based on Subscription ID.
	Subscription(SubscriptionId<'static>),
	/// Get notifications based on method name.
	Method(String),
}

/// Active subscription on the client.
///
/// It will automatically unsubscribe in the [`Subscription::drop`] so no need to explicitly call
/// the `unsubscribe method` if it is an an subscription based on [`SubscriptionId`].
#[derive(Debug)]
pub struct Subscription<Notif> {
	/// Channel to send requests to the background task.
	to_back: mpsc::Sender<FrontToBack>,
	/// Channel from which we receive notifications from the server, as encoded `JsonValue`s.
	notifs_rx: mpsc::Receiver<JsonValue>,
	/// Callback kind.
	kind: SubscriptionKind,
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
		Self { to_back, notifs_rx, kind, marker: PhantomData }
	}
}

/// Batch request message.
#[derive(Debug)]
pub struct BatchMessage {
	/// Serialized batch request.
	pub raw: String,
	/// Request IDs.
	pub ids: Vec<Id<'static>>,
	/// One-shot channel over which we send back the result of this request.
	pub send_back: oneshot::Sender<Result<Vec<JsonValue>, Error>>,
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
		let n = futures_util::ready!(self.notifs_rx.poll_next_unpin(cx));
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
		let kind = std::mem::replace(&mut self.kind, SubscriptionKind::Subscription(SubscriptionId::Num(0)));

		let msg = match kind {
			SubscriptionKind::Method(notif) => FrontToBack::UnregisterNotification(notif),
			SubscriptionKind::Subscription(sub_id) => FrontToBack::SubscriptionClosed(sub_id),
		};
		let _ = self.to_back.send(msg).now_or_never();
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
	current_id: AtomicU64,
	/// Request ID type.
	id_kind: IdKind,
}

impl RequestIdManager {
	/// Create a new `RequestIdGuard` with the provided concurrency limit.
	pub fn new(limit: usize, id_kind: IdKind) -> Self {
		Self { current_pending: Arc::new(()), max_concurrent_requests: limit, current_id: AtomicU64::new(0), id_kind }
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
		let id = self.id_kind.into_id(self.current_id.fetch_add(1, Ordering::SeqCst));
		Ok(RequestIdGuard { _rc: rc, id })
	}

	/// Attempts to get the `n` number next IDs that only counts as one request.
	///
	/// Fails if request limit has been exceeded.
	pub fn next_request_ids(&self, len: usize) -> Result<RequestIdGuard<Vec<Id<'static>>>, Error> {
		let rc = self.get_slot()?;
		let mut ids = Vec::with_capacity(len);
		for _ in 0..len {
			let id = self.id_kind.into_id(self.current_id.fetch_add(1, Ordering::SeqCst));
			ids.push(id);
		}
		Ok(RequestIdGuard { _rc: rc, id: ids })
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
	fn into_id(self, id: u64) -> Id<'static> {
		match self {
			IdKind::Number => Id::Number(id),
			IdKind::String => Id::Str(format!("{}", id).into()),
		}
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
			let _second = manager.next_request_ids(13).unwrap();
			assert!(manager.next_request_id().is_err());
			// second dropped here.
		}

		assert!(manager.next_request_id().is_ok());
	}
}
