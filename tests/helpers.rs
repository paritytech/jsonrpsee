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

use jsonrpsee::http::{HttpConfig, HttpServer};
use jsonrpsee::types::jsonrpc::JsonValue;
use jsonrpsee::ws::WsServer;

use std::net::SocketAddr;
use std::time::Duration;

use futures::channel::oneshot::{Receiver, Sender};
use futures::future::FutureExt;

pub fn websocket_server(server_started: Sender<SocketAddr>) {
	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();

		let server = rt.block_on(WsServer::new("127.0.0.1:0")).unwrap();
		let mut sub_hello =
			server.register_subscription("subscribe_hello".to_owned(), "unsubscribe_hello".to_owned()).unwrap();
		let mut sub_foo =
			server.register_subscription("subscribe_foo".to_owned(), "unsubscribe_foo".to_owned()).unwrap();
		let mut call = server.register_method("say_hello".to_owned()).unwrap();
		server_started.send(*server.local_addr()).unwrap();

		rt.block_on(async move {
			loop {
				let hello_fut = async {
					let handle = call.next().await;
					handle.respond(Ok(JsonValue::String("hello".to_owned()))).await.unwrap();
				}
				.fuse();

				let timeout = tokio::time::sleep(Duration::from_millis(100)).fuse();
				futures::pin_mut!(hello_fut, timeout);
				futures::select! {
					_ = hello_fut => (),
					_ = timeout => {
						sub_hello.send(JsonValue::String("hello from subscription".to_owned())).await.unwrap();
						sub_foo.send(JsonValue::Number(1337_u64.into())).await.unwrap();
					}
				}
			}
		});
	});
}

pub fn websocket_server_with_wait_period(server_started: Sender<SocketAddr>, wait: Receiver<()>) {
	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();

		let server = rt.block_on(WsServer::new("127.0.0.1:0")).unwrap();
		let mut respond = server.register_method("say_hello".to_owned()).unwrap();
		server_started.send(*server.local_addr()).unwrap();

		rt.block_on(async move {
			wait.await.unwrap();
			loop {
				let handle = respond.next().await;
				handle.respond(Ok(JsonValue::String("hello".to_owned()))).await.unwrap();
			}
		});
	});
}

pub fn http_server(server_started: Sender<SocketAddr>) {
	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();

		let server = rt.block_on(HttpServer::new("127.0.0.1:0", HttpConfig::default())).unwrap();
		server_started.send(*server.local_addr()).unwrap();
		let mut call = server.register_method("say_hello".to_owned()).unwrap();

		rt.block_on(async move {
			loop {
				let handle = call.next().await;
				handle.respond(Ok(JsonValue::String("hello".to_owned()))).await.unwrap();
			}
		});
	});
}
