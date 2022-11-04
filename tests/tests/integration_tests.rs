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

use std::sync::Arc;
use std::time::Duration;

use futures::{channel::mpsc, StreamExt, TryStreamExt};
use helpers::{
	init_logger, server, server_with_access_control, server_with_health_api, server_with_subscription,
	server_with_subscription_and_handle,
};
use hyper::http::HeaderValue;
use jsonrpsee::core::client::{ClientT, IdKind, Subscription, SubscriptionClientT};
use jsonrpsee::core::error::SubscriptionClosed;
use jsonrpsee::core::params::{ArrayParams, BatchRequestBuilder};
use jsonrpsee::core::{Error, JsonValue};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use jsonrpsee::types::error::{ErrorObject, UNKNOWN_ERROR_CODE};
use jsonrpsee::ws_client::WsClientBuilder;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use tower_http::cors::CorsLayer;

#[tokio::test]
async fn ws_subscription_works() {
	init_logger();

	let server_addr = server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();
	let mut hello_sub: Subscription<String> =
		client.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await.unwrap();
	let mut foo_sub: Subscription<u64> =
		client.subscribe("subscribe_foo", rpc_params![], "unsubscribe_foo").await.unwrap();

	for _ in 0..10 {
		let hello = hello_sub.next().await.unwrap().unwrap();
		let foo = foo_sub.next().await.unwrap().unwrap();
		assert_eq!(hello, "hello from subscription".to_string());
		assert_eq!(foo, 1337);
	}
}

#[tokio::test]
async fn ws_unsubscription_works() {
	init_logger();

	let server_addr = server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().max_concurrent_requests(1).build(&server_url).await.unwrap();

	let mut sub: Subscription<usize> =
		client.subscribe("subscribe_foo", rpc_params![], "unsubscribe_foo").await.unwrap();

	// It's technically possible to have race-conditions between the notifications and the unsubscribe message.
	// So let's wait for the first notification and then unsubscribe.
	let _item = sub.next().await.unwrap().unwrap();

	sub.unsubscribe().await.unwrap();

	let mut success = false;

	// Wait until a slot is available, as only one concurrent call is allowed.
	// Then when this finishes we know that unsubscribe call has been finished.
	for _ in 0..30 {
		let res: Result<String, _> = client.request("say_hello", rpc_params![]).await;
		if res.is_ok() {
			success = true;
			break;
		}
		tokio::time::sleep(std::time::Duration::from_millis(100)).await;
	}

	assert!(success);
}

#[tokio::test]
async fn ws_subscription_with_input_works() {
	init_logger();

	let server_addr = server_with_subscription().await;
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
	init_logger();

	let server_addr = server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();
	let response: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(&response, "hello");
}

#[tokio::test]
async fn ws_method_call_str_id_works() {
	init_logger();

	let server_addr = server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().id_format(IdKind::String).build(&server_url).await.unwrap();
	let response: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(&response, "hello");
}

#[tokio::test]
async fn http_method_call_works() {
	init_logger();

	let server_addr = server().await;
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&uri).unwrap();
	let response: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(&response, "hello");
}

#[tokio::test]
async fn http_method_call_str_id_works() {
	init_logger();

	let server_addr = server().await;
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().id_format(IdKind::String).build(&uri).unwrap();
	let response: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(&response, "hello");
}

#[tokio::test]
async fn http_concurrent_method_call_limits_works() {
	init_logger();

	let server_addr = server().await;
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().max_concurrent_requests(1).build(&uri).unwrap();

	let (first, second) = tokio::join!(
		client.request::<String, ArrayParams>("say_hello", rpc_params!()),
		client.request::<String, ArrayParams>("say_hello", rpc_params![]),
	);

	assert!(first.is_ok());
	assert!(matches!(second, Err(Error::MaxSlotsExceeded)));
}

#[tokio::test]
async fn ws_subscription_several_clients() {
	init_logger();

	let server_addr = server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let mut clients = Vec::with_capacity(10);
	for _ in 0..10 {
		let client = WsClientBuilder::default().build(&server_url).await.unwrap();
		let hello_sub: Subscription<JsonValue> =
			client.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await.unwrap();
		let foo_sub: Subscription<JsonValue> =
			client.subscribe("subscribe_foo", rpc_params![], "unsubscribe_foo").await.unwrap();
		clients.push((client, hello_sub, foo_sub))
	}
}

#[tokio::test]
async fn ws_subscription_several_clients_with_drop() {
	init_logger();

	let server_addr = server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let mut clients = Vec::with_capacity(10);
	for _ in 0..10 {
		let client =
			WsClientBuilder::default().max_notifs_per_subscription(u32::MAX as usize).build(&server_url).await.unwrap();
		let hello_sub: Subscription<String> =
			client.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await.unwrap();
		let foo_sub: Subscription<u64> =
			client.subscribe("subscribe_foo", rpc_params![], "unsubscribe_foo").await.unwrap();
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
	init_logger();

	let server_addr = server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let client = WsClientBuilder::default().max_notifs_per_subscription(4).build(&server_url).await.unwrap();
	let mut hello_sub: Subscription<JsonValue> =
		client.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await.unwrap();

	// don't poll the subscription stream for 2 seconds, should be full now.
	tokio::time::sleep(Duration::from_secs(2)).await;

	// Capacity is `num_sender` + `capacity`
	for _ in 0..5 {
		assert!(hello_sub.next().await.unwrap().is_ok());
	}

	// NOTE: this is now unuseable and unregistered.
	assert!(hello_sub.next().await.is_none());

	// The client should still be useable => make sure it still works.
	let _hello_req: JsonValue = client.request("say_hello", rpc_params![]).await.unwrap();

	// The same subscription should be possible to register again.
	let mut other_sub: Subscription<JsonValue> =
		client.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await.unwrap();

	other_sub.next().await.unwrap().unwrap();
}

#[tokio::test]
async fn ws_making_more_requests_than_allowed_should_not_deadlock() {
	init_logger();

	let server_addr = server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = Arc::new(WsClientBuilder::default().max_concurrent_requests(2).build(&server_url).await.unwrap());

	let mut requests = Vec::new();

	for _ in 0..6 {
		let c = client.clone();
		requests.push(tokio::spawn(async move { c.request::<String, ArrayParams>("say_hello", rpc_params![]).await }));
	}

	for req in requests {
		let _ = req.await.unwrap();
	}
}

#[tokio::test]
async fn http_making_more_requests_than_allowed_should_not_deadlock() {
	init_logger();

	let server_addr = server().await;
	let server_url = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().max_concurrent_requests(2).build(&server_url).unwrap();
	let client = Arc::new(client);

	let mut requests = Vec::new();

	for _ in 0..6 {
		let c = client.clone();
		requests.push(tokio::spawn(async move { c.request::<String, ArrayParams>("say_hello", rpc_params![]).await }));
	}

	for req in requests {
		let _ = req.await.unwrap();
	}
}

#[tokio::test]
async fn https_works() {
	init_logger();

	let client = HttpClientBuilder::default().build("https://kusama-rpc.polkadot.io:443").unwrap();
	let response: String = client.request("system_chain", rpc_params![]).await.unwrap();
	assert_eq!(&response, "Kusama");
}

#[tokio::test]
async fn wss_works() {
	init_logger();

	let client = WsClientBuilder::default().build("wss://kusama-rpc.polkadot.io:443").await.unwrap();
	let response: String = client.request("system_chain", rpc_params![]).await.unwrap();
	assert_eq!(&response, "Kusama");
}

#[tokio::test]
async fn ws_with_non_ascii_url_doesnt_hang_or_panic() {
	init_logger();

	let err = WsClientBuilder::default().build("wss://♥♥♥♥♥♥∀∂").await;
	assert!(matches!(err, Err(Error::Transport(_))));
}

#[tokio::test]
async fn http_with_non_ascii_url_doesnt_hang_or_panic() {
	init_logger();

	let err = HttpClientBuilder::default().build("http://♥♥♥♥♥♥∀∂");
	assert!(matches!(err, Err(Error::Transport(_))));
}

#[tokio::test]
async fn ws_unsubscribe_releases_request_slots() {
	init_logger();

	let server_addr = server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let client = WsClientBuilder::default().max_concurrent_requests(1).build(&server_url).await.unwrap();

	let sub1: Subscription<JsonValue> =
		client.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await.unwrap();
	drop(sub1);
	let _: Subscription<JsonValue> =
		client.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await.unwrap();
}

#[tokio::test]
async fn server_should_be_able_to_close_subscriptions() {
	init_logger();

	let server_addr = server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);

	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	let mut sub: Subscription<String> =
		client.subscribe("subscribe_noop", rpc_params![], "unsubscribe_noop").await.unwrap();

	assert!(sub.next().await.is_none());
}

#[tokio::test]
async fn ws_close_pending_subscription_when_server_terminated() {
	init_logger();

	let (server_addr, server_handle) = server_with_subscription_and_handle().await;
	let server_url = format!("ws://{}", server_addr);

	let c1 = WsClientBuilder::default().build(&server_url).await.unwrap();

	let mut sub: Subscription<String> =
		c1.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await.unwrap();

	assert!(matches!(sub.next().await, Some(Ok(_))));

	server_handle.stop().unwrap();
	server_handle.stopped().await;

	let sub2: Result<Subscription<String>, _> =
		c1.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await;

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
	use jsonrpsee::{server::ServerBuilder, RpcModule};

	init_logger();

	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let server_url = format!("ws://{}", server.local_addr().unwrap());

	let (tx, mut rx) = mpsc::channel(1);
	let mut module = RpcModule::new(tx);

	module
		.register_subscription("subscribe_hello", "subscribe_hello", "unsubscribe_hello", |_, mut sink, mut tx| {
			sink.accept().unwrap();
			tokio::spawn(async move {
				let close_err = loop {
					if !sink.send(&1_usize).expect("usize can be serialized; qed") {
						break ErrorObject::borrowed(0, &"Subscription terminated successfully", None);
					}
					tokio::time::sleep(Duration::from_millis(100)).await;
				};
				let send_back = Arc::make_mut(&mut tx);
				send_back.feed(close_err).await.unwrap();
			});
			Ok(())
		})
		.unwrap();

	let _handle = server.start(module).unwrap();

	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	let mut sub: Subscription<usize> =
		client.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await.unwrap();

	let res = sub.next().await.unwrap().unwrap();

	assert_eq!(res, 1);
	drop(client);
	let close_err = rx.next().await.unwrap();

	// assert that the server received `SubscriptionClosed` after the client was dropped.
	assert_eq!(close_err, ErrorObject::borrowed(0, &"Subscription terminated successfully", None));
}

#[tokio::test]
async fn ws_server_notify_client_on_disconnect() {
	use futures::channel::oneshot;

	init_logger();

	let (server_addr, server_handle) = server_with_subscription_and_handle().await;
	let server_url = format!("ws://{}", server_addr);

	let (up_tx, up_rx) = oneshot::channel();
	let (dis_tx, mut dis_rx) = oneshot::channel();
	let (multiple_tx, multiple_rx) = oneshot::channel();

	tokio::spawn(async move {
		let client = WsClientBuilder::default().build(&server_url).await.unwrap();
		// Validate server is up.
		client.request::<String, ArrayParams>("say_hello", rpc_params![]).await.unwrap();

		// Signal client is waiting for the server to disconnect.
		up_tx.send(()).unwrap();

		client.on_disconnect().await;

		// Signal disconnect finished.
		dis_tx.send(()).unwrap();

		// Call `on_disconnect` a few more times to ensure it does not block.
		client.on_disconnect().await;
		client.on_disconnect().await;
		multiple_tx.send(()).unwrap();
	});

	// Ensure the client validated the server and is waiting for the disconnect.
	up_rx.await.unwrap();

	// Let A = dis_rx try_recv and server stop
	//     B = client on_disconnect
	//
	// Precautionary wait to ensure that a buggy `on_disconnect` (B) cannot be called
	// after the server shutdowns (A).
	tokio::time::sleep(Duration::from_secs(5)).await;

	// Make sure the `on_disconnect` method did not return before stopping the server.
	assert_eq!(dis_rx.try_recv().unwrap(), None);

	server_handle.stop().unwrap();
	server_handle.stopped().await;

	// The `on_disconnect()` method returned.
	dis_rx.await.unwrap();

	// Multiple `on_disconnect()` calls did not block.
	multiple_rx.await.unwrap();
}

#[tokio::test]
async fn ws_server_notify_client_on_disconnect_with_closed_server() {
	init_logger();

	let (server_addr, server_handle) = server_with_subscription_and_handle().await;
	let server_url = format!("ws://{}", server_addr);

	let client = WsClientBuilder::default().build(&server_url).await.unwrap();
	// Validate server is up.
	client.request::<String, ArrayParams>("say_hello", rpc_params![]).await.unwrap();

	// Stop the server.
	server_handle.stop().unwrap();
	server_handle.stopped().await;

	// Ensure `on_disconnect` returns when the call is made after the server is closed.
	client.on_disconnect().await;
}

#[tokio::test]
async fn ws_server_cancels_subscriptions_on_reset_conn() {
	init_logger();

	let (tx, rx) = mpsc::channel(1);
	let server_url = format!("ws://{}", helpers::server_with_sleeping_subscription(tx).await);

	let client = WsClientBuilder::default().build(&server_url).await.unwrap();
	let mut subs = Vec::new();

	for _ in 0..10 {
		subs.push(
			client
				.subscribe::<usize, ArrayParams>("subscribe_sleep", rpc_params![], "unsubscribe_sleep")
				.await
				.unwrap(),
		);
	}

	// terminate connection.
	drop(client);

	let rx_len = rx.take(10).fold(0, |acc, _| async move { acc + 1 }).await;

	assert_eq!(rx_len, 10);
}

#[tokio::test]
async fn ws_server_cancels_sub_stream_after_err() {
	init_logger();

	let addr = server_with_subscription().await;
	let server_url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&server_url).await.unwrap();
	let mut sub: Subscription<serde_json::Value> = client
		.subscribe("subscribe_with_err_on_stream", rpc_params![], "unsubscribe_with_err_on_stream")
		.await
		.unwrap();

	assert_eq!(sub.next().await.unwrap().unwrap(), 1);
	// The server closed down the subscription with the underlying error from the stream.
	assert!(sub.next().await.is_none());
}

#[tokio::test]
async fn ws_server_subscribe_with_stream() {
	init_logger();

	let addr = server_with_subscription().await;
	let server_url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&server_url).await.unwrap();
	let mut sub1: Subscription<usize> =
		client.subscribe("subscribe_5_ints", rpc_params![], "unsubscribe_5_ints").await.unwrap();
	let mut sub2: Subscription<usize> =
		client.subscribe("subscribe_5_ints", rpc_params![], "unsubscribe_5_ints").await.unwrap();

	let (r1, r2) = futures::future::try_join(
		sub1.by_ref().take(2).try_collect::<Vec<_>>(),
		sub2.by_ref().take(3).try_collect::<Vec<_>>(),
	)
	.await
	.unwrap();

	assert_eq!(r1, vec![1, 2]);
	assert_eq!(r2, vec![1, 2, 3]);

	// Be rude, don't run the destructor
	std::mem::forget(sub2);

	// sub1 is still in business, read remaining items.
	assert_eq!(sub1.by_ref().take(3).try_collect::<Vec<usize>>().await.unwrap(), vec![3, 4, 5]);

	assert!(sub1.next().await.is_none());
}

#[tokio::test]
async fn ws_server_pipe_from_stream_should_cancel_tasks_immediately() {
	init_logger();

	let (tx, rx) = mpsc::channel(1);
	let server_url = format!("ws://{}", helpers::server_with_sleeping_subscription(tx).await);

	let client = WsClientBuilder::default().build(&server_url).await.unwrap();
	let mut subs = Vec::new();

	for _ in 0..10 {
		subs.push(
			client.subscribe::<i32, ArrayParams>("subscribe_sleep", rpc_params![], "unsubscribe_sleep").await.unwrap(),
		)
	}

	// This will call the `unsubscribe method`.
	drop(subs);

	let rx_len = rx.take(10).fold(0, |acc, _| async move { acc + 1 }).await;

	assert_eq!(rx_len, 10);
}

#[tokio::test]
async fn ws_server_pipe_from_stream_can_be_reused() {
	init_logger();

	let addr = server_with_subscription().await;
	let client = WsClientBuilder::default().build(&format!("ws://{}", addr)).await.unwrap();
	let sub = client
		.subscribe::<i32, ArrayParams>("can_reuse_subscription", rpc_params![], "u_can_reuse_subscription")
		.await
		.unwrap();

	let items = sub.fold(0, |acc, _| async move { acc + 1 }).await;

	assert_eq!(items, 10);
}

#[tokio::test]
async fn ws_batch_works() {
	init_logger();

	let server_addr = server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	let mut batch = BatchRequestBuilder::new();
	batch.insert("say_hello", rpc_params![]).unwrap();
	batch.insert("slow_hello", rpc_params![]).unwrap();

	let res = client.batch_request::<String>(batch).await.unwrap();
	assert_eq!(res.len(), 2);
	assert_eq!(res.num_successful_calls(), 2);
	assert_eq!(res.num_failed_calls(), 0);
	let responses: Vec<_> = res.into_ok().unwrap().collect();
	assert_eq!(responses, vec!["hello".to_string(), "hello".to_string()]);

	let mut batch = BatchRequestBuilder::new();
	batch.insert("say_hello", rpc_params![]).unwrap();
	batch.insert("err", rpc_params![]).unwrap();

	let res = client.batch_request::<String>(batch).await.unwrap();
	assert_eq!(res.len(), 2);
	assert_eq!(res.num_successful_calls(), 1);
	assert_eq!(res.num_failed_calls(), 1);

	let ok_responses: Vec<_> = res.iter().filter_map(|r| r.as_ref().ok()).collect();
	let err_responses: Vec<_> = res
		.iter()
		.filter_map(|r| match r {
			Err(e) => Some(e),
			_ => None,
		})
		.collect();
	assert_eq!(ok_responses, vec!["hello"]);
	assert_eq!(err_responses, vec![&ErrorObject::borrowed(UNKNOWN_ERROR_CODE, &"Custom error: err", None)]);
}

#[tokio::test]
async fn http_batch_works() {
	init_logger();

	let server_addr = server().await;
	let server_url = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&server_url).unwrap();

	let mut batch = BatchRequestBuilder::new();
	batch.insert("say_hello", rpc_params![]).unwrap();
	batch.insert("slow_hello", rpc_params![]).unwrap();

	let res = client.batch_request::<String>(batch).await.unwrap();
	assert_eq!(res.len(), 2);
	assert_eq!(res.num_successful_calls(), 2);
	assert_eq!(res.num_failed_calls(), 0);
	let responses: Vec<_> = res.into_ok().unwrap().collect();
	assert_eq!(responses, vec!["hello".to_string(), "hello".to_string()]);

	let mut batch = BatchRequestBuilder::new();
	batch.insert("say_hello", rpc_params![]).unwrap();
	batch.insert("err", rpc_params![]).unwrap();

	let res = client.batch_request::<String>(batch).await.unwrap();
	assert_eq!(res.len(), 2);
	assert_eq!(res.num_successful_calls(), 1);
	assert_eq!(res.num_failed_calls(), 1);

	let ok_responses: Vec<_> = res.iter().filter_map(|r| r.as_ref().ok()).collect();
	let err_responses: Vec<_> = res
		.iter()
		.filter_map(|r| match r {
			Err(e) => Some(e),
			_ => None,
		})
		.collect();
	assert_eq!(ok_responses, vec!["hello"]);
	assert_eq!(err_responses, vec![&ErrorObject::borrowed(UNKNOWN_ERROR_CODE, &"Custom error: err", None)]);
}

#[tokio::test]
async fn ws_server_limit_subs_per_conn_works() {
	use futures::StreamExt;
	use jsonrpsee::types::error::{CallError, TOO_MANY_SUBSCRIPTIONS_CODE, TOO_MANY_SUBSCRIPTIONS_MSG};
	use jsonrpsee::{server::ServerBuilder, RpcModule};

	init_logger();

	let server = ServerBuilder::default().max_subscriptions_per_connection(10).build("127.0.0.1:0").await.unwrap();
	let server_url = format!("ws://{}", server.local_addr().unwrap());

	let mut module = RpcModule::new(());

	module
		.register_subscription("subscribe_forever", "n", "unsubscribe_forever", |_, mut sink, _| {
			tokio::spawn(async move {
				let interval = interval(Duration::from_millis(50));
				let stream = IntervalStream::new(interval).map(move |_| 0_usize);

				match sink.pipe_from_stream(stream).await {
					SubscriptionClosed::Success => {
						sink.close(SubscriptionClosed::Success);
					}
					_ => unreachable!(),
				};
			});
			Ok(())
		})
		.unwrap();
	let _handle = server.start(module).unwrap();

	let c1 = WsClientBuilder::default().build(&server_url).await.unwrap();
	let c2 = WsClientBuilder::default().build(&server_url).await.unwrap();

	let mut subs1 = Vec::new();
	let mut subs2 = Vec::new();

	for _ in 0..10 {
		subs1.push(
			c1.subscribe::<usize, ArrayParams>("subscribe_forever", rpc_params![], "unsubscribe_forever")
				.await
				.unwrap(),
		);
		subs2.push(
			c2.subscribe::<usize, ArrayParams>("subscribe_forever", rpc_params![], "unsubscribe_forever")
				.await
				.unwrap(),
		);
	}

	let err1 = c1.subscribe::<usize, ArrayParams>("subscribe_forever", rpc_params![], "unsubscribe_forever").await;
	let err2 = c1.subscribe::<usize, ArrayParams>("subscribe_forever", rpc_params![], "unsubscribe_forever").await;

	let data = "\"Exceeded max limit of 10\"";

	assert!(
		matches!(err1, Err(Error::Call(CallError::Custom(err))) if err.code() == TOO_MANY_SUBSCRIPTIONS_CODE && err.message() == TOO_MANY_SUBSCRIPTIONS_MSG && err.data().unwrap().get() == data)
	);
	assert!(
		matches!(err2, Err(Error::Call(CallError::Custom(err))) if err.code() == TOO_MANY_SUBSCRIPTIONS_CODE && err.message() == TOO_MANY_SUBSCRIPTIONS_MSG && err.data().unwrap().get() == data)
	);
}

#[tokio::test]
async fn ws_server_unsub_methods_should_ignore_sub_limit() {
	use futures::StreamExt;
	use jsonrpsee::core::client::SubscriptionKind;
	use jsonrpsee::{server::ServerBuilder, RpcModule};

	init_logger();

	let server = ServerBuilder::default().max_subscriptions_per_connection(10).build("127.0.0.1:0").await.unwrap();
	let server_url = format!("ws://{}", server.local_addr().unwrap());

	let mut module = RpcModule::new(());

	module
		.register_subscription("subscribe_forever", "n", "unsubscribe_forever", |_, mut sink, _| {
			tokio::spawn(async move {
				let interval = interval(Duration::from_millis(50));
				let stream = IntervalStream::new(interval).map(move |_| 0_usize);

				match sink.pipe_from_stream(stream).await {
					SubscriptionClosed::RemotePeerAborted => {
						sink.close(SubscriptionClosed::RemotePeerAborted);
					}
					_ => unreachable!(),
				};
			});
			Ok(())
		})
		.unwrap();
	let _handle = server.start(module).unwrap();

	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	// Add 10 subscriptions (this should fill our subscrition limit for this connection):
	let mut subs = Vec::new();
	for _ in 0..10 {
		subs.push(
			client
				.subscribe::<usize, ArrayParams>("subscribe_forever", rpc_params![], "unsubscribe_forever")
				.await
				.unwrap(),
		);
	}

	// Get the ID of one of them:
	let last_sub = subs.pop().unwrap();
	let last_sub_id = match last_sub.kind() {
		SubscriptionKind::Subscription(id) => id.clone(),
		_ => panic!("Expected a subscription Id to be present"),
	};

	// Manually call the unsubscribe function for this subscription:
	let res: Result<bool, _> = client.request("unsubscribe_forever", rpc_params![last_sub_id]).await;

	// This should not hit any limits, and unsubscription should have worked:
	assert!(res.is_ok(), "Unsubscription method was successfully called");
	assert!(res.unwrap(), "Unsubscription was successful");
}

#[tokio::test]
async fn http_unsupported_methods_dont_work() {
	use hyper::{Body, Client, Method, Request};

	init_logger();
	let server_addr = server().await;

	let http_client = Client::new();
	let uri = format!("http://{}", server_addr);

	let req_is_client_error = |method| async {
		let req = Request::builder()
			.method(method)
			.uri(&uri)
			.header("content-type", "application/json")
			.body(Body::from(r#"{ "jsonrpc": "2.0", method: "say_hello", "id": 1 }"#))
			.expect("request builder");

		let res = http_client.request(req).await.unwrap();
		res.status().is_client_error()
	};

	for verb in [Method::GET, Method::PUT, Method::PATCH, Method::DELETE] {
		assert!(req_is_client_error(verb).await);
	}
	assert!(!req_is_client_error(Method::POST).await);
}

#[tokio::test]
async fn http_correct_content_type_required() {
	use hyper::{Body, Client, Method, Request};

	init_logger();

	let server_addr = server().await;

	let http_client = Client::new();
	let uri = format!("http://{}", server_addr);

	// We don't set content-type at all
	let req = Request::builder()
		.method(Method::POST)
		.uri(&uri)
		.body(Body::from(r#"{ "jsonrpc": "2.0", method: "say_hello", "id": 1 }"#))
		.expect("request builder");

	let res = http_client.request(req).await.unwrap();
	assert!(res.status().is_client_error());

	// We use the wrong content-type
	let req = Request::builder()
		.method(Method::POST)
		.uri(&uri)
		.header("content-type", "application/text")
		.body(Body::from(r#"{ "jsonrpc": "2.0", method: "say_hello", "id": 1 }"#))
		.expect("request builder");

	let res = http_client.request(req).await.unwrap();
	assert!(res.status().is_client_error());

	// We use the correct content-type
	let req = Request::builder()
		.method(Method::POST)
		.uri(&uri)
		.header("content-type", "application/json")
		.body(Body::from(r#"{ "jsonrpc": "2.0", method: "say_hello", "id": 1 }"#))
		.expect("request builder");

	let res = http_client.request(req).await.unwrap();
	assert!(res.status().is_success());
}

#[tokio::test]
async fn http_cors_preflight_works() {
	use hyper::{Body, Client, Method, Request};
	use jsonrpsee::server::AllowHosts;

	init_logger();

	let cors = CorsLayer::new()
		.allow_methods([Method::POST])
		.allow_origin("https://foo.com".parse::<HeaderValue>().unwrap())
		.allow_headers([hyper::header::CONTENT_TYPE]);
	let (server_addr, _handle) = server_with_access_control(AllowHosts::Any, cors).await;

	let http_client = Client::new();
	let uri = format!("http://{}", server_addr);

	// First, make a preflight request.
	// See https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS#preflighted_requests for examples.
	// See https://fetch.spec.whatwg.org/#http-cors-protocol for the spec.
	let preflight_req = Request::builder()
		.method(Method::OPTIONS)
		.uri(&uri)
		.header("host", "bar.com") // <- host that request is being sent _to_
		.header("origin", "https://foo.com") // <- where request is being sent _from_
		.header("access-control-request-method", "POST")
		.header("access-control-request-headers", "content-type")
		.body(Body::empty())
		.expect("preflight request builder");

	let has = |v: &[String], s| v.iter().any(|v| v == s);

	let preflight_res = http_client.request(preflight_req).await.unwrap();
	let preflight_headers = preflight_res.headers();

	let allow_origins = comma_separated_header_values(preflight_headers, "access-control-allow-origin");
	let allow_methods = comma_separated_header_values(preflight_headers, "access-control-allow-methods");
	let allow_headers = comma_separated_header_values(preflight_headers, "access-control-allow-headers");

	// We expect the preflight response to tell us that our origin, methods and headers are all OK to use.
	// If they aren't, the browser will not make the actual request. Note that if these `access-control-*`
	// headers aren't return, the default is that the origin/method/headers are not allowed, I think.
	assert!(preflight_res.status().is_success());
	assert!(has(&allow_origins, "https://foo.com") || has(&allow_origins, "*"));
	assert!(has(&allow_methods, "post") || has(&allow_methods, "*"));
	assert!(has(&allow_headers, "content-type") || has(&allow_headers, "*"));

	// Assuming that that was successful, we now make the actual request. No CORS headers are needed here
	// as the browser checked their validity in the preflight request.
	let req = Request::builder()
		.method(Method::POST)
		.uri(&uri)
		.header("host", "bar.com")
		.header("origin", "https://foo.com")
		.header("content-type", "application/json")
		.body(Body::from(r#"{ "jsonrpc": "2.0", method: "say_hello", "id": 1 }"#))
		.expect("actual request builder");

	let res = http_client.request(req).await.unwrap();
	assert!(res.status().is_success());
	assert!(has(&allow_origins, "https://foo.com") || has(&allow_origins, "*"));
}

fn comma_separated_header_values(headers: &hyper::HeaderMap, header: &str) -> Vec<String> {
	headers
		.get_all(header)
		.into_iter()
		.flat_map(|value| value.to_str().unwrap().split(',').map(|val| val.trim()))
		.map(|header| header.to_ascii_lowercase())
		.collect()
}

#[tokio::test]
async fn ws_subscribe_with_bad_params() {
	init_logger();

	let server_addr = server_with_subscription().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	let err = client
		.subscribe::<serde_json::Value, ArrayParams>("subscribe_add_one", rpc_params!["0x0"], "unsubscribe_add_one")
		.await
		.unwrap_err();
	assert!(matches!(err, Error::Call(_)));
}

#[tokio::test]
async fn http_health_api_works() {
	use hyper::{Body, Client, Request};

	init_logger();

	let (server_addr, _handle) = server_with_health_api().await;

	let http_client = Client::new();
	let uri = format!("http://{}/health", server_addr);

	let req = Request::builder().method("GET").uri(&uri).body(Body::empty()).expect("request builder");
	let res = http_client.request(req).await.unwrap();

	assert!(res.status().is_success());

	let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
	let out = String::from_utf8(bytes.to_vec()).unwrap();
	assert_eq!(out.as_str(), "{\"health\":true}");
}

#[tokio::test]
async fn ws_host_filtering_wildcard_works() {
	use jsonrpsee::server::*;

	init_logger();

	let acl = AllowHosts::Only(vec!["http://localhost:*".into(), "http://127.0.0.1:*".into()]);

	let server = ServerBuilder::default().set_host_filtering(acl).build("127.0.0.1:0").await.unwrap();
	let mut module = RpcModule::new(());
	let addr = server.local_addr().unwrap();
	module.register_method("say_hello", |_, _| Ok("hello")).unwrap();

	let _handle = server.start(module).unwrap();

	let server_url = format!("ws://{}", addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	assert!(client.request::<String, ArrayParams>("say_hello", rpc_params![]).await.is_ok());
}

#[tokio::test]
async fn http_host_filtering_wildcard_works() {
	use jsonrpsee::server::*;

	init_logger();

	let allowed_hosts = AllowHosts::Only(vec!["http://localhost:*".into(), "http://127.0.0.1:*".into()]);

	let server = ServerBuilder::default().set_host_filtering(allowed_hosts).build("127.0.0.1:0").await.unwrap();
	let mut module = RpcModule::new(());
	let addr = server.local_addr().unwrap();
	module.register_method("say_hello", |_, _| Ok("hello")).unwrap();

	let _handle = server.start(module).unwrap();

	let server_url = format!("http://{}", addr);
	let client = HttpClientBuilder::default().build(&server_url).unwrap();

	assert!(client.request::<String, ArrayParams>("say_hello", rpc_params![]).await.is_ok());
}

#[tokio::test]
async fn deny_invalid_host() {
	use jsonrpsee::server::*;

	init_logger();

	let allowed_hosts = AllowHosts::Only(vec!["http://example.com".into()]);

	let server = ServerBuilder::default().set_host_filtering(allowed_hosts).build("127.0.0.1:0").await.unwrap();
	let mut module = RpcModule::new(());
	let addr = server.local_addr().unwrap();
	module.register_method("say_hello", |_, _| Ok("hello")).unwrap();

	let _handle = server.start(module).unwrap();

	// HTTP
	{
		let server_url = format!("http://{}", addr);
		let client = HttpClientBuilder::default().build(&server_url).unwrap();
		assert!(client.request::<String, _>("say_hello", rpc_params![]).await.is_err());
	}

	// WebSocket
	{
		let server_url = format!("ws://{}", addr);
		let err = WsClientBuilder::default().build(&server_url).await.unwrap_err();
		assert!(
			matches!(err, Error::Transport(e) if e.to_string().contains("Connection rejected with status code: 403"))
		)
	}
}
