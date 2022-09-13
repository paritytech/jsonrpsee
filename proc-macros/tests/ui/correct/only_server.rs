use std::net::SocketAddr;

use jsonrpsee::core::{async_trait, RpcResult};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::{ServerBuilder, SubscriptionSink};
use jsonrpsee::types::SubscriptionResult;

#[rpc(server)]
pub trait Rpc {
	#[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

	#[method(name = "bar")]
	fn sync_method(&self) -> RpcResult<u16>;

	#[subscription(name = "subscribe", item = String)]
	fn sub(&self);
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

	fn sub(&self, mut sink: SubscriptionSink) -> SubscriptionResult {
		sink.accept()?;

		let _ = sink.send(&"Response_A");
		let _ = sink.send(&"Response_B");
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
