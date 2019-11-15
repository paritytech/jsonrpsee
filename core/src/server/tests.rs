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

#![cfg(test)]

use crate::{common, local_raw, RawClient, Server, ServerEvent};

#[test]
fn notifications_work() {
    let (mut client, mut server) = {
        let (c, s) = local_raw();
        (c, Server::new(s))
    };

    async_std::task::spawn(async move {
        let n = common::Notification {
            jsonrpc: common::Version::V2,
            method: "foo".to_string(),
            params: common::Params::Array(vec!["bar".to_string().into(), 52.into()]),
        };

        let request = common::Request::Single(common::Call::Notification(n));
        client.send_request(request).await.unwrap();
    });

    async_std::task::block_on(async move {
        match server.next_event().await {
            ServerEvent::Notification(n) => {
                assert_eq!(n.method(), "foo");
                assert_eq!(
                    {
                        let v: String = n.params().get(0).unwrap();
                        v
                    },
                    "bar"
                );
                assert_eq!(
                    {
                        let v: i32 = n.params().get(1).unwrap();
                        v
                    },
                    52
                );
            }
            _ => panic!(),
        }
    });
}

#[test]
fn subscriptions_work() {
    let (mut client, mut server) = {
        let (c, s) = local_raw();
        (c, Server::new(s))
    };

    async_std::task::spawn(async move {
        let request = common::Request::Single(common::Call::MethodCall(common::MethodCall {
            jsonrpc: common::Version::V2,
            method: "foo".to_string(),
            id: common::Id::Num(981),
            params: common::Params::Array(vec!["bar".to_string().into(), 52.into()]),
        }));

        client.send_request(request).await.unwrap();

        let sub_id = match client.next_response().await.unwrap() {
            common::Response::Single(common::Output::Success(succ)) => {
                assert_eq!(succ.id, common::Id::Num(981));
                succ.result
            }
            _ => panic!(),
        };

        for expected in &["hey there!", "notif #2"] {
            match client.next_response().await.unwrap() {
                common::Response::Notif(notif) => {
                    assert_eq!(notif.method, "foo");
                    assert_eq!(notif.params.subscription.clone().into_string(), sub_id);
                    assert_eq!(notif.params.result, *expected);
                }
                _ => panic!(),
            };
        }

        // We destroy the client here, which triggers the `SubscriptionsClosed` event on the
        // server side.
    });

    async_std::task::block_on(async move {
        let sub_id = match server.next_event().await {
            ServerEvent::Request(rq) => {
                assert_eq!(rq.method(), "foo");
                assert_eq!(
                    {
                        let v: String = rq.params().get(0).unwrap();
                        v
                    },
                    "bar"
                );
                assert_eq!(
                    {
                        let v: i32 = rq.params().get(1).unwrap();
                        v
                    },
                    52
                );

                rq.into_subscription().await.unwrap()
            }
            _ => panic!(),
        };

        match server.next_event().await {
            ServerEvent::SubscriptionsReady(ready) => {
                assert_eq!(ready, vec![sub_id]);
            }
            _ => panic!(),
        }

        server
            .subscription_by_id(sub_id)
            .unwrap()
            .push("hey there!")
            .await;
        server
            .subscription_by_id(sub_id)
            .unwrap()
            .push("notif #2")
            .await;

        match server.next_event().await {
            ServerEvent::SubscriptionsClosed(closed) => {
                assert_eq!(closed, vec![sub_id]);
            }
            _ => panic!(),
        }
    });
}
