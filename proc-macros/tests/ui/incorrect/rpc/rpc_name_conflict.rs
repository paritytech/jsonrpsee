use jsonrpsee::proc_macros::rpc;

// Associated items are forbidden.
#[rpc(client, server)]
pub trait MethodNameConflict {
	#[method(name = "foo")]
	async fn foo(&self) -> u8;

	#[method(name = "foo")]
	async fn bar(&self) -> u8;
}

fn main() {}
