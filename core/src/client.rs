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

//! Performing JSON-RPC requests.
//!
//! The [`Client`] struct wraps around a [`RawClient`] and handles the higher-level JSON-RPC logic
//! on top of it. In order to build a [`Client`], you need to pass to it an implementation of
//! [`RawClient`]. There exists shortcut methods that directly create a [`Client`] on top of a
//! specific [`RawClient`] implementations.
//!
//! Once created, a [`Client`] can be used to send out notifications, requests, and subscription
//! requests to the server. Request identifiers are automatically assigned by the client.
//!
//! # Notifications
//!
//! **Notifications** are one-shot messages to the server that don't expect any response. They can
//! be sent using the [`send_notification`](Client::send_notification) method.
//!
//! # Requests
//!
//! **Requests** are messages that expect an answer. A request can be sent using the
//! [`start_request`](Client::start_request) method. This method returns a [`ClientRequestId`] that
//! is used to identify this request within the internals of the [`Client`]. You can then call
//! [`request_by_id`](Client::request_by_id) to wait for a response from a server about a specific
//! request. You are however encouraged to use [`next_event`](Client::next_event) instead, which
//! produces a [`ClientEvent`] indicating you what the server did.
//!
//! > **Note**: At the time of writing, the [`Client`] never uses batches and only sends out
//! >           individual requests.
//!
//! # Subscriptions
//!
//! **Subscriptions** are similar to requests, except that we stay connected to the server
//! after the request ended, and expect notifications back from it. The [`Client`] will notify
//! you about subscriptions through the [`next_event`](Client::next_event) method and the
//! [`ClientEvent`] enum.
//!
//! > **Note**: The [`request_by_id`](Client::request_by_id) method will buffer up incoming
//! >           notifications up to a certain limit. Once this limit is reached, new notifications
//! >           will be silently discarded. This behaviour exists to prevent DoS attacks from
//! >           the server. If you want to be certain to not miss any notification, please only
//! >           use the [`next_event`](Client::next_event) method.
//!

pub use self::core::*;
pub use self::raw::RawClient;

pub mod raw;

mod core;
