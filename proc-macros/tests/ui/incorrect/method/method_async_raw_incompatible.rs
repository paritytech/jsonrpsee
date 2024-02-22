use jsonrpsee::proc_macros::rpc;

#[rpc(server)]
pub trait AsyncMethodCannotBeRaw {
	#[method(name = "a", raw_method)]
	async fn a(&self, param: Vec<u8>);
}

fn main() {}
