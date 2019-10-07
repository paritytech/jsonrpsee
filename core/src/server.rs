//! Listening for incoming JSON-RPC requests.
//!
//! A [`Server`](crate::server::Server) can be seen as a collection of requests and subscriptions.
//! Calling [`next_event`](crate::server::Server::next_event) returns a `Future` that returns
//! the next incoming request from a client.
//!
//! When a request arrives, can choose to:
//!
//! - Answer the request immediately.
//! - Turn the request into a subscription.
//! - Ignore this request and process it later. This can only be done for requests that have an ID,
//! and not for notifications.
//!
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

//! # About batches
//!
//! If a client sends [a batch](https://www.jsonrpc.org/specification#batch) of requests and/or
//! notification, the `Server` automatically splits each element of the batch. The batch is later
//! properly recomposed when the answer is sent back.
//!
//! # Example usage
//!
//! TODO: write example
//!

pub use self::core::{Server, ServerEvent, ServerRequest, ServerRequestId, ServerSubscriptionId};
pub use self::notification::Notification;
pub use self::params::{
    Iter as ParamsIter, ParamKey as ParamsKey, Params,
};
pub use self::typed_rp::TypedResponder;

pub mod raw;

mod batch;
mod batches;
mod core;
mod notification;
mod params;
mod tests;
mod typed_rp;
