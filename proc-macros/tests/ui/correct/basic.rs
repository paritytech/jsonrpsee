//! Example of using proc macro to generate working client and server.

use jsonrpsee::{
	proc_macros::rpc,
	types::{async_trait, to_json_value, traits::Client, v2::params::JsonRpcParams, JsonRpcResult},
	ws_client::*,
	ws_server::{SubscriptionSink, WsServerBuilder},
};
use std::{net::SocketAddr, sync::mpsc::channel};

#[rpc(client, server, namespace = "foo")]
pub trait Rpc {
	#[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: String) -> JsonRpcResult<u16>;

	#[method(name = "optional_params")]
	async fn optional_params(&self, a: std::option::Option<u8>, b: String) -> JsonRpcResult<bool>;

	#[method(name = "optional_param")]
	async fn optional_param(&self, a: Option<u8>) -> JsonRpcResult<bool>;

	#[method(name = "array_params")]
	async fn array_params(&self, items: Vec<u64>) -> JsonRpcResult<u64>;

	#[method(name = "bar")]
	fn sync_method(&self) -> JsonRpcResult<u16>;

	#[subscription(name = "sub", unsub = "unsub", item = String)]
	fn sub(&self);

	#[subscription(name = "echo", unsub = "no_more_echo", item = u32)]
	fn sub_with_params(&self, val: u32);
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn async_method(&self, _param_a: u8, _param_b: String) -> JsonRpcResult<u16> {
		Ok(42u16)
	}

	async fn optional_params(&self, a: core::option::Option<u8>, _b: String) -> JsonRpcResult<bool> {
		let res = if a.is_some() { true } else { false };
		Ok(res)
	}

	async fn optional_param(&self, a: Option<u8>) -> JsonRpcResult<bool> {
		let res = if a.is_some() { true } else { false };
		Ok(res)
	}

	async fn array_params(&self, items: Vec<u64>) -> JsonRpcResult<u64> {
		Ok(items.len() as u64)
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
	assert_eq!(client.optional_params(None, "a".into()).await.unwrap(), false);
	assert_eq!(client.optional_params(Some(1), "a".into()).await.unwrap(), true);

	assert_eq!(client.array_params(vec![1]).await.unwrap(), 1);
	assert_eq!(
		client
			.request::<u64>("foo_array_params", vec![to_json_value(Vec::<u64>::new()).unwrap()].into())
			.await
			.unwrap(),
		0
	);

	assert_eq!(client.request::<bool>("foo_optional_param", vec![].into()).await.unwrap(), false);
	assert_eq!(client.request::<bool>("foo_optional_param", JsonRpcParams::NoParams).await.unwrap(), false);
	assert_eq!(
		client.request::<bool>("foo_optional_param", vec![to_json_value(Some(1)).unwrap()].into()).await.unwrap(),
		true
	);

	let mut sub = client.sub().await.unwrap();
	let first_recv = sub.next().await.unwrap();
	assert_eq!(first_recv, Some("Response_A".to_string()));
	let second_recv = sub.next().await.unwrap();
	assert_eq!(second_recv, Some("Response_B".to_string()));
}
