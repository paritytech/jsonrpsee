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

//! JSON-RPC specific types.

#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]

extern crate alloc;

/// JSON-RPC params related types.
pub mod params;

/// JSON-RPC request object related types
pub mod request;

/// JSON-RPC response object related types.
pub mod response;

/// JSON-RPC response error object related types.
pub mod error;

pub use error::{ErrorCode, ErrorObject, ErrorObjectOwned};
pub use params::{Id, InvalidRequestId, Params, ParamsSequence, SubscriptionId, TwoPointZero};
pub use request::{InvalidRequest, Notification, NotificationSer, Request, RequestSer};
pub use response::{Response, ResponsePayload, SubscriptionPayload, SubscriptionResponse, Success as ResponseSuccess};

/// Helper to deserialize calls with extensions.
pub mod deserialize_with_ext {

	/// Request.
	pub mod request {
		use crate::Request;

		/// Helper to deserialize a request with extensions.
		pub fn from_slice(data: &[u8], extensions: http::Extensions) -> Result<Request, serde_json::Error> {
			let mut req: Request = serde_json::from_slice(data)?;
			*req.extensions_mut() = extensions;
			Ok(req)
		}

		/// Helper to deserialize a request with extensions.
		pub fn from_str(data: &str, extensions: http::Extensions) -> Result<Request, serde_json::Error> {
			let mut req: Request = serde_json::from_str(data)?;
			*req.extensions_mut() = extensions;
			Ok(req)
		}
	}

	/// Notification.
	pub mod notification {
		use crate::Notification;

		/// Helper to deserialize a request with extensions.
		pub fn from_slice<'a, T>(
			data: &'a [u8],
			extensions: http::Extensions,
		) -> Result<Notification<'a, T>, serde_json::Error>
		where
			T: serde::Deserialize<'a>,
		{
			let mut notif: Notification<T> = serde_json::from_slice(data)?;
			*notif.extensions_mut() = extensions;
			Ok(notif)
		}

		/// Helper to deserialize a request with extensions.
		pub fn from_str<'a, T>(
			data: &'a str,
			extensions: http::Extensions,
		) -> Result<Notification<'a, T>, serde_json::Error>
		where
			T: serde::Deserialize<'a>,
		{
			let mut notif: Notification<T> = serde_json::from_str(data)?;
			*notif.extensions_mut() = extensions;
			Ok(notif)
		}
	}

	/// Response
	pub mod response {
		use crate::Response;

		/// Helper to deserialize a response with extensions.
		pub fn from_slice<'a, T>(
			data: &'a [u8],
			extensions: http::Extensions,
		) -> Result<Response<'a, T>, serde_json::Error>
		where
			T: serde::Deserialize<'a> + Clone,
		{
			let mut res: Response<T> = serde_json::from_slice(data)?;
			*res.extensions_mut() = extensions;
			Ok(res)
		}

		/// Helper to deserialize a response with extensions.
		pub fn from_str<'a, T>(
			data: &'a str,
			extensions: http::Extensions,
		) -> Result<Response<'a, T>, serde_json::Error>
		where
			T: serde::Deserialize<'a> + Clone,
		{
			let mut res: Response<T> = serde_json::from_str(data)?;
			*res.extensions_mut() = extensions;
			Ok(res)
		}
	}
}
