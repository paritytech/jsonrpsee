//! Shared types in `jsonrpsee` for clients, servers and utilities.

#![deny(unsafe_code)]
#![warn(missing_docs, missing_debug_implementations)]

extern crate alloc;

#[doc(hidden)]
pub use v2::params::MaybeOptionalParams;

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

pub use beef::Cow;
pub use client::*;
pub use error::Error;
pub use serde::{de::DeserializeOwned, Serialize};
pub use serde_json::{to_value as to_json_value, value::RawValue as JsonRawValue, Value as JsonValue};
