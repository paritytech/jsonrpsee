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
    client::ClientEvent,
    common::{self, JsonValue},
    Client, RawClient,
};
use std::{collections::HashMap, marker::PhantomData, mem};

#[derive(Clone)]
pub struct SharedClient {
    /// Channel to send requests to the background task.
    to_back: mpsc::Sender<FrontToBack>,
}

pub struct Subscription<Notif> {
    to_back: mpsc::Sender<FrontToBack>,
    notifs_rx: mpsc::Receiver<JsonValue>,
    /// Name of the method to call in order to unsubscribe.
    unsubscribe_method: String,
    /// Marker in order to pin the `Notif` parameter.
    marker: PhantomData<mpsc::Receiver<Notif>>,
}

enum FrontToBack {
    Notification {
        method: String,
        params: common::Params,
    },
    StartRequest {
        method: String,
        params: common::Params,
        send_back: oneshot::Sender<Result<JsonValue, common::Error>>,
    },
    Subscribe {
        subscribe_method: String,
        params: common::Params,
        notifs_tx: mpsc::Sender<JsonValue>,
        unsubscribe_method: String,
    },
    Unsubscribe {
        method: String,
    },
}

impl SharedClient {
    /// Initializes a new client based upon this raw client.
    pub fn new<R>(mut client: Client<R>) -> SharedClient
    where
        R: RawClient + Send + 'static,
        R::Error: Send,
    {
        let (to_back, mut from_front) = mpsc::channel(16);

        async_std::task::spawn(async move {
            let mut subscriptions = HashMap::new();
            let mut requests = HashMap::new();

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
                    // If the channel is closed, then the `SharedClient` has been destroyed and we
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
                    })) => {
                        if let Ok(id) = client.start_request(method, params).await {
                            requests.insert(id, send_back);
                        } else {
                            // TODO: send back error
                        }
                    }

                    // User called `subscribe` on the front-end.
                    Either::Left(Some(FrontToBack::Subscribe {
                        subscribe_method,
                        unsubscribe_method,
                        params,
                        notifs_tx,
                    })) => {
                        if let Ok(id) = client.start_subscription(subscribe_method, params).await {
                            subscriptions.insert(id, notifs_tx);
                        } else {
                            // TODO: send back error?
                        }
                    }

                    Either::Left(Some(FrontToBack::Unsubscribe { method })) => {}

                    // Received a response to a request from the server.
                    Either::Right(Ok(ClientEvent::Response { request_id, result })) => {
                        let _ = requests.remove(&request_id).unwrap().send(result);
                    }

                    Either::Right(Ok(ClientEvent::SubscriptionResponse { request_id, result })) => {
                    }

                    Either::Right(Ok(ClientEvent::SubscriptionNotif { request_id, result })) => {
                        subscriptions.get_mut(&request_id).unwrap().send(result).await;
                    }

                    // Request for the server to unsubscribe us has succeeded.
                    Either::Right(Ok(ClientEvent::Unsubscribed { request_id })) => {
                        subscriptions.remove(&request_id).unwrap();
                    }

                    Either::Right(Err(_)) => {} // TODO: https://github.com/paritytech/jsonrpsee/issues/67
                }
            }
        });

        SharedClient { to_back }
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

        let json_value = match send_back_rx.await {
            Ok(Ok(v)) => v,
            _ => return Err(()),
        };

        if let Ok(parsed) = common::from_value(json_value) {
            return Ok(parsed);
        }

        Err(())
    }

    pub async fn subscribe<Notif>(
        &self,
        subscribe_method: impl Into<String>,
        params: impl Into<jsonrpsee_core::common::Params>,
        unsubscribe_method: impl Into<String>,
    ) -> Subscription<Notif> {
        // TODO: what's a good limit here? way more tricky than it looks
        let (notifs_tx, notifs_rx) = mpsc::channel(4);
        let unsubscribe_method = unsubscribe_method.into();
        self.to_back
            .clone()
            .send(FrontToBack::Subscribe {
                subscribe_method: subscribe_method.into(),
                unsubscribe_method: unsubscribe_method.clone(),
                params: params.into(),
                notifs_tx,
            })
            .await;

        Subscription {
            to_back: self.to_back.clone(),
            notifs_rx,
            unsubscribe_method,
            marker: PhantomData,
        }
    }
}

impl<Notif> Subscription<Notif>
where
    Notif: common::DeserializeOwned,
{
    /// Returns the next notification sent from the server.
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
        let _ = self
            .to_back
            .send(FrontToBack::Unsubscribe {
                method: mem::replace(&mut self.unsubscribe_method, String::new()),
            })
            .now_or_never();
    }
}
