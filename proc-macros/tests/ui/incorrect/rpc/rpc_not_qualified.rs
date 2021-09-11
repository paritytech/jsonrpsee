use jsonrpsee::proc_macros::rpc;

// Method without type marker, `#[method(…)]` or `#[subscription(…)]`.
#[rpc(client, server)]
pub trait NotQualified {
	async fn async_method(&self) -> jsonrpsee::types::RpcResult<u8>;
}

fn main() {}
