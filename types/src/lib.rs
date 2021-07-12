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
