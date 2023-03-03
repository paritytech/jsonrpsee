use std::net::SocketAddr;

use jsonrpsee::core::{async_trait, RpcResult, SubscriptionResult};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::{PendingSubscriptionSink, ServerBuilder};

#[rpc(server)]
pub trait Rpc {
	#[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

	#[method(name = "bar")]
	fn sync_method(&self) -> RpcResult<u16>;

	#[subscription(name = "subscribe", item = String)]
	async fn sub(&self);
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn async_method(&self, _param_a: u8, _param_b: String) -> RpcResult<u16> {
		Ok(42u16)
	}

	fn sync_method(&self) -> RpcResult<u16> {
		Ok(10u16)
	}

	async fn sub(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
		let sink = pending.accept().await.map_err(|_| None)?;

		sink.send("Response_A".into()).await.map_err(|_| None)?;
		sink.send("Response_B".into()).await.map_err(|_| None)?;

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
	let _server_addr = server().await;
}
