//! Example of using proc macro to generate working client and server with bounds applied.

use std::net::SocketAddr;

use jsonrpsee::core::{async_trait, RpcResult};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::ServerBuilder;
use jsonrpsee::ws_client::*;

pub trait Config {
	type Hash: Send + Sync + 'static;
	type NotUsed;
}

type ExampleHash = [u8; 32];
impl Config for ExampleHash {
	type Hash = Self;
	type NotUsed = ();
}

/// Client only RPC.
#[rpc(client, namespace = "foo", client_bounds(Conf::Hash: jsonrpsee::core::DeserializeOwned))]
pub trait MyRpcC<Conf: Config> {
	#[method(name = "bar")]
	fn method(&self) -> RpcResult<Conf::Hash>;
}

/// Server only RPC.
#[rpc(server, namespace = "foo", server_bounds(Conf::Hash: jsonrpsee::core::Serialize))]
pub trait MyRpcS<Conf: Config> {
	#[method(name = "bar")]
	fn method(&self) -> RpcResult<Conf::Hash>;
}

/// Client and server RPC.
#[rpc(server, client, namespace = "foo", client_bounds(Conf::Hash: jsonrpsee::core::DeserializeOwned), server_bounds(Conf::Hash: jsonrpsee::core::Serialize))]
pub trait MyRpcSC<Conf: Config> {
	#[method(name = "bar")]
	fn method(&self) -> RpcResult<Conf::Hash>;
}

/// Implementation for the `MyRpcS` trait (server only).
pub struct ServerOnlyImpl;
#[async_trait]
impl MyRpcSServer<ExampleHash> for ServerOnlyImpl {
	fn method(&self) -> RpcResult<<ExampleHash as Config>::Hash> {
		Ok([0u8; 32])
	}
}

/// Implementation for the `MyRpcSC` trait (client server rpc).
pub struct ServerClientServerImpl;
#[async_trait]
impl MyRpcSCServer<ExampleHash> for ServerClientServerImpl {
	fn method(&self) -> RpcResult<<ExampleHash as Config>::Hash> {
		Ok([0u8; 32])
	}
}

pub async fn websocket_servers() -> (SocketAddr, SocketAddr) {
	// Start server from `MyRpcS` trait.
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr_server_only = server.local_addr().unwrap();
	let server_handle = server.start(ServerOnlyImpl.into_rpc()).unwrap();

	tokio::spawn(server_handle.stopped());

	// Start server from `MyRpcSC` trait.
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr_server_client = server.local_addr().unwrap();
	let server_handle = server.start(ServerClientServerImpl.into_rpc()).unwrap();

	tokio::spawn(server_handle.stopped());

	(addr_server_only, addr_server_client)
}

#[tokio::main]
async fn main() {
	let (server_addr, server_addr_w_client) = websocket_servers().await;
	let (server_url, server_w_client_url) = (format!("ws://{}", server_addr), format!("ws://{}", server_addr_w_client));
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();
	let client_second = WsClientBuilder::default().build(&server_w_client_url).await.unwrap();

	// Use `MyRpcC` client to communicate to the `MyRpcS` server.
	assert_eq!(MyRpcCClient::<ExampleHash>::method(&client).await.unwrap(), [0u8; 32]);
	// Use `MyRpcC` client to communicate to the `MyRpcSC` server.
	assert_eq!(MyRpcCClient::<ExampleHash>::method(&client_second).await.unwrap(), [0u8; 32]);

	// Use `MyRpcSC` client to communicate to the `MyRpcS` server.
	assert_eq!(MyRpcCClient::<ExampleHash>::method(&client).await.unwrap(), [0u8; 32]);
	// Use `MyRpcSC` client to communicate to the `MyRpcSC` server.
	assert_eq!(MyRpcSCClient::<ExampleHash>::method(&client_second).await.unwrap(), [0u8; 32]);
}
