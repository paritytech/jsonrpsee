// Copyright 2019 Parity Technologies (UK) Ltd.
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

use crate::jsonrpc::{
	self,
	wrapped::{batch, Notification, Params},
};

use alloc::vec::Vec;
use core::fmt;

/// Collection of multiple batches.
///
/// This struct manages the state of the requests that have been received by the server and that
/// are waiting for a response. Due to the batching mechanism in the JSON-RPC protocol, one single
/// message can contain multiple requests and notifications that must all be answered at once.
///
/// # Usage
///
/// - Create a new empty [`BatchesState`] with [`new`](BatchesState::new).
/// - Whenever the server receives a JSON message, call [`inject`](BatchesState::inject).
/// - Call [`next_event`](BatchesState::next_event) in a loop and process the events buffered
/// within the object.
///
/// The [`BatchesState`] also acts as a collection of pending requests, which you can query using
/// [`request_by_id`](BatchesState::request_by_id).
///
pub struct BatchesState<T> {
	/// For each batch, the individual batch's state and the user parameter.
	batches: Vec<Option<(batch::BatchState, T)>>,

	/// Vacant re-usable indices into `batches`. All indices here must point to a `None`.
	vacant: Vec<usize>,
}

/// Event generated by [`next_event`](BatchesState::next_event).
#[derive(Debug)]
pub enum BatchesEvent<'a, T> {
	/// A notification has been extracted from a batch.
	Notification {
		/// Notification in question.
		notification: Notification,
		/// User parameter passed when calling [`inject`](BatchesState::inject).
		user_param: &'a mut T,
	},

	/// A request has been extracted from a batch.
	Request(BatchesRequest<'a, T>),

	/// A batch has gotten all its requests answered and a response is ready to be sent out.
	ReadyToSend {
		/// Response to send out to the JSON-RPC client.
		response: jsonrpc::Response,
		/// User parameter passed when calling [`inject`](BatchesState::inject).
		user_param: T,
	},
}

/// Request within the batches.
pub struct BatchesRequest<'a, T> {
	/// Id of the batch that contains this element.
	batch_id: usize,
	/// Inner reference to a request within a batch.
	request: batch::BatchRequest<'a>,
	/// User parameter passed when calling `inject`.
	user_param: &'a mut T,
}

/// Identifier of a request within a [`BatchesState`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BatchesElemId {
	/// Id of the batch within `BatchesState::batches`.
	batch_id: usize,
	/// Id of the request within the batch.
	request_id: usize,
}

/// Minimal capacity for the `batches` container.
const BATCHES_MIN_CAPACITY: usize = 256;

impl<T> BatchesState<T> {
	/// Creates a new empty `BatchesState`.
	pub fn new() -> BatchesState<T> {
		BatchesState {
			batches: Vec::with_capacity(BATCHES_MIN_CAPACITY),
			vacant: Vec::with_capacity(BATCHES_MIN_CAPACITY),
		}
	}

	/// Processes one step from a batch and returns an event. Returns `None` if there is nothing
	/// to do. After you call `inject`, then this will return `Some` at least once.
	pub fn next_event(&mut self) -> Option<BatchesEvent<T>> {
		// Note that this function has a complexity of `O(n)`, as we iterate over every single
		// batch every single time. This is however the most straight-forward way to implement it,
		// and while better strategies might yield better complexities, it might not actually yield
		// better performances in real-world situations. More brainstorming and benchmarking could
		// get helpful here.

		for (batch_id, entry) in self.batches.iter_mut().enumerate() {
			enum WhatCanWeDo {
				ReadyToRespond,
				Notification(Notification),
				Request(usize),
			}

			let what_can_we_do = {
				// Unwrap the entry, skipping `None`s
				let (batch, _) = match entry {
					Some(entry) => entry,
					None => continue,
				};

				let is_ready_to_respond = batch.is_ready_to_respond();
				match batch.next() {
					None if is_ready_to_respond => WhatCanWeDo::ReadyToRespond,
					None => continue,
					Some(batch::BatchInc::Notification(n)) => WhatCanWeDo::Notification(n),
					Some(batch::BatchInc::Request(inner)) => WhatCanWeDo::Request(inner.id()),
				}
			};

			match what_can_we_do {
				WhatCanWeDo::ReadyToRespond => {
					// Here we take ownership of the entry, leaving `None` in its place, so
					// we also must store the `batch_id` as being now vacant
					self.vacant.push(batch_id);
					let (batch, user_param) = entry.take().expect("entry is checked for `None`s above; qed");

					let response = batch.into_response().expect("is_ready_to_respond returned true; qed");
					if let Some(response) = response {
						return Some(BatchesEvent::ReadyToSend { response, user_param });
					}
				}
				WhatCanWeDo::Notification(notification) => {
					let (_, user_param) = entry.as_mut().expect("entry is checked for `None`s above; qed");

					return Some(BatchesEvent::Notification { notification, user_param });
				}
				WhatCanWeDo::Request(id) => {
					let (batch, user_param) = entry.as_mut().expect("entry is checked for `None`s above; qed");

					return Some(BatchesEvent::Request(BatchesRequest {
						batch_id,
						request: batch.request_by_id(id).unwrap(),
						user_param,
					}));
				}
			}
		}

		None
	}

	/// Injects a newly received batch into the list. You must then call
	/// [`next_event`] in order to process it.
	pub fn inject(&mut self, request: jsonrpc::Request, user_param: T) {
		let batch = batch::BatchState::from_request(request);

		match self.vacant.pop() {
			Some(id) => self.batches[id] = Some((batch, user_param)),
			None => self.batches.push(Some((batch, user_param))),
		}
	}

	/// Returns a list of all user data associated to active batches.
	pub fn batches<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> + 'a {
		self.batches.iter_mut().filter_map(|entry| entry.as_mut().map(|(_, user_param)| user_param))
	}

	/// Returns a request previously returned by [`next_event`](crate::raw::RawServer::next_event)
	/// by its id.
	///
	/// Note that previous notifications don't have an ID and can't be accessed with this method.
	///
	/// Returns `None` if the request ID is invalid or if the request has already been answered in
	/// the past.
	pub fn request_by_id(&mut self, id: BatchesElemId) -> Option<BatchesRequest<T>> {
		if let Some(Some((batch, user_param))) = self.batches.get_mut(id.batch_id) {
			Some(BatchesRequest { batch_id: id.batch_id, request: batch.request_by_id(id.request_id)?, user_param })
		} else {
			None
		}
	}
}

impl<T> Default for BatchesState<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> fmt::Debug for BatchesState<T>
where
	T: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list()
			.entries(self.batches.iter().enumerate().filter_map(|(batch_id, value)| match value {
				Some(value) => Some((batch_id, value)),
				None => None,
			}))
			.finish()
	}
}

impl<'a, T> BatchesRequest<'a, T> {
	/// Returns the id of the request within the [`BatchesState`].
	///
	/// > **Note**: This is NOT the request id that the client passed.
	pub fn id(&self) -> BatchesElemId {
		BatchesElemId { batch_id: self.batch_id, request_id: self.request.id() }
	}

	/// Returns the user parameter passed when calling [`inject`](BatchesState::inject).
	pub fn user_param(&mut self) -> &mut T {
		&mut self.user_param
	}

	/// Returns the id that the client sent out.
	pub fn request_id(&self) -> &jsonrpc::Id {
		self.request.request_id()
	}

	/// Returns the method of this request.
	pub fn method(&self) -> &str {
		self.request.method()
	}

	/// Returns the parameters of the request, as a `jsonrpc::Params`.
	pub fn params(&self) -> Params {
		self.request.params()
	}

	/// Responds to the request. This destroys the request object, meaning you can no longer
	/// retrieve it with [`request_by_id`](BatchesState::request_by_id) later anymore.
	///
	/// A [`ReadyToSend`](BatchesEvent::ReadyToSend) event containing this response might be
	/// generated the next time you call [`next_event`](BatchesState::next_event).
	pub fn set_response(self, response: Result<jsonrpc::JsonValue, jsonrpc::Error>) {
		self.request.set_response(response)
	}
}

impl<'a, T> fmt::Debug for BatchesRequest<'a, T>
where
	T: fmt::Debug,
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("BatchesRequest")
			.field("id", &self.id())
			.field("user_param", &self.user_param)
			.field("request_id", &self.request_id())
			.field("method", &self.method())
			.field("params", &self.params())
			.finish()
	}
}

#[cfg(test)]
mod tests {
	use super::{BatchesEvent, BatchesState};
	use crate::jsonrpc::{self, wrapped::Notification};

	#[test]
	fn basic_notification() {
		let notif = jsonrpc::Notification {
			jsonrpc: jsonrpc::Version::V2,
			method: "foo".to_string(),
			params: jsonrpc::Params::None,
		};

		let mut state = BatchesState::new();
		assert!(state.next_event().is_none());
		state.inject(jsonrpc::Request::Single(jsonrpc::Call::Notification(notif.clone())), ());
		match state.next_event() {
			Some(BatchesEvent::Notification { ref notification, .. }) if *notification == Notification::from(notif) => {
			}
			_ => panic!(),
		}
		assert!(state.next_event().is_none());
	}

	#[test]
	fn basic_request() {
		let call = jsonrpc::MethodCall {
			jsonrpc: jsonrpc::Version::V2,
			method: "foo".to_string(),
			params: jsonrpc::Params::Map(serde_json::from_str("{\"test\":\"foo\"}").unwrap()),
			id: jsonrpc::Id::Num(123),
		};

		let mut state = BatchesState::new();
		assert!(state.next_event().is_none());
		state.inject(jsonrpc::Request::Single(jsonrpc::Call::MethodCall(call)), 8889);

		let rq_id = match state.next_event() {
			Some(BatchesEvent::Request(rq)) => {
				assert_eq!(rq.method(), "foo");
				assert_eq!(
					{
						let v: String = rq.params().get("test").unwrap();
						v
					},
					"foo"
				);
				assert_eq!(rq.request_id(), &jsonrpc::Id::Num(123));
				rq.id()
			}
			_ => panic!(),
		};

		assert!(state.next_event().is_none());

		assert_eq!(state.request_by_id(rq_id).unwrap().method(), "foo");
		state.request_by_id(rq_id).unwrap().set_response(Err(jsonrpc::Error::method_not_found()));
		assert!(state.request_by_id(rq_id).is_none());

		match state.next_event() {
			Some(BatchesEvent::ReadyToSend { response, user_param }) => {
				assert_eq!(user_param, 8889);
				match response {
					jsonrpc::Response::Single(jsonrpc::Output::Failure(f)) => {
						assert_eq!(f.id, jsonrpc::Id::Num(123));
					}
					_ => panic!(),
				}
			}
			_ => panic!(),
		};
	}

	#[test]
	fn empty_batch() {
		let mut state = BatchesState::new();
		assert!(state.next_event().is_none());
		state.inject(jsonrpc::Request::Batch(Vec::new()), ());
		assert!(state.next_event().is_none());
	}

	#[test]
	fn batch_of_notifs() {
		let notif1 = jsonrpc::Notification {
			jsonrpc: jsonrpc::Version::V2,
			method: "foo".to_string(),
			params: jsonrpc::Params::None,
		};

		let notif2 = jsonrpc::Notification {
			jsonrpc: jsonrpc::Version::V2,
			method: "bar".to_string(),
			params: jsonrpc::Params::None,
		};

		let mut state = BatchesState::new();
		assert!(state.next_event().is_none());
		state.inject(
			jsonrpc::Request::Batch(vec![
				jsonrpc::Call::Notification(notif1.clone()),
				jsonrpc::Call::Notification(notif2.clone()),
			]),
			2,
		);

		match state.next_event() {
			Some(BatchesEvent::Notification { ref notification, ref user_param })
				if *notification == Notification::from(notif1) && **user_param == 2 => {}
			_ => panic!(),
		}

		match state.next_event() {
			Some(BatchesEvent::Notification { ref notification, ref user_param })
				if *notification == Notification::from(notif2) && **user_param == 2 => {}
			_ => panic!(),
		}

		assert!(state.next_event().is_none());
	}
}
