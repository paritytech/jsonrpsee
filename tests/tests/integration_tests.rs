// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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
#![allow(clippy::blacklisted_name)]

mod helpers;

use helpers::{http_server, websocket_server, websocket_server_with_subscription};
use jsonrpsee::{
	http_client::HttpClientBuilder,
	rpc_params,
	types::{
		error::SubscriptionClosedReason,
		traits::{Client, SubscriptionClient},
		Error, JsonValue, Subscription,
	},
	ws_client::WsClientBuilder,
};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn ws_subscription_works() {
	let (server_addr, _) = websocket_server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();
	let mut hello_sub: Subscription<String> =
		client.subscribe("subscribe_hello", None, "unsubscribe_hello").await.unwrap();
	let mut foo_sub: Subscription<u64> = client.subscribe("subscribe_foo", None, "unsubscribe_foo").await.unwrap();

	for _ in 0..10 {
		let hello = hello_sub.next().await.unwrap().unwrap();
		let foo = foo_sub.next().await.unwrap().unwrap();
		assert_eq!(hello, "hello from subscription".to_string());
		assert_eq!(foo, 1337);
	}
}

#[tokio::test]
async fn ws_subscription_with_input_works() {
	let (server_addr, _) = websocket_server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();
	let mut add_one: Subscription<u64> =
		client.subscribe("subscribe_add_one", rpc_params![1], "unsubscribe_add_one").await.unwrap();

	for i in 2..4 {
		let next = add_one.next().await.unwrap().unwrap();
		assert_eq!(next, i);
	}
}

#[tokio::test]
async fn ws_method_call_works() {
	let server_addr = websocket_server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();
	let response: String = client.request("say_hello", None).await.unwrap();
	assert_eq!(&response, "hello");
}

#[tokio::test]
async fn http_method_call_works() {
	let (server_addr, _handle) = http_server().await;
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&uri).unwrap();
	let response: String = client.request("say_hello", None).await.unwrap();
	assert_eq!(&response, "hello");
}

#[tokio::test]
async fn http_concurrent_method_call_limits_works() {
	let (server_addr, _handle) = http_server().await;
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().max_concurrent_requests(1).build(&uri).unwrap();

	let (first, second) =
		tokio::join!(client.request::<String>("say_hello", None), client.request::<String>("say_hello", None),);

	assert!(first.is_ok());
	assert!(matches!(second, Err(Error::MaxSlotsExceeded)));
}

#[tokio::test]
async fn ws_subscription_several_clients() {
	let (server_addr, _) = websocket_server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let mut clients = Vec::with_capacity(10);
	for _ in 0..10 {
		let client = WsClientBuilder::default().build(&server_url).await.unwrap();
		let hello_sub: Subscription<JsonValue> =
			client.subscribe("subscribe_hello", None, "unsubscribe_hello").await.unwrap();
		let foo_sub: Subscription<JsonValue> =
			client.subscribe("subscribe_foo", None, "unsubscribe_foo").await.unwrap();
		clients.push((client, hello_sub, foo_sub))
	}
}

#[tokio::test]
async fn ws_subscription_several_clients_with_drop() {
	let (server_addr, _) = websocket_server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let mut clients = Vec::with_capacity(10);
	for _ in 0..10 {
		let client =
			WsClientBuilder::default().max_notifs_per_subscription(u32::MAX as usize).build(&server_url).await.unwrap();
		let hello_sub: Subscription<String> =
			client.subscribe("subscribe_hello", None, "unsubscribe_hello").await.unwrap();
		let foo_sub: Subscription<u64> = client.subscribe("subscribe_foo", None, "unsubscribe_foo").await.unwrap();
		clients.push((client, hello_sub, foo_sub))
	}

	for _ in 0..10 {
		for (_client, hello_sub, foo_sub) in &mut clients {
			let hello = hello_sub.next().await.unwrap().unwrap();
			let foo = foo_sub.next().await.unwrap().unwrap();
			assert_eq!(&hello, "hello from subscription");
			assert_eq!(foo, 1337);
		}
	}

	for i in 0..5 {
		let (client, hello_sub, foo_sub) = clients.remove(i);
		drop(hello_sub);
		drop(foo_sub);
		assert!(client.is_connected());
		drop(client);
	}

	// make sure nothing weird happened after dropping half of the clients (should be `unsubscribed` in the server)
	// would be good to know that subscriptions actually were removed but not possible to verify at
	// this layer.
	for _ in 0..10 {
		for (client, hello_sub, foo_sub) in &mut clients {
			assert!(client.is_connected());
			let hello = hello_sub.next().await.unwrap().unwrap();
			let foo = foo_sub.next().await.unwrap().unwrap();
			assert_eq!(&hello, "hello from subscription");
			assert_eq!(foo, 1337);
		}
	}
}

#[tokio::test]
async fn ws_subscription_without_polling_doesnt_make_client_unuseable() {
	let (server_addr, _) = websocket_server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let client = WsClientBuilder::default().max_notifs_per_subscription(4).build(&server_url).await.unwrap();
	let mut hello_sub: Subscription<JsonValue> =
		client.subscribe("subscribe_hello", None, "unsubscribe_hello").await.unwrap();

	// don't poll the subscription stream for 2 seconds, should be full now.
	tokio::time::sleep(Duration::from_secs(2)).await;

	// Capacity is `num_sender` + `capacity`
	for _ in 0..5 {
		assert!(hello_sub.next().await.unwrap().is_ok());
	}

	// NOTE: this is now unuseable and unregistered.
	assert!(hello_sub.next().await.is_none());

	// The client should still be useable => make sure it still works.
	let _hello_req: JsonValue = client.request("say_hello", None).await.unwrap();

	// The same subscription should be possible to register again.
	let mut other_sub: Subscription<JsonValue> =
		client.subscribe("subscribe_hello", None, "unsubscribe_hello").await.unwrap();

	other_sub.next().await.unwrap().unwrap();
}

#[tokio::test]
async fn ws_making_more_requests_than_allowed_should_not_deadlock() {
	let server_addr = websocket_server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = Arc::new(WsClientBuilder::default().max_concurrent_requests(2).build(&server_url).await.unwrap());

	let mut requests = Vec::new();

	for _ in 0..6 {
		let c = client.clone();
		requests.push(tokio::spawn(async move { c.request::<String>("say_hello", None).await }));
	}

	for req in requests {
		let _ = req.await.unwrap();
	}
}

#[tokio::test]
async fn http_making_more_requests_than_allowed_should_not_deadlock() {
	let (server_addr, _handle) = http_server().await;
	let server_url = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().max_concurrent_requests(2).build(&server_url).unwrap();
	let client = Arc::new(client);

	let mut requests = Vec::new();

	for _ in 0..6 {
		let c = client.clone();
		requests.push(tokio::spawn(async move { c.request::<String>("say_hello", None).await }));
	}

	for req in requests {
		let _ = req.await.unwrap();
	}
}

#[tokio::test]
#[ignore]
async fn https_works() {
	let client = HttpClientBuilder::default().build("https://kusama-rpc.polkadot.io").unwrap();
	let response: String = client.request("system_chain", None).await.unwrap();
	assert_eq!(&response, "Kusama");
}

#[tokio::test]
#[ignore]
async fn wss_works() {
	let client = WsClientBuilder::default().build("wss://kusama-rpc.polkadot.io").await.unwrap();
	let response: String = client.request("system_chain", None).await.unwrap();
	assert_eq!(&response, "Kusama");
}

#[tokio::test]
async fn ws_with_non_ascii_url_doesnt_hang_or_panic() {
	let err = WsClientBuilder::default().build("wss://♥♥♥♥♥♥∀∂").await;
	assert!(matches!(err, Err(Error::Transport(_))));
}

#[tokio::test]
async fn http_with_non_ascii_url_doesnt_hang_or_panic() {
	let err = HttpClientBuilder::default().build("http://♥♥♥♥♥♥∀∂");
	assert!(matches!(err, Err(Error::Transport(_))));
}

#[tokio::test]
async fn ws_unsubscribe_releases_request_slots() {
	let (server_addr, _) = websocket_server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let client = WsClientBuilder::default().max_concurrent_requests(1).build(&server_url).await.unwrap();

	let sub1: Subscription<JsonValue> = client.subscribe("subscribe_hello", None, "unsubscribe_hello").await.unwrap();
	drop(sub1);
	let _: Subscription<JsonValue> = client.subscribe("subscribe_hello", None, "unsubscribe_hello").await.unwrap();
}

#[tokio::test]
async fn server_should_be_able_to_close_subscriptions() {
	let (server_addr, _) = websocket_server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	let mut sub: Subscription<String> = client.subscribe("subscribe_noop", None, "unsubscribe_noop").await.unwrap();

	let res = sub.next().await;

	assert!(matches!(res, Some(Err(Error::SubscriptionClosed(_)))));
}

#[tokio::test]
async fn ws_close_pending_subscription_when_server_terminated() {
	let (server_addr, handle) = websocket_server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let c1 = WsClientBuilder::default().build(&server_url).await.unwrap();

	let mut sub: Subscription<String> = c1.subscribe("subscribe_hello", None, "unsubscribe_hello").await.unwrap();

	assert!(matches!(sub.next().await, Some(Ok(_))));

	handle.stop().unwrap().await;

	let sub2: Result<Subscription<String>, _> = c1.subscribe("subscribe_hello", None, "unsubscribe_hello").await;

	// no new request should be accepted.
	assert!(matches!(sub2, Err(_)));

	// consume final message
	for _ in 0..2 {
		match sub.next().await {
			// All good, exit test
			None => return,
			// Try again
			_ => continue,
		}
	}

	panic!("subscription keeps sending messages after server shutdown");
}

#[tokio::test]
async fn ws_server_should_stop_subscription_after_client_drop() {
	use futures::{channel::mpsc, SinkExt, StreamExt};
	use jsonrpsee::{ws_server::WsServerBuilder, RpcModule};

	let server = WsServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let server_url = format!("ws://{}", server.local_addr().unwrap());

	let (tx, mut rx) = mpsc::channel(1);
	let mut module = RpcModule::new(tx);

	module
		.register_subscription("subscribe_hello", "subscribe_hello", "unsubscribe_hello", |_, mut sink, mut tx| {
			tokio::spawn(async move {
				let close_err = loop {
					if let Err(Error::SubscriptionClosed(err)) = sink.send(&1) {
						break err;
					}
					tokio::time::sleep(Duration::from_millis(100)).await;
				};
				let send_back = Arc::make_mut(&mut tx);
				send_back.feed(close_err).await.unwrap();
			});
			Ok(())
		})
		.unwrap();

	server.start(module).unwrap();

	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	let mut sub: Subscription<usize> = client.subscribe("subscribe_hello", None, "unsubscribe_hello").await.unwrap();

	let res = sub.next().await.unwrap().unwrap();

	assert_eq!(res, 1);
	drop(client);
	let close_err = rx.next().await.unwrap();

	// assert that the server received `SubscriptionClosed` after the client was dropped.
	assert!(matches!(close_err.close_reason(), &SubscriptionClosedReason::ConnectionReset));
}

#[tokio::test]
async fn ws_batch_works() {
	let server_addr = websocket_server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	let mut batch = Vec::new();

	batch.push(("say_hello", rpc_params![]));
	batch.push(("slow_hello", rpc_params![]));

	let responses: Vec<String> = client.batch_request(batch).await.unwrap();
	assert_eq!(responses, vec!["hello".to_string(), "hello".to_string()]);
}
