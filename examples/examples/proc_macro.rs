//! Example of using proc macro to generate working client and server.

use jsonrpsee::{core::RpcResult, proc_macros::rpc};

#[rpc(client)]
pub trait Rpc {
	#[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

	// #[method(name = "bar")]
	// fn sync_method(&self) -> RpcResult<u16>;

	// #[subscription(name = "subscribe", item = String)]
	// fn sub(&self);
}

fn main() {}
