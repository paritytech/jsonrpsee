use jsonrpsee::{proc_macros::rpc, types::JsonRpcResult};

// Associated items are forbidden.
#[rpc(client, server)]
pub trait MethodNameConflict {
	#[method(name = "foo")]
	async fn foo(&self) -> JsonRpcResult<u8>;

	#[method(name = "foo")]
	async fn bar(&self) -> JsonRpcResult<u8>;
}

fn main() {}
