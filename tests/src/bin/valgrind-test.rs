// Copyright 2022 Parity Technologies (UK) Ltd.
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

use futures::stream::StreamExt;
use jsonrpsee::core::client::{ClientT, SubscriptionClientT};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use jsonrpsee::server::{RpcModule, ServerBuilder, ServerHandle};
use jsonrpsee::ws_client::WsClientBuilder;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

fn main() {
	let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();

	rt.block_on(async move {
		let (addr, handle) = run_server().await;

		// run ws client.
		{
			let client = WsClientBuilder::default().build(&format!("ws://{}", addr)).await.unwrap();
			run_requests(&client).await;
			let mut sub = client.subscribe("hi", rpc_params![], "goodbye").await.unwrap();
			let s1: usize = sub.next().await.unwrap().unwrap();
			println!("s1 response: {}", s1);

			drop(client);

			// wait for the client to be dropped.
			tokio::time::sleep(Duration::from_millis(100)).await;
			// the subscription should closed when the client is dropped.
			drop(sub);
		}

		// run http client.
		{
			let client = HttpClientBuilder::default().build(&format!("http://{}", addr)).unwrap();
			run_requests(&client).await;
		}

		handle.stop().unwrap();
		handle.stopped().await
	});

	drop(rt);
}

async fn run_requests(client: &impl ClientT) {
	let r1: String = client.request("say_hello", rpc_params![]).await.unwrap();
	println!("r1 response: {}", r1);
	let r2: String = client.request("say_hello_async", rpc_params![]).await.unwrap();
	println!("r2 response: {}", r2);
	let r3: String = client.request("blocking", rpc_params![]).await.unwrap();
	println!("r3 response: {}", r3);
}

async fn run_server() -> (SocketAddr, ServerHandle) {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let mut module = RpcModule::new(());

	module.register_method("say_hello", |_, _| Ok("lo")).unwrap();
	module.register_async_method("say_hello_async", |_, _| async { Ok("lo_async") }).unwrap();
	module
		.register_blocking_method("blocking", |_, _| {
			std::thread::sleep(Duration::from_millis(100));
			Ok("blocking")
		})
		.unwrap();
	module
		.register_subscription("hi", "hi", "goodbye", |_, mut sink, _| {
			let interval = interval(Duration::from_millis(200));
			let stream = IntervalStream::new(interval).map(move |_| 1);

			tokio::spawn(async move {
				sink.pipe_from_stream(stream).await;
			});

			Ok(())
		})
		.unwrap();

	let addr = server.local_addr().unwrap();
	let handle = server.start(module).unwrap();
	(addr, handle)
}
