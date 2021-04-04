//! Module for shared traits in JSON-RPC related types.

use super::{error::RpcError, RpcParams};

/// RPC Call.
pub trait RpcMethod<R>: Fn(RpcParams) -> Result<R, RpcError> + Send + Sync + 'static {}

impl<R, T> RpcMethod<R> for T where T: Fn(RpcParams) -> Result<R, RpcError> + Send + Sync + 'static {}
