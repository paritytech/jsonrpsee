use thiserror::Error;

/// Error.
#[derive(Error, Debug)]
pub enum RpcError {
	/// Unknown error.
	#[error("unknown rpc error")]
	Unknown,
	/// Invalid params in the RPC call.
	#[error("invalid params")]
	InvalidParams,
}

/// Parse error code.
pub const PARSE_ERROR_CODE: i32 = -32700;
/// Internal error code.
pub const INTERNAL_ERROR_CODE: i32 = -32603;
/// Invalid params error code.
pub const INVALID_PARAMS_CODE: i32 = -32602;
/// Invalid request error code.
pub const INVALID_REQUEST_CODE: i32 = -32600;
/// Method not found error code.
pub const METHOD_NOT_FOUND_CODE: i32 = -32601;

/// Parse error message
pub const PARSE_ERROR_MSG: &str = "Parse error";
/// Internal error message.
pub const INTERNAL_ERROR_MSG: &str = "Internal error";
/// Invalid params error message.
pub const INVALID_PARAMS_MSG: &str = "Invalid params";
/// Invalid request error message.
pub const INVALID_REQUEST_MSG: &str = "Invalid request";
/// Method not found error message.
pub const METHOD_NOT_FOUND_MSG: &str = "Method not found";
