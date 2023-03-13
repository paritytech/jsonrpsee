use crate::server::subscription::SubscriptionMessage;
use std::fmt;
use tokio::sync::mpsc;

/// Error that may occur during [`SubscriptionSink::try_send`].
#[derive(Debug)]
pub enum TrySendError {
	/// The channel is closed.
	Closed(SubscriptionMessage),
	/// The channel is full.
	Full(SubscriptionMessage),
}

/// Error that may occur during `MethodSink::send` or `SubscriptionSink::send`.
#[derive(Debug)]
pub struct DisconnectError(pub SubscriptionMessage);

/// Error that may occur during `SubscriptionSink::send_timeout`.
#[derive(Debug)]
pub enum SendTimeoutError {
	/// The data could not be sent because the timeout elapsed
	/// which most likely is that the channel is full.
	Timeout(SubscriptionMessage),
	/// The channel is full.
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

impl std::fmt::Display for TrySendError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let msg = match self {
			Self::Closed(_) => "closed",
			Self::Full(_) => "full",
		};
		f.write_str(msg)
	}
}

impl std::fmt::Display for SendTimeoutError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let msg = match self {
			Self::Timeout(_) => "timed out waiting on send operation",
			Self::Closed(_) => "closed",
		};
		f.write_str(msg)
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
