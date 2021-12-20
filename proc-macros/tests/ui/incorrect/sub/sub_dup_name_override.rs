use jsonrpsee::{proc_macros::rpc, core::RpcResult};

// Subscription method must not use the same override name.
#[rpc(client, server)]
pub trait DupOverride {
	#[subscription(name = "one" => "override", item = u8)]
	fn one(&self) -> RpcResult<()>;
	#[subscription(name = "two" => "override", item = u8)]
	fn two(&self) -> RpcResult<()>;
}

fn main() {}
