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

#![cfg(test)]

mod helpers;

use std::net::SocketAddr;
use std::time::Duration;

use futures::channel::oneshot;
use helpers::{http_server, websocket_server, websocket_server_with_wait_period};
use jsonrpsee_http_client::{HttpClient, HttpConfig};
use jsonrpsee_types::jsonrpc::{JsonValue, Params};
use jsonrpsee_ws_client::{WsClient, WsConfig, WsSubscription};

#[tokio::test]
async fn ws_subscription_works() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	websocket_server(server_started_tx);
	let server_addr = server_started_rx.await.unwrap();
	let server_url = format!("ws://{}", server_addr);
	let config = WsConfig::with_url(&server_url);
	let client = WsClient::new(config).await.unwrap();
	let mut hello_sub: WsSubscription<JsonValue> =
		client.subscribe("subscribe_hello", Params::None, "unsubscribe_hello").await.unwrap();
	let mut foo_sub: WsSubscription<JsonValue> =
		client.subscribe("subscribe_foo", Params::None, "unsubscribe_foo").await.unwrap();

	for _ in 0..10 {
		let hello = hello_sub.next().await.unwrap();
		let foo = foo_sub.next().await.unwrap();
		assert_eq!(hello, JsonValue::String("hello from subscription".to_owned()));
		assert_eq!(foo, JsonValue::Number(1337_u64.into()));
	}
}

#[tokio::test]
async fn ws_method_call_works() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	websocket_server(server_started_tx);
	let server_addr = server_started_rx.await.unwrap();
	let server_url = format!("ws://{}", server_addr);
	let config = WsConfig::with_url(&server_url);
	let client = WsClient::new(config).await.unwrap();
	let response: JsonValue = client.request("say_hello", Params::None).await.unwrap();
	assert_eq!(response, JsonValue::String("hello".into()));
}

#[tokio::test]
async fn http_method_call_works() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	http_server(server_started_tx);
	let server_addr = server_started_rx.await.unwrap();
	let uri = format!("http://{}", server_addr);
	let client = HttpClient::new(&uri, HttpConfig::default()).unwrap();
	let response: JsonValue = client.request("say_hello", Params::None).await.unwrap();
	assert_eq!(response, JsonValue::String("hello".into()));
}

#[tokio::test]
async fn ws_subscription_several_clients() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	websocket_server(server_started_tx);
	let server_addr = server_started_rx.await.unwrap();
	let server_url = format!("ws://{}", server_addr);

	let mut clients = Vec::with_capacity(10);
	for _ in 0..10 {
		let config = WsConfig::with_url(&server_url);
		let client = WsClient::new(config).await.unwrap();
		let hello_sub: WsSubscription<JsonValue> =
			client.subscribe("subscribe_hello", Params::None, "unsubscribe_hello").await.unwrap();
		let foo_sub: WsSubscription<JsonValue> =
			client.subscribe("subscribe_foo", Params::None, "unsubscribe_foo").await.unwrap();
		clients.push((client, hello_sub, foo_sub))
	}
}

#[tokio::test]
async fn ws_subscription_several_clients_with_drop() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	websocket_server(server_started_tx);
	let server_addr = server_started_rx.await.unwrap();
	let server_url = format!("ws://{}", server_addr);

	let mut clients = Vec::with_capacity(10);
	for _ in 0..10 {
		let mut config = WsConfig::with_url(&server_url);
		config.max_subscription_capacity = u32::MAX as usize;

		let client = WsClient::new(config).await.unwrap();
		let hello_sub: WsSubscription<JsonValue> =
			client.subscribe("subscribe_hello", Params::None, "unsubscribe_hello").await.unwrap();
		let foo_sub: WsSubscription<JsonValue> =
			client.subscribe("subscribe_foo", Params::None, "unsubscribe_foo").await.unwrap();
		clients.push((client, hello_sub, foo_sub))
	}

	for _ in 0..10 {
		for (_client, hello_sub, foo_sub) in &mut clients {
			let hello = hello_sub.next().await.unwrap();
			let foo = foo_sub.next().await.unwrap();
			assert_eq!(hello, JsonValue::String("hello from subscription".to_owned()));
			assert_eq!(foo, JsonValue::Number(1337_u64.into()));
		}
	}

	for i in 0..5 {
		let (client, _, _) = clients.remove(i);
		drop(client);
	}

	// make sure nothing weird happened after dropping half the clients (should be `unsubscribed` in the server)
	// would be good to know that subscriptions actually were removed but not possible to verify at
	// this layer.
	for _ in 0..10 {
		for (_client, hello_sub, foo_sub) in &mut clients {
			let hello = hello_sub.next().await.unwrap();
			let foo = foo_sub.next().await.unwrap();
			assert_eq!(hello, JsonValue::String("hello from subscription".to_owned()));
			assert_eq!(foo, JsonValue::Number(1337_u64.into()));
		}
	}
}

#[tokio::test]
async fn ws_subscription_without_polling_doesnt_make_client_unuseable() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	websocket_server(server_started_tx);
	let server_addr = server_started_rx.await.unwrap();
	let server_url = format!("ws://{}", server_addr);

	let mut config = WsConfig::with_url(&server_url);
	config.max_subscription_capacity = 4;
	let client = WsClient::new(config).await.unwrap();
	let mut hello_sub: WsSubscription<JsonValue> =
		client.subscribe("subscribe_hello", Params::None, "unsubscribe_hello").await.unwrap();

	// don't poll the subscription stream for 2 seconds, should be full now.
	std::thread::sleep(Duration::from_secs(2));

	// Capacity is `num_sender` + `capacity`
	for _ in 0..5 {
		assert!(hello_sub.next().await.is_some());
	}

	// NOTE: this is now unuseable and unregistered.
	assert!(hello_sub.next().await.is_none());

	// The client should still be useable => make sure it still works.
	let _hello_req: JsonValue = client.request("say_hello", Params::None).await.unwrap();

	// The same subscription should be possible to register again.
	let mut other_sub: WsSubscription<JsonValue> =
		client.subscribe("subscribe_hello", Params::None, "unsubscribe_hello").await.unwrap();

	other_sub.next().await.unwrap();
}

#[tokio::test]
async fn ws_more_request_than_buffer_should_not_deadlock() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	let (concurrent_tx, concurrent_rx) = oneshot::channel::<()>();
	websocket_server_with_wait_period(server_started_tx, concurrent_rx);
	let server_addr = server_started_rx.await.unwrap();
	let server_url = format!("ws://{}", server_addr);

	let mut config = WsConfig::with_url(&server_url);
	config.max_subscription_capacity = 2;
	let client = WsClient::new(config).await.unwrap();

	let mut requests = Vec::new();
	//NOTE: we use less than 8 because of https://github.com/paritytech/jsonrpsee/issues/168.
	for _ in 0..6 {
		let c = client.clone();
		requests.push(tokio::spawn(async move {
			let _: JsonValue = c.request("say_hello", Params::None).await.unwrap();
		}));
	}

	concurrent_tx.send(()).unwrap();
	for req in requests {
		req.await.unwrap();
	}
}
