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
use helpers::{http_server, websocket_server};
use jsonrpsee::client::{HttpClient, HttpConfig, WsClient, WsConfig, WsSubscription};
use jsonrpsee::types::jsonrpc::{JsonValue, Params};

#[tokio::test]
async fn ws_subscription_works() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	websocket_server(server_started_tx);
	let server_addr = server_started_rx.await.unwrap();
	let uri = format!("ws://{}", server_addr);
	let client = WsClient::new(&uri, WsConfig::default()).await.unwrap();
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
	let uri = format!("ws://{}", server_addr);
	let client = WsClient::new(&uri, WsConfig::default()).await.unwrap();
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

	let mut clients = Vec::with_capacity(10);
	for _ in 0..10 {
		let uri = format!("ws://{}", server_addr);
		let client = WsClient::new(&uri, WsConfig::default()).await.unwrap();
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

	let mut clients = Vec::with_capacity(10);
	for _ in 0..10 {
		let uri = format!("ws://{}", server_addr);
		let client =
			WsClient::new(&uri, WsConfig { subscription_channel_capacity: u32::MAX as usize, ..Default::default() })
				.await
				.unwrap();
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

	let uri = format!("ws://{}", server_addr);
	let client =
		WsClient::new(&uri, WsConfig { subscription_channel_capacity: 4, ..Default::default() }).await.unwrap();
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
