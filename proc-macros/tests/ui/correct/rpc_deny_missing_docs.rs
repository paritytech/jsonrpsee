//! Test to check that the proc macros actually generates documentation.

#![deny(missing_docs)]

use jsonrpsee::proc_macros::rpc;

#[rpc(client, server)]
pub trait ApiWithoutDocumentation {
	/// Async method.
	#[method(name = "foo")]
	async fn async_method(&self) -> jsonrpsee::types::JsonRpcResult<u8>;

	/// Subscription docs.
	#[subscription(name = "sub", item = String)]
	fn sub(&self);
}

fn main() {}
