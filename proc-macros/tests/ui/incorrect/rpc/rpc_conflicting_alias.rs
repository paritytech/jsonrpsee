use jsonrpsee::proc_macros::rpc;

#[rpc(client, server)]
pub trait DuplicatedAlias {
	#[method(name = "foo", alias = "foo_dup", "foo_dup")]
	async fn async_method(&self) -> jsonrpsee::types::JsonRpcResult<u8>;
}

fn main() {}
