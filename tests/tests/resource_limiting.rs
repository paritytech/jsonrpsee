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

use jsonrpsee::{
	http_client::HttpClientBuilder,
	http_server::HttpServerBuilder,
	proc_macros::rpc,
	types::{traits::Client, v2::ParamsSer, Error},
	ws_client::WsClientBuilder,
	ws_server::{WsServerBuilder, WsStopHandle},
	RpcModule,
};
use tokio::time::sleep;

use std::net::SocketAddr;
use std::time::Duration;

fn module_manual() -> Result<RpcModule<()>, Error> {
	let mut module = RpcModule::new(());

	module.register_async_method("say_hello", |_, _| async move {
		sleep(Duration::from_millis(50)).await;
		Ok("hello")
	})?;

	module
		.register_async_method("expensive_call", |_, _| async move {
			sleep(Duration::from_millis(50)).await;
			Ok("hello expensive call")
		})?
		.resource("CPU", 3)?;

	module
		.register_async_method("memory_hog", |_, _| async move {
			sleep(Duration::from_millis(50)).await;
			Ok("hello memory hog")
		})?
		.resource("CPU", 0)?
		.resource("MEM", 8)?;

	Ok(module)
}

fn module_macro() -> RpcModule<()> {
	#[rpc(server)]
	pub trait Rpc {
		#[method(name = "say_hello")]
		async fn hello(&self) -> Result<&'static str, Error> {
			sleep(Duration::from_millis(50)).await;
			Ok("hello")
		}

		#[method(name = "expensive_call", resources("CPU" = 3))]
		async fn expensive(&self) -> Result<&'static str, Error> {
			sleep(Duration::from_millis(50)).await;
			Ok("hello expensive call")
		}

		#[method(name = "memory_hog", resources("CPU" = 0, "MEM" = 8))]
		async fn memory(&self) -> Result<&'static str, Error> {
			sleep(Duration::from_millis(50)).await;
			Ok("hello memory hog")
		}
	}

	impl RpcServer for () {}

	().into_rpc()
}

async fn websocket_server(module: RpcModule<()>) -> Result<(SocketAddr, WsStopHandle), Error> {
	let server = WsServerBuilder::default()
		.register_resource("CPU", 6, 2)?
		.register_resource("MEM", 10, 1)?
		.build("127.0.0.1:0")
		.await?;

	let addr = server.local_addr()?;
	let handle = server.start(module)?;

	Ok((addr, handle))
}

async fn http_server(module: RpcModule<()>) -> Result<SocketAddr, Error> {
	let server = HttpServerBuilder::default()
		.register_resource("CPU", 6, 2)?
		.register_resource("MEM", 10, 1)?
		.build("127.0.0.1:0".parse().unwrap())?;

	let addr = server.local_addr()?;

	tokio::spawn(server.start(module));

	Ok(addr)
}

fn assert_server_busy(fail: Result<String, Error>) {
	match fail {
		Err(Error::Request(msg)) => {
			let err: serde_json::Value = serde_json::from_str(&msg).unwrap();

			assert_eq!(err["error"]["code"], -32604);
			assert_eq!(err["error"]["message"], "Server is busy, try again later");
		}
		fail => panic!("Expected error, got: {:?}", fail),
	}
}

async fn run_tests_on_ws_server(server_addr: SocketAddr, stop_handle: WsStopHandle) {
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	// 2 CPU units (default) per call, so 4th call exceeds cap
	let (pass1, pass2, pass3, fail) = tokio::join!(
		client.request::<String>("say_hello", ParamsSer::NoParams),
		client.request::<String>("say_hello", ParamsSer::NoParams),
		client.request::<String>("say_hello", ParamsSer::NoParams),
		client.request::<String>("say_hello", ParamsSer::NoParams),
	);

	assert!(pass1.is_ok());
	assert!(pass2.is_ok());
	assert!(pass3.is_ok());
	assert_server_busy(fail);

	// 3 CPU units per call, so 3rd call exceeds CPU cap, but we can still get on MEM
	let (pass_cpu1, pass_cpu2, fail_cpu, pass_mem, fail_mem) = tokio::join!(
		client.request::<String>("expensive_call", ParamsSer::NoParams),
		client.request::<String>("expensive_call", ParamsSer::NoParams),
		client.request::<String>("expensive_call", ParamsSer::NoParams),
		client.request::<String>("memory_hog", ParamsSer::NoParams),
		client.request::<String>("memory_hog", ParamsSer::NoParams),
	);

	assert!(pass_cpu1.is_ok());
	assert!(pass_cpu2.is_ok());
	assert_server_busy(fail_cpu);
	assert!(pass_mem.is_ok());
	assert_server_busy(fail_mem);

	// Client being active prevents the server from shutting down?!
	drop(client);
	stop_handle.stop().unwrap().await;
}

async fn run_tests_on_http_server(server_addr: SocketAddr) {
	let server_url = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&server_url).unwrap();

	// 2 CPU units (default) per call, so 4th call exceeds cap
	let (a, b, c, d) = tokio::join!(
		client.request::<String>("say_hello", ParamsSer::NoParams),
		client.request::<String>("say_hello", ParamsSer::NoParams),
		client.request::<String>("say_hello", ParamsSer::NoParams),
		client.request::<String>("say_hello", ParamsSer::NoParams),
	);

	// HTTP does not guarantee ordering
	let mut passes = 0;

	for result in [a, b, c, d] {
		if result.is_ok() {
			passes += 1;
		} else {
			assert_server_busy(result);
		}
	}

	assert_eq!(passes, 3);
}

#[tokio::test]
async fn ws_server_with_manual_module() {
	let (server_addr, stop_handle) = websocket_server(module_manual().unwrap()).await.unwrap();

	run_tests_on_ws_server(server_addr, stop_handle).await;
}

#[tokio::test]
async fn ws_server_with_macro_module() {
	let (server_addr, stop_handle) = websocket_server(module_macro()).await.unwrap();

	run_tests_on_ws_server(server_addr, stop_handle).await;
}

#[tokio::test]
async fn http_server_with_manual_module() {
	let server_addr = http_server(module_manual().unwrap()).await.unwrap();

	run_tests_on_http_server(server_addr).await;
}

#[tokio::test]
async fn http_server_with_macro_module() {
	let server_addr = http_server(module_macro()).await.unwrap();

	run_tests_on_http_server(server_addr).await;
}
