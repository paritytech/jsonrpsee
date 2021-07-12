use jsonrpsee::proc_macros::rpc;

// Missing mandatory `name` field.
#[rpc(client, server)]
pub trait NoMethodName {
	#[method()]
	async fn async_method(&self) -> u8;
}

fn main() {}
