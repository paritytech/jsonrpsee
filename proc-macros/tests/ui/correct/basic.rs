//! Example of using proc macro to generate working client and server.

use jsonrpsee::{
	proc_macros::rpc,
	types::{async_trait, JsonRpcResult},
	ws_client::*,
	ws_server::{SubscriptionSink, WsServerBuilder},
};
use std::{net::SocketAddr, sync::mpsc::channel};

#[rpc(client, server, namespace = "foo")]
pub trait Rpc {
	#[method(name = "foo", aliases = "fooAlias, Other")]
	async fn async_method(&self, param_a: u8, param_b: String) -> JsonRpcResult<u16>;

	#[method(name = "bar")]
	fn sync_method(&self) -> JsonRpcResult<u16>;

	#[subscription(name = "sub", item = String)]
	fn sub(&self);

	#[subscription(name = "echo", aliases = "ECHO", item = u32, unsubscribe_aliases = "NotInterested, listenNoMore")]
	fn sub_with_params(&self, val: u32);
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn async_method(&self, _param_a: u8, _param_b: String) -> JsonRpcResult<u16> {
		Ok(42u16)
	}

	fn sync_method(&self) -> JsonRpcResult<u16> {
		Ok(10u16)
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

pub async fn websocket_server() -> SocketAddr {
	let (server_started_tx, server_started_rx) = channel();

	std::thread::spawn(move || {
		let rt = tokio::runtime::Runtime::new().unwrap();
		let server = rt.block_on(WsServerBuilder::default().build("127.0.0.1:0")).unwrap();

		rt.block_on(async move {
			server_started_tx.send(server.local_addr().unwrap()).unwrap();

			server.start(RpcServerImpl.into_rpc()).await
		});
	});

	server_started_rx.recv().unwrap()
}

#[tokio::main]
async fn main() {
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
