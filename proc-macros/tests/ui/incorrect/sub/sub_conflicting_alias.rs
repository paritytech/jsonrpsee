use jsonrpsee::proc_macros::rpc;

#[rpc(client, server)]
pub trait DuplicatedSubAlias {
	#[subscription(name = "alias", item = String, alias = "hello_is_goodbye", unsubscribe_alias = "hello_is_goodbye")]
	fn async_method(&self);
}

fn main() {}
