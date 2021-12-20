use jsonrpsee::{proc_macros::rpc, core::RpcResult};

// Subscription method name conflict with notif override.
#[rpc(client, server)]
pub trait DupName {
	#[subscription(name = "one" => "one", item = u8)]
	fn one(&self) -> RpcResult<()>;
}

fn main() {}
