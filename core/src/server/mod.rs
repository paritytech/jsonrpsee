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

/// Error types.
mod error;
/// Helpers.
pub mod helpers;
/// Host filtering.
mod host_filtering;
/// JSON-RPC "modules" group sets of methods that belong together and handles method/subscription registration.
mod rpc_module;
/// Subscription related types.
mod subscription;

pub use error::*;
pub use helpers::{BatchResponseBuilder, BoundedWriter, MethodResponse, MethodSink, MethodSinkPermit};
pub use host_filtering::*;
pub use rpc_module::*;
pub use subscription::*;

use jsonrpsee_types::{ErrorObjectOwned, PartialResponse};
use serde::{Serialize, Serializer};

/// Something that can be converted into a JSON-RPC error object.
pub trait IntoErrorObject {
	/// Something that can be converted into a JSON-RPC error object.
	fn into_error_object(self) -> ErrorObjectOwned;
}

impl IntoErrorObject for crate::Error {
	fn into_error_object(self) -> ErrorObjectOwned {
		self.into()
	}
}

/// Something that can be converted into a JSON-RPC method call response.
pub trait IntoResponse {
	type Output: serde::Serialize;

	/// Something that can be converted into a JSON-RPC method call response.
	fn into_response(self) -> PartialResponse<Self::Output>;
}

impl<T: serde::Serialize, E: IntoErrorObject> IntoResponse for Result<T, E> {
	type Output = T;

	fn into_response(self) -> PartialResponse<T> {
		match self {
			Ok(val) => PartialResponse::Result(val),
			Err(e) => PartialResponse::Error(e.into_error_object()),
		}
	}
}

macro_rules! impl_into_response {
	($($n:ty),*) => {
		$(
			impl IntoResponse for $n {
				type Output = $n;

				fn into_response(self) -> PartialResponse<Self::Output> {
					PartialResponse::Result(self)
				}
			}
		)+
	}
}

impl_into_response!(
	u8,
	u16,
	u32,
	u64,
	u128,
	usize,
	i8,
	i16,
	i32,
	i64,
	i128,
	isize,
	String,
	&'static str,
	bool,
	serde_json::Value,
	()
);
