#![cfg(test)]

use crate::{common, local_raw, Server, ServerEvent, RawClient};

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
            params: common::Params::Array(vec![
                "bar".to_string().into(),
                52.into(),
            ]),
        };

        let request = common::Request::Single(common::Call::Notification(n));
        client.send_request(request).await.unwrap();
    });

    async_std::task::block_on(async move {
        match server.next_event().await.unwrap() {
            ServerEvent::Notification(n) => {
                assert_eq!(n.method(), "foo");
                assert_eq!({ let v: String = n.params().get(0).unwrap(); v }, "bar");
                assert_eq!({ let v: i32 = n.params().get(1).unwrap(); v }, 52);
            },
            _ => panic!()
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
        let call = common::MethodCall {
            jsonrpc: common::Version::V2,
            method: "foo".to_string(),
            id: common::Id::Num(981),
            params: common::Params::Array(vec![
                "bar".to_string().into(),
                52.into(),
            ]),
        };

        let request = common::Request::Single(common::Call::MethodCall(call));
        client.send_request(request).await.unwrap();

        let sub_id = match client.next_response().await.unwrap() {
            common::Response::Single(common::Output::Success(succ)) => {
                assert_eq!(succ.id, common::Id::Num(981));
                succ.result
            }
            _ => panic!()
        };

        for expected in &["hey there!", "notif #2"] {
            match client.next_response().await.unwrap() {
                common::Response::Notif(notif) => {
                    assert_eq!(notif.method, "foo");
                    assert_eq!(notif.params.subscription.clone().into_string(), sub_id);
                    assert_eq!(notif.params.result, *expected);
                }
                _ => panic!()
            };
        }

        // TODO: unsubscribe
    });

    async_std::task::block_on(async move {
        let sub_id = match server.next_event().await.unwrap() {
            ServerEvent::Request(rq) => {
                assert_eq!(rq.method(), "foo");
                assert_eq!({ let v: String = rq.params().get(0).unwrap(); v }, "bar");
                assert_eq!({ let v: i32 = rq.params().get(1).unwrap(); v }, 52);

                rq.into_subscription().await.unwrap()
            },
            _ => panic!()
        };

        match server.next_event().await.unwrap() {
            ServerEvent::SubscriptionsReady(ready) => {
                assert_eq!(ready, vec![sub_id]);
            },
            _ => panic!()
        }

        server.subscription_by_id(sub_id).unwrap().push("hey there!").await;
        server.subscription_by_id(sub_id).unwrap().push("notif #2").await;

        match server.next_event().await.unwrap() {
            ServerEvent::SubscriptionsClosed(closed) => {
                assert_eq!(closed, vec![sub_id]);
            },
            _ => panic!()
        }
    });
}
