use jsonrpsee::proc_macros::rpc;

// Method without type marker.
#[rpc(client, server)]
pub trait NotQualified {
	async fn async_method(&self) -> u8;
}

fn main() {}
