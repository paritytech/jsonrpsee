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

use futures::{
    channel::mpsc,
    future::Either,
    pin_mut,
    prelude::*,
};
use jsonrpsee_core::{
    server::{ServerEvent, ServerRequestId},
    common::{self, JsonValue},
    Server, server::raw::TransportServer,
};
use parking_lot::Mutex;
use std::{collections::HashMap, collections::HashSet, hash::Hash, sync::Arc};

/// Server that can be cloned.
///
/// > **Note**: This struct is designed to be easy to use, but it works by maintaining a background
/// >           task running in parallel. If this is not desirable, you are encouraged to use the
/// >           [`RawServer`] struct instead.
#[derive(Clone)]
pub struct SharedServer {
    /// Channel to send requests to the background task.
    to_back: mpsc::UnboundedSender<FrontToBack>,
    /// List of methods (for RPC queries, subscriptions, and unsubscriptions) that have been
    /// registered. Makes it possible to check for duplicates.
    registered_methods: Arc<Mutex<HashSet<String>>>,
}

/// Notifications method that's been registered.
pub struct RegisteredNotifications {
    /// Receives notifications that the client sent to us.
    queries_rx: mpsc::Receiver<common::Params>,
}

/// Method that's been registered.
pub struct RegisteredMethod {
    /// Clone of [`SharedServer::to_back`].
    to_back: mpsc::UnboundedSender<FrontToBack>,
    /// Receives requests that the client sent to us.
    queries_rx: mpsc::Receiver<(ServerRequestId, common::Params)>,
}

/// Active request that needs to be answered.
pub struct IncomingRequest {
    /// Clone of [`SharedServer::to_back`].
    to_back: mpsc::UnboundedSender<FrontToBack>,
    /// Identifier of the request towards the server.
    request_id: ServerRequestId,
    /// Parameters of the request.
    params: common::Params,
}

/// Message that the [`SharedServer`] can send to the background task.
enum FrontToBack {
    /// Registers a notifications endpoint.
    RegisterNotifications {
        /// Name of the method.
        name: String,
        /// Where to send incoming notifications.
        handler: mpsc::Sender<common::Params>,
    },

    /// Registers a method. The server will then handle requests using this method.
    RegisterMethod {
        /// Name of the method.
        name: String,
        /// Where to send requests.
        handler: mpsc::Sender<(ServerRequestId, common::Params)>,
    },

    /// Registers a subscription. The server will then handle subscription requests of that
    /// method.
    RegisterSubscription {
        /// Name of the method that registers the subscription.
        subscribe_method: String,
        /// Name of the method that unregisters the subscription.
        unsubscribe_method: String,
    },

    /// Send a response to a request that a client made.
    AnswerRequest {
        /// Request to answer.
        request_id: ServerRequestId,
        /// Response to send back.
        answer: Result<common::JsonValue, common::Error>,
    },
}

impl SharedServer {
    /// Initializes a new server based upon this raw server.
    pub fn new<R, I>(server: Server<R, I>) -> SharedServer
    where
        R: TransportServer<RequestId = I> + Send + Sync + 'static,
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

        SharedServer {
            to_back,
            registered_methods: Arc::new(Mutex::new(Default::default())),
        }
    }

    /// Registers a notification method name towards the server.
    ///
    /// Clients will then be able to call this method.
    /// The returned object allows you to process incoming notifications.
    ///
    /// Returns an error if the method name was already registered.
    pub fn register_notifications(&self, method_name: String) -> Result<RegisteredNotifications, ()> {
        if !self.registered_methods.lock().insert(method_name.clone()) {
            return Err(());
        }

        let (tx, rx) = mpsc::channel(8);

        let _ = self.to_back.unbounded_send(FrontToBack::RegisterNotifications {
            name: method_name,
            handler: tx,
        });

        Ok(RegisteredNotifications {
            queries_rx: rx,
        })
    }

    /// Registers a method towards the server.
    ///
    /// Clients will then be able to call this method.
    /// The returned object allows you to handle incoming requests.
    ///
    /// Returns an error if the method name was already registered.
    pub fn register_method(&self, method_name: String) -> Result<RegisteredMethod, ()> {
        if !self.registered_methods.lock().insert(method_name.clone()) {
            return Err(());
        }

        let (tx, rx) = mpsc::channel(8);

        let _ = self.to_back.unbounded_send(FrontToBack::RegisterMethod {
            name: method_name,
            handler: tx,
        });

        Ok(RegisteredMethod {
            to_back: self.to_back.clone(),
            queries_rx: rx,
        })
    }

    /*/// Send a notification to the server.
    pub async fn notification(
        &self,
        method: impl Into<String>,
        params: impl Into<jsonrpsee_core::common::Params>,
    ) {
        let _ = self
            .to_back
            .clone()
            .send(FrontToBack::Notification {
                method: method.into(),
                params: params.into(),
            })
            .await;
    }

    /// Perform a request towards the server.
    pub async fn request<Ret>(
        &self,
        method: impl Into<String>,
        params: impl Into<jsonrpsee_core::common::Params>,
    ) -> Result<Ret, ()>
    where
        Ret: common::DeserializeOwned,
    {
        let (send_back_tx, send_back_rx) = oneshot::channel();
        let _ = self
            .to_back
            .clone()
            .send(FrontToBack::StartRequest {
                method: method.into(),
                params: params.into(),
                send_back: send_back_tx,
            })
            .await;

        // TODO: send a `ChannelClosed` message if we close the channel unexpectedly

        let json_value = match send_back_rx.await {
            Ok(Ok(v)) => v,
            _ => return Err(()),
        };

        if let Ok(parsed) = common::from_value(json_value) {
            return Ok(parsed);
        }

        Err(())
    }

    /// Send a subscription request to the server.
    ///
    /// The `subscribe_method` and `params` are used to ask for the subscription towards the
    /// server. The `unsubscribe_method` is used to close the subscription.
    pub async fn subscribe<Notif>(
        &self,
        subscribe_method: impl Into<String>,
        params: impl Into<jsonrpsee_core::common::Params>,
        unsubscribe_method: impl Into<String>,
    ) -> Result<Subscription<Notif>, ()> {
        let (send_back_tx, send_back_rx) = oneshot::channel();
        let _ = self
            .to_back
            .clone()
            .send(FrontToBack::Subscribe {
                subscribe_method: subscribe_method.into(),
                unsubscribe_method: unsubscribe_method.into(),
                params: params.into(),
                send_back: send_back_tx,
            })
            .await;

        let notifs_rx = send_back_rx.await.map_err(|_| ())?.map_err(|_| ())?;

        Ok(Subscription {
            to_back: self.to_back.clone(),
            notifs_rx,
            marker: PhantomData,
        })
    }*/
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

impl IncomingRequest {
    /// Returns the parameters of the request.
    pub fn params(&self) -> &common::Params {
        &self.params
    }

    /// Respond to the request.
    pub async fn respond(mut self, response: impl Into<Result<common::JsonValue, common::Error>>) {
        let _ = self.to_back.send(FrontToBack::AnswerRequest {
            request_id: self.request_id,
            answer: response.into(),
        }).await;
    }
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task<R, I>(mut server: Server<R, I>, mut from_front: mpsc::UnboundedReceiver<FrontToBack>)
where
    R: TransportServer<RequestId = I> + Send + 'static,
    I: Clone + PartialEq + Eq + Hash + Send + Sync + 'static,
{
    // List of notifications methods that the user has registered, and the channels to dispatch
    // incoming notifications.
    let mut registered_notifications: HashMap<String, mpsc::Sender<_>> = HashMap::new();
    // List of methods that the user has registered, and the channels to dispatch incoming
    // requests.
    let mut registered_methods: HashMap<String, mpsc::Sender<_>> = HashMap::new();

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
                server.request_by_id(&request_id).unwrap().respond(answer).await;
            }
            Either::Left(Some(FrontToBack::RegisterNotifications { name, handler })) => {
                registered_notifications.insert(name, handler);
            },
            Either::Left(Some(FrontToBack::RegisterMethod { name, handler })) => {
                registered_methods.insert(name, handler);
            },
            Either::Left(Some(FrontToBack::RegisterSubscription { subscribe_method, unsubscribe_method })) => {

            }
            Either::Right(ServerEvent::Notification(notification)) => {
                if let Some(handler) = registered_notifications.get_mut(notification.method()) {
                    let params: &common::Params = notification.params().into();
                    handler.send(params.clone()).await;
                }
            }
            Either::Right(ServerEvent::Request(request)) => {
                let rq_id = request.request_id();
                let method = request.method();
                let params = request.params();
                if let Some(handler) = registered_methods.get(request.method()) {
                    unimplemented!()    // TODO:
                    /*let params: &common::Params = request.params().into();
                    handler.send(params.clone()).await;*/
                } else {
                    request.respond(Err(From::from(common::ErrorCode::InvalidRequest))).await;
                }
            }
            Either::Right(ServerEvent::SubscriptionsReady(iter)) => {

            }
            Either::Right(ServerEvent::SubscriptionsClosed(iter)) => {

            }
        }
    }
}
