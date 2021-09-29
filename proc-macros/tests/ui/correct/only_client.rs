//! Example of using proc macro to generate working client and server.

use jsonrpsee::{proc_macros::rpc, types::RpcResult};

#[rpc(client)]
pub trait Rpc {
	#[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

	#[method(name = "bar")]
	fn sync_method(&self) -> RpcResult<u16>;

	#[subscription(name = "sub", item = String)]
	fn sub(&self);
}

fn main() {}
