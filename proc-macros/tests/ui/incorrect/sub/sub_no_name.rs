use jsonrpsee::proc_macros::rpc;

// Missing mandatory `name` field.
#[rpc(client, server)]
pub trait NoSubName {
	#[subscription(unsub = "unsub", item = String)]
	async fn async_method(&self) -> u8;
}

fn main() {}
