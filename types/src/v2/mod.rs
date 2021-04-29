use crate::error::Error;
use serde::de::DeserializeOwned;
use serde_json::value::RawValue;

/// JSON-RPC error related types.
pub mod error;
/// JSON_RPC params related types.
pub mod params;
/// JSON-RPC request object related types
pub mod request;
/// JSON-RPC response object related types.
pub mod response;

/// Parse request ID from RawValue.
pub fn parse_request_id<T: DeserializeOwned>(raw: Option<&RawValue>) -> Result<T, Error> {
	match raw {
		None => Err(Error::InvalidRequestId),
		Some(v) => {
			let val = serde_json::from_str(v.get()).map_err(|_| Error::InvalidRequestId)?;
			Ok(val)
		}
	}
}
