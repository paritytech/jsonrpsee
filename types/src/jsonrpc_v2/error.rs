use thiserror::Error;

/// Error.
#[derive(Error, Debug)]
pub enum RpcError {
	#[error("unknown rpc error")]
	Unknown,
	#[error("invalid params")]
	InvalidParams,
}
