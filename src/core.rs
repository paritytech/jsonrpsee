// Copyright 2019 Parity Technologies (UK) Ltd.
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
//! - The [`TransportClient`] and [`TransportServer`] traits are implemented on structs that allow performing
//!   low-level communication with respectively a server or a client. These are the traits that
//!   you must implement if you are writing a custom transport (similar to HTTP, WebSockets,
//!   IPC, etc.).
//! - The [`RawClient`] and [`RawServer`] structs wrap around respectively a [`TransportClient`] or a
//!   [`TransportServer`] and allow correctly associating requests with responses and managing pub-sub
//!   subscriptions.
//!
//! In order to start a client or a server, first create a struct that implements respectively
//! [`TransportClient`] or [`TransportServer`], then wrap a [`RawClient`] or a [`RawServer`] around them.

pub use crate::core::client::raw::TransportClient;
pub use crate::core::client::{RawClient, RawClientError, RawClientEvent, RawClientRequestId};
pub use crate::core::local::local_transport;
pub use crate::core::server::raw::{TransportServer, TransportServerEvent};
pub use crate::core::server::{
    RawServer, RawServerEvent, RawServerRequestId, RawServerSubscriptionId,
};

pub mod client;
pub mod common;
pub mod local;
pub mod server;
