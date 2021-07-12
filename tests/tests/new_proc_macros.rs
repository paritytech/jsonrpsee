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

//! Example of using proc macro to generate working client and server.

use std::net::SocketAddr;

use futures_channel::oneshot;
use jsonrpsee::{ws_client::*, ws_server::WsServerBuilder};

mod rpc_impl {
	use jsonrpsee::{proc_macros::rpc, types::async_trait, ws_server::SubscriptionSink};

	#[rpc(client, server, namespace = "foo")]
	pub trait Rpc {
		#[method(name = "foo")]
		async fn async_method(&self, param_a: u8, param_b: String) -> u16;

		#[method(name = "bar")]
		fn sync_method(&self) -> u16;

		#[subscription(name = "sub", unsub = "unsub", item = String)]
		fn sub(&self);

		#[subscription(name = "echo", unsub = "no_more_echo", item = u32)]
		fn sub_with_params(&self, val: u32);
	}

	pub struct RpcServerImpl;

	#[async_trait]
	impl RpcServer for RpcServerImpl {
		async fn async_method(&self, _param_a: u8, _param_b: String) -> u16 {
			42u16
		}

		fn sync_method(&self) -> u16 {
			10u16
		}

		fn sub(&self, mut sink: SubscriptionSink) {
			sink.send(&"Response_A").unwrap();
			sink.send(&"Response_B").unwrap();
		}

		fn sub_with_params(&self, mut sink: SubscriptionSink, val: u32) {
			sink.send(&val).unwrap();
			sink.send(&val).unwrap();
		}
	}
}

// Use generated implementations of server and client.
use rpc_impl::{RpcClient, RpcServer, RpcServerImpl};

pub async fn websocket_server() -> SocketAddr {
	let (server_started_tx, server_started_rx) = oneshot::channel();

	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();
		let server = rt.block_on(WsServerBuilder::default().build("127.0.0.1:0")).unwrap();

		rt.block_on(async move {
			server_started_tx.send(server.local_addr().unwrap()).unwrap();

			server.start(RpcServerImpl.into_rpc()).await
		});
	});

	server_started_rx.await.unwrap()
}

#[tokio::test]
async fn proc_macros_generic_ws_client_api() {
	let server_addr = websocket_server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	assert_eq!(client.async_method(10, "a".into()).await.unwrap(), 42);
	assert_eq!(client.sync_method().await.unwrap(), 10);

	// Sub without params
	let mut sub = client.sub().await.unwrap();
	let first_recv = sub.next().await.unwrap();
	assert_eq!(first_recv, Some("Response_A".to_string()));
	let second_recv = sub.next().await.unwrap();
	assert_eq!(second_recv, Some("Response_B".to_string()));

	// Sub with params
	let mut sub = client.sub_with_params(42).await.unwrap();
	let first_recv = sub.next().await.unwrap();
	assert_eq!(first_recv, Some(42));
	let second_recv = sub.next().await.unwrap();
	assert_eq!(second_recv, Some(42));
}
