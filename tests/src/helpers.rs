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

use jsonrpsee_http_server::{HttpConfig, HttpServer};
use jsonrpsee_types::jsonrpc::JsonValue;
use jsonrpsee_ws_server::WsServer;

use std::net::SocketAddr;
use std::time::Duration;

use futures::channel::oneshot;

pub async fn websocket_server_with_subscription() -> SocketAddr {
	let (server_started_tx, server_started_rx) = oneshot::channel();

	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();

		let mut server = rt.block_on(WsServer::new("127.0.0.1:0")).unwrap();
		let mut sub_hello = server.register_subscription("subscribe_hello", "unsubscribe_hello").unwrap();
		let mut sub_foo = server.register_subscription("subscribe_foo", "unsubscribe_foo").unwrap();

		server.register_method("say_hello", |_| Ok("hello")).unwrap();

		server_started_tx.send(server.local_addr().unwrap()).unwrap();

		rt.spawn(server.start());

		rt.block_on(async move {
			loop {
				tokio::time::sleep(Duration::from_millis(100)).await;

				sub_hello.send(&"hello from subscription").unwrap();
				sub_foo.send(&1337_u64).unwrap();
			}
		});
	});

	server_started_rx.await.unwrap()
}

pub async fn websocket_server() -> SocketAddr {
	let (server_started_tx, server_started_rx) = oneshot::channel();

	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();

		let mut server = rt.block_on(WsServer::new("127.0.0.1:0")).unwrap();

		server.register_method("say_hello", |_| Ok("hello")).unwrap();

		rt.block_on(async move {
			server_started_tx.send(server.local_addr().unwrap()).unwrap();

			server.start().await
		});
	});

	server_started_rx.await.unwrap()
}

pub async fn http_server() -> SocketAddr {
	let (server_started_tx, server_started_rx) = oneshot::channel();

	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();

		let server = rt.block_on(HttpServer::new("127.0.0.1:0", HttpConfig::default())).unwrap();
		server_started_tx.send(*server.local_addr()).unwrap();
		let mut call = server.register_method("say_hello".to_owned()).unwrap();

		rt.block_on(async move {
			loop {
				let handle = call.next().await;
				handle.respond(Ok(JsonValue::String("hello".to_owned()))).await.unwrap();
			}
		});
	});

	server_started_rx.await.unwrap()
}
