//! Performing JSON-RPC requests.
// TODO: expand

pub use crate::{client::raw::RawClient, common};
use fnv::FnvHashMap;
use serde::de::DeserializeOwned;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{collections::VecDeque, error, fmt};

pub mod raw;

/// Wraps around a "raw client" and analyzes everything correctly.
///
/// A `Client` can be seen as a collection of requests.
pub struct Client<R> {
    /// Inner raw client.
    inner: R,
    /// Id to assign to the next request.
    next_request_id: u64,
    /// List of active requests.
    requests: FnvHashMap<u64, Request>,
    /// Queue of events to return from [`Client::next_event`].
    events_queue: VecDeque<ClientEvent>,
}

struct Request {
    
}

#[derive(Debug)]
pub enum ClientEvent {
    /// A request has received a response.
    Response {
        /// The response itself.
        result: Result<common::JsonValue, common::Error>,
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
            next_request_id: 0,
            requests: FnvHashMap::default(),
            events_queue: VecDeque::with_capacity(6),
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

    /// Assigns an id for a request.
    fn assign_id(&mut self) -> u64 {
        let id = self.next_request_id;
        if let Some(i) = self.next_request_id.checked_add(1) {
            self.next_request_id = i;
        } else {
            // TODO: what to do here?
            log::error!("Overflow in client request ID assignment");
        }
        id
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
    ) -> Result<(), ClientError<R::Error>> {
        let id = {
            let i = self.next_request_id;
            if i == u64::max_value() {
                log::error!("Overflow in client request ID assignment");
            }
            i
        };

        let request = common::Request::Single(common::Call::MethodCall(common::MethodCall {
            jsonrpc: common::Version::V2,
            method: method.into(),
            params: params.into(),
            id: common::Id::Num(id),
        }));

        self.inner
            .send_request(request)
            .await
            .map_err(ClientError::Inner)?;
        self.requests.insert(id, Request {});
        Ok(())
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
    ) -> Result<(), ClientError<R::Error>> {
        unimplemented!()
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
            }
        }
    }

    /// Processes the response obtained from the server. updates the internal state of `self` to
    /// account for it.
    fn process_response(&mut self, response: common::Output) {
        match response.id() {
            common::Id::Str(s) => {
                log::error!("Server responses with an invalid request id: {:?}", s);
            }
            common::Id::Null => {
                // TODO: subscriptions
            }
            common::Id::Num(n) => {
                // Find the request that this answered.
                let answered_request = match self.requests.remove(&n) {
                    Some(r) => r,
                    None => {
                        log::error!("Server responses with an invalid request id: {:?}", n);
                        return;
                    }
                };

                self.events_queue.push_back(ClientEvent::Response {
                    result: response.into(),
                });
            }
        }
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
