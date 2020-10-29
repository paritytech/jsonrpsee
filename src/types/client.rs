use crate::types::jsonrpc_v2;

/// Error produced by the client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Networking error or error on the low-level protocol layer (e.g. missing field,
	/// invalid ID, etc.).
	#[error("Networking or low-level protocol error: {0}")]
	TransportError(#[source] Box<dyn std::error::Error + Send + Sync>),
	/// RawServer responded to our request with an error.
	#[error("Server responded to our request with an error: {0:?}")]
	Request(#[source] jsonrpc_v2::Error),
	/// Subscription error.
	#[error("Subscription to subscribe_method: {0} with unsubscribe_metho: {1} failed")]
	Subscription(String, String),
	/// Frontend/backend channel error.
	#[error("Frontend/backend channel error: {0}")]
	InternalChannel(#[from] futures::channel::mpsc::SendError),
	/// Failed to parse the data that the server sent back to us.
	#[error("Parse error: {0}")]
	ParseError(#[source] jsonrpc_v2::ParseError),
}
