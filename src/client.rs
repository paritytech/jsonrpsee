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
    channel::{mpsc, oneshot},
    future::Either,
    pin_mut,
    prelude::*,
};
use jsonrpsee_core::{
    client::RawClientEvent,
    common::{self, JsonValue},
    RawClient, RawClientRequestId, TransportClient,
};
use std::{collections::HashMap, error, io, marker::PhantomData};

/// Client that can be cloned.
///
/// > **Note**: This struct is designed to be easy to use, but it works by maintaining a background
/// >           task running in parallel. If this is not desirable, you are encouraged to use the
/// >           [`RawClient`] struct instead.
#[derive(Clone)]
pub struct Client {
    /// Channel to send requests to the background task.
    to_back: mpsc::Sender<FrontToBack>,
}

/// Active subscription on a [`Client`].
pub struct Subscription<Notif> {
    /// Channel to send requests to the background task.
    to_back: mpsc::Sender<FrontToBack>,
    /// Channel from which we receive notifications from the server, as undecoded `JsonValue`s.
    notifs_rx: mpsc::Receiver<JsonValue>,
    /// Marker in order to pin the `Notif` parameter.
    marker: PhantomData<mpsc::Receiver<Notif>>,
}

/// Error produced by [`Client::request`] and [`Client::subscribe`].
#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    /// Networking error or error on the low-level protocol layer (e.g. missing field,
    /// invalid ID, etc.).
    #[error("Networking or low-level protocol error: {0}")]
    TransportError(#[source] Box<dyn error::Error + Send + Sync>),
    /// RawServer responded to our request with an error.
    #[error("Server responded to our request with an error: {0:?}")]
    Request(#[source] common::Error),
    /// Failed to parse the data that the server sent back to us.
    #[error("Parse error: {0}")]
    ParseError(#[source] common::ParseError),
}

/// Message that the [`Client`] can send to the background task.
enum FrontToBack {
    /// Send a one-shot notification to the server. The server doesn't give back any feedback.
    Notification {
        /// Method for the notification.
        method: String,
        /// Parameters to send to the server.
        params: common::Params,
    },

    /// Send a request to the server.
    StartRequest {
        /// Method for the request.
        method: String,
        /// Parameters of the request.
        params: common::Params,
        /// One-shot channel where to send back the outcome of that request.
        send_back: oneshot::Sender<Result<JsonValue, RequestError>>,
    },

    /// Send a subscription request to the server.
    Subscribe {
        /// Method for the subscription request.
        subscribe_method: String,
        /// Parameters to send for the subscription.
        params: common::Params,
        /// Method to use to later unsubscription. Used if the channel unexpectedly closes.
        unsubscribe_method: String,
        /// When we get a response from the server about that subscription, we send the result on
        /// this channel. If the subscription succeeds, we return a `Receiver` that will receive
        /// notifications.
        send_back: oneshot::Sender<Result<mpsc::Receiver<JsonValue>, RequestError>>,
    },

    /// When a request or subscription channel is closed, we send this message to the background
    /// task in order for it to garbage collect closed requests and subscriptions.
    ///
    /// While this means that closing a request or a subscription is a `O(n)` operation, it is
    /// expected that the volume of requests and subscriptions is low enough that this isn't
    /// a problem in practice.
    ChannelClosed,
}

impl Client {
    /// Initializes a new client based upon this raw client.
    pub fn new<R>(client: RawClient<R>) -> Client
    where
        R: TransportClient + Send + 'static,
        R::Error: Send + Sync,
    {
        let (to_back, from_front) = mpsc::channel(16);
        async_std::task::spawn(async move {
            background_task(client, from_front).await;
        });
        Client { to_back }
    }

    /// Send a notification to the server.
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
    ) -> Result<Ret, RequestError>
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
            Ok(Err(err)) => return Err(err),
            Err(_) => {
                let err = io::Error::new(io::ErrorKind::Other, "background task closed");
                return Err(RequestError::TransportError(Box::new(err)));
            }
        };

        common::from_value(json_value).map_err(RequestError::ParseError)
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
    ) -> Result<Subscription<Notif>, RequestError> {
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

        let notifs_rx = match send_back_rx.await {
            Ok(Ok(v)) => v,
            Ok(Err(err)) => return Err(err),
            Err(_) => {
                let err = io::Error::new(io::ErrorKind::Other, "background task closed");
                return Err(RequestError::TransportError(Box::new(err)));
            }
        };

        Ok(Subscription {
            to_back: self.to_back.clone(),
            notifs_rx,
            marker: PhantomData,
        })
    }
}

impl<R> From<RawClient<R>> for Client
where
    R: TransportClient + Send + 'static,
    R::Error: Send + Sync,
{
    fn from(client: RawClient<R>) -> Client {
        Client::new(client)
    }
}

impl<Notif> Subscription<Notif>
where
    Notif: common::DeserializeOwned,
{
    /// Returns the next notification sent from the server.
    ///
    /// Ignores any malformed packet.
    pub async fn next(&mut self) -> Notif {
        loop {
            match self.notifs_rx.next().await {
                Some(n) => {
                    if let Ok(parsed) = common::from_value(n) {
                        return parsed;
                    }
                }
                None => futures::pending!(),
            }
        }
    }
}

impl<Notif> Drop for Subscription<Notif> {
    fn drop(&mut self) {
        // We can't actually guarantee that this goes through. If the background task is busy, then
        // the channel's buffer will be full, and our unsubscription request will never make it.
        // However, when a notification arrives, the background task will realize that the channel
        // to the `Subscription` has been closed, and will perform the unsubscribe.
        let _ = self.to_back.send(FrontToBack::ChannelClosed).now_or_never();
    }
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task<R>(mut client: RawClient<R>, mut from_front: mpsc::Receiver<FrontToBack>)
where
    R: TransportClient + Send + 'static,
    R::Error: Send + Sync,
{
    // List of subscription requests that have been sent to the server, with the method name to
    // unsubscribe.
    let mut pending_subscriptions: HashMap<RawClientRequestId, (oneshot::Sender<_>, _)> =
        HashMap::new();
    // List of subscription that are active on the server, with the method name to unsubscribe.
    let mut active_subscriptions: HashMap<
        RawClientRequestId,
        (mpsc::Sender<common::JsonValue>, _),
    > = HashMap::new();
    // List of requests that the server must answer.
    let mut ongoing_requests: HashMap<RawClientRequestId, oneshot::Sender<Result<_, _>>> =
        HashMap::new();

    loop {
        // We need to do a little transformation in order to destroy the borrow to `client`
        // and `from_front`.
        let outcome = {
            let next_message = from_front.next();
            let next_event = client.next_event();
            pin_mut!(next_message);
            pin_mut!(next_event);
            match future::select(next_message, next_event).await {
                Either::Left((v, _)) => Either::Left(v),
                Either::Right((v, _)) => Either::Right(v),
            }
        };

        match outcome {
            // If the channel is closed, then the `Client` has been destroyed and we
            // stop this task.
            Either::Left(None) => return,

            // User called `notification` on the front-end.
            Either::Left(Some(FrontToBack::Notification { method, params })) => {
                let _ = client.send_notification(method, params).await;
            }

            // User called `request` on the front-end.
            Either::Left(Some(FrontToBack::StartRequest {
                method,
                params,
                send_back,
            })) => match client.start_request(method, params).await {
                Ok(id) => {
                    ongoing_requests.insert(id, send_back);
                }
                Err(err) => {
                    let _ = send_back.send(Err(RequestError::TransportError(Box::new(err))));
                }
            },

            // User called `subscribe` on the front-end.
            Either::Left(Some(FrontToBack::Subscribe {
                subscribe_method,
                unsubscribe_method,
                params,
                send_back,
            })) => match client.start_subscription(subscribe_method, params).await {
                Ok(id) => {
                    pending_subscriptions.insert(id, (send_back, unsubscribe_method));
                }
                Err(err) => {
                    let _ = send_back.send(Err(RequestError::TransportError(Box::new(err))));
                }
            },

            Either::Left(Some(FrontToBack::ChannelClosed)) => {
                // TODO: there's no way to cancel pending subscriptions and requests, otherwise
                // we should clean them up as well
                while let Some(rq_id) = active_subscriptions
                    .iter()
                    .find(|(_, (v, _))| v.is_closed())
                    .map(|(k, _)| *k)
                {
                    let (_, unsubscribe) = active_subscriptions.remove(&rq_id).unwrap();
                    client
                        .subscription_by_id(rq_id)
                        .unwrap()
                        .into_active()
                        .unwrap()
                        .close(unsubscribe)
                        .await
                        .unwrap();
                }
            }

            // Received a response to a request from the server.
            Either::Right(Ok(RawClientEvent::Response { request_id, result })) => {
                let _ = ongoing_requests
                    .remove(&request_id)
                    .unwrap()
                    .send(result.map_err(RequestError::Request));
            }

            // Receive a response from the server about a subscription.
            Either::Right(Ok(RawClientEvent::SubscriptionResponse { request_id, result })) => {
                let (send_back, unsubscribe) = pending_subscriptions.remove(&request_id).unwrap();
                if let Err(err) = result {
                    let _ = send_back.send(Err(RequestError::Request(err)));
                } else {
                    // TODO: what's a good limit here? way more tricky than it looks
                    let (notifs_tx, notifs_rx) = mpsc::channel(4);
                    if send_back.send(Ok(notifs_rx)).is_ok() {
                        active_subscriptions.insert(request_id, (notifs_tx, unsubscribe));
                    } else {
                        client
                            .subscription_by_id(request_id)
                            .unwrap()
                            .into_active()
                            .unwrap()
                            .close(unsubscribe)
                            .await
                            .unwrap();
                    }
                }
            }

            Either::Right(Ok(RawClientEvent::SubscriptionNotif { request_id, result })) => {
                // TODO: unsubscribe if channel is closed
                let (notifs_tx, _) = active_subscriptions.get_mut(&request_id).unwrap();
                if notifs_tx.send(result).await.is_err() {
                    let (_, unsubscribe) = active_subscriptions.remove(&request_id).unwrap();
                    client
                        .subscription_by_id(request_id)
                        .unwrap()
                        .into_active()
                        .unwrap()
                        .close(unsubscribe)
                        .await
                        .unwrap();
                }
            }

            // Request for the server to unsubscribe us has succeeded.
            Either::Right(Ok(RawClientEvent::Unsubscribed { request_id: _ })) => {}

            Either::Right(Err(e)) => {
                // TODO: https://github.com/paritytech/jsonrpsee/issues/67
                log::error!("Client Error: {:?}", e);
            }
        }
    }
}
