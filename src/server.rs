// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

use crate::common::{self, JsonValue};
use crate::raw::{
    server::{RawServerEvent, RawServerRequestId, RawServerSubscriptionId},
    RawServer,
};
use crate::transport::TransportServer;

use futures::{channel::mpsc, future::Either, pin_mut, prelude::*};
use parking_lot::Mutex;
use std::{
    collections::HashMap,
    collections::HashSet,
    hash::Hash,
    sync::{atomic, Arc},
};

/// Server that can be cloned.
///
/// > **Note**: This struct is designed to be easy to use, but it works by maintaining a background
/// >           task running in parallel. If this is not desirable, you are encouraged to use the
/// >           [`RawServer`] struct instead.
#[derive(Clone)]
pub struct Server {
    /// Channel to send requests to the background task.
    to_back: mpsc::UnboundedSender<FrontToBack>,
    /// List of methods (for RPC queries, subscriptions, and unsubscriptions) that have been
    /// registered. Serves no purpose except to check for duplicates.
    registered_methods: Arc<Mutex<HashSet<String>>>,
    /// Next unique ID used when registering a subscription.
    next_subscription_unique_id: Arc<atomic::AtomicUsize>,
}

/// Notifications method that's been registered.
pub struct RegisteredNotifications {
    /// Receives notifications that the client sent to us.
    queries_rx: mpsc::Receiver<common::Params>,
}

/// Method that's been registered.
pub struct RegisteredMethod {
    /// Clone of [`Server::to_back`].
    to_back: mpsc::UnboundedSender<FrontToBack>,
    /// Receives requests that the client sent to us.
    queries_rx: mpsc::Receiver<(RawServerRequestId, common::Params)>,
}

/// Pub-sub subscription that's been registered.
// TODO: unregister on drop
pub struct RegisteredSubscription {
    /// Clone of [`Server::to_back`].
    to_back: mpsc::UnboundedSender<FrontToBack>,
    /// Value passed to [`FrontToBack::RegisterSubscription::unique_id`].
    unique_id: usize,
}

/// Active request that needs to be answered.
pub struct IncomingRequest {
    /// Clone of [`Server::to_back`].
    to_back: mpsc::UnboundedSender<FrontToBack>,
    /// Identifier of the request towards the server.
    request_id: RawServerRequestId,
    /// Parameters of the request.
    params: common::Params,
}

/// Message that the [`Server`] can send to the background task.
enum FrontToBack {
    /// Registers a notifications endpoint.
    RegisterNotifications {
        /// Name of the method.
        name: String,
        /// Where to send incoming notifications.
        handler: mpsc::Sender<common::Params>,
        /// See the documentation of [`Server::register_notifications`].
        allow_losses: bool,
    },

    /// Registers a method. The server will then handle requests using this method.
    RegisterMethod {
        /// Name of the method.
        name: String,
        /// Where to send requests.
        handler: mpsc::Sender<(RawServerRequestId, common::Params)>,
    },

    /// Send a response to a request that a client made.
    AnswerRequest {
        /// Request to answer.
        request_id: RawServerRequestId,
        /// Response to send back.
        answer: Result<JsonValue, common::Error>,
    },

    /// Registers a subscription. The server will then handle subscription requests of that
    /// method.
    RegisterSubscription {
        /// Unique identifier decided by the front-end in order to identify this registered
        /// subscription.
        unique_id: usize,
        /// Name of the method that registers the subscription.
        subscribe_method: String,
        /// Name of the method that unregisters the subscription.
        unsubscribe_method: String,
    },

    /// Send out a notification to all the clients registered to a subscription.
    SendOutNotif {
        /// The value that was passed in [`FrontToBack::RegisterSubscription::unique_id`] earlier.
        unique_id: usize,
        /// Notification to send to the subscribed clients.
        notification: JsonValue,
    },
}

impl Server {
    /// Initializes a new server based upon this raw server.
    pub fn new<R, I>(server: RawServer<R, I>) -> Server
    where
        R: TransportServer<RequestId = I> + Send + 'static,
        I: Clone + PartialEq + Eq + Hash + Send + Sync + 'static,
    {
        // We use an unbounded channel because the only exchanged messages concern registering
        // methods. The volume of messages is therefore very low and it doesn't make sense to have
        // a backpressure mechanism.
        // TODO: that's not true anymore ^
        let (to_back, from_front) = mpsc::unbounded();

        async_std::task::spawn(async move {
            background_task(server, from_front).await;
        });

        Server {
            to_back,
            registered_methods: Arc::new(Mutex::new(Default::default())),
            next_subscription_unique_id: Arc::new(atomic::AtomicUsize::new(0)),
        }
    }

    /// Registers a notification method name towards the server.
    ///
    /// Clients will then be able to call this method.
    /// The returned object allows you to process incoming notifications.
    ///
    /// If `allow_losses` is true, then the server is allowed to drop notifications if the
    /// notifications handler (i.e. the code that uses [`RegisteredNotifications`]) is too slow
    /// to process notifications.
    ///
    /// Returns an error if the method name was already registered.
    pub fn register_notifications(
        &self,
        method_name: String,
        allow_losses: bool,
    ) -> Result<RegisteredNotifications, ()> {
        if !self.registered_methods.lock().insert(method_name.clone()) {
            return Err(());
        }

        let (tx, rx) = mpsc::channel(32);

        let _ = self
            .to_back
            .unbounded_send(FrontToBack::RegisterNotifications {
                name: method_name,
                handler: tx,
                allow_losses,
            });

        Ok(RegisteredNotifications { queries_rx: rx })
    }

    /// Registers a method towards the server.
    ///
    /// Clients will then be able to call this method.
    /// The returned object allows you to handle incoming requests.
    ///
    /// Contrary to [`register_notifications`](Server::register_notifications), there is no
    /// `allow_losses` parameter here. If the handler is too slow to process requests, then the
    /// server automatically returns an "internal error" to the client.
    ///
    /// Returns an error if the method name was already registered.
    pub fn register_method(&self, method_name: String) -> Result<RegisteredMethod, ()> {
        if !self.registered_methods.lock().insert(method_name.clone()) {
            return Err(());
        }

        let (tx, rx) = mpsc::channel(32);

        let _ = self.to_back.unbounded_send(FrontToBack::RegisterMethod {
            name: method_name,
            handler: tx,
        });

        Ok(RegisteredMethod {
            to_back: self.to_back.clone(),
            queries_rx: rx,
        })
    }

    /// Registers a subscription towards the server.
    ///
    /// Clients will then be able to call this method.
    /// The returned object allows you to send out notifications.
    ///
    /// Returns an error if one of the method names was already registered.
    pub fn register_subscription(
        &self,
        subscribe_method_name: String,
        unsubscribe_method_name: String,
    ) -> Result<RegisteredSubscription, ()> {
        {
            let mut registered_methods = self.registered_methods.lock();
            if !registered_methods.insert(subscribe_method_name.clone()) {
                return Err(());
            }
            if !registered_methods.insert(unsubscribe_method_name.clone()) {
                registered_methods.remove(&subscribe_method_name);
                return Err(());
            }
        }

        let unique_id = self
            .next_subscription_unique_id
            .fetch_add(1, atomic::Ordering::Relaxed);

        let _ = self
            .to_back
            .unbounded_send(FrontToBack::RegisterSubscription {
                unique_id,
                subscribe_method: subscribe_method_name,
                unsubscribe_method: unsubscribe_method_name,
            });

        Ok(RegisteredSubscription {
            to_back: self.to_back.clone(),
            unique_id,
        })
    }
}

impl RegisteredNotifications {
    /// Returns the next notification.
    pub async fn next(&mut self) -> common::Params {
        loop {
            match self.queries_rx.next().await {
                Some(v) => break v,
                None => futures::pending!(),
            }
        }
    }
}

/// Initializes a new server based upon this raw server.
impl<R, I> From<RawServer<R, I>> for Server
where
    R: TransportServer<RequestId = I> + Send + 'static,
    I: Clone + PartialEq + Eq + Hash + Send + Sync + 'static,
{
    fn from(server: RawServer<R, I>) -> Server {
        Server::new(server)
    }
}

impl RegisteredMethod {
    /// Returns the next request.
    pub async fn next(&mut self) -> IncomingRequest {
        let (request_id, params) = loop {
            match self.queries_rx.next().await {
                Some(v) => break v,
                None => futures::pending!(),
            }
        };

        IncomingRequest {
            to_back: self.to_back.clone(),
            request_id,
            params,
        }
    }
}

impl RegisteredSubscription {
    /// Sends out a value to all the registered clients.
    pub async fn send(&mut self, value: JsonValue) {
        let _ = self.to_back.send(FrontToBack::SendOutNotif {
            unique_id: self.unique_id,
            notification: value,
        });
    }
}

impl IncomingRequest {
    /// Returns the parameters of the request.
    pub fn params(&self) -> &common::Params {
        &self.params
    }

    /// Respond to the request.
    pub async fn respond(mut self, response: impl Into<Result<JsonValue, common::Error>>) {
        let _ = self
            .to_back
            .send(FrontToBack::AnswerRequest {
                request_id: self.request_id,
                answer: response.into(),
            })
            .await;
    }
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task<R, I>(
    mut server: RawServer<R, I>,
    mut from_front: mpsc::UnboundedReceiver<FrontToBack>,
) where
    R: TransportServer<RequestId = I> + Send + 'static,
    I: Clone + PartialEq + Eq + Hash + Send + Sync + 'static,
{
    // List of notifications methods that the user has registered, and the channels to dispatch
    // incoming notifications.
    let mut registered_notifications: HashMap<String, (mpsc::Sender<_>, bool)> = HashMap::new();
    // List of methods that the user has registered, and the channels to dispatch incoming
    // requests.
    let mut registered_methods: HashMap<String, mpsc::Sender<_>> = HashMap::new();
    // For each registered subscription, a subscribe method linked to a unique identifier for
    // that subscription.
    let mut subscribe_methods: HashMap<String, usize> = HashMap::new();
    // For each registered subscription, an unsubscribe method linked to a unique identifier for
    // that subscription.
    let mut unsubscribe_methods: HashMap<String, usize> = HashMap::new();
    // For each registered subscription, a list of clients that are registered towards us.
    let mut subscribed_clients: HashMap<usize, Vec<RawServerSubscriptionId>> = HashMap::new();
    // Reversed mapping of `subscribed_clients`. Must always be in sync.
    let mut active_subscriptions: HashMap<RawServerSubscriptionId, usize> = HashMap::new();

    loop {
        // We need to do a little transformation in order to destroy the borrow to `client`
        // and `from_front`.
        let outcome = {
            let next_message = from_front.next();
            let next_event = server.next_event();
            pin_mut!(next_message);
            pin_mut!(next_event);
            match future::select(next_message, next_event).await {
                Either::Left((v, _)) => Either::Left(v),
                Either::Right((v, _)) => Either::Right(v),
            }
        };

        match outcome {
            Either::Left(None) => return,
            Either::Left(Some(FrontToBack::AnswerRequest { request_id, answer })) => {
                server
                    .request_by_id(&request_id)
                    .unwrap()
                    .respond(answer);
            }
            Either::Left(Some(FrontToBack::RegisterNotifications {
                name,
                handler,
                allow_losses,
            })) => {
                registered_notifications.insert(name, (handler, allow_losses));
            }
            Either::Left(Some(FrontToBack::RegisterMethod { name, handler })) => {
                registered_methods.insert(name, handler);
            }
            Either::Left(Some(FrontToBack::RegisterSubscription {
                unique_id,
                subscribe_method,
                unsubscribe_method,
            })) => {
                debug_assert_ne!(subscribe_method, unsubscribe_method);
                debug_assert!(!subscribe_methods.contains_key(&subscribe_method));
                debug_assert!(!subscribe_methods.contains_key(&unsubscribe_method));
                debug_assert!(!unsubscribe_methods.contains_key(&subscribe_method));
                debug_assert!(!unsubscribe_methods.contains_key(&unsubscribe_method));
                debug_assert!(!registered_methods.contains_key(&subscribe_method));
                debug_assert!(!registered_methods.contains_key(&unsubscribe_method));
                debug_assert!(!registered_notifications.contains_key(&subscribe_method));
                debug_assert!(!registered_notifications.contains_key(&unsubscribe_method));
                debug_assert!(!subscribed_clients.contains_key(&unique_id));
                subscribe_methods.insert(subscribe_method, unique_id);
                unsubscribe_methods.insert(unsubscribe_method, unique_id);
                subscribed_clients.insert(unique_id, Vec::new());
            }
            Either::Left(Some(FrontToBack::SendOutNotif {
                unique_id,
                notification,
            })) => {
                debug_assert!(subscribed_clients.contains_key(&unique_id));
                if let Some(clients) = subscribed_clients.get(&unique_id) {
                    for client in clients {
                        debug_assert_eq!(active_subscriptions.get(client), Some(&unique_id));
                        debug_assert!(server.subscription_by_id(*client).is_some());
                        if let Some(sub) = server.subscription_by_id(*client) {
                            sub.push(notification.clone()).await;
                        }
                    }
                }
            }
            Either::Right(RawServerEvent::Notification(notification)) => {
                if let Some((handler, allow_losses)) =
                    registered_notifications.get_mut(notification.method())
                {
                    let params: &common::Params = notification.params().into();
                    // Note: we just ignore errors. It doesn't make sense logically speaking to
                    // unregister the notification here.
                    if *allow_losses {
                        let _ = handler.send(params.clone()).now_or_never();
                    } else {
                        let _ = handler.send(params.clone()).await;
                    }
                }
            }
            Either::Right(RawServerEvent::Request(request)) => {
                if let Some(handler) = registered_methods.get_mut(request.method()) {
                    let params: &common::Params = request.params().into();
                    match handler.send((request.id(), params.clone())).now_or_never() {
                        Some(Ok(())) => {}
                        Some(Err(_)) | None => {
                            request
                                .respond(Err(From::from(common::ErrorCode::ServerError(0))));
                        }
                    }
                } else if let Some(sub_unique_id) = subscribe_methods.get(request.method()) {
                    if let Ok(sub_id) = request.into_subscription() {
                        debug_assert!(subscribed_clients.contains_key(&sub_unique_id));
                        if let Some(clients) = subscribed_clients.get_mut(&sub_unique_id) {
                            debug_assert!(clients.iter().all(|c| *c != sub_id));
                            clients.push(sub_id);
                        }

                        debug_assert!(!active_subscriptions.contains_key(&sub_id));
                        active_subscriptions.insert(sub_id, *sub_unique_id);
                    }
                } else if let Some(sub_unique_id) = unsubscribe_methods.get(request.method()) {
                    if let Ok(sub_id) = RawServerSubscriptionId::from_wire_message(&JsonValue::Null)
                    {
                        // FIXME: from request params
                        debug_assert!(subscribed_clients.contains_key(&sub_unique_id));
                        if let Some(clients) = subscribed_clients.get_mut(&sub_unique_id) {
                            // TODO: we don't actually check whether the unscribe comes from the right
                            //       clients, but since this the ID is randomly-generated, it should be
                            //       fine
                            if let Some(client_pos) = clients.iter().position(|c| *c == sub_id) {
                                clients.remove(client_pos);
                            }

                            if let Some(s_u_id) = active_subscriptions.remove(&sub_id) {
                                debug_assert_eq!(s_u_id, *sub_unique_id);
                            }
                        }
                    }
                } else {
                    request
                        .respond(Err(From::from(common::ErrorCode::InvalidRequest)));
                }
            }
            Either::Right(RawServerEvent::SubscriptionsReady(_)) => {
                // We don't really care whether subscriptions are now ready.
            }
            Either::Right(RawServerEvent::SubscriptionsClosed(iter)) => {
                // Remove all the subscriptions from `active_subscriptions` and
                // `subscribed_clients`.
                for sub_id in iter {
                    debug_assert!(active_subscriptions.contains_key(&sub_id));
                    if let Some(unique_id) = active_subscriptions.remove(&sub_id) {
                        debug_assert!(subscribed_clients.contains_key(&unique_id));
                        if let Some(clients) = subscribed_clients.get_mut(&unique_id) {
                            assert_eq!(clients.iter().filter(|c| **c == sub_id).count(), 1);
                            clients.retain(|c| *c != sub_id);
                        }
                    }
                }
            }
        }
    }
}
