use jsonrpsee::proc_macros::rpc;

// Missing all the mandatory fields.
#[rpc(client, server)]
pub trait SubEmptyAttr {
	#[subscription()]
	fn sub(&self);
}

fn main() {}
