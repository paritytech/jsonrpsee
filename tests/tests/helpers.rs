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

use std::net::SocketAddr;
use std::time::Duration;

use futures::{SinkExt, StreamExt};
use jsonrpsee::core::error::SubscriptionClosed;
use jsonrpsee::http_server::{AccessControl, HttpServerBuilder, HttpServerHandle};
use jsonrpsee::types::error::{ErrorObject, SUBSCRIPTION_CLOSED_WITH_ERROR};
use jsonrpsee::ws_server::{WsServerBuilder, WsServerHandle};
use jsonrpsee::RpcModule;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

pub async fn websocket_server_with_subscription() -> (SocketAddr, WsServerHandle) {
	let server = WsServerBuilder::default().build("127.0.0.1:0").await.unwrap();

	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| Ok("hello")).unwrap();

	module
		.register_subscription("subscribe_hello", "subscribe_hello", "unsubscribe_hello", |_, pending, _| {
			let mut sink = pending.accept().unwrap();
			std::thread::spawn(move || loop {
				if let Ok(false) = sink.send(&"hello from subscription") {
					break;
				}
				std::thread::sleep(Duration::from_millis(50));
			});
		})
		.unwrap();

	module
		.register_subscription("subscribe_foo", "subscribe_foo", "unsubscribe_foo", |_, pending, _| {
			let mut sink = pending.accept().unwrap();
			std::thread::spawn(move || loop {
				if let Ok(false) = sink.send(&1337_usize) {
					break;
				}
				std::thread::sleep(Duration::from_millis(100));
			});
		})
		.unwrap();

	module
		.register_subscription("subscribe_add_one", "subscribe_add_one", "unsubscribe_add_one", |params, pending, _| {
			let mut count = match params.one::<usize>() {
				Ok(count) => count,
				_ => return,
			};

			let mut sink = pending.accept().unwrap();

			std::thread::spawn(move || loop {
				count = count.wrapping_add(1);
				if let Err(_) | Ok(false) = sink.send(&count) {
					break;
				}
				std::thread::sleep(Duration::from_millis(100));
			});
		})
		.unwrap();

	module
		.register_subscription("subscribe_noop", "subscribe_noop", "unsubscribe_noop", |_, pending, _| {
			let sink = pending.accept().unwrap();
			std::thread::spawn(move || {
				std::thread::sleep(Duration::from_secs(1));
				let err = ErrorObject::owned(
					SUBSCRIPTION_CLOSED_WITH_ERROR,
					"Server closed the stream because it was lazy",
					None::<()>,
				);
				sink.close(err);
			});
		})
		.unwrap();

	module
		.register_subscription("subscribe_5_ints", "n", "unsubscribe_5_ints", |_, pending, _| {
			let mut sink = pending.accept().unwrap();

			tokio::spawn(async move {
				let interval = interval(Duration::from_millis(50));
				let stream = IntervalStream::new(interval).zip(futures::stream::iter(1..=5)).map(|(_, c)| c);

				match sink.pipe_from_stream(stream).await {
					SubscriptionClosed::Success => {
						sink.close(SubscriptionClosed::Success);
					}
					_ => unreachable!(),
				}
			});
		})
		.unwrap();

	module
		.register_subscription(
			"subscribe_with_err_on_stream",
			"n",
			"unsubscribe_with_err_on_stream",
			move |_, pending, _| {
				let mut sink = pending.accept().unwrap();

				let err: &'static str = "error on the stream";

				// create stream that produce an error which will cancel the subscription.
				let stream = futures::stream::iter(vec![Ok(1_u32), Err(err), Ok(2), Ok(3)]);
				tokio::spawn(async move {
					match sink.pipe_from_try_stream(stream).await {
						SubscriptionClosed::Failed(e) => {
							sink.close(e);
						}
						_ => unreachable!(),
					}
				});
			},
		)
		.unwrap();

	let addr = server.local_addr().unwrap();
	let server_handle = server.start(module).unwrap();

	(addr, server_handle)
}

pub async fn websocket_server() -> SocketAddr {
	let server = WsServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| Ok("hello")).unwrap();

	module
		.register_async_method("slow_hello", |_, _| async {
			tokio::time::sleep(std::time::Duration::from_secs(1)).await;
			Ok("hello")
		})
		.unwrap();

	let addr = server.local_addr().unwrap();

	server.start(module).unwrap();

	addr
}

/// Yields one item then sleeps for an hour.
pub async fn websocket_server_with_sleeping_subscription(tx: futures::channel::mpsc::Sender<()>) -> SocketAddr {
	let server = WsServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr = server.local_addr().unwrap();

	let mut module = RpcModule::new(tx);

	module
		.register_subscription("subscribe_sleep", "n", "unsubscribe_sleep", |_, pending, mut tx| {
			let mut sink = pending.accept().unwrap();

			tokio::spawn(async move {
				let interval = interval(Duration::from_secs(60 * 60));
				let stream = IntervalStream::new(interval).zip(futures::stream::iter(1..=5)).map(|(_, c)| c);

				sink.pipe_from_stream(stream).await;
				let send_back = std::sync::Arc::make_mut(&mut tx);
				send_back.send(()).await.unwrap();
			});
		})
		.unwrap();
	server.start(module).unwrap();
	addr
}

pub async fn http_server() -> (SocketAddr, HttpServerHandle) {
	http_server_with_access_control(AccessControl::default()).await
}

pub async fn http_server_with_access_control(acl: AccessControl) -> (SocketAddr, HttpServerHandle) {
	let server = HttpServerBuilder::default().set_access_control(acl).build("127.0.0.1:0").await.unwrap();
	let mut module = RpcModule::new(());
	let addr = server.local_addr().unwrap();
	module.register_method("say_hello", |_, _| Ok("hello")).unwrap();
	module.register_method("notif", |_, _| Ok("")).unwrap();

	let handle = server.start(module).unwrap();
	(addr, handle)
}
