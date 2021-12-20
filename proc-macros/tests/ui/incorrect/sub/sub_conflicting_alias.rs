use jsonrpsee::{proc_macros::rpc, core::RpcResult};

#[rpc(client, server)]
pub trait DuplicatedSubAlias {
	#[subscription(name = "alias", item = String, aliases = ["hello_is_goodbye"], unsubscribe_aliases = ["hello_is_goodbye"])]
	fn async_method(&self) -> RpcResult<()>;
}

fn main() {}
