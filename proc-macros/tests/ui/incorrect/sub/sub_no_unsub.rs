use jsonrpsee::proc_macros::rpc;

// Missing mandatory `unsub` field.
#[rpc(client, server)]
pub trait NoSubUnsub {
	#[subscription(name = "sub", item = String)]
	fn sub(&self);
}

fn main() {}
