//! Example of using proc macro to generate working client and server.

use jsonrpsee_proc_macros::rpc;
use std::borrow::Cow;

#[rpc(client, server, namespace = "foo")]
pub trait Rpc {
	#[method(name = "foo")]
	async fn async_method(&self, param_a: u8, param_b: Option<Cow<'_, str>>) -> u16;

	#[method(name = "bar")]
	fn sync_method(&self) -> u16;

	#[subscription(name = "sub", unsub = "unsub", item = String)]
	fn sub(&self);
}
