//! Module for shared traits in JSON-RPC related types.

use super::error::RpcError;
use super::RpcParams;
use serde_json::value::RawValue;

/// RPC Call.
pub trait RpcMethod<R>: Fn(RpcParams) -> Result<R, RpcError> + Send + Sync + 'static {}

impl<R, T> RpcMethod<R> for T where T: Fn(RpcParams) -> Result<R, RpcError> + Send + Sync + 'static {}

/// RPC Call Result.
trait RpcResult {
	fn into_json(self, id: Option<&RawValue>) -> anyhow::Result<String>;
}
