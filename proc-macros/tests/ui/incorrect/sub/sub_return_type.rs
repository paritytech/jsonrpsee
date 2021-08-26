use jsonrpsee::proc_macros::rpc;

// Subscription method must not have return type.
#[rpc(client, server)]
pub trait SubWithReturnType {
	#[subscription(name = "sub", item = u8)]
	fn sub(&self) -> u8;
}

fn main() {}
