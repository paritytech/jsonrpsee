//! Shared types in `jsonrpsee` for clients, servers and utilities.

#![deny(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

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
pub use serde_json::{to_value as to_json_value, value::RawValue as JsonRawValue, Map as JsonMap, Value as JsonValue};
