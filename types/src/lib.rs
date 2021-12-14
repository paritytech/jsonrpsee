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

/// Error type.
pub mod error;

/// Client types.
mod client;

/// JSON-RPC params related types.
pub mod params;

/// JSON-RPC request object related types
pub mod request;

/// JSON-RPC response object related types.
pub mod response;

// pub use async_trait::async_trait;
pub use beef::Cow;
pub use client::*;
pub use error::{CallError, Error, RpcError};
pub use params::{Id, Params, ParamsSequence, ParamsSer, SubscriptionId, TwoPointZero};
pub use serde::{de::DeserializeOwned, Serialize};
pub use serde_json::{
	to_value as to_json_value, value::to_raw_value as to_json_raw_value, value::RawValue as JsonRawValue,
	Value as JsonValue,
};

/// JSON-RPC result.
pub type RpcResult<T> = std::result::Result<T, Error>;

pub use request::{InvalidRequest, Notification, NotificationSer, Request, RequestSer};
pub use response::{Response, SubscriptionPayload, SubscriptionResponse};

/// Empty `RpcParams` type;
pub type EmptyParams = Vec<()>;
