//! Handles and monitors JSONRPC v2 method calls, notification and  subscriptions
//!

use crate::types::error::Error;
use crate::types::jsonrpc::{self, JsonValue};
use futures::channel::{mpsc, oneshot};
use std::collections::{HashMap, HashSet, VecDeque};

/// Indicates the status of a given request/response.
pub enum RequestStatus {
	PendingMethodCall,
	PendingSubscription,
	Subscription,
	Invalid,
}

type RequestCallback = oneshot::Sender<Result<JsonValue, Error>>;
type PendingSubscriptionCallback = oneshot::Sender<Result<(mpsc::Receiver<JsonValue>, RequestId), Error>>;
type SubscriptionCallback = mpsc::Sender<JsonValue>;
type Unsubscribe = String;
type RequestId = u64;

/// Manages and monitors JSONRPC v2 method calls and subscriptions.
pub struct RequestManager {
	/// List of requests that are waiting for a response from the server.
	pending_requests: HashMap<RequestId, RequestCallback>,
	/// List of subscription requests that have been sent to the server, with the method name to
	/// unsubscribe.
	pending_subscriptions: HashMap<RequestId, (PendingSubscriptionCallback, Unsubscribe)>,
	/// List of subscription that are active on the server, with the method name to unsubscribe.
	active_subscriptions: HashMap<RequestId, (SubscriptionCallback, Unsubscribe)>,
	/// Responses that have been received but not yet propagated the frontend.
	waiting_responses: VecDeque<(RequestId, String)>,
}

impl RequestManager {
	pub fn new() -> Self {
		Self {
			pending_requests: HashMap::new(),
			pending_subscriptions: HashMap::new(),
			active_subscriptions: HashMap::new(),
			waiting_responses: VecDeque::new(),
		}
	}

	/// Inserts a new pending request, fails if the request_id was already registred.
	pub fn insert_pending_request(&mut self, id: u64, callback: RequestCallback) -> Result<(), ()> {
		assert!(!self.pending_requests.contains_key(&id) && !self.pending_subscriptions.contains_key(&id));
		assert!(!self.pending_requests.contains_key(&id) && !self.active_subscriptions.contains_key(&id));
		assert!(!self.pending_subscriptions.contains_key(&id) && !self.active_subscriptions.contains_key(&id));

		if self.pending_requests.insert(id, callback).is_none() {
			Ok(())
		} else {
			Err(())
		}
	}

	/// Inserts a new pending request, fails if the request_id was already registred.
	pub fn insert_pending_subscription(
		&mut self,
		id: u64,
		callback: PendingSubscriptionCallback,
		unsubscribe_method: String,
	) -> Result<(), ()> {
		assert!(!self.pending_requests.contains_key(&id) && !self.pending_subscriptions.contains_key(&id));
		assert!(!self.pending_requests.contains_key(&id) && !self.active_subscriptions.contains_key(&id));
		assert!(!self.pending_subscriptions.contains_key(&id) && !self.active_subscriptions.contains_key(&id));

		if self.pending_subscriptions.insert(id, (callback, unsubscribe_method)).is_none() {
			Ok(())
		} else {
			Err(())
		}
	}

	/// Inserts a new pending request, fails if the request_id was already registred.
	pub fn insert_active_subscription(
		&mut self,
		id: u64,
		callback: SubscriptionCallback,
		unsubscribe_method: String,
	) -> Result<(), ()> {
		assert!(!self.pending_requests.contains_key(&id) && !self.pending_subscriptions.contains_key(&id));
		assert!(!self.pending_requests.contains_key(&id) && !self.active_subscriptions.contains_key(&id));
		assert!(!self.pending_subscriptions.contains_key(&id) && !self.active_subscriptions.contains_key(&id));

		if self.active_subscriptions.insert(id, (callback, unsubscribe_method)).is_none() {
			Ok(())
		} else {
			Err(())
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

	pub fn as_subscription(&mut self, id: &RequestId) -> Option<&mut SubscriptionCallback> {
		self.active_subscriptions.get_mut(id).map(|(callback, _)| callback)
	}

	pub fn try_complete_method_call(&mut self, id: &RequestId) -> Result<RequestCallback, ()> {
		self.pending_requests.remove(id).ok_or(())
	}

	pub fn try_complete_pending_subscription(
		&mut self,
		id: &RequestId,
	) -> Result<(PendingSubscriptionCallback, Unsubscribe), ()> {
		self.pending_subscriptions.remove(id).ok_or(())
	}
}
