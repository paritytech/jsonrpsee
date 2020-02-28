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

use crate::common;

use alloc::boxed::Box;
use core::{fmt, pin::Pin};
use futures::prelude::*;

/// Objects that can act as clients.
///
/// > **Note**: Implementations of this trait are allowed (and encouraged, for example for
/// >           HTTP 1.x) to open multiple simultaneous connections to the same server. However,
/// >           since this trait doesn't expose the concept of a connection, and since
/// >           implementations aren't expected to associated requests with responses, we have no
/// >           way to enforce that the response to a request arrived on the same connection as the
/// >           one where the request has been sent. In practice, though, this shouldn't be a
/// >           problem.
///
pub trait TransportClient {
    /// Error that can happen during a request.
    type Error: fmt::Display;

    /// Sends out out a request. Returns a `Future` that finishes when the request has been
    /// successfully sent.
    fn send_request<'a>(
        &'a mut self,
        request: common::Request,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>;

    /// Returns a `Future` resolving when the server sent us something back.
    fn next_response<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = Result<common::Response, Self::Error>> + Send + 'a>>;
}
