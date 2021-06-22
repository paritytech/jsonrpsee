use jsonrpsee::proc_macros::rpc;

// Unsupported attribute field.
#[rpc(client, server)]
pub trait UnexpectedField {
	#[method(name = "foo", magic = false)]
	async fn async_method(&self) -> u8;
}

fn main() {}
