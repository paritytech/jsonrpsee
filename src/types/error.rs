use crate::types::jsonrpc;

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
	#[error("Invalid ID in response from the server.")]
	InvalidRequestId,
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
