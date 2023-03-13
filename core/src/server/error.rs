use crate::server::subscription::SubscriptionMessage;
use std::fmt;
use tokio::sync::mpsc;

/// Error that may occur during [`SubscriptionSink::try_send`].
#[derive(Debug, thiserror::Error)]
pub enum TrySendError {
	/// The channel is closed.
	#[error("Closed")]
	Closed(SubscriptionMessage),
	/// The channel is full.
	#[error("Full")]
	Full(SubscriptionMessage),
}

/// Error that may occur during `MethodSink::send` or `SubscriptionSink::send`.
#[derive(Debug)]
pub struct DisconnectError(pub SubscriptionMessage);

/// Error that may occur during `SubscriptionSink::send_timeout`.
#[derive(Debug, thiserror::Error)]
pub enum SendTimeoutError {
	/// The data could not be sent because the timeout elapsed
	/// which most likely is that the channel is full.
	#[error("Timed out waiting on send operation")]
	Timeout(SubscriptionMessage),
	/// The channel is full.
	#[error("Closed")]
	Closed(SubscriptionMessage),
}

/// The error returned while accepting or rejecting a subscription.
#[derive(Debug, Copy, Clone, thiserror::Error)]
pub enum SubscriptionAcceptRejectError {
	/// The remote peer closed the connection or called the unsubscribe method.
	#[error("The remote peer closed the connection or called the unsubscribe method")]
	RemotePeerAborted,
	/// The subscription response message was too large.
	#[error("The subscription response message was too large")]
	MessageTooLarge,
}

impl std::fmt::Display for DisconnectError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("closed")
	}
}

impl std::error::Error for DisconnectError {
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
		None
	}
}

impl From<mpsc::error::SendError<String>> for DisconnectError {
	fn from(e: mpsc::error::SendError<String>) -> Self {
		DisconnectError(SubscriptionMessage::from_complete_message(e.0))
	}
}

impl From<mpsc::error::TrySendError<String>> for TrySendError {
	fn from(e: mpsc::error::TrySendError<String>) -> Self {
		match e {
			mpsc::error::TrySendError::Closed(m) => Self::Closed(SubscriptionMessage::from_complete_message(m)),
			mpsc::error::TrySendError::Full(m) => Self::Full(SubscriptionMessage::from_complete_message(m)),
		}
	}
}

impl From<mpsc::error::SendTimeoutError<String>> for SendTimeoutError {
	fn from(e: mpsc::error::SendTimeoutError<String>) -> Self {
		match e {
			mpsc::error::SendTimeoutError::Closed(m) => Self::Closed(SubscriptionMessage::from_complete_message(m)),
			mpsc::error::SendTimeoutError::Timeout(m) => Self::Timeout(SubscriptionMessage::from_complete_message(m)),
		}
	}
}
