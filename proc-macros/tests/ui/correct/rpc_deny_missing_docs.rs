//! Test to check that the proc macros actually generates documentation.

#![deny(missing_docs)]

use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::RpcResult;

#[rpc(client, server)]
pub trait ApiWithDocumentation {
	/// Async method.
	#[method(name = "foo")]
	async fn async_method(&self) -> RpcResult<u8>;

	/// Subscription docs.
	#[subscription(name = "sub", item = String)]
	fn sub(&self) -> RpcResult<()>;
}

fn main() {}
