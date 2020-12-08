//! Handles and monitors JSONRPC v2 method calls and subscriptions
//!
//! Definitions:
//! 	* Request ID - request ID in JSONRPC-v2 specification (may be number, string or null but only numbers are supported currently)
//! 	* Subscription ID - unique ID generated by server
//!
//!

use crate::types::error::Error;
use crate::types::jsonrpc::JsonValue;
use fnv::FnvHashMap;
use futures::channel::{mpsc, oneshot};
use std::collections::HashMap;

/// Indicates the status of a given request/response.
pub enum RequestStatus {
	PendingMethodCall,
	PendingSubscription,
	Subscription,
	Invalid,
}

type RequestCallback = oneshot::Sender<Result<JsonValue, Error>>;
type PendingSubscriptionCallback = oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, String), Error>>;
type SubscriptionCallback = mpsc::Sender<JsonValue>;
type Unsubscribe = String;
type RequestId = u64;

/// Manages and monitors JSONRPC v2 method calls and subscriptions.
pub struct RequestManager {
	/// List of requests that are waiting for a response from the server.
	pending_requests: FnvHashMap<RequestId, RequestCallback>,
	/// List of subscription requests that have been sent to the server.
	pending_subscriptions: FnvHashMap<RequestId, (PendingSubscriptionCallback, Unsubscribe)>,
	/// List of subscriptions that are active on the server.
	active_subscriptions: FnvHashMap<RequestId, (SubscriptionCallback, Unsubscribe)>,
	/// Unique subscription ID received from the server.
	subscriptions: HashMap<String, RequestId>,
}

impl RequestManager {
	pub fn new() -> Self {
		Self {
			pending_requests: FnvHashMap::default(),
			pending_subscriptions: FnvHashMap::default(),
			active_subscriptions: FnvHashMap::default(),
			subscriptions: HashMap::new(),
		}
	}

	fn contains(&self, id: &RequestId) -> bool {
		self.pending_requests.contains_key(id)
			|| self.pending_subscriptions.contains_key(id)
			|| self.active_subscriptions.contains_key(id)
	}

	/// Tries to insert a new pending request.
	pub fn insert_pending_request(&mut self, id: u64, callback: RequestCallback) -> bool {
		if !self.contains(&id) {
			self.pending_requests.insert(id, callback);
			true
		} else {
			false
		}
	}

	/// Tries to inserts a new pending subscription.
	pub fn insert_pending_subscription(
		&mut self,
		id: RequestId,
		callback: PendingSubscriptionCallback,
		unsubscribe_method: Unsubscribe,
	) -> bool {
		if !self.contains(&id) {
			self.pending_subscriptions.insert(id, (callback, unsubscribe_method));
			true
		} else {
			false
		}
	}

	/// Inserts a new active subscription
	pub fn insert_active_subscription(
		&mut self,
		request_id: u64,
		subscription_id: String,
		callback: SubscriptionCallback,
		unsubscribe_method: String,
	) -> bool {
		if !self.contains(&request_id) && !self.subscriptions.contains_key(&subscription_id) {
			self.subscriptions.insert(subscription_id, request_id);
			self.active_subscriptions.insert(request_id, (callback, unsubscribe_method));
			true
		} else {
			false
		}
	}

	/// Remove an active subscription
	pub fn remove_active_subscription(
		&mut self,
		request_id: &RequestId,
		subscription_id: &String,
	) -> Option<(SubscriptionCallback, Unsubscribe)> {
		let res1 = self.active_subscriptions.remove(&request_id);
		let res2 = self.subscriptions.remove(subscription_id);

		match (res1, res2) {
			(Some(unsubscribe), Some(_)) => Some(unsubscribe),
			_ => None,
		}
	}

	/// Returns the status of a request ID
	pub fn request_status(&self, id: &RequestId) -> RequestStatus {
		if self.pending_requests.contains_key(id) {
			RequestStatus::PendingMethodCall
		} else if self.pending_subscriptions.contains_key(id) {
			RequestStatus::PendingSubscription
		} else if self.active_subscriptions.contains_key(id) {
			RequestStatus::Subscription
		} else {
			RequestStatus::Invalid
		}
	}

	pub fn try_complete_method_call(&mut self, id: &RequestId) -> Option<RequestCallback> {
		self.pending_requests.remove(id)
	}

	pub fn try_complete_pending_subscription(
		&mut self,
		id: &RequestId,
	) -> Option<(PendingSubscriptionCallback, Unsubscribe)> {
		self.pending_subscriptions.remove(id)
	}

	pub fn get_request_id(&self, unique_id: &String) -> Option<RequestId> {
		self.subscriptions.get(unique_id).copied()
	}

	pub fn as_active_subscription_mut(&mut self, id: &RequestId) -> Option<&mut SubscriptionCallback> {
		self.active_subscriptions.get_mut(id).map(|(cb, _)| cb)
	}
}

#[cfg(test)]
mod tests {
	use super::{Error, RequestManager};
	use crate::types::jsonrpc::JsonValue;
	use futures::channel::{mpsc, oneshot};

	#[test]
	fn it_works() {
		let (request_tx, _) = oneshot::channel::<Result<JsonValue, Error>>();
		let (pending_sub_tx, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, String), Error>>();
		let (sub_tx, _) = mpsc::channel::<JsonValue>(1);

		let mut manager = RequestManager::new();
		assert_eq!(true, manager.insert_pending_request(0, request_tx));
		assert_eq!(true, manager.insert_pending_subscription(1, pending_sub_tx, "unsubscribe_method".into()));

		let _callback = manager.try_complete_method_call(&0).unwrap();
		let (_callback, unsubscribe_method) = manager.try_complete_pending_subscription(&1).unwrap();
		assert_eq!(
			true,
			manager.insert_active_subscription(1, "uniq_id_from_server".to_string(), sub_tx, unsubscribe_method)
		);
		manager.remove_active_subscription(&1, &"uniq_id_from_server".to_string()).unwrap();
	}

	#[test]
	fn same_request_id_registrered_more_once_should_not_work() {
		let (request_tx, _) = oneshot::channel::<Result<JsonValue, Error>>();
		let (pending_sub_tx, _) = oneshot::channel::<Result<(mpsc::Receiver<JsonValue>, String), Error>>();
		let (sub_tx, _) = mpsc::channel::<JsonValue>(1);

		let mut manager = RequestManager::new();
		assert_eq!(true, manager.insert_pending_request(99, request_tx));
		assert_eq!(false, manager.insert_pending_subscription(99, pending_sub_tx, "unsubscribe_method".into()));
		assert_eq!(
			false,
			manager.insert_active_subscription(
				99,
				"uniq_id_from_server".to_string(),
				sub_tx,
				"unsubscribe_method".into()
			)
		);
	}
}
