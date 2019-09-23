use crate::common::{self, JsonValue};
use crate::server::{batches, raw::RawServer, raw::RawServerEvent, Notification, Params};
use fnv::FnvHashMap;
use std::{collections::hash_map::Entry, collections::HashMap, fmt, hash::Hash, num::NonZeroUsize};

/// Wraps around a "raw server" and adds capabilities.
///
/// See the module-level documentation for more information.
pub struct Server<R, I> {
    /// Internal "raw" server.
    raw: R,

    /// List of requests that are in the progress of being answered. Each batch is associated with
    /// the raw request ID, or with `None` if this raw request has been closed.
    ///
    /// See the documentation of [`BatchesState`][batches::BatchesState] for more information.
    batches: batches::BatchesState<Option<I>>,

    /// Identifier of the next subscription to add to `subscriptions`.
    next_subscription_id: u64,

    /// List of active subscriptions.
    /// The identifier is lineraly increasing and is never leaked on the wire or outside of this
    /// module. Therefore there is no risk of hash collision.
    subscriptions: FnvHashMap<u64, I>,

    /// For each raw request ID (i.e. client connection), the number of active subscriptions
    /// that are using it.
    ///
    /// If this reaches 0, we can tell the raw server to close the request.
    ///
    /// Because we don't have any information about `I`, we have to use a collision-resistant
    /// hashing algorithm. This incurs a performance cost that is theoretically avoidable (if `I`
    /// is always local), but that should be negligible in practice.
    num_subscriptions: HashMap<I, NonZeroUsize>,
}

/// Identifier of a request within a `Server`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ServerRequestId {
    inner: batches::BatchesElemId,
}

/// Identifier of a subscription within a [`Server`](crate::server::Server).
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ServerSubscriptionId(u64);

/// Event generated by a [`Server`](crate::server::Server).
///
/// > **Note**: Holds a borrow of the `Server`. Therefore, must be dropped before the `Server` can
/// >           be dropped.
#[derive(Debug)]
pub enum ServerEvent<'a, R, I> {
    /// Request is a notification.
    Notification(Notification),

    /// Request is a method call.
    Request(ServerRequest<'a, R, I>),

    /// Subscriptions have been closed because the client closed the connection.
    SubscriptionsClosed(Vec<ServerSubscriptionId>),
}

/// Request received by a [`Server`](crate::Server).
pub struct ServerRequest<'a, R, I> {
    /// Reference to the request within `self.batches`.
    inner: batches::BatchesElem<'a, Option<I>>,

    /// Reference to the corresponding field in `Server`.
    raw: &'a mut R,

    /// Reference to the corresponding field in `Server`.
    next_subscription_id: &'a mut u64,

    /// Reference to the corresponding field in `Server`.
    subscriptions: &'a mut FnvHashMap<u64, I>,

    /// Reference to the corresponding field in `Server`.
    num_subscriptions: &'a mut HashMap<I, NonZeroUsize>,
}

/// Active subscription of a client towards a server.
///
/// > **Note**: Holds a borrow of the `Server`. Therefore, must be dropped before the `Server` can
/// >           be dropped.
pub struct ServerSubscription<'a, R, I> {
    server: &'a mut Server<R, I>,
    id: u64,
}

impl<R, I> Server<R, I>
where
    R: RawServer<RequestId = I>,
    // Note: annoyingly, the `HashMap` constructor with hasher requires trait bounds on the key.
    I: Clone + PartialEq + Eq + Hash + Send + Sync,
{
    /// Starts a [`Server`](crate::Server) using the given raw server internally.
    pub fn new(inner: R) -> Server<R, I> {
        Server {
            raw: inner,
            batches: batches::BatchesState::new(),
            next_subscription_id: 0,
            subscriptions: HashMap::with_capacity_and_hasher(8, Default::default()),
            num_subscriptions: HashMap::with_capacity_and_hasher(8, Default::default()),
        }
    }
}

impl<R, I> Server<R, I>
where
    R: RawServer<RequestId = I>,
    I: Clone + PartialEq + Eq + Hash + Send + Sync,
{
    /// Returns a `Future` resolving to the next event that this server generates.
    pub async fn next_event<'a>(&'a mut self) -> Result<ServerEvent<'a, R, I>, ()> {
        let request_id = loop {
            match self.batches.next_event() {
                None => {}
                Some(batches::BatchesEvent::Notification { notification, .. }) => {
                    return Ok(ServerEvent::Notification(notification))
                }
                Some(batches::BatchesEvent::Request(inner)) => {
                    break ServerRequestId { inner: inner.id() };
                }
                Some(batches::BatchesEvent::ReadyToSend {
                    response,
                    user_param: Some(raw_request_id),
                }) => {
                    // If we have any active subscription, we only use `send` to not close the
                    // client request.
                    if self.num_subscriptions.contains_key(&raw_request_id) {
                        debug_assert!(self.raw.supports_resuming(&raw_request_id).unwrap_or(false));
                        let _ = self.raw.send(&raw_request_id, &response).await;
                    } else {
                        let _ = self.raw.finish(&raw_request_id, Some(&response)).await;
                    }
                    continue;
                }
                Some(batches::BatchesEvent::ReadyToSend { response: _, user_param: None }) => {
                    // This situation happens if the connection has been closed by the client.
                    // Clients who close their connection.
                    continue;
                }
            };

            match self.raw.next_request().await {
                RawServerEvent::Request { id, request } => {
                    self.batches.inject(request, Some(id))
                },
                RawServerEvent::Closed(raw_id) => {
                    // The client has a closed their connection. We eliminate all traces of the
                    // raw request ID from our state.
                    // TODO: this has an O(n) complexity; make sure that this is not attackable
                    for ud in self.batches.batches() {
                        if ud.as_ref() == Some(&raw_id) {
                            *ud = None;
                        }
                    }

                    // Additionally, active subscriptions that were using this subscription are
                    // closed.
                    if let Some(_) = self.num_subscriptions.remove(&raw_id) {
                        let ids = self.subscriptions.iter()
                            .filter(|(_, v)| **v == raw_id)
                            .map(|(k, _)| ServerSubscriptionId(*k))
                            .collect::<Vec<_>>();
                        for id in &ids { let _ = self.subscriptions.remove(&id.0); }
                        return Ok(ServerEvent::SubscriptionsClosed(ids));
                    }
                },
                RawServerEvent::ServerClosed => return Err(()),
            };
        };

        return Ok(ServerEvent::Request(
            self.request_by_id(&request_id).unwrap(),
        ));
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
        Some(ServerRequest {
            inner: self.batches.request_by_id(id.inner)?,
            raw: &mut self.raw,
            next_subscription_id: &mut self.next_subscription_id,
            subscriptions: &mut self.subscriptions,
            num_subscriptions: &mut self.num_subscriptions,
        })
    }

    /// Returns a subscription previously returned by
    /// [`into_subscription`](crate::server::ServerRequest::into_subscription).
    pub fn subscription_by_id(
        &mut self,
        id: ServerSubscriptionId,
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

impl<'a, R, I> ServerRequest<'a, R, I> {
    /// Returns the id of the request.
    ///
    /// If this request object is dropped, you can retreive it again later by calling
    /// [`request_by_id`](crate::Server::request_by_id).
    pub fn id(&self) -> ServerRequestId {
        ServerRequestId {
            inner: self.inner.id(),
        }
    }

    /// Returns the id that the client sent out.
    // TODO: can return None, which is wrong
    pub fn request_id(&self) -> &common::Id {
        self.inner.request_id()
    }

    /// Returns the method of this request.
    pub fn method(&self) -> &str {
        self.inner.method()
    }

    /// Returns the parameters of the request, as a `common::Params`.
    pub fn params(&self) -> Params {
        self.inner.params()
    }
}

impl<'a, R, I> ServerRequest<'a, R, I>
where
    R: RawServer<RequestId = I>,
    I: Clone + PartialEq + Eq + Hash + Send + Sync,
{
    /// Send back a response.
    ///
    /// If this request is part of a batch:
    ///
    /// - If all requests of the batch have been responded to, then the response is actively
    ///   sent out.
    /// - Otherwise, this response is buffered.
    ///
    /// > **Note**: This method is implemented in a way that doesn't wait for long to send the
    /// >           response. While calling this method will block your entire server, it
    /// >           should only block it for a short amount of time. See also [the equivalent
    /// >           method](crate::RawServer::finish) on the [`RawServer`](crate::RawServer) trait.
    ///
    pub async fn respond(self, response: Result<common::JsonValue, common::Error>) {
        self.inner.set_response(response);
        //unimplemented!();
        // TODO: actually send out response?
    }

    /// Sends back a response similar to `respond`, then returns a `ServerSubscription` object
    /// that allows you to push more data on the corresponding connection.
    ///
    /// Returns an error and doesn't do anything if the underlying server doesn't support
    /// subscriptions, or if the connection has already been closed by the client.
    pub async fn into_subscription(
        mut self,
        response: JsonValue,
    ) -> Result<ServerSubscription<'a, R, I>, ()> {
        let raw_request_id = match self.inner.user_param().clone() {
            Some(id) => id,
            None => return Err(())
        };

        if !self.raw.supports_resuming(&raw_request_id).unwrap_or(false) {
            return Err(());
        }

        let new_id = {
            let id = *self.next_subscription_id;
            *self.next_subscription_id += 1;
            id
        };

        self.num_subscriptions
            .entry(raw_request_id)
            .and_modify(|e| {
                *e = NonZeroUsize::new(e.get() + 1)
                    .expect("we add 1 to an existing non-zero value; qed");
            })
            .or_insert_with(|| NonZeroUsize::new(1).expect("1 != 0"));

        // TODO:
        /*let server = self.respond_inner(Ok(response), false).await;
        self.subscriptions.insert(new_id, raw_request_id);*/

        //Ok(ServerSubscription { server, id: new_id })
        unimplemented!()
    }

    /*/// Inner implementation of both `respond` and `into_subscription`.
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
    }*/
}

impl<'a, R, I> fmt::Debug for ServerRequest<'a, R, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerRequest")
            .field("request_id", &self.request_id())
            .field("method", &self.method())
            .field("params", &self.params())
            .finish()
    }
}

impl<'a, R, I> ServerSubscription<'a, R, I>
where
    R: RawServer<RequestId = I>,
    I: Clone + PartialEq + Eq + Hash + Send + Sync,
{
    /// Returns the id of the subscription.
    ///
    /// If this subscription object is dropped, you can retreive it again later by calling
    /// [`subscription_by_id`](crate::Server::subscription_by_id).
    pub fn id(&self) -> ServerSubscriptionId {
        ServerSubscriptionId(self.id)
    }

    /// Pushes a notification.
    pub async fn push(self, message: impl Into<JsonValue>) {
        let raw_id = self.server.subscriptions.get(&self.id).unwrap();
        let output =
            common::Output::from(Ok(message.into()), common::Id::Null, common::Version::V2);
        let response = common::Response::Single(output);
        let _ = self.server.raw.send(&raw_id, &response).await; // TODO: error handling?
    }

    /// Destroys the subscription object.
    ///
    /// If this was the last active subscription, also closes the connection ("raw request") with
    /// the client.
    // TODO: what if batch response hasn't been sent out yet?
    pub async fn close(self) {
        let raw_id = self.server.subscriptions.remove(&self.id).unwrap();

        let finish = match self.server.num_subscriptions.entry(raw_id.clone()) {
            Entry::Vacant(_) => unreachable!(),
            Entry::Occupied(ref mut e) if e.get().get() >= 2 => {
                let e = e.get_mut();
                *e = NonZeroUsize::new(e.get() - 1).expect("e is >= 2; qed");
                false
            }
            Entry::Occupied(e) => {
                e.remove();
                true
            }
        };

        if finish {
            let _ = self.server.raw.finish(&raw_id, None).await;
        }
    }
}
