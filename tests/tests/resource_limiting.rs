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

use jsonrpsee::core::client::{ClientT, SubscriptionClientT};
use jsonrpsee::core::Error;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::http_server::{HttpServerBuilder, HttpServerHandle};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::error::CallError;
use jsonrpsee::types::ReturnTypeSubscription;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::ws_server::{WsServerBuilder, WsServerHandle};
use jsonrpsee::{PendingSubscription, RpcModule};
use tokio::time::sleep;

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

	// Drop the `SubscriptionSink` to cause the internal `ResourceGuard` allocated per subscription call
	// to get dropped. This is the equivalent of not having any resource limits (ie, sink is never used).
	module
		.register_subscription("subscribe_hello", "s_hello", "unsubscribe_hello", move |_, pending, _| {
			let mut _sink = match pending.accept() {
				Some(sink) => sink,
				_ => return Ok(()),
			};
			Ok(())
		})?
		.resource("SUB", 3)?;

	// Keep the `SubscriptionSink` alive for a bit to validate that `ResourceGuard` is alive
	// and the subscription method gets limited.
	module
		.register_subscription("subscribe_hello_limit", "s_hello", "unsubscribe_hello_limit", move |_, pending, _| {
			let mut sink = match pending.accept() {
				Some(sink) => sink,
				_ => return Ok(()),
			};

			tokio::spawn(async move {
				for val in 0..10 {
					sink.send(&val).unwrap();
					sleep(Duration::from_secs(1)).await;
				}
			});

			Ok(())
		})?
		.resource("SUB", 3)?;

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

		#[subscription(name = "subscribe_hello", item = String, resources("SUB" = 3))]
		fn sub_hello(&self);

		#[subscription(name = "subscribe_hello_limit", item = String, resources("SUB" = 3))]
		fn sub_hello_limit(&self);
	}

	impl RpcServer for () {
		fn sub_hello(&self, pending: PendingSubscription) -> ReturnTypeSubscription {
			let mut _sink = match pending.accept() {
				Some(sink) => sink,
				_ => return Ok(()),
			};
			Ok(())
		}

		fn sub_hello_limit(&self, pending: PendingSubscription) -> ReturnTypeSubscription {
			let mut sink = match pending.accept() {
				Some(sink) => sink,
				_ => return Ok(()),
			};

			tokio::spawn(async move {
				for val in 0..10 {
					sink.send(&val).unwrap();
					sleep(Duration::from_secs(1)).await;
				}
			});

			Ok(())
		}
	}

	().into_rpc()
}

async fn websocket_server(module: RpcModule<()>) -> Result<(SocketAddr, WsServerHandle), Error> {
	let server = WsServerBuilder::default()
		.register_resource("CPU", 6, 2)?
		.register_resource("MEM", 10, 1)?
		.register_resource("SUB", 6, 1)?
		.build("127.0.0.1:0")
		.await?;

	let addr = server.local_addr()?;
	let handle = server.start(module)?;

	Ok((addr, handle))
}

async fn http_server(module: RpcModule<()>) -> Result<(SocketAddr, HttpServerHandle), Error> {
	let server = HttpServerBuilder::default()
		.register_resource("CPU", 6, 2)?
		.register_resource("MEM", 10, 1)?
		.register_resource("SUB", 6, 1)?
		.build("127.0.0.1:0")
		.await?;

	let addr = server.local_addr()?;
	let handle = server.start(module)?;

	Ok((addr, handle))
}

fn assert_server_busy<T: std::fmt::Debug>(fail: Result<T, Error>) {
	match fail {
		Err(Error::Call(CallError::Custom(err))) => {
			assert_eq!(err.code(), -32604);
			assert_eq!(err.message(), "Server is busy, try again later");
		}
		fail => panic!("Expected error, got: {:?}", fail),
	}
}

async fn run_tests_on_ws_server(server_addr: SocketAddr, server_handle: WsServerHandle) {
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	// 2 CPU units (default) per call, so 4th call exceeds cap
	let (pass1, pass2, pass3, fail) = tokio::join!(
		client.request::<String>("say_hello", None),
		client.request::<String>("say_hello", None),
		client.request::<String>("say_hello", None),
		client.request::<String>("say_hello", None),
	);

	assert!(pass1.is_ok());
	assert!(pass2.is_ok());
	assert!(pass3.is_ok());
	assert_server_busy(fail);

	// 3 CPU units per call, so 3rd call exceeds CPU cap, but we can still get on MEM
	let (pass_cpu1, pass_cpu2, fail_cpu, pass_mem, fail_mem) = tokio::join!(
		client.request::<String>("expensive_call", None),
		client.request::<String>("expensive_call", None),
		client.request::<String>("expensive_call", None),
		client.request::<String>("memory_hog", None),
		client.request::<String>("memory_hog", None),
	);

	assert!(pass_cpu1.is_ok());
	assert!(pass_cpu2.is_ok());
	assert_server_busy(fail_cpu);
	assert!(pass_mem.is_ok());
	assert_server_busy(fail_mem);

	// If we issue multiple subscription requests at the same time from the same client,
	// but the subscriptions immediately drop their sinks, no resources will obviously be held,
	// and so there is no limit to how many can be executed.
	let (pass1, pass2, pass3) = tokio::join!(
		client.subscribe::<i32>("subscribe_hello", None, "unsubscribe_hello"),
		client.subscribe::<i32>("subscribe_hello", None, "unsubscribe_hello"),
		client.subscribe::<i32>("subscribe_hello", None, "unsubscribe_hello"),
	);

	assert!(pass1.is_ok());
	assert!(pass2.is_ok());
	assert!(pass3.is_ok());

	// 3 CPU units (manually set for subscriptions) per call, so 3th call exceeds cap
	let (pass1, pass2, fail) = tokio::join!(
		client.subscribe::<i32>("subscribe_hello_limit", None, "unsubscribe_hello_limit"),
		client.subscribe::<i32>("subscribe_hello_limit", None, "unsubscribe_hello_limit"),
		client.subscribe::<i32>("subscribe_hello_limit", None, "unsubscribe_hello_limit"),
	);

	assert!(pass1.is_ok());
	assert!(pass2.is_ok());
	assert_server_busy(fail);

	server_handle.stop().unwrap().await;
}

async fn run_tests_on_http_server(server_addr: SocketAddr, server_handle: HttpServerHandle) {
	let server_url = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&server_url).unwrap();

	// 2 CPU units (default) per call, so 4th call exceeds cap
	let (a, b, c, d) = tokio::join!(
		client.request::<String>("say_hello", None),
		client.request::<String>("say_hello", None),
		client.request::<String>("say_hello", None),
		client.request::<String>("say_hello", None),
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

	server_handle.stop().unwrap().await.unwrap();
}

#[tokio::test]
async fn ws_server_with_manual_module() {
	let (server_addr, server_handle) = websocket_server(module_manual().unwrap()).await.unwrap();

	run_tests_on_ws_server(server_addr, server_handle).await;
}

#[tokio::test]
async fn ws_server_with_macro_module() {
	let (server_addr, server_handle) = websocket_server(module_macro()).await.unwrap();

	run_tests_on_ws_server(server_addr, server_handle).await;
}

#[tokio::test]
async fn http_server_with_manual_module() {
	let (server_addr, server_handle) = http_server(module_manual().unwrap()).await.unwrap();

	run_tests_on_http_server(server_addr, server_handle).await;
}

#[tokio::test]
async fn http_server_with_macro_module() {
	let (server_addr, server_handle) = http_server(module_macro()).await.unwrap();

	run_tests_on_http_server(server_addr, server_handle).await;
}
