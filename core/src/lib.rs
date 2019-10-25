//! Core traits and structs of the `jsonrpsee` library.
//!
//! > **Note**: This document mostly covers the core internal concepts of the `jsonrpsee` library.
//! >           See instead the documentation of `jsonrpsee` for how to use the library if you
//! >           just want to use it.
//!
//! jsonrpsee-core has five main concepts:
//!
//! - The [`common`] module contains all the primitive types of the JSON-RPC protocol, and
//!   utilities to convert between them and JSON.
//! - The [`RawClient`] and [`RawServer`] traits are implemented on structs that allow performing
//!   low-level communication with respectively a server or a client. These are the traits that
//!   you must implement if you are writing a custom transport (similar to HTTP, WebSockets,
//!   IPC, etc.).
//! - The [`Client`] and [`Server`] structs wrap around respectively a [`RawClient`] or a
//!   [`RawServer`] and allow correctly associating requests with responses and managing pub-sub
//!   subscriptions.
//!
//! In order to start a client or a server, first create a struct that implements respectively
//! [`RawClient`] or [`RawServer`], then wrap a [`Client`] or a [`Server`] around them.

#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

pub use crate::client::raw::RawClient;
pub use crate::client::{Client, ClientEvent, ClientRequestId};
pub use crate::local::local_raw;
pub use crate::server::raw::{RawServer, RawServerEvent};
pub use crate::server::{Server, ServerEvent, ServerRequestId, ServerSubscriptionId};

pub mod client;
pub mod common;
pub mod local;
pub mod server;
