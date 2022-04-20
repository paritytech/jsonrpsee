use jsonrpsee::proc_macros::rpc;

#[rpc(client, server)]
pub trait DuplicatedSubAlias {
	#[subscription(name = "subscribeAlias", item = String, aliases = ["hello_is_goodbye"], unsubscribe_aliases = ["hello_is_goodbye"])]
	fn async_method(&self);
}

fn main() {}
