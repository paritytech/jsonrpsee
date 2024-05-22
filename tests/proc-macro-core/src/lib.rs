//! Test module for the proc-macro API to make sure that it compiles with the core features.

use jsonrpsee::core::{async_trait, SubscriptionResult};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::types::ErrorObjectOwned;
use jsonrpsee::{Extensions, PendingSubscriptionSink, SubscriptionMessage};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum PubSubKind {
	A,
	B,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubSubParams {
	params: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubSubItem {
	result: String,
}

#[rpc(client, server)]
pub trait Api {
	#[method(name = "sync_call")]
	fn sync_call(&self, a: String) -> Result<String, ErrorObjectOwned>;

	#[method(name = "async_call")]
	async fn async_call(&self, a: String) -> Result<String, ErrorObjectOwned>;

	#[subscription(name = "subscribe", item = PubSubItem)]
	async fn sub(&self, kind: PubSubKind, params: PubSubParams) -> SubscriptionResult;

	#[subscription(name = "subscribeSync", item = String)]
	fn sync_sub(&self, a: String) -> SubscriptionResult;

	#[method(name = "blocking", blocking)]
	fn blocking_method(&self, a: String) -> Result<u16, ErrorObjectOwned>;
}

#[async_trait]
impl ApiServer for () {
	fn sync_call(&self, _ext: &Extensions, _: String) -> Result<String, ErrorObjectOwned> {
		Ok("sync_call".to_string())
	}

	async fn async_call(&self, _ext: &Extensions, _: String) -> Result<String, ErrorObjectOwned> {
		Ok("async_call".to_string())
	}

	async fn sub(
		&self,
		pending: PendingSubscriptionSink,
		_ext: &Extensions,
		_: PubSubKind,
		_: PubSubParams,
	) -> SubscriptionResult {
		let sink = pending.accept().await?;
		sink.send(SubscriptionMessage::from("msg")).await?;
		Ok(())
	}

	fn sync_sub(&self, _: PendingSubscriptionSink, _ext: &Extensions, _: String) -> SubscriptionResult {
		Ok(())
	}

	fn blocking_method(&self, _ext: &Extensions, _: String) -> Result<u16, ErrorObjectOwned> {
		Ok(42)
	}
}
