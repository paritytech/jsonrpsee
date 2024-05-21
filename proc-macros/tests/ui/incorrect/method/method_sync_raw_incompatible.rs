use jsonrpsee::proc_macros::rpc;

#[rpc(server)]
pub trait SyncMethodCannotBeRaw {
	#[method(name = "a", raw_method)]
	fn a(&self, param: Vec<u8>) -> RpcResult<u16>;
}

fn main() {}
