use crate::common::{self, from_value, to_value, JsonValue};
use crate::server::{raw::RawServer, ServerRequestParams};
use fnv::FnvHashMap;
use futures::prelude::*;
use smallvec::SmallVec;
use std::{collections::HashMap, fmt, io, marker::PhantomData, pin::Pin};

/// Wraps around a "raw server".
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

    /// Pending list of requests to yield from `next_request`.
    ///
    /// Since `next_request` can pull several requests at once from the inner server, but can only
    /// return one at a time, we need a buffering mechanism.
    /// The first thing that `next_request` does is pop the first element of this array.
    // TODO: call shrink_to_fit from time to time
    to_yield: SmallVec<[ToYield; 6]>,
}

enum ToYield {
    Id(ServerRequestId),
    Notification(common::Notification),
}

struct Batch<I> {
    /// Corresponding id in the raw server.
    raw_request_id: I,
    /// List of requests that are part of this batch, and their on-going responses.
    requests: SmallVec<[(bool, Option<common::MethodCall>, Option<common::Output>); 1]>,
}

impl<R, I> Server<R, I> {
    /// Starts a `Server` using the given raw server internally.
    pub fn new(inner: R) -> Server<R, I> {
        Server {
            raw: inner,
            next_batch_id: 0,
            batches: HashMap::with_capacity_and_hasher(16, Default::default()),
            to_yield: SmallVec::new(),
        }
    }
}

impl<R, I> Server<R, I>
where
    R: RawServer<RequestId = I>,
    I: Clone + PartialEq + Eq + Send + Sync,
{
    /// Returns a `Future` resolving to the next request that this server generates.
    pub async fn next_request<'a>(&'a mut self) -> Result<ServerRq<'a, R, I>, ()> {
        loop {
            if !self.to_yield.is_empty() {
                let to_yield_id = match self.to_yield.remove(0) {
                    ToYield::Id(id) => id,
                    ToYield::Notification(n) => return Ok(ServerRq(ServerRqInner::Notification(n))),
                };
                // TODO: debug assert falseness
                self.batches.get_mut(&to_yield_id.batch_id).unwrap().requests[to_yield_id.index_in_batch].0 = true;
                return Ok(ServerRq(ServerRqInner::Call {
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
                self.next_batch_id += 1;        // TODO: overflows
                id
            };

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
                    },
                    common::Call::Notification(n) => {
                        self.to_yield.push(ToYield::Notification(n));
                    },
                    common::Call::Invalid { id } => {
                        batch.requests.push((true, None, None));        // TODO: Some
                    }
                }
            }

            if !batch.requests.is_empty() {
                self.batches.insert(new_batch_id, batch);
            } else {
                self.next_batch_id -= 1;
            }

            // TODO: what if batch is full? return it immediately?
        }
    }

    /// Returns a request previously returned by `next_request` by its id.
    ///
    /// Note that previous notifications don't have an ID and can't be accessed with this method.
    ///
    /// Returns `None` if the request ID is invalid or if the request has already been answered in
    /// the past.
    pub fn request_by_id<'a>(&'a mut self, id: &ServerRequestId) -> Option<ServerRq<'a, R, I>> {
        if !self.batches.get_mut(id.batch_id)?.requests.get(id.index_in_batch)?.0 {
            return None;
        }

        Some(ServerRq(ServerRqInner::Call {
            server: self,
            batch_id: id.batch_id,
            index_in_batch: id.index_in_batch,
        }))
    }

    /*pub fn subscriptions_by_id(&mut self, id: &String) -> Option<ServerSubscription<R>> {
        unimplemented!()
    }*/
}

impl<R> From<R> for Server<R, R::RequestId>
    where R: RawServer,
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

/// Identifier of a subscription within a `Server`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ServerSubscriptionId(u64);

/// Request generated by a `Server`.
///
/// > **Note**: Holds a borrow of the `Server`. Therefore, must be dropped before the `Server` can
/// >           be dropped.
pub struct ServerRq<'a, R, I>(ServerRqInner<'a, R, I>);

enum ServerRqInner<'a, R, I> {
    /// Request is a notification. It isn't tied to the `&mut Server` in any way. That's the only
    /// chance the user has to process this notification.
    Notification(common::Notification),

    /// Request is a method call. The request it is actually stored within the `&mut Server` at
    /// the given indices. Since we hold a mutable borrow to the `Server`, the fact that a
    /// `ServerRq` exists is a proof that there is a request at the given indices.
    Call {
        /// Server that holds the request.
        server: &'a mut Server<R, I>,
        /// Index within `Server::batches` where the request is located.
        batch_id: u64,
        /// Index within `Batch::requests` where the request is located.
        index_in_batch: usize,
    },
}

impl<'a, R, I> ServerRq<'a, R, I>
where
    R: RawServer<RequestId = I>,
    I: Clone + PartialEq + Eq + Send + Sync,
{
    /// Returns the id of the request.
    ///
    /// If this request object is dropped, you can retreive it again later by calling
    /// `request_by_id`. This isn't possible for notifications.
    pub fn id(&self) -> Option<ServerRequestId> {
        match self.call() {
            common::Call::MethodCall(common::MethodCall { id, .. }) => Some(id),
            common::Call::Notification(common::Notification { .. }) => None,
            common::Call::Invalid { id } => Some(id),        // TODO: shouldn't we panic here or something?
        }
    }

    /// Returns the method of this request.
    pub fn method(&self) -> &str {
        match self.0 {
            ServerRqInner::Notification(n) => &n.method,
            ServerRqInner::Call { server, batch_id, index_in_batch } =>
                &server.batches.get(&batch_id).unwrap().requests[index_in_batch].method,
        }
    }

    /// Returns the parameters of the request, as a `common::Params`.
    pub fn params(&self) -> ServerRequestParams {
        match self.0 {
            ServerRqInner::Notification(n) => &n.params,
            ServerRqInner::Call { server, batch_id, index_in_batch } =>
                &server.batches.get(&batch_id).unwrap().requests[index_in_batch].params,
        }
    }

    /// Send back a response.
    ///
    /// If this request is part of a batch:
    ///
    /// - If all requests of the batch have been responded to, then the response is actively
    ///   sent out.
    /// - Otherwise, this response is buffered.
    ///
    /// Returns an error if this is a notification that shouldn't receive any answer.
    ///
    pub async fn respond(self, response: Result<common::JsonValue, common::Error>) -> Result<(), ()> {
        let my_id = self.

        let output = common::Output::from(response, common::Id::Null, common::Version::V2); // TODO: id
        let response = common::Response::Single(output);
        self.server.raw.finish(&self.request_id, Some(&response)).await;
    }

    /*/// Sends back a response similar to `respond`, then returns a `ServerSubscription` object
    /// that allows you to push more data on the corresponding connection.
    // TODO: better docs
    pub async fn into_subscription(self, response: JsonValue)
        -> Result<ServerSubscription<'a, R>, io::Error>
    {
        unimplemented!();
        Ok(ServerSubscription {
            server: self.server,
        })
    }*/
}

/*/// Active subscription of a client towards a server.
///
/// > **Note**: Holds a borrow of the `Server`. Therefore, must be dropped before the `Server` can
/// >           be dropped.
pub struct ServerSubscription<'a, R> {
    server: &'a Server<R>,
}

impl<'a, R> ServerSubscription<'a, R>
where
    for<'r> &'r R: RawServerRef<'r>
{
    pub fn id(&self) -> String {
        unimplemented!()
    }

    pub fn is_valid(&self) -> bool {
        true        // TODO:
    }

    /// Pushes a notification.
    pub async fn push(self, message: JsonValue) -> Result<(), io::Error> {
        unimplemented!()
    }
}*/
