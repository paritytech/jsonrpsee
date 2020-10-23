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

use crate::common;
use crate::ws::raw::{notification::Notification, params::Params};

use alloc::vec::Vec;
use core::{fmt, iter};
use smallvec::SmallVec;

/// Batch corresponding to a request from a
/// [`TransportServer`](crate::transport::TransportServer).
///
/// A [`BatchState`] combines three things:
///
/// - An incoming batch waiting to be split into requests.
/// - A list of requests that have been extracted from the batch but are yet to be answered.
/// - A list of responses waiting to be sent out.
///
/// Using the [`BatchState`] is done in the following steps:
///
/// - Construct a [`BatchState`] from a raw request.
/// - Extract one by one the requests and notifications by calling [`next`](BatchState::next). This
/// moves requests from the batch to the list of requests that are yet to be answered.
/// - Answer these requests by calling [`set_response`](BatchElem::set_response).
/// - Once all the requests have been answered, call
/// [`into_response`](BatchState::into_response) and send back the response.
/// - Once [`next`](BatchState::next) returns `None` and the response has been extracted, you can
/// destroy the [`BatchState`].
///
pub struct BatchState {
	/// List of elements to present to the user.
	to_yield: SmallVec<[ToYield; 1]>,

	/// List of requests to be answered. When a request is answered, we replace it with `None` so
	/// that indices don't change.
	requests: SmallVec<[Option<common::MethodCall>; 1]>,

	/// List of pending responses.
	responses: SmallVec<[common::Output; 1]>,

	/// True if the original request was a batch. We need to keep track of this because we need to
	/// respond differently depending on whether we have a single request or a batch with one
	/// request.
	is_batch: bool,
}

/// Element remaining to be yielded to the user.
#[derive(Debug)]
enum ToYield {
	Notification(common::Notification),
	Request(common::MethodCall),
}

/// Event generated by the [`next`](BatchState::next) function.
#[derive(Debug)]
pub enum BatchInc<'a> {
	/// Request is a notification.
	Notification(Notification),
	/// Request is a method call.
	Request(BatchElem<'a>),
}

/// References to a request within the batch that must be answered.
pub struct BatchElem<'a> {
	/// Index within the `BatchState::requests` list.
	index: usize,
	/// Reference to the actual element. Must always be `Some` for the lifetime of this object.
	/// We hold a `&mut Option<Elecommon::MethodCallm>` rather than a `&mut common::MethodCall` so
	/// that we can put `None` in it.
	elem: &'a mut Option<common::MethodCall>,
	/// Reference to the `BatchState::responses` list so that we can push a response.
	responses: &'a mut SmallVec<[common::Output; 1]>,
}

impl BatchState {
	/// Creates a `BatchState` that will manage the given request.
	pub fn from_request(raw_request_body: common::Request) -> BatchState {
		match raw_request_body {
			common::Request::Single(rq) => BatchState::from_iter(iter::once(rq), false),
			common::Request::Batch(requests) => BatchState::from_iter(requests.into_iter(), true),
		}
	}

	/// Internal implementation of [`from_request`](BatchState::from_request). Generic over the
	/// iterator.
	fn from_iter(calls_list: impl ExactSizeIterator<Item = common::Call>, is_batch: bool) -> BatchState {
		debug_assert!(!(!is_batch && calls_list.len() >= 2));

		let mut to_yield = SmallVec::with_capacity(calls_list.len());
		let mut responses = SmallVec::with_capacity(calls_list.len());
		let mut num_requests = 0;

		for call in calls_list {
			match call {
				common::Call::MethodCall(call) => {
					to_yield.push(ToYield::Request(call));
					num_requests += 1;
				}
				common::Call::Notification(n) => {
					to_yield.push(ToYield::Notification(n));
				}
				common::Call::Invalid { id } => {
					let err = Err(common::Error::invalid_request());
					let out = common::Output::from(err, id, common::Version::V2);
					responses.push(out);
				}
			}
		}

		BatchState { to_yield, requests: SmallVec::with_capacity(num_requests), responses, is_batch }
	}

	/// Returns a request previously returned by [`next_event`](crate::raw::RawServer::next_event)
	/// by its id.
	///
	/// Note that previous notifications don't have an ID and can't be accessed with this method.
	///
	/// Returns `None` if the request ID is invalid or if the request has already been answered in
	/// the past.
	pub fn request_by_id(&mut self, id: usize) -> Option<BatchElem> {
		if let Some(elem) = self.requests.get_mut(id) {
			if elem.is_none() {
				return None;
			}
			Some(BatchElem { elem, index: id, responses: &mut self.responses })
		} else {
			None
		}
	}

	/// Extracts the next request from the batch. Returns `None` if the batch is empty.
	pub fn next(&mut self) -> Option<BatchInc> {
		if self.to_yield.is_empty() {
			return None;
		}

		match self.to_yield.remove(0) {
			ToYield::Notification(n) => Some(BatchInc::Notification(From::from(n))),
			ToYield::Request(n) => {
				let request_id = self.requests.len();
				self.requests.push(Some(n));
				Some(BatchInc::Request(BatchElem {
					index: request_id,
					elem: &mut self.requests[request_id],
					responses: &mut self.responses,
				}))
			}
		}
	}

	/// Returns true if this batch is ready to send out its response.
	pub fn is_ready_to_respond(&self) -> bool {
		self.to_yield.is_empty() && self.requests.iter().all(|r| r.is_none())
	}

	/// Turns this batch into a response to send out to the client.
	///
	/// Returns `Ok(None)` if there is actually nothing to send to the client, such as when the
	/// client has only sent notifications.
	pub fn into_response(mut self) -> Result<Option<common::Response>, Self> {
		if !self.is_ready_to_respond() {
			return Err(self);
		}

		let raw_response = if self.is_batch {
			let list: Vec<_> = self.responses.drain(..).collect();
			if list.is_empty() {
				None
			} else {
				Some(common::Response::Batch(list))
			}
		} else {
			debug_assert!(self.responses.len() <= 1);
			if self.responses.is_empty() {
				None
			} else {
				Some(common::Response::Single(self.responses.remove(0)))
			}
		};

		Ok(raw_response)
	}
}

impl fmt::Debug for BatchState {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list()
			.entries(self.to_yield.iter())
			.entries(self.requests.iter().filter(|r| r.is_some()))
			.entries(self.responses.iter())
			.finish()
	}
}

impl<'a> BatchElem<'a> {
	/// Returns the id of the request within the [`BatchState`].
	///
	/// > **Note**: This is NOT the request id that the client passed.
	pub fn id(&self) -> usize {
		self.index
	}

	/// Returns the id that the client sent out.
	pub fn request_id(&self) -> &common::Id {
		let request = self.elem.as_ref().expect("elem must be Some for the lifetime of the object; qed");
		&request.id
	}

	/// Returns the method of this request.
	pub fn method(&self) -> &str {
		let request = self.elem.as_ref().expect("elem must be Some for the lifetime of the object; qed");
		&request.method
	}

	/// Returns the parameters of the request, as a `common::Params`.
	pub fn params(&self) -> Params {
		let request = self.elem.as_ref().expect("elem must be Some for the lifetime of the object; qed");
		Params::from(&request.params)
	}

	/// Responds to the request. This destroys the request object, meaning you can no longer
	/// retrieve it with [`request_by_id`](BatchState::request_by_id) later anymore.
	pub fn set_response(self, response: Result<common::JsonValue, common::Error>) {
		let request = self.elem.take().expect("elem must be Some for the lifetime of the object; qed");
		let response = common::Output::from(response, request.id, common::Version::V2);
		self.responses.push(response);
	}
}

impl<'a> fmt::Debug for BatchElem<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("BatchElem").field("method", &self.method()).field("params", &self.params()).finish()
	}
}

#[cfg(test)]
mod tests {
	use super::{BatchInc, BatchState};
	use crate::{common, ws::WsRawNotification};

	#[test]
	fn basic_notification() {
		let notif = common::Notification {
			jsonrpc: common::Version::V2,
			method: "foo".to_string(),
			params: common::Params::None,
		};

		let mut state = {
			let rq = common::Request::Single(common::Call::Notification(notif.clone()));
			BatchState::from_request(rq)
		};

		assert!(!state.is_ready_to_respond());
		match state.next() {
			Some(BatchInc::Notification(ref n)) if n == &WsRawNotification::from(notif) => {}
			_ => panic!(),
		}
		assert!(state.is_ready_to_respond());
		assert!(state.next().is_none());
		match state.into_response() {
			Ok(None) => {}
			_ => panic!(),
		}
	}

	#[test]
	fn basic_request() {
		let call = common::MethodCall {
			jsonrpc: common::Version::V2,
			method: "foo".to_string(),
			params: common::Params::Map(serde_json::from_str("{\"test\":\"foo\"}").unwrap()),
			id: common::Id::Num(123),
		};

		let mut state = {
			let rq = common::Request::Single(common::Call::MethodCall(call.clone()));
			BatchState::from_request(rq)
		};

		assert!(!state.is_ready_to_respond());
		let rq_id = match state.next() {
			Some(BatchInc::Request(rq)) => {
				assert_eq!(rq.method(), "foo");
				assert_eq!(
					{
						let v: String = rq.params().get("test").unwrap();
						v
					},
					"foo"
				);
				assert_eq!(rq.request_id(), &common::Id::Num(123));
				rq.id()
			}
			_ => panic!(),
		};

		assert!(state.next().is_none());
		assert!(!state.is_ready_to_respond());
		assert!(state.next().is_none());

		assert_eq!(state.request_by_id(rq_id).unwrap().method(), "foo");
		state.request_by_id(rq_id).unwrap().set_response(Err(common::Error::method_not_found()));

		assert!(state.is_ready_to_respond());
		assert!(state.next().is_none());

		match state.into_response() {
			Ok(Some(common::Response::Single(common::Output::Failure(f)))) => {
				assert_eq!(f.id, common::Id::Num(123));
			}
			_ => panic!(),
		}
	}

	#[test]
	fn empty_batch() {
		let mut state = {
			let rq = common::Request::Batch(Vec::new());
			BatchState::from_request(rq)
		};

		assert!(state.is_ready_to_respond());
		assert!(state.next().is_none());
		match state.into_response() {
			Ok(None) => {}
			_ => panic!(),
		}
	}
}
