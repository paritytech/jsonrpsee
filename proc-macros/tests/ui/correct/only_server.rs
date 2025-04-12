use std::net::SocketAddr;

use jsonrpsee::core::{RpcResult, SubscriptionResult, async_trait};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::{PendingSubscriptionSink, ServerBuilder, SubscriptionMessage};

#[rpc(server)]
pub trait Rpc {
	#[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

	#[method(name = "bar")]
	fn sync_method(&self) -> RpcResult<u16>;

	#[subscription(name = "subscribe", item = String)]
	async fn sub(&self) -> SubscriptionResult;
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
		let sink = pending.accept().await?;

		let response_a = RawValue::from_string("\"Response_A\"".into()).unwrap();
		let response_b = RawValue::from_string("\"Response_B\"".into()).unwrap();

		sink.send(response_a.into()).await?;
		sink.send(response_b.into()).await?;

		Ok(())
	}
}

pub async fn server() -> SocketAddr {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr = server.local_addr().unwrap();
	let server_handle = server.start(RpcServerImpl.into_rpc());

	tokio::spawn(server_handle.stopped());
	addr
}

#[tokio::main]
async fn main() {
	let _server_addr = server().await;
}
