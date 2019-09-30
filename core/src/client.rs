//! Performing JSON-RPC requests.
// TODO: expand

pub use crate::{client::raw::RawClient, common};
use fnv::{FnvHashMap, FnvHashSet};
use serde::de::DeserializeOwned;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{collections::HashMap, collections::VecDeque, error, fmt};

pub mod raw;

/// Wraps around a "raw client" and analyzes everything correctly.
///
/// A `Client` can be seen as a collection of requests.
pub struct Client<R> {
    /// Inner raw client.
    inner: R,
    /// Id to assign to the next request.
    next_request_id: ClientRequestId,
    /// List of requests that have been sent out and that are waiting for a response.
    requests: FnvHashMap<ClientRequestId, Request>,
    /// List of active subscriptions by ID known to the server.
    subscriptions: HashMap<String, ClientRequestId>,
    /// Queue of pending events to return from [`Client::next_event`].
    events_queue: VecDeque<ClientEvent>,
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
    /// Error while deserializing the server response.
    Deserialize(serde_json::Error),
}

impl<R> Client<R> {
    /// Initializes a new `Client` using the given raw client as backend.
    pub fn new(inner: R) -> Self {
        Client {
            inner,
            next_request_id: ClientRequestId(0),
            requests: FnvHashMap::default(),
            events_queue: VecDeque::with_capacity(6),
            subscriptions: HashMap::default(),
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
    ) -> Result<(), ClientError<R::Error>> {
        let request = common::Request::Single(common::Call::Notification(common::Notification {
            jsonrpc: common::Version::V2,
            method: method.into(),
            params: params.into(),
        }));

        self.inner
            .send_request(request)
            .await
            .map_err(ClientError::Inner)?;
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
    ) -> Result<ClientRequestId, ClientError<R::Error>> {
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

        self.inner
            .send_request(request)
            .await
            .map_err(ClientError::Inner)?;
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
    ) -> Result<ClientRequestId, ClientError<R::Error>> {
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

        self.inner
            .send_request(request)
            .await
            .map_err(ClientError::Inner)?;
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
    /// > **Note**: Be careful when using this method, as all the responses and pubsub events
    /// >           returned by the server will be buffered up indefinitely until a response to
    /// >           the right request comes.
    // TODO: if rq_id is subscription, will just block forever
    pub async fn wait_response(&mut self, rq_id: ClientRequestId)
        -> Result<Result<common::JsonValue, common::Error>, ClientError<R::Error>>
    {
        let mut events_queue_loopkup = 0;

        loop {
            for (offset, ev) in self.events_queue.iter().enumerate().skip(events_queue_loopkup) {
                match ev {
                    ClientEvent::Response { request_id, result } if *request_id == rq_id => {
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

    /// Waits for one server message and processes it.
    async fn event_step(&mut self) -> Result<(), ClientError<R::Error>> {
        let result = self.inner
            .next_response()
            .await
            .map_err(ClientError::Inner)?;
        
        match result {
            common::Response::Single(rp) => self.process_response(rp),
            common::Response::Batch(rps) => {
                for rp in rps {
                    self.process_response(rp);
                }
            },
            common::Response::Notif(notif) => {
                let sub_id = &notif.params.subscription;
                if let Some(request_id) = self.subscriptions.get(sub_id) {
                    self.events_queue.push_back(ClientEvent::SubscriptionNotif {
                        request_id: *request_id,
                        result: notif.params.result,
                    });
                } else {
                    // TODO: should that be a variant in ClientEvent?
                    log::warn!("Server sent subscription notif with an invalid id: {:?}", sub_id);
                }
            }
        }

        Ok(())
    }

    /// Processes the response obtained from the server. updates the internal state of `self` to
    /// account for it.
    fn process_response(&mut self, response: common::Output) {
        let request_id = match response.id() {
            common::Id::Num(n) => ClientRequestId(*n),
            common::Id::Str(s) => {
                // TODO: should that be a variant in ClientEvent?
                log::warn!("Server responsed with an invalid request id: {:?}", s);
                return;
            }
            common::Id::Null => {
                // TODO: should that be a variant in ClientEvent?
                log::warn!("Server responsed with a null request id");
                return;
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
                        return;
                    }
                };

                let sub_id: String = match common::from_value(response) {
                    Ok(id) => id,
                    Err(err) => {
                        // TODO: should that be a variant in ClientEvent? probably yes, otherwise users won't clean up pending subscription
                        log::warn!("Failed to parse string subscription id: {:?}", err);
                        return;
                    }
                };

                self.subscriptions.insert(sub_id, request_id);
                self.events_queue.push_back(ClientEvent::SubscriptionResponse {
                    result: Ok(()),
                    request_id,
                });
            }

            None => {
                // TODO: should that be a variant in ClientEvent?
                log::warn!("Server responsed with an invalid request id: {:?}", request_id);
                return;
            }
        };
    }
}

impl<E> error::Error for ClientError<E>
where
    E: error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ClientError::Inner(ref err) => Some(err),
            ClientError::Deserialize(ref err) => Some(err),
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
            ClientError::Deserialize(ref err) => write!(f, "Error when deserializing: {}", err),
        }
    }
}
