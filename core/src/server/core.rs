use crate::common::{self, JsonValue};
use crate::server::{batches, raw::RawServer, raw::RawServerEvent, Notification, Params};
use err_derive::*;
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

    /// List of active subscriptions.
    /// The identifier is chosen randomly and uniformy distributed. It is never decided by the
    /// client. There is therefore no risk of hash collision attack.
    subscriptions: FnvHashMap<[u8; 32], SubscriptionState<I>>,

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
pub struct ServerSubscriptionId([u8; 32]);

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

    /// Subscriptions are now ready.
    SubscriptionsReady(Vec<ServerSubscriptionId>),

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
    subscriptions: &'a mut FnvHashMap<[u8; 32], SubscriptionState<I>>,

    /// Reference to the corresponding field in `Server`.
    num_subscriptions: &'a mut HashMap<I, NonZeroUsize>,
}

/// Active subscription of a client towards a server.
///
/// > **Note**: Holds a borrow of the `Server`. Therefore, must be dropped before the `Server` can
/// >           be dropped.
pub struct ServerSubscription<'a, R, I> {
    server: &'a mut Server<R, I>,
    id: [u8; 32],
}

/// Error that can happen when calling `into_subscription`.
#[derive(Debug, Error)]
pub enum IntoSubscriptionErr {
    /// Underlying server doesn't support subscriptions.
    #[error(display = "Underlying server doesn't support subscriptions")]
    NotSupported,
    /// Request has already been closed by the client.
    #[error(display = "Request is already closed")]
    Closed,
}

/// Internal structure. Information about a subscription.
#[derive(Debug)]
struct SubscriptionState<I> {
    /// Identifier of the connection in the raw server.
    raw_id: I,
    /// Method that triggered the subscription. Must be sent to the client at each notification.
    method: String,
    /// If true, the subscription shouldn't accept any notification push because the confirmation
    /// hasn't been sent to the client yet. Once this has switched to `false`, it can never be
    /// switched to `true` ever again.
    pending: bool,
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
                        // TODO: that's O(n)
                        let mut ready = Vec::new();     // TODO: with_capacity
                        for (sub_id, sub) in self.subscriptions.iter_mut() {
                            if sub.raw_id == raw_request_id {
                                ready.push(ServerSubscriptionId(sub_id.clone()));
                                sub.pending = false;
                            }
                        }
                        debug_assert!(!ready.is_empty());       // TODO: assert that capacity == len
                        return Ok(ServerEvent::SubscriptionsReady(ready));
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

                    // Additionally, active subscriptions that were using this connection are
                    // closed.
                    if let Some(_) = self.num_subscriptions.remove(&raw_id) {
                        let ids = self.subscriptions.iter()
                            .filter(|(_, v)| v.raw_id == raw_id)
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

    /// Sends back a response similar to `respond`, then returns a [`ServerSubscriptionId`] object
    /// that allows you to push more data on the corresponding connection.
    ///
    /// The [`ServerSubscriptionId`] corresponds to the identifier that has been sent back to the
    /// client. If the client refers to this subscription id, you can turn it into a
    /// [`ServerSubscriptionId`] using
    /// [`from_wire_message`](ServerSubscriptionId::from_wire_message).
    ///
    /// After the request has been turned into a subscription, the subscription might be in
    /// "pending mode". Pushing notifications on that subscription will return an error. This
    /// mechanism is necessary because the subscription request might be part of a batch, and all
    /// the requests of that batch have to be processed before informing the client of the start
    /// of the subscription.
    ///
    /// Returns an error and doesn't do anything if the underlying server doesn't support
    /// subscriptions, or if the connection has already been closed by the client.
    ///
    /// > **Note**: Because of borrowing issues, we return a [`ServerSubscriptionId`] rather than
    /// >           a [`ServerSubscription`]. You will have to call
    /// >           [`subscription_by_id`](Server::subscription_by_id) in order to manipulate the
    /// >           subscription.
    // TODO: solve the note
    pub async fn into_subscription(
        mut self,
    ) -> Result<ServerSubscriptionId, IntoSubscriptionErr> {
        let raw_request_id = match self.inner.user_param().clone() {
            Some(id) => id,
            None => return Err(IntoSubscriptionErr::Closed)
        };

        if !self.raw.supports_resuming(&raw_request_id).unwrap_or(false) {
            return Err(IntoSubscriptionErr::NotSupported);
        }

        loop {
            let new_subscr_id: [u8; 32] = rand::random();

            match self.subscriptions.entry(new_subscr_id) {
                Entry::Vacant(e) => {
                    e.insert(SubscriptionState {
                        raw_id: raw_request_id.clone(),
                        method: self.inner.method().to_owned(),
                        pending: true,
                    })
                },
                // Continue looping if we accidentally chose an existing ID.
                Entry::Occupied(_) => continue,
            };

            self.num_subscriptions
                .entry(raw_request_id)
                .and_modify(|e| {
                    *e = NonZeroUsize::new(e.get() + 1)
                        .expect("we add 1 to an existing non-zero value; qed");
                })
                .or_insert_with(|| NonZeroUsize::new(1).expect("1 != 0"));

            let subscr_id_string = bs58::encode(&new_subscr_id).into_string();
            self.inner.set_response(Ok(subscr_id_string.into()));
            break Ok(ServerSubscriptionId(new_subscr_id));
        }
    }
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

impl ServerSubscriptionId {
    /// When the client sends a unsubscribe message containing a subscription ID, this function can
    /// be used to parse it into a [`ServerSubscriptionId`].
    pub fn from_wire_message(params: &JsonValue) -> Result<Self, ()> {
        let string = match params {
            JsonValue::String(s) => s,
            _ => return Err(())
        };
        
        let decoded = bs58::decode(&string).into_vec().map_err(|_| ())?;
        if decoded.len() > 32 {
            return Err(());
        }

        let mut out = [0; 32];
        out[(32 - decoded.len())..].copy_from_slice(&decoded);
        // TODO: write a test to check that encoding/decoding match
        Ok(ServerSubscriptionId(out))
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
        let subscription_state = self.server.subscriptions.get(&self.id).unwrap();
        if subscription_state.pending {
            return;     // TODO: notify user with error
        }

        let output = common::SubscriptionNotif {
            jsonrpc: common::Version::V2,
            method: subscription_state.method.clone(),
            params: common::SubscriptionNotifParams {
                subscription: common::SubscriptionId::Str(bs58::encode(&self.id).into_string()),
                result: message.into(),
            },
        };
        let response = common::Response::Notif(output);
        let _ = self.server.raw.send(&subscription_state.raw_id, &response).await; // TODO: error handling?
    }

    /// Destroys the subscription object.
    ///
    /// This does not send any message back to the client. Instead, this function is supposed to
    /// be used in reaction to the client requesting to be unsubscribed.
    ///
    /// If this was the last active subscription, also closes the connection ("raw request") with
    /// the client.
    pub async fn close(self) {
        let subscription_state = self.server.subscriptions.remove(&self.id).unwrap();

        // Check if we're the last subscription on this connection.
        // Remove entry from `num_subscriptions` if so.
        let is_last_sub = match self.server.num_subscriptions.entry(subscription_state.raw_id.clone()) {
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

        // If the subscription is pending, we have yet to send something back on that connection
        // and thus shouldn't close it.
        // When the response is sent back later, the code will realize that `num_subscriptions`
        // is zero/empty and call `finish`.
        if is_last_sub && !subscription_state.pending {
            let _ = self.server.raw.finish(&subscription_state.raw_id, None).await;
        }
    }
}
