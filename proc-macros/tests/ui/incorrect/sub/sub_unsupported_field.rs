use jsonrpsee::proc_macros::rpc;

// Unsupported attribute field.
#[rpc(client, server)]
pub trait UnsupportedField {
	#[subscription(name = "sub", unsub = "unsub", item = u8, magic = true)]
	fn sub(&self);
}

fn main() {}
