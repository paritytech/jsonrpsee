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

//! Shared types in `jsonrpsee` for clients, servers and utilities.

#![deny(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]

extern crate alloc;

/// Ten megabytes.
pub const TEN_MB_SIZE_BYTES: u32 = 10 * 1024 * 1024;

/// JSON-RPC 2.0 specification related types v2.
pub mod v2;

/// Shared error type.
pub mod error;

/// Shared client types.
mod client;

/// Traits
pub mod traits;

pub use async_trait::async_trait;
pub use beef::Cow;
pub use client::*;
pub use error::{CallError, Error};
pub use serde::{de::DeserializeOwned, Serialize};
pub use serde_json::{
	to_value as to_json_value, value::to_raw_value as to_json_raw_value, value::RawValue as JsonRawValue,
	Value as JsonValue,
};

/// Re-exports for proc-macro library to not require any additional
/// dependencies to be explicitly added on the client side.
#[doc(hidden)]
pub mod __reexports {
	pub use async_trait::async_trait;
	pub use serde;
	pub use serde_json;
}

/// JSON-RPC result.
pub type JsonRpcResult<T> = Result<T, Error>;
