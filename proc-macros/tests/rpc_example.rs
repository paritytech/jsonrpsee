//! Example of using proc macro to generate working client and server.

use jsonrpsee_proc_macros::rpc;

#[rpc(client, namespace = "foo")]
pub trait Rpc {
	#[method(name = "foo")]
	async fn foo(&self, param_a: u8, param_b: &str) -> u16;
}
