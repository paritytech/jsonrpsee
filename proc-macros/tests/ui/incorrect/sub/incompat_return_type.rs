use jsonrpsee::proc_macros::rpc;

struct R;

// Unsupported attribute field.
#[rpc(client, server)]
pub trait InvalidReturnType {
	#[subscription(name = "sub", unsubscribe = "unsub", item = u8)]
	async fn sub(&self) -> R;
}

fn main() {}
