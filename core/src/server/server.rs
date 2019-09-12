use crate::common::{self, from_value, to_value, JsonValue};
use crate::server::{raw::RawServer, ServerRequestParams};
use fnv::FnvHashMap;
use futures::prelude::*;
use smallvec::SmallVec;
use std::{collections::HashMap, fmt, io, marker::PhantomData, pin::Pin};

/// Wraps around a "raw server" and adds capabilities.
///
/// See the module-level documentation for more information.
pub struct Server<R, I> {
    /// Internal "raw" server.
    raw: R,

    /// Identifier of the next batch to add to `batches`.
    next_batch_id: u64,

    /// List of requests that are in the progress of being answered. Grouped by batch.
    /// Whenever the server receives a request from the raw server, it creates a new `Batch` and
    /// stores in it the requests that expect an answer.
    /// If the `Batch` is empty, it is immediately discarded instead of being added to this list.
    ///
    /// The identifier is lineraly increasing and is never leaked on the wire or outside of this
    /// module. Therefore there is no risk of hash collision.
    // TODO: call shrink_to_fit from time to time
    batches: FnvHashMap<u64, Batch<I>>,

    /// Pending list of requests to yield from `next_event`.
    ///
    /// Since `next_event` can pull several requests at once from the inner server, but can only
    /// return one at a time, we need a buffering mechanism.
    /// The first thing that `next_event` does is pop the first element of this array.
    // TODO: call shrink_to_fit from time to time
    to_yield: SmallVec<[ToYield; 6]>,

    /// Identifier of the next subscription to add to `subscriptions`.
    next_subscription_id: u64,

    /// List of active subscriptions.
    subscriptions: FnvHashMap<u64, I>,
}

/// Internal structure indicating which event to yield from `next_event`.
enum ToYield {
    /// Yield a request from the internals of `Server`.
    Id(ServerRequestId),

    /// Yield a one-time notification.
    Notification(common::Notification),
}

struct Batch<I> {
    /// Corresponding id in the raw server.
    raw_request_id: I,
    /// List of requests that are part of this batch, and their on-going responses.
    requests: SmallVec<[(bool, Option<common::MethodCall>, Option<common::Output>); 1]>,
}

impl<R, I> Server<R, I> {
    /// Starts a [`Server`](crate::Server) using the given raw server internally.
    pub fn new(inner: R) -> Server<R, I> {
        Server {
            raw: inner,
            next_batch_id: 0,
            batches: HashMap::with_capacity_and_hasher(16, Default::default()),
            to_yield: SmallVec::new(),
            next_subscription_id: 0,
            subscriptions: HashMap::with_capacity_and_hasher(8, Default::default()),
        }
    }
}

impl<R, I> Server<R, I>
where
    R: RawServer<RequestId = I>,
    I: Clone + PartialEq + Eq + Send + Sync,
{
    /// Returns a `Future` resolving to the next event that this server generates.
    pub async fn next_event<'a>(&'a mut self) -> Result<ServerEvent<'a, R, I>, ()> {
        loop {
            if !self.to_yield.is_empty() {
                let to_yield_id = match self.to_yield.remove(0) {
                    ToYield::Id(id) => id,
                    ToYield::Notification(n) => return Ok(ServerEvent::Notification(n)),
                };
                // TODO: debug assert falseness
                self.batches
                    .get_mut(&to_yield_id.batch_id)
                    .unwrap()
                    .requests[to_yield_id.index_in_batch]
                    .0 = true;
                return Ok(ServerEvent::Request(ServerRequest {
                    server: self,
                    batch_id: to_yield_id.batch_id,
                    index_in_batch: to_yield_id.index_in_batch,
                }));
            }

            let (raw_request_id, raw_request_body) = self.raw.next_request().await?;
            let calls_list = match raw_request_body {
                // TODO: make more efficient
                common::Request::Single(rq) => vec![rq].into_iter(),
                common::Request::Batch(requests) => requests.into_iter(),
            };

            let new_batch_id = {
                let id = self.next_batch_id;
                self.next_batch_id += 1; // TODO: overflows
                id
            };

            // Every 128 batches, we shrink the containers.
            if new_batch_id % 128 == 0 {
                self.batches.shrink_to_fit();
                self.to_yield.shrink_to_fit();
            }

            let mut batch = Batch {
                raw_request_id,
                requests: SmallVec::new(),
            };

            for call in calls_list {
                match call {
                    common::Call::MethodCall(call) => {
                        self.to_yield.push(ToYield::Id(ServerRequestId {
                            batch_id: new_batch_id,
                            index_in_batch: batch.requests.len(),
                        }));
                        batch.requests.push((false, Some(call), None));
                    }
                    common::Call::Notification(n) => {
                        self.to_yield.push(ToYield::Notification(n));
                    }
                    common::Call::Invalid { id } => {
                        batch.requests.push((true, None, None)); // TODO: Some
                    }
                }
            }

            if !batch.requests.is_empty() {
                self.batches.insert(new_batch_id, batch);
            } else {
                self.next_batch_id -= 1;
                // TODO: respond with `None`
            }

            // TODO: what if batch is full of responses? return it immediately?
        }
    }

    /// Returns a request previously returned by [`next_event`](crate::Server::next_event) by its
    /// id.
    ///
    /// Note that previous notifications don't have an ID and can't be accessed with this method.
    ///
    /// Returns `None` if the request ID is invalid or if the request has already been answered in
    /// the past.
    pub fn request_by_id<'a>(
        &'a mut self,
        id: &ServerRequestId,
    ) -> Option<ServerRequest<'a, R, I>> {
        if !self
            .batches
            .get_mut(&id.batch_id)?
            .requests
            .get(id.index_in_batch)?
            .0
        {
            return None;
        }

        Some(ServerRequest {
            server: self,
            batch_id: id.batch_id,
            index_in_batch: id.index_in_batch,
        })
    }

    /// Returns a subscription previously returned by
    /// [`into_subscription`](crate::server::ServerRequest::into_subscription).
    pub fn subscriptions_by_id(
        &mut self,
        id: &ServerSubscriptionId,
    ) -> Option<ServerSubscription<R, I>> {
        if self.subscriptions.contains_key(&id.0) {
            Some(ServerSubscription {
                server: self,
                id: id.0,
            })
        } else {
            None
        }
    }
}

impl<R> From<R> for Server<R, R::RequestId>
where
    R: RawServer,
{
    fn from(inner: R) -> Server<R, R::RequestId> {
        Server::new(inner)
    }
}

/// Identifier of a request within a `Server`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ServerRequestId {
    batch_id: u64,
    index_in_batch: usize,
}

/// Identifier of a subscription within a [`Server`](crate::server::Server).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ServerSubscriptionId(u64);

/// Event generated by a [`Server`](crate::server::Server).
///
/// > **Note**: Holds a borrow of the `Server`. Therefore, must be dropped before the `Server` can
/// >           be dropped.
// TODO: implement getters on ServerEvent as well
pub enum ServerEvent<'a, R, I> {
    /// Request is a notification.
    // TODO: change type of content?
    Notification(common::Notification),

    /// Request is a method call.
    Request(ServerRequest<'a, R, I>),
}

pub struct ServerRequest<'a, R, I> {
    /// Server that holds the request.
    server: &'a mut Server<R, I>,
    /// Index within `Server::batches` where the request is located.
    batch_id: u64,
    /// Index within `Batch::requests` where the request is located.
    index_in_batch: usize,
}

impl<'a, R, I> ServerRequest<'a, R, I> {
    /// Returns the id of the request.
    ///
    /// If this request object is dropped, you can retreive it again later by calling
    /// `request_by_id`.
    pub fn id(&self) -> ServerRequestId {
        ServerRequestId {
            batch_id: self.batch_id,
            index_in_batch: self.index_in_batch,
        }
    }

    /// Returns the method of this request.
    pub fn method(&self) -> &str {
        &self.server.batches.get(&self.batch_id).unwrap().requests[self.index_in_batch]
            .1
            .as_ref()
            .unwrap()
            .method
    }

    /// Returns the parameters of the request, as a `common::Params`.
    pub fn params(&self) -> ServerRequestParams {
        ServerRequestParams::from(
            &self.server.batches.get(&self.batch_id).unwrap().requests[self.index_in_batch]
                .1
                .as_ref()
                .unwrap()
                .params,
        )
    }
}

impl<'a, R, I> ServerRequest<'a, R, I>
where
    R: RawServer<RequestId = I>,
    I: Clone + PartialEq + Eq + Send + Sync,
{
    /// Send back a response.
    ///
    /// If this request is part of a batch:
    ///
    /// - If all requests of the batch have been responded to, then the response is actively
    ///   sent out.
    /// - Otherwise, this response is buffered.
    ///
    pub async fn respond(self, response: Result<common::JsonValue, common::Error>) {
        let _ = self.respond_inner(response, true).await;
    }

    /// Sends back a response similar to `respond`, then returns a `ServerSubscription` object
    /// that allows you to push more data on the corresponding connection.
    ///
    /// Returns an error and doesn't do anything if the underlying server doesn't support
    /// subscriptions.
    pub async fn into_subscription(
        self,
        response: JsonValue,
    ) -> Result<ServerSubscription<'a, R, I>, ()> {
        let raw_request_id = self
            .server
            .batches
            .get_mut(&self.batch_id)
            .unwrap()
            .raw_request_id
            .clone();
        if !self.server.raw.supports_resuming(&raw_request_id) {
            return Err(());
        }

        let new_id = {
            let id = self.server.next_subscription_id;
            self.server.next_subscription_id += 1;
            id
        };

        let server = self.respond_inner(Ok(response), false).await;
        server.subscriptions.insert(new_id, raw_request_id);

        Ok(ServerSubscription { server, id: new_id })
    }

    /// Inner implementation of both `respond` and `into_subscription`.
    ///
    /// Removes the batch from the server if necessary. Returns the reference to the server.
    async fn respond_inner(
        self,
        response: Result<common::JsonValue, common::Error>,
        finish: bool,
    ) -> &'a mut Server<R, I> {
        let is_full = {
            let batch = &mut self.server.batches.get_mut(&self.batch_id).unwrap();
            let request_id = batch.requests[self.index_in_batch]
                .1
                .as_ref()
                .unwrap()
                .id
                .clone();
            let output = common::Output::from(response, request_id, common::Version::V2);
            batch.requests[self.index_in_batch].2 = Some(output);
            batch.requests.iter().all(|b| b.2.is_some())
        };

        let mut batch = self.server.batches.remove(&self.batch_id).unwrap();
        let raw_response = if batch.requests.len() == 1 {
            // TODO: not necessarily true, could be a batch
            common::Response::Single(batch.requests.remove(0).2.unwrap())
        } else {
            common::Response::Batch(batch.requests.drain().map(|r| r.2.unwrap()).collect())
        };

        if finish {
            self.server
                .raw
                .finish(&batch.raw_request_id, Some(&raw_response))
                .await;
        } else {
            self.server
                .raw
                .send(&batch.raw_request_id, &raw_response)
                .await;
        }

        self.server
    }
}

impl<'a, R, I> fmt::Debug for ServerRequest<'a, R, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerRequest")
            // TODO: print request id
            .field("method", &self.method())
            .field("params", &self.params())
            .finish()
    }
}

/// Active subscription of a client towards a server.
///
/// > **Note**: Holds a borrow of the `Server`. Therefore, must be dropped before the `Server` can
/// >           be dropped.
pub struct ServerSubscription<'a, R, I> {
    server: &'a mut Server<R, I>,
    id: u64,
}

impl<'a, R, I> ServerSubscription<'a, R, I>
where
    R: RawServer<RequestId = I>,
    I: Clone + PartialEq + Eq + Send + Sync,
{
    pub fn id(&self) -> ServerSubscriptionId {
        ServerSubscriptionId(self.id)
    }

    pub fn is_valid(&self) -> bool {
        true // TODO:
    }

    /// Pushes a notification.
    pub async fn push(self, message: impl Into<JsonValue>) {
        let raw_id = self.server.subscriptions.get(&self.id).unwrap();
        // TODO: self.server.raw.send(&raw_id, message).await.unwrap();
        unimplemented!()
    }

    // TODO: close operation
    // TODO: closing is hard because we might have multiple subscriptions for
    //       the same raw request
}
