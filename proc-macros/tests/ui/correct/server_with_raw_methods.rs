//! Example of using proc macro to generate working client.

use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::ErrorObjectOwned;

#[rpc(server)]
pub trait Rpc {
	#[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: String) -> Result<u16, ErrorObjectOwned>;

	#[method(name = "bar", raw_method)]
	fn sync_method(&self) -> Result<u16, ErrorObjectOwned>;
}

fn main() {}
