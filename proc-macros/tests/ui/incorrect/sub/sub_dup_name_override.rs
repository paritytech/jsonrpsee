use jsonrpsee::{core::RpcResult, proc_macros::rpc};

// Subscription method must not use the same override name.
#[rpc(client, server)]
pub trait DupOverride {
	#[subscription(name = "subscribeOne" => "override", item = u8)]
	fn one(&self) -> RpcResult<()>;
	#[subscription(name = "subscribeTwo" => "override", item = u8)]
	fn two(&self) -> RpcResult<()>;
}

fn main() {}
