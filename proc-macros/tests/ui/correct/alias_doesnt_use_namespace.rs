use jsonrpsee::{proc_macros::rpc, types::JsonRpcResult};

#[rpc(client, server, namespace = "myapi")]
pub trait Rpc {
	/// Alias doesn't use the namespace so not duplicated.
	#[method(name = "getTemp", aliases = "getTemp")]
	async fn async_method(&self, param_a: u8, param_b: String) -> JsonRpcResult<u16>;

	#[subscription(name = "getFood", item = String, aliases = "getFood", unsubscribe_aliases = "unsubscribegetFood")]
	fn sub(&self);
}

fn main() {}
