use std::net::SocketAddr;

use jsonrpsee::core::{RpcResult, async_trait};
use jsonrpsee::open_rpc::utoipa;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::ServerBuilder;

#[derive(utoipa::ToSchema, serde::Serialize, serde::Deserialize)]
pub struct AddRequest {
	pub a: u8,
	pub b: u8,
}

#[derive(utoipa::ToSchema, serde::Serialize, serde::Deserialize, Clone)]
pub struct AddResponse {
	pub sum: u16,
}

#[derive(utoipa::ToSchema, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
	Add,
	Mul,
	Sub,
}

#[rpc(server, discover)]
pub trait Rpc {
	#[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

	#[method(name = "add")]
	async fn add(&self, request: AddRequest) -> RpcResult<AddResponse>;

	#[method(name = "calculate")]
	async fn calculate(&self, args: Vec<i64>, operation: Operation) -> RpcResult<i64>;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn async_method(&self, _param_a: u8, _param_b: String) -> RpcResult<u16> {
		Ok(42u16)
	}

	async fn add(&self, request: AddRequest) -> RpcResult<AddResponse> {
		Ok(AddResponse { sum: request.a as u16 + request.b as u16 })
	}

	async fn calculate(&self, args: Vec<i64>, operation: Operation) -> RpcResult<i64> {
		match operation {
			Operation::Add => Ok(args.iter().sum()),
			Operation::Mul => Ok(args.iter().product()),
			Operation::Sub => Ok(args.iter().skip(1).fold(args[0], |acc, x| acc - x)),
		}
	}
}

pub async fn server() -> SocketAddr {
	let server = ServerBuilder::default().build("127.0.0.1:8080").await.unwrap();
	let addr = server.local_addr().unwrap();
	let server_handle = server.start(RpcServerImpl.into_rpc());

	tokio::spawn(server_handle.stopped());

	tokio::signal::ctrl_c().await.unwrap();
	addr
}

#[tokio::main]
async fn main() {
	let _server_addr = server().await;
}
