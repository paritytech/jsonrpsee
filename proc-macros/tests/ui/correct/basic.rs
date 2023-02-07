//! Example of using proc macro to generate working client and server.

use std::net::SocketAddr;

use jsonrpsee::core::params::ArrayParams;
use jsonrpsee::core::SubscriptionResult;
use jsonrpsee::core::{async_trait, client::ClientT, RpcResult};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::{ServerBuilder, SubscriptionMessage};
use jsonrpsee::ws_client::*;
use jsonrpsee::{rpc_params, PendingSubscriptionSink};

#[rpc(client, server, namespace = "foo")]
pub trait Rpc {
	#[method(name = "foo", aliases = ["fooAlias", "Other"])]
	async fn async_method(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

	#[method(name = "optional_params")]
	async fn optional_params(&self, a: std::option::Option<u8>, b: String) -> RpcResult<bool>;

	#[method(name = "optional_param")]
	async fn optional_param(&self, a: Option<u8>) -> RpcResult<bool>;

	#[method(name = "array_params")]
	async fn array_params(&self, items: Vec<u64>) -> RpcResult<u64>;

	#[method(name = "bar")]
	fn sync_method(&self) -> RpcResult<u16>;

	#[subscription(name = "subscribe", item = String)]
	async fn sub(&self);

	#[subscription(name = "echo", unsubscribe = "unsubscribeEcho", aliases = ["ECHO"], item = u32, unsubscribe_aliases = ["NotInterested", "listenNoMore"])]
	async fn sub_with_params(&self, val: u32);

	// This will send data to subscribers with the `method` field in the JSON payload set to `foo_subscribe_override`
	// because it's in the `foo` namespace.
	#[subscription(name = "subscribe_method" => "subscribe_override", item = u32)]
	async fn sub_with_override_notif_method(&self);
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn async_method(&self, _param_a: u8, _param_b: String) -> RpcResult<u16> {
		Ok(42u16)
	}

	async fn optional_params(&self, a: core::option::Option<u8>, _b: String) -> RpcResult<bool> {
		let res = if a.is_some() { true } else { false };
		Ok(res)
	}

	async fn optional_param(&self, a: Option<u8>) -> RpcResult<bool> {
		let res = if a.is_some() { true } else { false };
		Ok(res)
	}

	async fn array_params(&self, items: Vec<u64>) -> RpcResult<u64> {
		Ok(items.len() as u64)
	}

	fn sync_method(&self) -> RpcResult<u16> {
		Ok(10u16)
	}

	async fn sub(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
		let sink = pending.accept().await?;

		let msg1 = SubscriptionMessage::from_json(&"Response_A").unwrap();
		let msg2 = SubscriptionMessage::from_json(&"Response_B").unwrap();

		sink.send(msg1).await.unwrap();
		sink.send(msg2).await.unwrap();

		Ok(())
	}

	async fn sub_with_params(&self, pending: PendingSubscriptionSink, val: u32) -> SubscriptionResult {
		let sink = pending.accept().await?;

		let msg = SubscriptionMessage::from_json(&val).unwrap();

		sink.send(msg.clone()).await.unwrap();
		sink.send(msg).await.unwrap();

		Ok(())
	}

	async fn sub_with_override_notif_method(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
		let sink = pending.accept().await?;
		let msg = SubscriptionMessage::from_json(&1).unwrap();
		sink.send(msg).await.unwrap();

		Ok(())
	}
}

pub async fn server() -> SocketAddr {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr = server.local_addr().unwrap();
	let server_handle = server.start(RpcServerImpl.into_rpc()).unwrap();

	tokio::spawn(server_handle.stopped());

	addr
}

#[tokio::main]
async fn main() {
	let server_addr = server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	assert_eq!(client.async_method(10, "a".into()).await.unwrap(), 42);
	assert_eq!(client.sync_method().await.unwrap(), 10);
	assert_eq!(client.optional_params(None, "a".into()).await.unwrap(), false);
	assert_eq!(client.optional_params(Some(1), "a".into()).await.unwrap(), true);

	assert_eq!(client.array_params(vec![1]).await.unwrap(), 1);
	assert_eq!(
		client.request::<u64, ArrayParams>("foo_array_params", rpc_params![Vec::<u64>::new()]).await.unwrap(),
		0
	);

	assert_eq!(client.request::<bool, ArrayParams>("foo_optional_param", rpc_params![]).await.unwrap(), false);
	assert_eq!(client.request::<bool, ArrayParams>("foo_optional_param", rpc_params![1]).await.unwrap(), true);

	let mut sub = client.sub().await.unwrap();
	let first_recv = sub.next().await.transpose().unwrap();
	assert_eq!(first_recv, Some("Response_A".to_string()));
	let second_recv = sub.next().await.transpose().unwrap();
	assert_eq!(second_recv, Some("Response_B".to_string()));

	let mut sub = client.sub_with_override_notif_method().await.unwrap();
	let recv = sub.next().await.transpose().unwrap();
	assert_eq!(recv, Some(1));
}
