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
	}
}

// Use generated implementations of server and client.
use rpc_impl::{RpcClient, RpcServer, RpcServerImpl};

pub async fn websocket_server() -> SocketAddr {
	let (server_started_tx, server_started_rx) = oneshot::channel();

	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();
		let mut server = rt.block_on(WsServerBuilder::default().build("127.0.0.1:0")).unwrap();
		server.register_module(RpcServerImpl.into_rpc().unwrap()).unwrap();

		rt.block_on(async move {
			server_started_tx.send(server.local_addr().unwrap()).unwrap();

			server.start().await
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
	let mut sub = client.sub().await.unwrap();
	let first_recv = sub.next().await.unwrap();
	assert_eq!(first_recv, Some("Response_A".to_string()));
	let second_recv = sub.next().await.unwrap();
	assert_eq!(second_recv, Some("Response_B".to_string()));
}
