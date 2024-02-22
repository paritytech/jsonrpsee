use jsonrpsee::proc_macros::rpc;

#[rpc(server)]
pub trait BlockingMethodCannotBeRaw {
	#[method(name = "a", blocking, raw_method)]
	fn a(&self, param: Vec<u8>);
}

fn main() {}
