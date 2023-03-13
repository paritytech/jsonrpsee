// Copyright 2019-2021 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::server::subscription::SubscriptionMessage;
use std::fmt;
use tokio::sync::mpsc;

/// Error that may occur during [`crate::server::SubscriptionSink::try_send`].
#[derive(Debug, thiserror::Error)]
pub enum TrySendError {
	/// The channel is closed.
	#[error("Closed")]
	Closed(SubscriptionMessage),
	/// The channel is full.
	#[error("Full")]
	Full(SubscriptionMessage),
}

/// Error that may occur during [`crate::server::MethodSink::send`] or [`crate::server::SubscriptionSink::send`].
#[derive(Debug, thiserror::Error)]
pub struct DisconnectError(pub SubscriptionMessage);

/// Error that may occur during [`crate::server::SubscriptionSink::send_timeout`].
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
pub struct SubscriptionAcceptRejectError;

impl std::fmt::Display for SubscriptionAcceptRejectError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("The remote peer closed the connection")
	}
}

impl std::fmt::Display for DisconnectError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str("Closed")
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
