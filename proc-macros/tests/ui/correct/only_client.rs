//! Example of using proc macro to generate working client and server.

use jsonrpsee::{proc_macros::rpc, types::JsonRpcResult};

#[rpc(client)]
pub trait Rpc {
	#[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: String) -> JsonRpcResult<u16>;

	#[method(name = "bar")]
	fn sync_method(&self) -> JsonRpcResult<u16>;

	#[subscription(name = "sub", unsub = "unsub", item = String)]
	fn sub(&self);
}

fn main() {}
