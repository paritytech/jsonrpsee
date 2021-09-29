use jsonrpsee::proc_macros::rpc;

// Subscription method must not be async.
#[rpc(client, server)]
pub trait AsyncSub {
	#[subscription(name = "sub", item = u8)]
	async fn sub(&self);
}

fn main() {}
