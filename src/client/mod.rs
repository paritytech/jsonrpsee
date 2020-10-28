use crate::common;
use std::error;

#[cfg(feature = "http")]
mod http;
#[cfg(feature = "ws")]
mod ws;

// TODO: just export `Client` because the underlying layers is not likely to be used.
// Unless we want the user to have to possibility to not spawn a background thread to
// handle responses.
#[cfg(feature = "http")]
pub use http::{Client as HttpClient, HttpTransportClient, RawClient as HttpRawClient};
#[cfg(feature = "ws")]
pub use ws::{Client as WsClient, RawClient as RawWsClient, Subscription as WsSubscription, WsTransportClient};

/// Error produced by the client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Networking error or error on the low-level protocol layer (e.g. missing field,
	/// invalid ID, etc.).
	#[error("Networking or low-level protocol error: {0}")]
	TransportError(#[source] Box<dyn error::Error + Send + Sync>),
	/// RawServer responded to our request with an error.
	#[error("Server responded to our request with an error: {0:?}")]
	Request(#[source] common::Error),
	/// Subscription error.
	#[error("Subscription of subscribe_method={0}, unsubscribe_method={1} failed")]
	Subscription(String, String),
	/// Frontend/backend channel error.
	#[error("Frontend/backend channel error: {0}")]
	InternalChannel(#[from] futures::channel::mpsc::SendError),
	/// Failed to parse the data that the server sent back to us.
	#[error("Parse error: {0}")]
	ParseError(#[source] common::ParseError),
}
