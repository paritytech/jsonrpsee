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

//! Handles and monitors JSONRPC v2 method calls and subscriptions
//!
//! Definitions:
//!
//!    - RequestId: request ID in the JSONRPC-v2 specification
//!    > **Note**: The spec allow number, string or null but this crate only supports numbers.
//!    - SubscriptionId: unique ID generated by server

use std::{
	collections::{hash_map::Entry, HashMap},
	ops::Range,
};

use crate::{client::BatchEntry, Error};
use futures_channel::{mpsc, oneshot};
use jsonrpsee_types::{ErrorResponse, Id, Response, SubscriptionId};
use rustc_hash::FxHashMap;
use serde_json::value::Value as JsonValue;

#[derive(Debug)]
enum Kind {
	PendingMethodCall(PendingCallOneshot),
	PendingSubscription((RequestId, PendingSubscriptionOneshot, UnsubscribeMethod)),
	Subscription((RequestId, SubscriptionSink, UnsubscribeMethod)),
}

#[derive(Debug)]
/// Indicates the status of a given request/response.
pub(crate) enum RequestStatus {
	/// The method call is waiting for a response,
	PendingMethodCall,
	/// The subscription is waiting for a response to become an active subscription.
	PendingSubscription,
	/// An active subscription.
	Subscription,
	/// Invalid request ID.
	Invalid,
}

type PendingCallOneshot = Option<oneshot::Sender<Result<JsonValue, Error>>>;
type PendingBatchOneshot = oneshot::Sender<Result<Vec<BatchEntry<'static, JsonValue>>, Error>>;
type PendingSubscriptionOneshot = oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, SubscriptionId<'static>), Error>>;
type SubscriptionSink = mpsc::Sender<JsonValue>;
type UnsubscribeMethod = String;
type RequestId = Id<'static>;

#[derive(Debug)]
/// Batch state.
pub(crate) struct BatchState {
	/// Oneshot send back.
	pub(crate) send_back: PendingBatchOneshot,
}

#[derive(Debug, Default)]
/// Manages and monitors JSONRPC v2 method calls and subscriptions.
pub(crate) struct RequestManager {
	/// List of requests that are waiting for a response from the server.
	// NOTE: FnvHashMap is used here because RequestId is not under the caller's control and is known to be a short
	// key.
	requests: FxHashMap<RequestId, Kind>,
	/// Reverse lookup, to find a request ID in constant time by `subscription ID` instead of looking through all
	/// requests.
	subscriptions: HashMap<SubscriptionId<'static>, RequestId>,
	/// Pending batch requests.
	batches: FxHashMap<Range<u64>, BatchState>,
	/// Registered Methods for incoming notifications.
	notification_handlers: HashMap<String, SubscriptionSink>,
}

impl RequestManager {
	/// Create a new `RequestManager`.
	pub(crate) fn new() -> Self {
		Self::default()
	}

	/// Tries to insert a new pending request.
	///
	/// Returns `Ok` if the pending request was successfully inserted otherwise `Err`.
	pub(crate) fn insert_pending_call(
		&mut self,
		id: RequestId,
		send_back: PendingCallOneshot,
	) -> Result<(), PendingCallOneshot> {
		if let Entry::Vacant(v) = self.requests.entry(id) {
			v.insert(Kind::PendingMethodCall(send_back));
			Ok(())
		} else {
			Err(send_back)
		}
	}

	/// Tries to insert a new batch request.
	///
	/// Returns `Ok` if the pending request was successfully inserted otherwise `Err`.
	pub(crate) fn insert_pending_batch(
		&mut self,
		batch: Range<u64>,
		send_back: PendingBatchOneshot,
	) -> Result<(), PendingBatchOneshot> {
		if let Entry::Vacant(v) = self.batches.entry(batch) {
			v.insert(BatchState { send_back });
			Ok(())
		} else {
			Err(send_back)
		}
	}

	/// Tries to insert a new pending subscription and reserves a slot for a "potential" unsubscription request.
	///
	/// Returns `Ok` if the pending request was successfully inserted otherwise `Err`.
	pub(crate) fn insert_pending_subscription(
		&mut self,
		sub_req_id: RequestId,
		unsub_req_id: RequestId,
		send_back: PendingSubscriptionOneshot,
		unsubscribe_method: UnsubscribeMethod,
	) -> Result<(), PendingSubscriptionOneshot> {
		// The request IDs are not in the manager and the `sub_id` and `unsub_id` are not equal.
		if !self.requests.contains_key(&sub_req_id)
			&& !self.requests.contains_key(&unsub_req_id)
			&& sub_req_id != unsub_req_id
		{
			self.requests
				.insert(sub_req_id, Kind::PendingSubscription((unsub_req_id.clone(), send_back, unsubscribe_method)));
			self.requests.insert(unsub_req_id, Kind::PendingMethodCall(None));
			Ok(())
		} else {
			Err(send_back)
		}
	}

	/// Tries to insert a new subscription.
	///
	/// Returns `Ok` if the pending request was successfully inserted otherwise `Err`.
	pub(crate) fn insert_subscription(
		&mut self,
		sub_req_id: RequestId,
		unsub_req_id: RequestId,
		subscription_id: SubscriptionId<'static>,
		send_back: SubscriptionSink,
		unsubscribe_method: UnsubscribeMethod,
	) -> Result<(), SubscriptionSink> {
		if let (Entry::Vacant(request), Entry::Vacant(subscription)) =
			(self.requests.entry(sub_req_id.clone()), self.subscriptions.entry(subscription_id))
		{
			request.insert(Kind::Subscription((unsub_req_id, send_back, unsubscribe_method)));
			subscription.insert(sub_req_id);
			Ok(())
		} else {
			Err(send_back)
		}
	}

	/// Inserts a handler for incoming notifications.
	pub(crate) fn insert_notification_handler(
		&mut self,
		method: &str,
		send_back: SubscriptionSink,
	) -> Result<(), Error> {
		if let Entry::Vacant(handle) = self.notification_handlers.entry(method.to_owned()) {
			handle.insert(send_back);
			Ok(())
		} else {
			Err(Error::MethodAlreadyRegistered(method.to_owned()))
		}
	}

	/// Removes a notification handler.
	pub(crate) fn remove_notification_handler(&mut self, method: String) -> Result<(), Error> {
		if self.notification_handlers.remove(&method).is_some() {
			Ok(())
		} else {
			Err(Error::UnregisteredNotification(method))
		}
	}

	/// Tries to complete a pending subscription.
	///
	/// Returns `Some` if the subscription was completed otherwise `None`.
	pub(crate) fn complete_pending_subscription(
		&mut self,
		request_id: RequestId,
	) -> Option<(RequestId, PendingSubscriptionOneshot, UnsubscribeMethod)> {
		match self.requests.entry(request_id) {
			Entry::Occupied(request) if matches!(request.get(), Kind::PendingSubscription(_)) => {
				let (_req_id, kind) = request.remove_entry();
				if let Kind::PendingSubscription(send_back) = kind {
					Some(send_back)
				} else {
					unreachable!("Pending subscription is Pending subscription checked above; qed");
				}
			}
			_ => None,
		}
	}

	/// Tries to complete a pending batch request.
	///
	/// Returns `Some` if the subscription was completed otherwise `None`.
	pub(crate) fn complete_pending_batch(&mut self, batch: Range<u64>) -> Option<BatchState> {
		match self.batches.entry(batch) {
			Entry::Occupied(request) => {
				let (_digest, state) = request.remove_entry();
				Some(state)
			}
			_ => None,
		}
	}

	/// Tries to complete a pending call.
	///
	/// Returns `Some` if the call was completed otherwise `None`.
	pub(crate) fn complete_pending_call(&mut self, request_id: RequestId) -> Option<PendingCallOneshot> {
		match self.requests.entry(request_id) {
			Entry::Occupied(request) if matches!(request.get(), Kind::PendingMethodCall(_)) => {
				let (_req_id, kind) = request.remove_entry();
				if let Kind::PendingMethodCall(send_back) = kind {
					Some(send_back)
				} else {
					unreachable!("Pending call is Pending call checked above; qed");
				}
			}
			_ => None,
		}
	}

	/// Tries to remove a subscription.
	///
	/// Returns `Some` if the subscription was removed otherwise `None`.
	pub(crate) fn remove_subscription(
		&mut self,
		request_id: RequestId,
		subscription_id: SubscriptionId<'static>,
	) -> Option<(RequestId, SubscriptionSink, UnsubscribeMethod, SubscriptionId)> {
		match (self.requests.entry(request_id), self.subscriptions.entry(subscription_id)) {
			(Entry::Occupied(request), Entry::Occupied(subscription))
				if matches!(request.get(), Kind::Subscription(_)) =>
			{
				let (_req_id, kind) = request.remove_entry();
				let (sub_id, _req_id) = subscription.remove_entry();
				if let Kind::Subscription((unsub_req_id, send_back, unsub)) = kind {
					Some((unsub_req_id, send_back, unsub, sub_id))
				} else {
					unreachable!("Subscription is Subscription checked above; qed");
				}
			}
			_ => None,
		}
	}

	/// Returns the status of a request ID
	pub(crate) fn request_status(&mut self, id: &RequestId) -> RequestStatus {
		self.requests.get(id).map_or(RequestStatus::Invalid, |kind| match kind {
			Kind::PendingMethodCall(_) => RequestStatus::PendingMethodCall,
			Kind::PendingSubscription(_) => RequestStatus::PendingSubscription,
			Kind::Subscription(_) => RequestStatus::Subscription,
		})
	}

	/// Get a mutable reference to underlying `Sink` in order to send messages to the subscription.
	///
	/// Returns `Some` if the `request_id` was registered as a subscription otherwise `None`.
	pub(crate) fn as_subscription_mut(&mut self, request_id: &RequestId) -> Option<&mut SubscriptionSink> {
		if let Some(Kind::Subscription((_, sink, _))) = self.requests.get_mut(request_id) {
			Some(sink)
		} else {
			None
		}
	}

	/// Get a mutable reference to underlying `Sink` in order to send incoming notifications to the subscription.
	///
	/// Returns `Some` if the `method` was registered as a NotificationHandler otherwise `None`.
	pub(crate) fn as_notification_handler_mut(&mut self, method: String) -> Option<&mut SubscriptionSink> {
		self.notification_handlers.get_mut(&method)
	}

	/// Reverse lookup to get the request ID for a subscription ID.
	///
	/// Returns `Some` if the subscription ID was registered as a subscription otherwise `None`.
	pub(crate) fn get_request_id_by_subscription_id(&self, sub_id: &SubscriptionId) -> Option<RequestId> {
		self.subscriptions.get(sub_id).map(|id| id.clone().into_owned())
	}
}

#[cfg(test)]
mod tests {
	use super::{Error, RequestManager};
	use futures_channel::{mpsc, oneshot};
	use jsonrpsee_types::{Id, SubscriptionId};
	use serde_json::Value as JsonValue;

	#[test]
	fn insert_remove_pending_request_works() {
		let (request_tx, _) = oneshot::channel::<Result<JsonValue, Error>>();

		let mut manager = RequestManager::new();
		assert!(manager.insert_pending_call(Id::Number(0), Some(request_tx)).is_ok());
		assert!(manager.complete_pending_call(Id::Number(0)).is_some());
	}

	#[test]
	fn insert_remove_subscription_works() {
		let (pending_sub_tx, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (sub_tx, _) = mpsc::channel::<JsonValue>(1);
		let mut manager = RequestManager::new();
		assert!(manager
			.insert_pending_subscription(Id::Number(1), Id::Number(2), pending_sub_tx, "unsubscribe_method".into())
			.is_ok());
		let (unsub_req_id, _send_back_oneshot, unsubscribe_method) =
			manager.complete_pending_subscription(Id::Number(1)).unwrap();
		assert_eq!(unsub_req_id, Id::Number(2));
		assert!(manager
			.insert_subscription(
				Id::Number(1),
				Id::Number(2),
				SubscriptionId::Str("uniq_id_from_server".into()),
				sub_tx,
				unsubscribe_method
			)
			.is_ok());

		assert!(manager.as_subscription_mut(&Id::Number(1)).is_some());
		assert!(manager
			.remove_subscription(Id::Number(1), SubscriptionId::Str("uniq_id_from_server".into()))
			.is_some());
	}

	#[test]
	fn insert_subscription_with_same_sub_and_unsub_id_should_err() {
		let (tx1, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (tx2, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (tx3, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (tx4, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let mut manager = RequestManager::new();
		assert!(manager
			.insert_pending_subscription(Id::Str("1".into()), Id::Str("1".into()), tx1, "unsubscribe_method".into())
			.is_err());
		assert!(manager
			.insert_pending_subscription(Id::Str("0".into()), Id::Str("1".into()), tx2, "unsubscribe_method".into())
			.is_ok());
		assert!(
			manager
				.insert_pending_subscription(
					Id::Str("99".into()),
					Id::Str("0".into()),
					tx3,
					"unsubscribe_method".into()
				)
				.is_err(),
			"unsub request ID already occupied"
		);
		assert!(
			manager
				.insert_pending_subscription(
					Id::Str("99".into()),
					Id::Str("1".into()),
					tx4,
					"unsubscribe_method".into()
				)
				.is_err(),
			"sub request ID already occupied"
		);
	}

	#[test]
	fn pending_method_call_faulty() {
		let (request_tx1, _) = oneshot::channel::<Result<JsonValue, Error>>();
		let (request_tx2, _) = oneshot::channel::<Result<JsonValue, Error>>();
		let (pending_sub_tx, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (sub_tx, _) = mpsc::channel::<JsonValue>(1);

		let mut manager = RequestManager::new();
		assert!(manager.insert_pending_call(Id::Number(0), Some(request_tx1)).is_ok());
		assert!(manager.insert_pending_call(Id::Number(0), Some(request_tx2)).is_err());
		assert!(manager
			.insert_pending_subscription(Id::Number(0), Id::Number(1), pending_sub_tx, "beef".to_string())
			.is_err());
		assert!(manager
			.insert_subscription(
				Id::Number(0),
				Id::Number(99),
				SubscriptionId::Num(137),
				sub_tx,
				"bibimbap".to_string()
			)
			.is_err());

		assert!(manager.remove_subscription(Id::Number(0), SubscriptionId::Num(137)).is_none());
		assert!(manager.complete_pending_subscription(Id::Number(0)).is_none());
		assert!(manager.complete_pending_call(Id::Number(0)).is_some());
	}

	#[test]
	fn pending_subscription_faulty() {
		let (request_tx, _) = oneshot::channel::<Result<JsonValue, Error>>();
		let (pending_sub_tx1, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (pending_sub_tx2, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (sub_tx, _) = mpsc::channel::<JsonValue>(1);

		let mut manager = RequestManager::new();
		assert!(manager
			.insert_pending_subscription(Id::Number(99), Id::Number(100), pending_sub_tx1, "beef".to_string())
			.is_ok());
		assert!(manager.insert_pending_call(Id::Number(99), Some(request_tx)).is_err());
		assert!(manager
			.insert_pending_subscription(Id::Number(99), Id::Number(1337), pending_sub_tx2, "vegan".to_string())
			.is_err());

		assert!(manager
			.insert_subscription(
				Id::Number(99),
				Id::Number(100),
				SubscriptionId::Num(0),
				sub_tx,
				"bibimbap".to_string()
			)
			.is_err());

		assert!(manager.remove_subscription(Id::Number(99), SubscriptionId::Num(0)).is_none());
		assert!(manager.complete_pending_call(Id::Number(99)).is_none());
		assert!(manager.complete_pending_subscription(Id::Number(99)).is_some());
	}

	#[test]
	fn active_subscriptions_faulty() {
		let (request_tx, _) = oneshot::channel::<Result<JsonValue, Error>>();
		let (pending_sub_tx, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (sub_tx1, _) = mpsc::channel::<JsonValue>(1);
		let (sub_tx2, _) = mpsc::channel::<JsonValue>(1);

		let mut manager = RequestManager::new();

		assert!(manager
			.insert_subscription(Id::Number(3), Id::Number(4), SubscriptionId::Num(0), sub_tx1, "bibimbap".to_string())
			.is_ok());
		assert!(manager
			.insert_subscription(Id::Number(3), Id::Number(4), SubscriptionId::Num(1), sub_tx2, "bibimbap".to_string())
			.is_err());
		assert!(manager
			.insert_pending_subscription(Id::Number(3), Id::Number(4), pending_sub_tx, "beef".to_string())
			.is_err());
		assert!(manager.insert_pending_call(Id::Number(3), Some(request_tx)).is_err());

		assert!(manager.remove_subscription(Id::Number(3), SubscriptionId::Num(7)).is_none());
		assert!(manager.complete_pending_call(Id::Number(3)).is_none());
		assert!(manager.complete_pending_subscription(Id::Number(3)).is_none());
		assert!(manager.remove_subscription(Id::Number(3), SubscriptionId::Num(1)).is_none());
		assert!(manager.remove_subscription(Id::Number(3), SubscriptionId::Num(0)).is_some());
	}
}
