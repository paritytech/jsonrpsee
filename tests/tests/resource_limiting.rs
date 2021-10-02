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
	types::{traits::Client, v2::ParamsSer, Error},
	ws_client::WsClientBuilder,
	ws_server::{WsServerBuilder, WsStopHandle},
	RpcModule,
};
use tokio::time::sleep;

use std::net::SocketAddr;
use std::time::Duration;

async fn websocket_server() -> Result<(SocketAddr, WsStopHandle), Error> {
	let server = WsServerBuilder::default().register_resource("CPU", 6, 2)?.build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());

	module.register_async_method("say_hello", |_, _| async move {
		sleep(Duration::from_millis(50)).await;
		Ok("hello")
	})?;

	module
		.register_async_method("expensive_call", |_, _| async move {
			sleep(Duration::from_millis(100)).await;
			Ok("hello expensive call")
		})?
		.resource("CPU", 3)?;

	let addr = server.local_addr()?;
	let handle = server.start(module)?;

	Ok((addr, handle))
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

#[tokio::test]
async fn server_rejects_requests_if_resources_are_claimed() {
	let (server_addr, stop_handle) = websocket_server().await.unwrap();

	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	// 2 units (default) per call, so 4th call exceeds cap
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

	// 3 units per call, so 3rd call exceeds cap
	let (pass1, pass2, fail) = tokio::join!(
		client.request::<String>("expensive_call", ParamsSer::NoParams),
		client.request::<String>("expensive_call", ParamsSer::NoParams),
		client.request::<String>("expensive_call", ParamsSer::NoParams),
	);

	assert!(pass1.is_ok());
	assert!(pass2.is_ok());
	assert_server_busy(fail);

	// Client being active prevents the server from shutting down?!
	drop(client);
	stop_handle.stop().unwrap().await;
}
