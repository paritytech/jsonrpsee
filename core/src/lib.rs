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

//! Shared utilities for `jsonrpsee`.

#![warn(missing_docs, missing_debug_implementations, unreachable_pub)]

use jsonrpsee_types::SubscriptionId;
use rand::{distributions::Alphanumeric, Rng};
use traits::IdProvider;

/// Error type.
pub mod error;

/// Traits
pub mod traits;

/// Middleware trait and implementation.
pub mod middleware;

/// Shared hyper helpers.
#[cfg(feature = "http-helpers")]
pub mod http_helpers;

/// Shared code for JSON-RPC servers.
#[cfg(feature = "server")]
pub mod server;

/// Shared code for JSON-RPC clients.
#[cfg(feature = "client")]
pub mod client;

pub use async_trait::async_trait;
pub use error::Error;

/// JSON-RPC result.
pub type RpcResult<T> = std::result::Result<T, Error>;

/// Re-exports for proc-macro library to not require any additional
/// dependencies to be explicitly added on the client side.
#[doc(hidden)]
pub mod __reexports {
	pub use async_trait::async_trait;
	pub use serde;
	pub use serde_json;
}

pub use beef::Cow;
pub use serde::{de::DeserializeOwned, Serialize};
pub use serde_json::{
	to_value as to_json_value, value::to_raw_value as to_json_raw_value, value::RawValue as JsonRawValue,
	Value as JsonValue,
};

/// Ten megabytes.
pub const TEN_MB_SIZE_BYTES: u32 = 10 * 1024 * 1024;

/// Generates random integers as subscription ID.
#[derive(Debug)]
pub struct RandomIntegerIdProvider;

impl IdProvider for RandomIntegerIdProvider {
	fn next_id(&self) -> SubscriptionId<'static> {
		const JS_NUM_MASK: u64 = !0 >> 11;
		(rand::random::<u64>() & JS_NUM_MASK).into()
	}
}

/// Generates random strings of length `len` as subscription ID.
#[derive(Debug)]
pub struct RandomStringIdProvider {
	len: usize,
}

impl RandomStringIdProvider {
	/// Create a new random string provider.
	pub fn new(len: usize) -> Self {
		Self { len }
	}
}

impl IdProvider for RandomStringIdProvider {
	fn next_id(&self) -> SubscriptionId<'static> {
		let mut rng = rand::thread_rng();
		(&mut rng).sample_iter(Alphanumeric).take(self.len).map(char::from).collect::<String>().into()
	}
}

/// No-op implementation to be used for servers that doesn't support subscriptions.
#[derive(Debug)]
pub struct NoopIdProvider;

impl IdProvider for NoopIdProvider {
	fn next_id(&self) -> SubscriptionId<'static> {
		0.into()
	}
}
