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
mod proc_macros;

use std::sync::Arc;
use std::time::Duration;

use helpers::{http_server, websocket_server, websocket_server_with_subscription};
use jsonrpsee_http_client::{HttpClient, HttpConfig};
use jsonrpsee_types::{
	error::Error,
	jsonrpc::{JsonValue, Params},
	traits::{Client, SubscriptionClient},
};
use jsonrpsee_ws_client::{WsClient, WsConfig, WsSubscription};

#[tokio::test]
async fn ws_subscription_works() {
	let server_addr = websocket_server_with_subscription().await;
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
	let server_addr = websocket_server().await;
	let server_url = format!("ws://{}", server_addr);
	let config = WsConfig::with_url(&server_url);
	let client = WsClient::new(config).await.unwrap();
	let response: JsonValue = client.request("say_hello", Params::None).await.unwrap();
	assert_eq!(response, JsonValue::String("hello".into()));
}

#[tokio::test]
async fn http_method_call_works() {
	let server_addr = http_server().await;
	let uri = format!("http://{}", server_addr);
	let client = HttpClient::new(&uri, HttpConfig::default()).unwrap();
	let response: JsonValue = client.request("say_hello", Params::None).await.unwrap();
	assert_eq!(response, JsonValue::String("hello".into()));
}

#[tokio::test]
async fn ws_subscription_several_clients() {
	let server_addr = websocket_server_with_subscription().await;
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
	let server_addr = websocket_server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let mut clients = Vec::with_capacity(10);
	for _ in 0..10 {
		let mut config = WsConfig::with_url(&server_url);
		config.max_notifs_per_subscription = u32::MAX as usize;

		let client = WsClient::new(config).await.unwrap();
		let hello_sub: WsSubscription<String> =
			client.subscribe("subscribe_hello", Params::None, "unsubscribe_hello").await.unwrap();
		let foo_sub: WsSubscription<u64> =
			client.subscribe("subscribe_foo", Params::None, "unsubscribe_foo").await.unwrap();
		clients.push((client, hello_sub, foo_sub))
	}

	for _ in 0..10 {
		for (_client, hello_sub, foo_sub) in &mut clients {
			let hello = hello_sub.next().await.unwrap();
			let foo = foo_sub.next().await.unwrap();
			assert_eq!(&hello, "hello from subscription");
			assert_eq!(foo, 1337);
		}
	}

	for i in 0..5 {
		let (client, hello_sub, foo_sub) = clients.remove(i);
		drop(hello_sub);
		drop(foo_sub);
		// Send this request to make sure that the client's background thread hasn't
		// been canceled.
		let _r: String = client.request("say_hello", Params::None).await.unwrap();
		drop(client);
	}

	// make sure nothing weird happened after dropping half the clients (should be `unsubscribed` in the server)
	// would be good to know that subscriptions actually were removed but not possible to verify at
	// this layer.
	for _ in 0..10 {
		for (_client, hello_sub, foo_sub) in &mut clients {
			let hello = hello_sub.next().await.unwrap();
			let foo = foo_sub.next().await.unwrap();
			assert_eq!(&hello, "hello from subscription");
			assert_eq!(foo, 1337);
		}
	}
}

#[tokio::test]
async fn ws_subscription_without_polling_doesnt_make_client_unuseable() {
	let server_addr = websocket_server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let mut config = WsConfig::with_url(&server_url);
	config.max_notifs_per_subscription = 4;
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
	let server_addr = websocket_server().await;
	let server_url = format!("ws://{}", server_addr);

	let mut config = WsConfig::with_url(&server_url);
	config.max_concurrent_requests = 2;
	let client = Arc::new(WsClient::new(config).await.unwrap());

	let mut requests = Vec::new();

	for _ in 0..6 {
		let c = client.clone();
		requests.push(tokio::spawn(async move { c.request::<String, _, _>("say_hello", Params::None).await }));
	}

	for req in requests {
		let _ = req.await.unwrap();
	}
}

#[tokio::test]
async fn wss_works() {
	let client = WsClient::new(WsConfig::with_url("wss://kusama-rpc.polkadot.io")).await.unwrap();
	let response: String = client.request("system_chain", Params::None).await.unwrap();
	assert_eq!(&response, "Kusama");
}

#[tokio::test]
async fn ws_with_non_ascii_url_doesnt_hang_or_panic() {
	let err = WsClient::new(WsConfig::with_url("wss://♥♥♥♥♥♥∀∂")).await;
	assert!(matches!(err, Err(Error::TransportError(_))));
}

#[tokio::test]
async fn http_with_non_ascii_url_doesnt_hang_or_panic() {
	let client = HttpClient::new("http://♥♥♥♥♥♥∀∂", HttpConfig::default()).unwrap();
	let err: Result<(), Error> = client.request("system_chain", Params::None).await;
	assert!(matches!(err, Err(Error::TransportError(_))));
}
