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

use jsonrpsee_types::error::{
	CallError, ErrorObject, ErrorObjectOwned, CALL_EXECUTION_FAILED_CODE, INVALID_PARAMS_CODE, UNKNOWN_ERROR_CODE,
};

/// Convenience type for displaying errors.
#[derive(Clone, Debug, PartialEq, Eq)]
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
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Error that occurs when a call failed.
	#[error("{0}")]
	Call(#[from] CallError),
	/// Networking error or error on the low-level protocol layer.
	#[error("Networking or low-level protocol error: {0}")]
	Transport(#[source] anyhow::Error),
	/// Invalid response,
	#[error("Invalid response: {0}")]
	InvalidResponse(Mismatch<String>),
	/// The background task has been terminated.
	#[error("The background task been terminated because: {0}; restart required")]
	RestartNeeded(String),
	/// Failed to parse the data.
	#[error("Parse error: {0}")]
	ParseError(#[from] serde_json::Error),
	/// Invalid subscription ID.
	#[error("Invalid subscription ID")]
	InvalidSubscriptionId,
	/// Invalid request ID.
	#[error("Invalid request ID")]
	InvalidRequestId,
	/// Client received a notification with an unregistered method
	#[error("Unregistered notification method")]
	UnregisteredNotification(String),
	/// A request with the same request ID has already been registered.
	#[error("A request with the same request ID has already been registered")]
	DuplicateRequestId,
	/// Method was already registered.
	#[error("Method: {0} was already registered")]
	MethodAlreadyRegistered(String),
	/// Method with that name has not yet been registered.
	#[error("Method: {0} has not yet been registered")]
	MethodNotFound(String),
	/// Subscribe and unsubscribe method names are the same.
	#[error("Cannot use the same method name for subscribe and unsubscribe, used: {0}")]
	SubscriptionNameConflict(String),
	/// Request timeout
	#[error("Request timeout")]
	RequestTimeout,
	/// Configured max number of request slots exceeded.
	#[error("Configured max number of request slots exceeded")]
	MaxSlotsExceeded,
	/// Attempted to stop server that is already stopped.
	#[error("Attempted to stop server that is already stopped")]
	AlreadyStopped,
	/// List passed into access control based on HTTP header verification.
	#[error("Must set at least one allowed value for the {0} header")]
	EmptyAllowList(&'static str),
	/// Access control verification of HTTP headers failed.
	#[error("HTTP header: `{0}` value: `{1}` verification failed")]
	HttpHeaderRejected(&'static str, String),
	/// Custom error.
	#[error("Custom error: {0}")]
	Custom(String),
	/// Not implemented for HTTP clients.
	#[error("Not implemented")]
	HttpNotImplemented,
	/// Empty batch request.
	#[error("Empty batch request is not allowed")]
	EmptyBatchRequest,
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

impl From<Error> for ErrorObjectOwned {
	fn from(err: Error) -> Self {
		match err {
			Error::Call(CallError::Custom(err)) => err,
			Error::Call(CallError::InvalidParams(e)) => {
				ErrorObject::owned(INVALID_PARAMS_CODE, e.to_string(), None::<()>)
			}
			Error::Call(CallError::Failed(e)) => {
				ErrorObject::owned(CALL_EXECUTION_FAILED_CODE, e.to_string(), None::<()>)
			}
			_ => ErrorObject::owned(UNKNOWN_ERROR_CODE, err.to_string(), None::<()>),
		}
	}
}

/// Generic transport error.
#[derive(Debug, thiserror::Error)]
pub enum GenericTransportError {
	/// Request was too large.
	#[error("The request was too big")]
	TooLarge,
	/// Malformed request
	#[error("Malformed request")]
	Malformed,
	/// Concrete transport error.
	#[error("Transport error: {0}")]
	Inner(anyhow::Error),
}

impl From<std::io::Error> for Error {
	fn from(io_err: std::io::Error) -> Error {
		Error::Transport(io_err.into())
	}
}

#[cfg(feature = "soketto")]
impl From<soketto::handshake::Error> for Error {
	fn from(handshake_err: soketto::handshake::Error) -> Error {
		Error::Transport(handshake_err.into())
	}
}

#[cfg(feature = "soketto")]
impl From<soketto::connection::Error> for Error {
	fn from(conn_err: soketto::connection::Error) -> Error {
		Error::Transport(conn_err.into())
	}
}

#[cfg(feature = "hyper")]
impl From<hyper::Error> for Error {
	fn from(hyper_err: hyper::Error) -> Error {
		Error::Transport(hyper_err.into())
	}
}

/// The error returned by the subscription's method for the rpc server implementation.
///
/// It provides an abstraction to make the API more ergonomic while handling errors
/// that may occur during the subscription callback.
#[derive(Debug)]
pub enum SubscriptionCallbackError {
	/// Error cause is propagated by other code or connection related.
	None,
	/// Some error happened to be logged.
	Some(String),
}

// User defined error.
impl From<anyhow::Error> for SubscriptionCallbackError {
	fn from(e: anyhow::Error) -> Self {
		Self::Some(format!("Other: {e}"))
	}
}

// User defined error.
impl From<Box<dyn std::error::Error>> for SubscriptionCallbackError {
	fn from(e: Box<dyn std::error::Error>) -> Self {
		Self::Some(format!("Other: {e}"))
	}
}

impl From<CallError> for SubscriptionCallbackError {
	fn from(e: CallError) -> Self {
		Self::Some(e.to_string())
	}
}

impl From<SubscriptionAcceptRejectError> for SubscriptionCallbackError {
	fn from(_: SubscriptionAcceptRejectError) -> Self {
		Self::None
	}
}

impl From<serde_json::Error> for SubscriptionCallbackError {
	fn from(e: serde_json::Error) -> Self {
		Self::Some(format!("Failed to parse SubscriptionMessage::from_json: {e}"))
	}
}

#[cfg(feature = "server")]
impl From<crate::server::rpc_module::TrySendError> for SubscriptionCallbackError {
	fn from(e: crate::server::rpc_module::TrySendError) -> Self {
		Self::Some(format!("SubscriptionSink::try_send failed: {e}"))
	}
}

#[cfg(feature = "server")]
impl From<crate::server::rpc_module::DisconnectError> for SubscriptionCallbackError {
	fn from(e: crate::server::rpc_module::DisconnectError) -> Self {
		Self::Some(format!("SubscriptionSink::send failed: {e}"))
	}
}

#[cfg(feature = "server")]
impl From<crate::server::rpc_module::SendTimeoutError> for SubscriptionCallbackError {
	fn from(e: crate::server::rpc_module::SendTimeoutError) -> Self {
		Self::Some(format!("SubscriptionSink::send_timeout failed: {e}"))
	}
}

/// The error returned while accepting or rejecting a subscription.
#[derive(Debug, Copy, Clone)]
pub enum SubscriptionAcceptRejectError {
	/// The method was already called.
	AlreadyCalled,
	/// The remote peer closed the connection or called the unsubscribe method.
	RemotePeerAborted,
	/// The subscription response message was too large.
	MessageTooLarge,
}
