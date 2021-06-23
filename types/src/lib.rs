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

/// Macro for mark the type to not implement `MaybeOptionalParams`
/// Ideally, this should only be implemented for `Option<T>`
/// but mutually exclusive trait bounds is not a thing.
#[macro_export]
macro_rules! impl_param_not_optional {
	($ty:ident) => {
		impl $crate::MaybeOptionalParams for $ty {
			fn default() -> Option<$ty> {
				None
			}
		}
	};
	($ty:ident < $( $N:ident),* >) => {
		impl<$( $N ),*> $crate::MaybeOptionalParams for $ty<$( $N ),*> {
			fn default() -> Option<$ty<$( $N ),*>> {
				None
			}
		}
	};
}

use alloc::collections::*;

impl_param_not_optional!(String);
impl_param_not_optional!(Vec<T>);
impl_param_not_optional!(BTreeMap<K, V>);
impl_param_not_optional!(bool);
impl_param_not_optional!(u8);
impl_param_not_optional!(u16);
impl_param_not_optional!(u32);
impl_param_not_optional!(u64);
impl_param_not_optional!(JsonValue);
