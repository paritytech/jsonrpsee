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

mod helpers;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use helpers::init_logger;
use jsonrpsee::core::{client::ClientT, Error};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::rpc_params;
use jsonrpsee::server::logger::{HttpRequest, Logger, MethodKind};
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use jsonrpsee::types::Params;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::RpcModule;
use tokio::time::sleep;

#[derive(Clone, Default)]
struct Counter {
	inner: Arc<Mutex<CounterInner>>,
}

#[derive(Default)]
struct CounterInner {
	/// (Number of started connections, number of finished connections)
	connections: (u32, u32),
	/// (Number of started requests, number of finished requests)
	requests: (u32, u32),
	/// Mapping method names to (number of calls, ids of successfully completed calls)
	calls: HashMap<String, (u32, Vec<u32>)>,
}

impl Logger for Counter {
	/// Auto-incremented id of the call
	type Instant = u32;

	fn on_connect(&self, _remote_addr: SocketAddr, _req: &HttpRequest) {
		self.inner.lock().unwrap().connections.0 += 1;
	}

	fn on_request(&self) -> u32 {
		let mut inner = self.inner.lock().unwrap();
		let n = inner.requests.0;

		inner.requests.0 += 1;

		n
	}

	fn on_call(&self, name: &str, _params: Params, _kind: MethodKind) {
		let mut inner = self.inner.lock().unwrap();
		let entry = inner.calls.entry(name.into()).or_insert((0, Vec::new()));

		entry.0 += 1;
	}

	fn on_result(&self, name: &str, success: bool, n: u32) {
		if success {
			self.inner.lock().unwrap().calls.get_mut(name).unwrap().1.push(n);
		}
	}

	fn on_response(&self, _result: &str, _: u32) {
		self.inner.lock().unwrap().requests.1 += 1;
	}

	fn on_disconnect(&self, _remote_addr: SocketAddr) {
		self.inner.lock().unwrap().connections.1 += 1;
	}
}

fn test_module() -> RpcModule<()> {
	#[rpc(server)]
	pub trait Rpc {
		#[method(name = "say_hello")]
		async fn hello(&self) -> Result<&'static str, Error> {
			sleep(Duration::from_millis(50)).await;
			Ok("hello")
		}
	}

	impl RpcServer for () {}

	().into_rpc()
}

async fn websocket_server(module: RpcModule<()>, counter: Counter) -> Result<(SocketAddr, ServerHandle), Error> {
	let server = ServerBuilder::default()
		.register_resource("CPU", 6, 2)?
		.register_resource("MEM", 10, 1)?
		.set_logger(counter)
		.build("127.0.0.1:0")
		.await?;

	let addr = server.local_addr()?;
	let handle = server.start(module)?;

	Ok((addr, handle))
}

async fn http_server(module: RpcModule<()>, counter: Counter) -> Result<(SocketAddr, ServerHandle), Error> {
	let server = ServerBuilder::default()
		.register_resource("CPU", 6, 2)?
		.register_resource("MEM", 10, 1)?
		.set_logger(counter)
		.build("127.0.0.1:0")
		.await?;

	let addr = server.local_addr()?;
	let handle = server.start(module)?;

	Ok((addr, handle))
}

#[tokio::test]
async fn ws_server_logger() {
	init_logger();

	let counter = Counter::default();
	let (server_addr, server_handle) = websocket_server(test_module(), counter.clone()).await.unwrap();

	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");

	let res: Result<String, Error> = client.request("unknown_method", rpc_params![]).await;
	assert!(res.is_err());

	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");
	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");

	let res: Result<String, Error> = client.request("unknown_method", rpc_params![]).await;
	assert!(res.is_err());

	{
		let inner = counter.inner.lock().unwrap();

		assert_eq!(inner.connections, (1, 0));
		assert_eq!(inner.requests, (5, 5));
		assert_eq!(inner.calls["say_hello"], (3, vec![0, 2, 3]));
		assert_eq!(inner.calls["unknown_method"], (2, vec![]));
	}

	server_handle.stop().unwrap();
	server_handle.stopped().await;

	assert_eq!(counter.inner.lock().unwrap().connections, (1, 1));
}

#[tokio::test]
async fn http_server_logger() {
	init_logger();

	let counter = Counter::default();
	let (server_addr, server_handle) = http_server(test_module(), counter.clone()).await.unwrap();

	let server_url = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&server_url).unwrap();

	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");

	let res: Result<String, Error> = client.request("unknown_method", rpc_params![]).await;
	assert!(res.is_err());

	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");
	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");

	let res: Result<String, Error> = client.request("unknown_method", rpc_params![]).await;
	assert!(res.is_err());

	{
		let inner = counter.inner.lock().unwrap();
		assert_eq!(inner.requests, (5, 5));
		assert_eq!(inner.calls["say_hello"], (3, vec![0, 2, 3]));
		assert_eq!(inner.calls["unknown_method"], (2, vec![]));
	}

	server_handle.stop().unwrap();
	server_handle.stopped().await;

	// HTTP server doesn't track connections
	let inner = counter.inner.lock().unwrap();
	assert_eq!(inner.connections, (0, 0));
}
