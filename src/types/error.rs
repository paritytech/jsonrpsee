use crate::types::jsonrpc::{self, JsonValue};
use std::fmt;
/// Convenience type for displaying errors.
#[derive(Clone, Debug, PartialEq)]
pub struct Mismatch<T> {
	/// Expected value.
	pub expected: T,
	/// Actual value.
	pub got: T,
}

impl<T: fmt::Display> fmt::Display for Mismatch<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_fmt(format_args!("Expected: {}, Got: {}", self.expected, self.got))
	}
}

/// Error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Networking error or error on the low-level protocol layer.
	#[error("Networking or low-level protocol error: {0}")]
	TransportError(#[source] Box<dyn std::error::Error + Send + Sync>),
	/// JSON-RPC request error.
	#[error("JSON-RPC request error: {0:?}")]
	Request(#[source] jsonrpc::Error),
	/// Subscription error.
	#[error("Subscription to subscribe_method: {0} with unsubscribe_method: {1} failed")]
	Subscription(String, String),
	/// Frontend/backend channel error.
	#[error("Frontend/backend channel error: {0}")]
	Internal(#[source] futures::channel::mpsc::SendError),
	/// Failed to parse the data that the server sent back to us.
	#[error("Parse error: {0}")]
	ParseError(#[source] jsonrpc::ParseError),
	/// Invalid id in response to a request.
	#[error("Invalid ID in response: {0}")]
	InvalidRequestId(Mismatch<JsonValue>),
	/// Method was already registered.
	#[error("Method: {0} already registered")]
	MethodAlreadyRegistered(String),
	#[error("Custom error: {0}")]
	/// Custom error.
	Custom(String),
}

/// Generic transport error.
#[derive(Debug, thiserror::Error)]
pub enum GenericTransportError<T: std::error::Error + Send + Sync> {
	/// Request was too large.
	#[error("The request was too big")]
	TooLarge,
	/// Concrete transport error.
	#[error("Transport error: {0}")]
	Inner(T),
}
