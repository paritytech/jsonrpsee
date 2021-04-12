//! Shared utilities for `jsonrpsee`.

#![warn(missing_docs)]

#[cfg(all(feature = "hyper13", feature = "hyper14"))]
compile_error!("feature `hyper13` and `hyper14` are mutably exclusive");

#[cfg(all(feature = "hyper13", not(feature = "hyper14")))]
extern crate hyper13 as hyper;

#[cfg(all(feature = "hyper14", not(feature = "hyper13")))]
extern crate hyper14 as hyper;

/// Shared hyper helpers.
#[cfg(any(feature = "hyper13", feature = "hyper14"))]
pub mod hyper_helpers;

/// Shared code for JSON-RPC servers.
#[cfg(feature = "server")]
pub mod server;
