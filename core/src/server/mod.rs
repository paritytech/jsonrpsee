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

//! Shared modules for the JSON-RPC servers.

/// Helpers.
pub mod helpers;
/// Host filtering.
pub mod host_filtering;
/// JSON-RPC "modules" group sets of methods that belong together and handles method/subscription registration.
pub mod rpc_module;

use self::rpc_module::DisconnectError;
use crate::error::SubscriptionAcceptRejectError;
use jsonrpsee_types::error::CallError;
use rpc_module::SubscriptionMessage;

type SubscriptionCloseResponse = Option<SubscriptionMessage>;

/// Extension trait that converts errors into an optional error message.
///
/// Internally it converts an `Result<T, Error>` into `Result<T, Option<Error>`
/// which forces users to convert `Error` to `Option<SubscriptionMessage>`
/// where `Some(msg)` represents that the message is sent as subscription notification error
/// and `None` doesn't do anything.
///
/// This is implemented for the types in jsonrpsee where the behavior is not application dependent.
/// For other types you have to implement this trait or deal with it manually.
pub trait MapSubscriptionError<T> {
	/// Convert an error to an optional subscription error.
	fn map_sub_err(self) -> Result<T, SubscriptionCloseResponse>;
}

impl<T> MapSubscriptionError<T> for Result<T, DisconnectError> {
	fn map_sub_err(self) -> Result<T, SubscriptionCloseResponse> {
		self.map_err(|_| None)
	}
}

impl<T> MapSubscriptionError<T> for Result<T, SubscriptionAcceptRejectError> {
	fn map_sub_err(self) -> Result<T, SubscriptionCloseResponse> {
		self.map_err(|_| None)
	}
}

impl<T> MapSubscriptionError<T> for Result<T, serde_json::Error> {
	fn map_sub_err(self) -> Result<T, SubscriptionCloseResponse> {
		self.map_err(|e| Some(e.to_string().as_str().into()))
	}
}

impl<T> MapSubscriptionError<T> for Result<T, CallError> {
	fn map_sub_err(self) -> Result<T, SubscriptionCloseResponse> {
		self.map_err(|_| None)
	}
}
