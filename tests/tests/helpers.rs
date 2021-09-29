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

use futures::channel::oneshot;
use jsonrpsee::{
	http_server::HttpServerBuilder,
	types::Error,
	ws_server::{WsServerBuilder, WsStopHandle},
	RpcModule,
};
use std::net::SocketAddr;
use std::time::Duration;

pub async fn websocket_server_with_subscription() -> (SocketAddr, WsStopHandle) {
	let (server_started_tx, server_started_rx) = oneshot::channel();

	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();

		let server = rt.block_on(WsServerBuilder::default().build("127.0.0.1:0")).unwrap();

		let mut module = RpcModule::new(());
		module.register_method("say_hello", |_, _| Ok("hello")).unwrap();

		module
			.register_subscription("subscribe_hello", "unsubscribe_hello", |_, mut sink, _| {
				std::thread::spawn(move || loop {
					if let Err(Error::SubscriptionClosed(_)) = sink.send(&"hello from subscription") {
						break;
					}
					std::thread::sleep(Duration::from_millis(50));
				});
				Ok(())
			})
			.unwrap();

		module
			.register_subscription("subscribe_foo", "unsubscribe_foo", |_, mut sink, _| {
				std::thread::spawn(move || loop {
					if let Err(Error::SubscriptionClosed(_)) = sink.send(&1337) {
						break;
					}
					std::thread::sleep(Duration::from_millis(100));
				});
				Ok(())
			})
			.unwrap();

		module
			.register_subscription("subscribe_add_one", "unsubscribe_add_one", |params, mut sink, _| {
				let mut count: usize = params.one()?;
				std::thread::spawn(move || loop {
					count = count.wrapping_add(1);
					if let Err(Error::SubscriptionClosed(_)) = sink.send(&count) {
						break;
					}
					std::thread::sleep(Duration::from_millis(100));
				});
				Ok(())
			})
			.unwrap();

		module
			.register_subscription("subscribe_noop", "unsubscribe_noop", |_, mut sink, _| {
				std::thread::spawn(move || {
					std::thread::sleep(Duration::from_secs(1));
					sink.close("Server closed the stream because it was lazy".into())
				});
				Ok(())
			})
			.unwrap();

		rt.block_on(async move {
			server_started_tx.send((server.local_addr().unwrap(), server.stop_handle())).unwrap();
			server.start(module).await
		});
	});

	server_started_rx.await.unwrap()
}

pub async fn websocket_server() -> SocketAddr {
	let (server_started_tx, server_started_rx) = oneshot::channel();

	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();
		let server = rt.block_on(WsServerBuilder::default().build("127.0.0.1:0")).unwrap();
		let mut module = RpcModule::new(());
		module.register_method("say_hello", |_, _| Ok("hello")).unwrap();

		rt.block_on(async move {
			server_started_tx.send(server.local_addr().unwrap()).unwrap();

			server.start(module).await
		});
	});

	server_started_rx.await.unwrap()
}

pub async fn http_server() -> SocketAddr {
	let server = HttpServerBuilder::default().build("127.0.0.1:0".parse().unwrap()).unwrap();
	let mut module = RpcModule::new(());
	let addr = server.local_addr().unwrap();
	module.register_method("say_hello", |_, _| Ok("hello")).unwrap();
	module.register_method("notif", |_, _| Ok("")).unwrap();

	tokio::spawn(server.start(module));
	addr
}
