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

use self::rpc_module::SubscriptionMessage;

/// Represents what action that will sent when a subscription callback returns.
#[derive(Debug)]
pub enum SubscriptionCloseResponse {
	/// No further message will be sent.
	None,
	/// Send a ordinary subscription response.
	Some(SubscriptionMessage),
	/// Send a subscription error response.
	Err(SubscriptionMessage),
}

/// Convert something into a response.
pub trait IntoSubscriptionResponse {
	/// Convert something into a response.
	fn into_response(self) -> SubscriptionCloseResponse;
}

impl<T> IntoSubscriptionResponse for Option<T>
where
	T: serde::Serialize,
{
	fn into_response(self) -> SubscriptionCloseResponse {
		match self {
			Some(msg) => match SubscriptionMessage::from_json(&msg) {
				Ok(m) => SubscriptionCloseResponse::Some(m),
				Err(e) => SubscriptionCloseResponse::Err(e.to_string().into()),
			},
			None => SubscriptionCloseResponse::None,
		}
	}
}

impl<T, E> IntoSubscriptionResponse for Result<T, E>
where
	T: serde::Serialize,
	E: ToString,
{
	fn into_response(self) -> SubscriptionCloseResponse {
		match self {
			Ok(msg) => match SubscriptionMessage::from_json(&msg) {
				Ok(m) => SubscriptionCloseResponse::Some(m),
				Err(e) => SubscriptionCloseResponse::Err(e.to_string().into()),
			},
			Err(e) => SubscriptionCloseResponse::Err(e.to_string().into()),
		}
	}
}
