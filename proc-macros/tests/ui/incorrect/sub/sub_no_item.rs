use jsonrpsee::proc_macros::rpc;

// Missing mandatory `item` field.
#[rpc(client, server)]
pub trait NoSubItem {
	#[subscription(name = "sub")]
	fn sub(&self);
}

fn main() {}
