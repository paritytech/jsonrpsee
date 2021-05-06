//! Handles and monitors JSONRPC v2 method calls and subscriptions
//!
//! Definitions:
//!
//!    - RequestId: request ID in the JSONRPC-v2 specification
//!    > **Note**: The spec allow number, string or null but this crate only supports numbers.
//!    - SubscriptionId: unique ID generated by server

use fnv::FnvHashMap;
use futures::channel::{mpsc, oneshot};
use jsonrpsee_types::{v2::params::SubscriptionId, Error, JsonValue};
use std::collections::hash_map::{Entry, HashMap};

#[derive(Debug)]
enum Kind {
	PendingMethodCall(PendingCallOneshot),
	PendingSubscription((RequestId, PendingSubscriptionOneshot, UnsubscribeMethod)),
	Subscription((RequestId, SubscriptionSink, UnsubscribeMethod)),
	NotificationHandler(SubscriptionSink),
}

#[derive(Debug)]
/// Indicates the status of a given request/response.
pub enum RequestStatus {
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
type PendingBatchOneshot = oneshot::Sender<Result<Vec<JsonValue>, Error>>;
type PendingSubscriptionOneshot = oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>;
type SubscriptionSink = mpsc::Sender<JsonValue>;
type UnsubscribeMethod = String;
type RequestId = u64;

#[derive(Debug)]
/// Batch state.
pub struct BatchState {
	/// Order that the request was performed in.
	pub order: FnvHashMap<RequestId, usize>,
	/// Oneshot send back.
	pub send_back: PendingBatchOneshot,
}

#[derive(Debug, Default)]
/// Manages and monitors JSONRPC v2 method calls and subscriptions.
pub struct RequestManager {
	/// List of requests that are waiting for a response from the server.
	// NOTE: FnvHashMap is used here because RequestId is not under the caller's control and is known to be a short key.
	requests: FnvHashMap<RequestId, Kind>,
	/// Reverse lookup, to find a request ID in constant time by `subscription ID` instead of looking through all requests.
	subscriptions: HashMap<SubscriptionId, RequestId>,
	/// Pending batch requests
	batches: FnvHashMap<Vec<RequestId>, BatchState>,
}

impl RequestManager {
	/// Create a new `RequestManager`.
	pub fn new() -> Self {
		Self::default()
	}

	/// Tries to insert a new pending call.
	///
	/// Returns `Ok` if the pending request was successfully inserted otherwise `Err`.
	pub fn insert_pending_call(
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

	/// Tries to insert a new batch request
	///
	/// Returns `Ok` if the pending request was successfully inserted otherwise `Err`.
	pub fn insert_pending_batch(
		&mut self,
		mut batch: Vec<RequestId>,
		send_back: PendingBatchOneshot,
	) -> Result<(), PendingBatchOneshot> {
		let mut order = FnvHashMap::with_capacity_and_hasher(batch.len(), Default::default());
		for (idx, batch_id) in batch.iter().enumerate() {
			order.insert(*batch_id, idx);
		}
		batch.sort_unstable();
		if let Entry::Vacant(v) = self.batches.entry(batch) {
			v.insert(BatchState { order, send_back });
			Ok(())
		} else {
			Err(send_back)
		}
	}
	/// Tries to insert a new pending subscription and reserves a slot for a "potential" unsubscription request.
	///
	/// Returns `Ok` if the pending request was successfully inserted otherwise `Err`.
	pub fn insert_pending_subscription(
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
			self.requests.insert(sub_req_id, Kind::PendingSubscription((unsub_req_id, send_back, unsubscribe_method)));
			self.requests.insert(unsub_req_id, Kind::PendingMethodCall(None));
			Ok(())
		} else {
			Err(send_back)
		}
	}

	/// Tries to insert a new subscription.
	///
	/// Returns `Ok` if the pending request was successfully inserted otherwise `Err`.
	pub fn insert_subscription(
		&mut self,
		sub_req_id: RequestId,
		unsub_req_id: RequestId,
		subscription_id: SubscriptionId,
		send_back: SubscriptionSink,
		unsubscribe_method: UnsubscribeMethod,
	) -> Result<(), SubscriptionSink> {
		if let (Entry::Vacant(request), Entry::Vacant(subscription)) =
			(self.requests.entry(sub_req_id), self.subscriptions.entry(subscription_id))
		{
			request.insert(Kind::Subscription((unsub_req_id, send_back, unsubscribe_method)));
			subscription.insert(sub_req_id);
			Ok(())
		} else {
			Err(send_back)
		}
	}

	/// Inserts a subscription for handling incoming notifications
	pub fn insert_notification_handler(
		&mut self,
		sub_req_id: RequestId,
		subscription_id: SubscriptionId,
		send_back: SubscriptionSink,
	) -> Result<(), Error> {
		if let (Entry::Vacant(request), Entry::Vacant(subscription)) =
			(self.requests.entry(sub_req_id), self.subscriptions.entry(subscription_id))
		{
			request.insert(Kind::NotificationHandler(send_back));
			subscription.insert(sub_req_id);
			Ok(())
		} else {
			Err(Error::InvalidRequestId)
		}
	}

	/// Tries to complete a pending subscription.
	///
	/// Returns `Some` if the subscription was completed otherwise `None`.
	pub fn complete_pending_subscription(
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

	/// Tries to complete a pending batch request
	///
	/// Returns `Some` if the subscription was completed otherwise `None`.
	pub fn complete_pending_batch(&mut self, batch: Vec<RequestId>) -> Option<BatchState> {
		match self.batches.entry(batch) {
			Entry::Occupied(request) => {
				let (_digest, state) = request.remove_entry();
				Some(state)
			}
			_ => None,
		}
	}

	/// Tries to complete a pending call..
	///
	/// Returns `Some` if the call was completed otherwise `None`.
	pub fn complete_pending_call(&mut self, request_id: RequestId) -> Option<PendingCallOneshot> {
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
	pub fn remove_subscription(
		&mut self,
		request_id: RequestId,
		subscription_id: SubscriptionId,
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
	pub fn request_status(&mut self, id: &RequestId) -> RequestStatus {
		self.requests.get(id).map_or(RequestStatus::Invalid, |kind| match kind {
			Kind::PendingMethodCall(_) => RequestStatus::PendingMethodCall,
			Kind::PendingSubscription(_) => RequestStatus::PendingSubscription,
			Kind::Subscription(_) => RequestStatus::Subscription,
			Kind::NotificationHandler(_) => RequestStatus::Subscription,
		})
	}

	/// Get a mutable reference to underlying `Sink` in order to send messages to the subscription.
	///
	/// Returns `Some` if the `request_id` was registered as a subscription otherwise `None`.
	pub fn as_subscription_mut(&mut self, request_id: &RequestId) -> Option<&mut SubscriptionSink> {
		if let Some(Kind::Subscription((_, sink, _))) = self.requests.get_mut(request_id) {
			Some(sink)
		} else {
			None
		}
	}

	/// Get a mutable reference to underlying `Sink` in order to send incmoing notifications to the subscription.
	///
	/// Returns `Some` if the `request_id` was registered as a NotificationHandler otherwise `None`.
	pub fn as_notification_handler_mut(&mut self, request_id: &RequestId) -> Option<&mut SubscriptionSink> {
		if let Some(Kind::NotificationHandler(sink)) = self.requests.get_mut(request_id) {
			Some(sink)
		} else {
			None
		}
	}

	/// Reverse lookup to get the request ID for a subscription ID.
	///
	/// Returns `Some` if the subscription ID was registered as a subscription otherwise `None`.
	pub fn get_request_id_by_subscription_id(&self, sub_id: &SubscriptionId) -> Option<RequestId> {
		self.subscriptions.get(sub_id).copied()
	}
}

#[cfg(test)]
mod tests {
	use super::{Error, RequestManager};
	use futures::channel::{mpsc, oneshot};
	use jsonrpsee_types::v2::params::SubscriptionId;
	use serde_json::Value as JsonValue;

	#[test]
	fn insert_remove_pending_request_works() {
		let (request_tx, _) = oneshot::channel::<Result<JsonValue, Error>>();

		let mut manager = RequestManager::new();
		assert!(manager.insert_pending_call(0, Some(request_tx)).is_ok());
		assert!(manager.complete_pending_call(0).is_some());
	}

	#[test]
	fn insert_remove_subscription_works() {
		let (pending_sub_tx, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (sub_tx, _) = mpsc::channel::<JsonValue>(1);
		let mut manager = RequestManager::new();
		assert!(manager.insert_pending_subscription(1, 2, pending_sub_tx, "unsubscribe_method".into()).is_ok());
		let (unsub_req_id, _send_back_oneshot, unsubscribe_method) = manager.complete_pending_subscription(1).unwrap();
		assert_eq!(unsub_req_id, 2);
		assert!(manager
			.insert_subscription(
				1,
				2,
				SubscriptionId::Str("uniq_id_from_server".to_string()),
				sub_tx,
				unsubscribe_method
			)
			.is_ok());

		assert!(manager.as_subscription_mut(&1).is_some());
		assert!(manager.remove_subscription(1, SubscriptionId::Str("uniq_id_from_server".to_string())).is_some());
	}

	#[test]
	fn insert_subscription_with_same_sub_and_unsub_id_should_err() {
		let (tx1, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (tx2, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (tx3, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (tx4, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let mut manager = RequestManager::new();
		assert!(manager.insert_pending_subscription(1, 1, tx1, "unsubscribe_method".into()).is_err());
		assert!(manager.insert_pending_subscription(0, 1, tx2, "unsubscribe_method".into()).is_ok());
		assert!(
			manager.insert_pending_subscription(99, 0, tx3, "unsubscribe_method".into()).is_err(),
			"unsub request ID already occupied"
		);
		assert!(
			manager.insert_pending_subscription(99, 1, tx4, "unsubscribe_method".into()).is_err(),
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
		assert!(manager.insert_pending_call(0, Some(request_tx1)).is_ok());
		assert!(manager.insert_pending_call(0, Some(request_tx2)).is_err());
		assert!(manager.insert_pending_subscription(0, 1, pending_sub_tx, "beef".to_string()).is_err());
		assert!(manager.insert_subscription(0, 99, SubscriptionId::Num(137), sub_tx, "bibimbap".to_string()).is_err());

		assert!(manager.remove_subscription(0, SubscriptionId::Num(137)).is_none());
		assert!(manager.complete_pending_subscription(0).is_none());
		assert!(manager.complete_pending_call(0).is_some());
	}

	#[test]
	fn pending_subscription_faulty() {
		let (request_tx, _) = oneshot::channel::<Result<JsonValue, Error>>();
		let (pending_sub_tx1, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (pending_sub_tx2, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (sub_tx, _) = mpsc::channel::<JsonValue>(1);

		let mut manager = RequestManager::new();
		assert!(manager.insert_pending_subscription(99, 100, pending_sub_tx1, "beef".to_string()).is_ok());
		assert!(manager.insert_pending_call(99, Some(request_tx)).is_err());
		assert!(manager.insert_pending_subscription(99, 1337, pending_sub_tx2, "vegan".to_string()).is_err());

		assert!(manager.insert_subscription(99, 100, SubscriptionId::Num(0), sub_tx, "bibimbap".to_string()).is_err());

		assert!(manager.remove_subscription(99, SubscriptionId::Num(0)).is_none());
		assert!(manager.complete_pending_call(99).is_none());
		assert!(manager.complete_pending_subscription(99).is_some());
	}

	#[test]
	fn active_subscriptions_faulty() {
		let (request_tx, _) = oneshot::channel::<Result<JsonValue, Error>>();
		let (pending_sub_tx, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, SubscriptionId), Error>>();
		let (sub_tx1, _) = mpsc::channel::<JsonValue>(1);
		let (sub_tx2, _) = mpsc::channel::<JsonValue>(1);

		let mut manager = RequestManager::new();

		assert!(manager.insert_subscription(3, 4, SubscriptionId::Num(0), sub_tx1, "bibimbap".to_string()).is_ok());
		assert!(manager.insert_subscription(3, 4, SubscriptionId::Num(1), sub_tx2, "bibimbap".to_string()).is_err());
		assert!(manager.insert_pending_subscription(3, 4, pending_sub_tx, "beef".to_string()).is_err());
		assert!(manager.insert_pending_call(3, Some(request_tx)).is_err());

		assert!(manager.remove_subscription(3, SubscriptionId::Num(7)).is_none());
		assert!(manager.complete_pending_call(3).is_none());
		assert!(manager.complete_pending_subscription(3).is_none());
		assert!(manager.remove_subscription(3, SubscriptionId::Num(1)).is_none());
		assert!(manager.remove_subscription(3, SubscriptionId::Num(0)).is_some());
	}
}
