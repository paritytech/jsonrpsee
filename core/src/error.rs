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

use std::fmt;

use jsonrpsee_types::error::CallError;
use serde::{Deserialize, Serialize};

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

// NOTE(niklasad1): this `From` impl is a bit opinionated to regard all generic errors as `CallError`.
// In practice this should be the most common use case for users of this library.
impl From<anyhow::Error> for Error {
	fn from(err: anyhow::Error) -> Self {
		Error::Call(CallError::Failed(err))
	}
}

/// Error type.
#[derive(Debug)]
pub enum Error {
	/// Error that occurs when a call failed.
	Call(CallError),
	/// Networking error or error on the low-level protocol layer.
	Transport(anyhow::Error),
	/// JSON-RPC request error.
	Request(String),
	/// Frontend/backend channel error.
	Internal(futures_channel::mpsc::SendError),
	/// Invalid response,
	InvalidResponse(Mismatch<String>),
	/// The background task has been terminated.
	RestartNeeded(String),
	/// Failed to parse the data.
	ParseError(serde_json::Error),
	/// Invalid subscription ID.
	InvalidSubscriptionId,
	/// Invalid request ID.
	InvalidRequestId,
	/// Client received a notification with an unregistered method
	UnregisteredNotification(String),
	/// A request with the same request ID has already been registered.
	DuplicateRequestId,
	/// Method was already registered.
	MethodAlreadyRegistered(String),
	MethodNotFound(String),
	/// Subscribe and unsubscribe method names are the same.
	SubscriptionNameConflict(String),
	/// Subscription got closed.
	SubscriptionClosed(SubscriptionClosed),
	/// Request timeout
	RequestTimeout,
	/// Configured max number of request slots exceeded.
	MaxSlotsExceeded,
	/// Attempted to stop server that is already stopped.
	AlreadyStopped,
	/// List passed into `set_allowed_origins` was empty
	EmptyAllowList(&'static str),
	/// Failed to execute a method because a resource was already at capacity
	ResourceAtCapacity(&'static str),
	/// Failed to register a resource due to a name conflict
	ResourceNameAlreadyTaken(&'static str),
	/// Failed to initialize resources for a method at startup
	ResourceNameNotFoundForMethod(&'static str, &'static str),
	/// Trying to claim resources for a method execution, but the method resources have not been initialized
	UninitializedMethod(Box<str>),
	/// Failed to register a resource due to a maximum number of resources already registered
	MaxResourcesReached,
	/// Custom error.
	Custom(String),
	/// Not implemented for HTTP clients.
	HttpNotImplemented,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "lazy")
	}
}

impl Error {
	/// Create `Error::CallError` from a generic error.
	/// Useful if you don't care about specific JSON-RPC error code and
	/// just wants to return your custom error type.
	pub fn to_call_error<E>(err: E) -> Self
	where
		E: std::error::Error + Send + Sync + 'static,
	{
		Error::Call(CallError::from_std_error(err))
	}
}

/// A type with a special `subscription_closed` field to detect that
/// a subscription has been closed to distinguish valid items produced
/// by the server on the subscription stream from an error.
///
/// This is included in the `result field` of the SubscriptionResponse
/// when an error is reported by the server.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SubscriptionClosed {
	reason: SubscriptionClosedReason,
}

impl From<SubscriptionClosedReason> for SubscriptionClosed {
	fn from(reason: SubscriptionClosedReason) -> Self {
		Self::new(reason)
	}
}

impl SubscriptionClosed {
	/// Create a new [`SubscriptionClosed`].
	pub fn new(reason: SubscriptionClosedReason) -> Self {
		Self { reason }
	}

	/// Get the close reason.
	pub fn close_reason(&self) -> &SubscriptionClosedReason {
		&self.reason
	}
}

/// A type to represent when a subscription gets closed
/// by either the server or client side.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub enum SubscriptionClosedReason {
	/// The subscription was closed by calling the unsubscribe method.
	Unsubscribed,
	/// The client closed the connection.
	ConnectionReset,
	/// The server closed the subscription, providing a description of the reason as a `String`.
	Server(String),
}

/// Generic transport error.
#[derive(Debug)]
pub enum GenericTransportError<T: Send + Sync> {
	/// Request was too large.
	TooLarge,
	/// Malformed request
	Malformed,
	/// Concrete transport error.
	Inner(T),
}

impl From<std::io::Error> for Error {
	fn from(io_err: std::io::Error) -> Error {
		Error::Transport(io_err.into())
	}
}

#[cfg(feature = "server")]
impl From<soketto::handshake::Error> for Error {
	fn from(handshake_err: soketto::handshake::Error) -> Error {
		Error::Transport(handshake_err.into())
	}
}

#[cfg(feature = "server")]
impl From<soketto::connection::Error> for Error {
	fn from(conn_err: soketto::connection::Error) -> Error {
		Error::Transport(conn_err.into())
	}
}

#[cfg(feature = "server")]
impl From<hyper::Error> for Error {
	fn from(hyper_err: hyper::Error) -> Error {
		Error::Transport(hyper_err.into())
	}
}

#[cfg(test)]
mod tests {
	use super::{SubscriptionClosed, SubscriptionClosedReason};

	#[test]
	fn subscription_closed_ser_deser_works() {
		let items: Vec<(&str, SubscriptionClosed)> = vec![
			(r#"{"reason":"Unsubscribed"}"#, SubscriptionClosedReason::Unsubscribed.into()),
			(r#"{"reason":"ConnectionReset"}"#, SubscriptionClosedReason::ConnectionReset.into()),
			(r#"{"reason":{"Server":"hoho"}}"#, SubscriptionClosedReason::Server("hoho".into()).into()),
		];

		for (s, d) in items {
			let dsr: SubscriptionClosed = serde_json::from_str(s).unwrap();
			assert_eq!(dsr, d);
			let ser = serde_json::to_string(&d).unwrap();
			assert_eq!(ser, s);
		}
	}

	#[test]
	fn subscription_closed_deny_unknown_field() {
		let ser = r#"{"reason":"Unsubscribed","deny":1}"#;
		assert!(serde_json::from_str::<SubscriptionClosed>(ser).is_err());
	}
}
