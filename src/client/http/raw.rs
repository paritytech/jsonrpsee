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

//! Performing JSON-RPC requests.
//!
//! The [`RawClient`] struct wraps around a [`TransportClient`] and handles the higher-level JSON-RPC logic
//! on top of it. In order to build a [`RawClient`], you need to pass to it an implementation of
//! [`TransportClient`].
//!
//! Once created, a [`RawClient`] can be used to send out notifications, requests, and subscription
//! requests to the server. Request identifiers are automatically assigned by the client.
//!
//! # Notifications
//!
//! **Notifications** are one-shot messages to the server that don't expect any response. They can
//! be sent using the [`send_notification`](RawClient::send_notification) method.
//!
//! # Requests
//!
//! **Requests** are messages that expect an answer. A request can be sent using the
//! [`start_request`](RawClient::start_request) method. This method returns a [`RawClientRequestId`] that
//! is used to identify this request within the internals of the [`RawClient`]. You can then call
//! [`request_by_id`](RawClient::request_by_id) to wait for a response from a server about a specific
//! request. You are however encouraged to use [`next_event`](RawClient::next_event) instead, which
//! produces a [`RawClientEvent`] indicating you what the server did.
//!
//! > **Note**: At the time of writing, the [`RawClient`] never uses batches and only sends out
//! >           individual requests.
//!
//! # Subscriptions
//!
//! **Subscriptions** are similar to requests, except that we stay connected to the server
//! after the request ended, and expect notifications back from it. The [`RawClient`] will notify
//! you about subscriptions through the [`next_event`](RawClient::next_event) method and the
//! [`RawClientEvent`] enum.
//!
//! > **Note**: The [`request_by_id`](RawClient::request_by_id) method will buffer up incoming
//! >           notifications up to a certain limit. Once this limit is reached, new notifications
//! >           will be silently discarded. This behaviour exists to prevent DoS attacks from
//! >           the server. If you want to be certain to not miss any notification, please only
//! >           use the [`next_event`](RawClient::next_event) method.
//!

use crate::client::http::transport::{HttpTransportClient, RequestError};
use crate::common;

use alloc::{collections::VecDeque, string::String};
use core::{fmt, future::Future};
use hashbrown::HashSet;

/// Wraps around a [`TransportClient`](crate::transport::TransportClient) and analyzes everything
/// correctly.
///
/// See [the module root documentation](crate::client) for more information.
pub struct RawClient {
    /// Inner raw client.
    inner: HttpTransportClient,

    /// Id to assign to the next request. We always assign linearly-increasing numeric keys.
    next_request_id: RawClientRequestId,

    /// List of requests and subscription requests that have been sent out and that are waiting
    /// for a response.
    ///
    // NOTE: `fnv - fowler-Noll-Vo hash function`, more efficient for smaller hash keys.
    requests: HashSet<RawClientRequestId, fnv::FnvBuildHasher>,

    /// Queue of pending events to return from [`RawClient::next_event`].
    // TODO: call shrink_to from time to time; see https://github.com/rust-lang/rust/issues/56431
    events_queue: VecDeque<RawClientEvent>,

    /// Maximum allowed size of [`RawClient::events_queue`].
    ///
    /// If this size is reached, elements can still be pushed to the queue if they are critical,
    /// but will be discarded if they are not.
    // TODO: make this configurable? note: if this is configurable, it should always be >= 1
    events_queue_max_size: usize,
}

/// Unique identifier of a request within a [`RawClient`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RawClientRequestId(u64);

/// Event returned by [`RawClient::next_event`].
#[derive(Debug)]
pub enum RawClientEvent {
    /// A request has received a response.
    Response {
        /// Identifier of the request. Can be matched with the value that [`RawClient::start_request`]
        /// has returned.
        request_id: RawClientRequestId,
        /// The response itself.
        result: Result<common::JsonValue, common::Error>,
    },
}

/// Error that can happen during a request.
#[derive(Debug)]
pub enum RawClientError {
    /// Error in the raw client.
    Inner(RequestError),
    /// RawServer returned an error for our request.
    RequestError(common::Error),
    /// RawServer has sent back a response containing an unknown request ID.
    UnknownRequestId,
    /// RawServer has sent back a response containing a null request ID.
    NullRequestId,
}

impl RawClient {
    /// Initializes a new `RawClient` using the given raw client as backend.
    pub fn new(inner: HttpTransportClient) -> Self {
        RawClient {
            inner,
            next_request_id: RawClientRequestId(0),
            requests: HashSet::default(),
            events_queue: VecDeque::with_capacity(16),
            events_queue_max_size: 64,
        }
    }
}

impl RawClient {
    /// Sends a notification to the server. The notification doesn't need any response.
    ///
    /// This asynchronous function finishes when the notification has finished being sent.
    pub async fn send_notification(
        &mut self,
        method: impl Into<String>,
        params: impl Into<common::Params>,
    ) -> Result<(), RequestError> {
        let request = common::Request::Single(common::Call::Notification(common::Notification {
            jsonrpc: common::Version::V2,
            method: method.into(),
            params: params.into(),
        }));

        self.inner.send_request(request).await?;
        Ok(())
    }

    /// Starts a request.
    ///
    /// This asynchronous function finishes when the request has been sent to the server. The
    /// request is added to the [`RawClient`]. You must then call [`next_event`](RawClient::next_event)
    /// until you get a response.
    pub async fn start_request(
        &mut self,
        method: impl Into<String>,
        params: impl Into<common::Params>,
    ) -> Result<RawClientRequestId, RequestError> {
        loop {
            let id = self.next_request_id;
            self.next_request_id.0 = self.next_request_id.0.wrapping_add(1);

            if self.requests.contains(&id) {
                continue;
            } else {
                self.requests.insert(id);
            }

            let request = common::Request::Single(common::Call::MethodCall(common::MethodCall {
                jsonrpc: common::Version::V2,
                method: method.into(),
                params: params.into(),
                id: common::Id::Num(id.0),
            }));

            log::debug!(target: "jsonrpsee-http-raw-client", "request={:?}", request);
            // Note that in case of an error, we "lose" the request id (as in, it will never be
            // used). This isn't a problem, however.
            self.inner.send_request(request).await?;

            break Ok(id);
        }
    }

    /// Waits until the client receives a message from the server.
    ///
    /// If this function returns an `Err`, it indicates a connectivity issue with the server or a
    /// low-level protocol error, and not a request that has failed to be answered.
    pub async fn next_event(&mut self) -> Result<RawClientEvent, RawClientError> {
        loop {
            if let Some(event) = self.events_queue.pop_front() {
                return Ok(event);
            }

            self.event_step().await?;
        }
    }

    /// Returns a `Future` that resolves when the server sends back a response for the given
    /// request.
    ///
    /// Returns `None` if the request identifier is invalid, or if the request is a subscription.
    ///
    /// > **Note**: While this function is waiting, all the other responses and pubsub events
    /// >           returned by the server will be buffered up to a certain limit. Once this
    /// >           limit is reached, server notifications will be discarded. If you want to be
    /// >           sure to catch all notifications, use [`next_event`](RawClient::next_event)
    /// >           instead.
    pub fn request_by_id<'a>(
        &'a mut self,
        rq_id: RawClientRequestId,
    ) -> Option<impl Future<Output = Result<common::JsonValue, RawClientError>> + 'a> {
        // First, let's check whether the request ID is valid.
        if !self.requests.contains(&rq_id) {
            return None;
        }

        Some(async move {
            let mut events_queue_loopkup = 0;

            loop {
                while events_queue_loopkup < self.events_queue.len() {
                    match &self.events_queue[events_queue_loopkup] {
                        RawClientEvent::Response { request_id, .. } if *request_id == rq_id => {
                            return match self.events_queue.remove(events_queue_loopkup) {
                                Some(RawClientEvent::Response { result, .. }) => {
                                    result.map_err(RawClientError::RequestError)
                                }
                                _ => unreachable!(),
                            }
                        }
                        _ => {}
                    }

                    events_queue_loopkup += 1;
                }

                self.event_step().await?;
            }
        })
    }

    /// Waits for one server message and processes it by updating the state of `self`.
    ///
    /// If the events queue is full (see [`RawClient::events_queue_max_size`]), then responses to
    /// requests will still be pushed to the queue, but notifications will be discarded.
    ///
    /// Check the content of [`events_queue`](RawClient::events_queue) afterwards for events to
    /// dispatch to the user.
    async fn event_step(&mut self) -> Result<(), RawClientError> {
        let result = self
            .inner
            .next_response()
            .await
            .map_err(RawClientError::Inner)?;

        match result {
            common::Response::Single(rp) => self.process_response(rp)?,
            common::Response::Batch(rps) => {
                for rp in rps {
                    // TODO: if an error happens, we throw away the entire batch
                    self.process_response(rp)?;
                }
            }
            // Server MUST NOT reply to a Notification.
            common::Response::Notif(_notif) => unreachable!(),
        }

        Ok(())
    }

    /// Processes the response obtained from the server. Updates the internal state of `self` to
    /// account for it.
    ///
    /// Regards all `response IDs` that is not a number as error because only numbers are used as
    /// `id` in this library even though that `JSONRPC 2.0` allows String and Null as valid IDs.
    fn process_response(&mut self, response: common::Output) -> Result<(), RawClientError> {
        let request_id = match response.id() {
            common::Id::Num(n) => RawClientRequestId(*n),
            common::Id::Str(s) => {
                log::warn!("Server responded with an invalid request id: {:?}", s);
                return Err(RawClientError::UnknownRequestId);
            }
            common::Id::Null => {
                log::warn!("Server responded with a null request id");
                return Err(RawClientError::NullRequestId);
            }
        };

        // Find the request that this answered.
        if self.requests.remove(&request_id) {
            self.events_queue.push_back(RawClientEvent::Response {
                result: response.into(),
                request_id,
            });
        } else {
            log::warn!(
                "Server responsed with an invalid request id: {:?}",
                request_id
            );
            return Err(RawClientError::UnknownRequestId);
        }

        Ok(())
    }
}

impl fmt::Debug for RawClient {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RawClient")
            .field("inner", &self.inner)
            .field("pending_requests", &self.requests)
            .finish()
    }
}

impl std::error::Error for RawClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RawClientError::Inner(err) => Some(err),
            RawClientError::RequestError(ref err) => Some(err),
            RawClientError::UnknownRequestId => None,
            RawClientError::NullRequestId => None,
        }
    }
}

impl fmt::Display for RawClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RawClientError::Inner(err) => write!(f, "Error in the raw client: {}", err),
            RawClientError::RequestError(ref err) => write!(f, "Server returned error: {}", err),
            RawClientError::UnknownRequestId => {
                write!(f, "Server responded with an unknown request ID")
            }
            RawClientError::NullRequestId => write!(f, "Server responded with a null request ID"),
        }
    }
}
