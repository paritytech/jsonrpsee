//! Performing JSON-RPC requests.
//!
//! The [`Client`] struct wraps around a [`RawClient`] and handles the higher-level JSON-RPC logic
//! on top of it. In order to build a [`Client`], you need to pass to it an implementation of
//! [`RawClient`]. There exists shortcut methods that directly create a [`Client`] on top of a
//! specific [`RawClient`] implementations.
//!
//! Once created, a [`Client`] can be used to send out notifications, requests, and subscription
//! requests to the server. Request identifiers are automatically assigned by the client.
//!
//! # Notifications
//!
//! **Notifications** are one-shot messages to the server that don't expect any response. They can
//! be sent using the [`send_notification`](Client::send_notification) method.
//!
//! # Requests
//!
//! **Requests** are messages that expect an answer. A request can be sent using the
//! [`start_request`](Client::start_request) method. This method returns a [`ClientRequestId`] that
//! is used to identify this request within the internals of the [`Client`]. You can then call
//! [`wait_response`](Client::wait_response) to wait for a response from a server about a specific
//! request. You are however encouraged to use [`next_event`](Client::next_event) instead, which
//! produces a [`ClientEvent`] indicating you what the server did.
//!
//! > **Note**: At the time of writing, the [`Client`] never uses batches and only sends out
//! >           individual requests.
//!
//! # Subscriptions
//!
//! **Subscriptions** are similar to requests, except that we stay connected to the server
//! after the request ended, and expect notifications back from it. The [`Client`] will notify
//! you about subscriptions through the [`next_event`](Client::next_event) method and the
//! [`ClientEvent`] enum.
//!
//! > **Note**: The [`wait_response`](Client::wait_response) method will buffer up incoming
//! >           notifications up to a certain limit. Once this limit is reached, new notifications
//! >           will be silently discarded. This behaviour exists to prevent DoS attacks from
//! >           the server. If you want to be certain to not miss any notification, please only
//! >           use the [`next_event`](Client::next_event) method.
//!

pub use crate::{client::raw::RawClient, common};
use fnv::FnvHashMap;
use std::{collections::{HashMap, VecDeque, hash_map::Entry}, error, fmt};

pub mod raw;

/// Wraps around a [`RawClient`](crate::RawClient) and analyzes everything correctly.
///
/// See [the module root documentation](crate::client) for more information.
pub struct Client<R> {
    /// Inner raw client.
    inner: R,

    /// Id to assign to the next request. We always assign linearly-increasing numeric keys.
    next_request_id: ClientRequestId,

    /// List of requests and subscription requests that have been sent out and that are waiting
    /// for a response.
    requests: FnvHashMap<ClientRequestId, Request>,

    /// List of active subscriptions by ID (ID is chosen by the server). Note that this doesn't
    /// cover subscription requests that have been sent out but not answered yet, as these are in
    /// the [`requests`](Client::requests) field.
    /// Since the keys are decided by the server, we use a regular HashMap and its
    /// hash-collision-resistant algorithm.
    subscriptions: HashMap<String, ClientRequestId>,

    /// Queue of pending events to return from [`Client::next_event`].
    // TODO: call shrink_to from time to time; see https://github.com/rust-lang/rust/issues/56431
    events_queue: VecDeque<ClientEvent>,

    /// Maximum allowed size of [`Client::events_queue`].
    ///
    /// If this size is reached, elements can still be pushed to the queue if they are critical,
    /// but will be discarded if they are not.
    // TODO: make this configurable? note: if this is configurable, it should always be >= 1
    events_queue_max_size: usize,
}

/// Type of request that has been sent out and that is waiting for a response.
#[derive(Debug)]
enum Request {
    /// A single request expecting a response.
    Request,
    /// A potential subscription. As a response, we expect a single subscription id.
    PendingSubscription,
}

/// Unique identifier of a request within a [`Client`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ClientRequestId(u64);

/// Event returned by [`Client::next_event`].
#[derive(Debug)]
pub enum ClientEvent {
    /// A request has received a response.
    Response {
        /// Identifier of the request. Can be matched with the value that [`Client::start_request`]
        /// has returned.
        request_id: ClientRequestId,
        /// The response itself.
        result: Result<common::JsonValue, common::Error>,
    },

    /// A subscription request has received a response.
    SubscriptionResponse {
        /// Identifier of the request. Can be matched with the value that
        /// [`Client::start_subscription`] has returned.
        request_id: ClientRequestId,
        /// On success, we are now actively subscribed.
        /// [`SubscriptionNotif`](ClientEvent::SubscriptionNotif) events will now be generated.
        result: Result<(), common::Error>,
    },

    /// Notification about something we are subscribed to.
    SubscriptionNotif {
        /// Identifier of the request. Can be matched with the value that
        /// [`Client::start_subscription`] has returned.
        request_id: ClientRequestId,
        /// Opaque data that the server wants to communicate to us.
        result: common::JsonValue,
    }
}

/// Error that can happen during a request.
#[derive(Debug)]
pub enum ClientError<E> {
    /// Error in the raw client.
    Inner(E),
    /// Server has sent back a subscription ID that has already been used by an earlier
    /// subscription.
    DuplicateSubscriptionId,
    /// Failed to parse subscription ID send by server.
    ///
    /// On a successful subscription, the server is expected to send back a single number or
    /// string representing the ID of the subscription. This error happens if the server returns
    /// something else than a number or string.
    SubscriptionIdParseError,
    /// Server has sent back a response containing an unknown request ID.
    UnknownRequestId,
    /// Server has sent back a response containing a null request ID.
    NullRequestId,
    /// Server has sent back a notification using an unknown subscription ID.
    UnknownSubscriptionId,
}

impl<R> Client<R> {
    /// Initializes a new `Client` using the given raw client as backend.
    pub fn new(inner: R) -> Self {
        Client {
            inner,
            next_request_id: ClientRequestId(0),
            requests: FnvHashMap::default(),
            subscriptions: HashMap::default(),
            events_queue: VecDeque::with_capacity(16),
            events_queue_max_size: 64,
        }
    }
}

impl<R> Client<R>
where
    R: RawClient,
{
    /// Sends a notification to the server. The notification doesn't need any response.
    ///
    /// This asynchronous function finishes when the notification has finished being sent.
    pub async fn send_notification(
        &mut self,
        method: impl Into<String>,
        params: impl Into<common::Params>,
    ) -> Result<(), R::Error> {
        let request = common::Request::Single(common::Call::Notification(common::Notification {
            jsonrpc: common::Version::V2,
            method: method.into(),
            params: params.into(),
        }));

        self.inner
            .send_request(request)
            .await?;
        Ok(())
    }

    /// Starts a request.
    ///
    /// This asynchronous function finishes when the request has been sent to the server. The
    /// request is added to the [`Client`]. You must then call [`next_event`](Client::next_event)
    /// until you get a response.
    pub async fn start_request(
        &mut self,
        method: impl Into<String>,
        params: impl Into<common::Params>,
    ) -> Result<ClientRequestId, R::Error> {
        let id = {
            let i = self.next_request_id;
            self.next_request_id.0 += 1;
            // TODO: handle overflows?
            i
        };

        let request = common::Request::Single(common::Call::MethodCall(common::MethodCall {
            jsonrpc: common::Version::V2,
            method: method.into(),
            params: params.into(),
            id: common::Id::Num(id.0),
        }));

        // Note that in case of an error, we "lose" the request id (as in, it will never be used).
        // This isn't a problem, however.
        self.inner
            .send_request(request)
            .await?;
        let old_val = self.requests.insert(id, Request::Request);
        assert!(old_val.is_none());
        Ok(id)
    }

    /// Starts a request.
    ///
    /// This asynchronous function finishes when the request has been sent to the server. The
    /// request is added to the [`Client`]. You must then call [`next_event`](Client::next_event)
    /// until you get a response.
    pub async fn start_subscription(
        &mut self,
        method: impl Into<String>,
        params: impl Into<common::Params>,
    ) -> Result<ClientRequestId, R::Error> {
        let id = {
            let i = self.next_request_id;
            self.next_request_id.0 += 1;
            // TODO: handle overflows?
            i
        };

        let request = common::Request::Single(common::Call::MethodCall(common::MethodCall {
            jsonrpc: common::Version::V2,
            method: method.into(),
            params: params.into(),
            id: common::Id::Num(id.0),
        }));

        // Note that in case of an error, we "lose" the request id (as in, it will never be used).
        // This isn't a problem, however.
        self.inner
            .send_request(request)
            .await?;
        let old_val = self.requests.insert(id, Request::PendingSubscription);
        assert!(old_val.is_none());
        Ok(id)
    }

    /// Waits until the client receives a message from the server.
    ///
    /// If this function returns an `Err`, it indicates a connectivity issue with the server or a
    /// low-level protocol error, and not a request that has failed to be answered.
    pub async fn next_event(&mut self) -> Result<ClientEvent, ClientError<R::Error>> {
        loop {
            if let Some(event) = self.events_queue.pop_front() {
                return Ok(event);
            }

            self.event_step().await?;
        }
    }

    /// Waits until the server sends back a response for the given request, and returns it.
    ///
    /// > **Note**: While this function is waiting, all the other responses and pubsub events
    /// >           returned by the server will be buffered up to a certain limit. Once this
    /// >           limit is reached, server notifications will be discarded. If you want to be
    /// >           sure to catch all notifications, use [`next_event`](Client::next_event)
    /// >           instead.
    // TODO: if rq_id is subscription, will just block forever
    pub async fn wait_response(&mut self, rq_id: ClientRequestId)
        -> Result<Result<common::JsonValue, common::Error>, ClientError<R::Error>>
    {
        let mut events_queue_loopkup = 0;

        loop {
            for (offset, ev) in self.events_queue.iter().enumerate().skip(events_queue_loopkup) {
                match ev {
                    ClientEvent::Response { request_id, .. } if *request_id == rq_id => {
                        match self.events_queue.remove(offset) {
                            Some(ClientEvent::Response { result, .. }) => return Ok(result),
                            _ => unreachable!()
                        }
                    },
                    _ => {}
                }
            }

            events_queue_loopkup = self.events_queue.len();
            self.event_step().await?;
        }
    }

    /// Waits for one server message and processes it by updating the state of `self`.
    ///
    /// If the events queue is full (see [`Client::events_queue_max_size`]), then responses to
    /// requests will still be pushed to the queue, but notifications will be discarded.
    ///
    /// Check the content of [`events_queue`](Client::events_queue) afterwards for events to
    /// dispatch to the user.
    async fn event_step(&mut self) -> Result<(), ClientError<R::Error>> {
        let result = self.inner
            .next_response()
            .await
            .map_err(ClientError::Inner)?;

        match result {
            common::Response::Single(rp) => self.process_response(rp)?,
            common::Response::Batch(rps) => {
                for rp in rps {
                    // TODO: if an errror happens, we throw away the entire batch
                    self.process_response(rp)?;
                }
            },
            common::Response::Notif(notif) => {
                let sub_id = notif.params.subscription.into_string();
                if let Some(request_id) = self.subscriptions.get(&sub_id) {
                    if self.events_queue.len() < self.events_queue_max_size {
                        self.events_queue.push_back(ClientEvent::SubscriptionNotif {
                            request_id: *request_id,
                            result: notif.params.result,
                        });
                    }
                } else {
                    log::warn!("Server sent subscription notif with an invalid id: {:?}", sub_id);
                    return Err(ClientError::UnknownSubscriptionId);
                }
            }
        }

        Ok(())
    }

    /// Processes the response obtained from the server. Updates the internal state of `self` to
    /// account for it.
    fn process_response(&mut self, response: common::Output) -> Result<(), ClientError<R::Error>> {
        let request_id = match response.id() {
            common::Id::Num(n) => ClientRequestId(*n),
            common::Id::Str(s) => {
                log::warn!("Server responded with an invalid request id: {:?}", s);
                return Err(ClientError::UnknownRequestId);
            }
            common::Id::Null => {
                log::warn!("Server responded with a null request id");
                return Err(ClientError::NullRequestId);
            }
        };

        // Find the request that this answered.
        match self.requests.remove(&request_id) {
            Some(Request::Request) => {
                self.events_queue.push_back(ClientEvent::Response {
                    result: response.into(),
                    request_id,
                });
            }
    
            Some(Request::PendingSubscription) => {
                let response = match Result::from(response) {
                    Ok(r) => r,
                    Err(err) => {
                        self.events_queue.push_back(ClientEvent::SubscriptionResponse {
                            result: Err(err),
                            request_id,
                        });
                        return Ok(());
                    }
                };

                let sub_id = match common::from_value::<common::SubscriptionId>(response) {
                    Ok(id) => id.into_string(),
                    Err(err) => {
                        log::warn!("Failed to parse string subscription id: {:?}", err);
                        return Err(ClientError::SubscriptionIdParseError);
                    }
                };

                match self.subscriptions.entry(sub_id) {
                    Entry::Vacant(e) => e.insert(request_id),
                    Entry::Occupied(e) => {
                        log::warn!("Duplicate subscription id sent by server: {:?}", e.key());
                        return Err(ClientError::DuplicateSubscriptionId);
                    }
                };

                self.events_queue.push_back(ClientEvent::SubscriptionResponse {
                    result: Ok(()),
                    request_id,
                });
            }

            None => {
                log::warn!("Server responsed with an invalid request id: {:?}", request_id);
                return Err(ClientError::UnknownRequestId);
            }
        };

        Ok(())
    }

    // TODO: add a way to close subscriptions
}

impl<E> error::Error for ClientError<E>
where
    E: error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ClientError::Inner(ref err) => Some(err),
            ClientError::DuplicateSubscriptionId => None,
            ClientError::SubscriptionIdParseError => None,
            ClientError::UnknownRequestId => None,
            ClientError::NullRequestId => None,
            ClientError::UnknownSubscriptionId => None,
        }
    }
}

impl<E> fmt::Display for ClientError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::Inner(ref err) => write!(f, "Error in the raw client: {}", err),
            ClientError::DuplicateSubscriptionId =>
                write!(f, "Server has responded with a subscription ID that's already in use"),
            ClientError::SubscriptionIdParseError => write!(f, "Subscription ID parse error"),
            ClientError::UnknownRequestId =>
                write!(f, "Server responded with an unknown request ID"),
            ClientError::NullRequestId => write!(f, "Server responded with a null request ID"),
            ClientError::UnknownSubscriptionId =>
                write!(f, "Server responded with an unknown subscription ID"),
        }
    }
}
