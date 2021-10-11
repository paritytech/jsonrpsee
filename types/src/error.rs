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

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
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

/// Error that occurs when a call failed.
#[derive(Debug, thiserror::Error)]
pub enum CallError {
	/// Invalid params in the call.
	#[error("Invalid params in the call: {0}")]
	InvalidParams(#[source] anyhow::Error),
	/// The call failed (let jsonrpsee assign default error code and error message).
	#[error("RPC Call failed: {0}")]
	Failed(#[from] anyhow::Error),
	/// Custom error with specific JSON-RPC error code, message and data.
	#[error("RPC Call failed: code: {code}, message: {message}, data: {data:?}")]
	Custom {
		/// JSON-RPC error code
		code: i32,
		/// Short description of the error.
		message: String,
		/// A primitive or structured value that contains additional information about the error.
		data: Option<Box<RawValue>>,
	},
}

impl CallError {
	/// Create `CallError` from a generic error.
	pub fn from_std_error<E>(err: E) -> Self
	where
		E: std::error::Error + Send + Sync + 'static,
	{
		CallError::Failed(err.into())
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
	#[error("Server call failed: {0}")]
	Call(#[from] CallError),
	/// Networking error or error on the low-level protocol layer.
	#[error("Networking or low-level protocol error: {0}")]
	Transport(#[source] anyhow::Error),
	/// JSON-RPC request error.
	#[error("JSON-RPC request error: {0:?}")]
	Request(String),
	/// Frontend/backend channel error.
	#[error("Frontend/backend channel error: {0}")]
	Internal(#[from] futures_channel::mpsc::SendError),
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
	/// Subscription got closed.
	#[error("Subscription closed: {0:?}")]
	SubscriptionClosed(SubscriptionClosedError),
	/// Request timeout
	#[error("Request timeout")]
	RequestTimeout,
	/// Configured max number of request slots exceeded.
	#[error("Configured max number of request slots exceeded")]
	MaxSlotsExceeded,
	/// Attempted to stop server that is already stopped.
	#[error("Attempted to stop server that is already stopped")]
	AlreadyStopped,
	/// List passed into `set_allowed_origins` was empty
	#[error("Must set at least one allowed value for the {0} header")]
	EmptyAllowList(&'static str),
	/// Failed to execute a method because a resource was already at capacity
	#[error("Resource at capacity: {0}")]
	ResourceAtCapacity(&'static str),
	/// Failed to register a resource due to a name conflict
	#[error("Resource name already taken: {0}")]
	ResourceNameAlreadyTaken(&'static str),
	/// Failed to initialize resources for a method at startup
	#[error("Resource name `{0}` not found for method `{1}`")]
	ResourceNameNotFoundForMethod(&'static str, &'static str),
	/// Trying to claim resources for a method execution, but the method resources have not been initialized
	#[error("Method `{0}` has uninitialized resources")]
	UninitializedMethod(Box<str>),
	/// Failed to register a resource due to a maximum number of resources already registered
	#[error("Maximum number of resources reached")]
	MaxResourcesReached,
	/// Custom error.
	#[error("Custom error: {0}")]
	Custom(String),
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

/// Error type with a special `subscription_closed` field to detect that
/// a subscription has been closed to distinguish valid items produced
/// by the server on the subscription stream from an error.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct SubscriptionClosedError {
	subscription_closed: String,
}

impl From<String> for SubscriptionClosedError {
	fn from(msg: String) -> Self {
		Self { subscription_closed: msg }
	}
}

/// Generic transport error.
#[derive(Debug, thiserror::Error)]
pub enum GenericTransportError<T: std::error::Error + Send + Sync> {
	/// Request was too large.
	#[error("The request was too big")]
	TooLarge,
	/// Malformed request
	#[error("Malformed request")]
	Malformed,
	/// Concrete transport error.
	#[error("Transport error: {0}")]
	Inner(T),
}

impl From<std::io::Error> for Error {
	fn from(io_err: std::io::Error) -> Error {
		Error::Transport(io_err.into())
	}
}

impl From<soketto::handshake::Error> for Error {
	fn from(handshake_err: soketto::handshake::Error) -> Error {
		Error::Transport(handshake_err.into())
	}
}

impl From<soketto::connection::Error> for Error {
	fn from(conn_err: soketto::connection::Error) -> Error {
		Error::Transport(conn_err.into())
	}
}

impl From<hyper::Error> for Error {
	fn from(hyper_err: hyper::Error) -> Error {
		Error::Transport(hyper_err.into())
	}
}
