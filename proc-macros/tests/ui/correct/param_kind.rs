use jsonrpsee::{
	proc_macros::rpc,
	types::{async_trait, RpcResult},
	ws_client::*,
	ws_server::WsServerBuilder,
};

use std::net::SocketAddr;

#[rpc(client, server, namespace = "foo")]
pub trait Rpc {
	#[method(name = "method_with_array_param", param_kind = array)]
	async fn method_with_array_param(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

	#[method(name="method_with_map_param", param_kind= map)]
	async fn method_with_map_param(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

	#[method(name="method_with_default_param")]
	async fn method_with_default_param(&self, param_a: u8, param_b: String) -> RpcResult<u16>;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn method_with_array_param(&self, param_a: u8, param_b: String) -> RpcResult<u16> {
		assert_eq!(param_a, 0);
		assert_eq!(&param_b, "a");
		Ok(42u16)
	}

	async fn method_with_map_param(&self, param_a: u8, param_b: String) -> RpcResult<u16> {
		assert_eq!(param_a, 0);
		assert_eq!(&param_b, "a");
		Ok(42u16)
	}

	async fn method_with_default_param(&self, param_a: u8, param_b: String) -> RpcResult<u16> {
		assert_eq!(param_a, 0);
		assert_eq!(&param_b, "a");
		Ok(42u16)
	}
}

pub async fn websocket_server() -> SocketAddr {
	let server = WsServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr = server.local_addr().unwrap();

	server.start(RpcServerImpl.into_rpc()).unwrap();

	addr
}

#[tokio::main]
async fn main() {
	let server_addr = websocket_server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	assert_eq!(client.method_with_array_param(0, "a".into()).await.unwrap(), 42);
	assert_eq!(client.method_with_map_param(0, "a".into()).await.unwrap(), 42);
	assert_eq!(client.method_with_default_param(0, "a".into()).await.unwrap(), 42);
}
