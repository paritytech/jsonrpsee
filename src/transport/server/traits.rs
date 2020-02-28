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
use core::{hash::Hash, pin::Pin};
use futures::prelude::*;

/// Reference to a server that can produce JSON payloads for us to answer.
///
/// This is a low-level trait implemented directly for example on HTTP or WebSockets servers.
///
/// ## Usage
///
/// A "raw server" can be seen as a state machine holding a collection of requests. Each request
/// has a corresponding unique identifier.
///
/// Calling `next_request` returns a `Future` that resolves when the server receives a new
/// incoming request from a connection. The `Future` updates the internal state of the server
/// to insert the new request, and returns, in addition to the body of the request, an identifier
/// that represents that newly-received request in the context of the server.
///
/// ## What to do in case of an error?
///
/// In order to avoid introducing ambiguities in the API, this trait has no way to notify the user
/// of a problem happening on the server side (e.g. the TCP listener being closed).
///
/// Instead, implementations are encouraged to try to maintain the server alive as much as
/// possible. If an unrecoverable error happens, implementations should become permanently idle.
///
pub trait TransportServer {
    /// Identifier for a request in the context of this server.
    type RequestId: Clone + PartialEq + Eq + Hash + Send + Sync;

    /// Returns the next event that the raw server wants to notify us.
    fn next_request<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = TransportServerEvent<Self::RequestId>> + Send + 'a>>;

    /// Sends back a response and destroys the request.
    ///
    /// You can pass `None` in order to destroy the request object without sending back anything.
    ///
    /// The implementation blindly sends back the response and doesn't check whether there is any
    /// correspondance with the request in terms of logic. For example, `respond` will accept
    /// sending back a batch of six responses even if the original request was a single
    /// notification.
    ///
    /// > **Note**: While this method returns a `Future` that must be driven to completion,
    /// >           implementations must be aware that the entire requests processing logic is
    /// >           blocked for as long as this `Future` is pending. As an example, you shouldn't
    /// >           use this `Future` to send back a TCP message, because if the remote is
    /// >           unresponsive and the buffers full, the `Future` would then wait for a long time.
    ///
    fn finish<'a>(
        &'a mut self,
        request_id: &'a Self::RequestId,
        response: Option<&'a common::Response>,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>>;

    /// Returns true if this implementation supports sending back data on this request without
    /// closing it.
    ///
    /// Returns an error if the request id is invalid.
    fn supports_resuming(&self, request_id: &Self::RequestId) -> Result<bool, ()>;

    /// Sends back some data on the request and keeps the request alive.
    ///
    /// You can continue sending data on that same request later.
    ///
    /// Returns an error if the request identifier is incorrect, or if the implementation doesn't
    /// support that operation (see [`supports_resuming`](TransportServer::supports_resuming)).
    ///
    /// > **Note**: This might not be supported by the underlying implementation. For example, a
    /// >           WebSockets server can support that, but not an HTTP server.
    ///
    /// > **Note**: Just like for [`finish`](TransportServer::finish), the returned `Future` shouldn't
    /// >           take too long to complete.
    fn send<'a>(
        &'a mut self,
        request_id: &'a Self::RequestId,
        response: &'a common::Response,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>>;
}

/// Event that the [`TransportServer`] can generate.
#[derive(Debug, PartialEq)]
pub enum TransportServerEvent<T> {
    /// A new request has arrived on the wire.
    ///
    /// This generates a new "request object" within the state of the [`TransportServer`] that is
    /// identified through the returned `id`. You can then use the other methods of the
    /// [`TransportServer`] trait in order to manipulate that request.
    Request {
        /// Identifier of the request within the state of the [`TransportServer`].
        id: T,
        /// Body of the request.
        request: common::Request,
    },

    /// A request has been cancelled, most likely because the client has closed the connection.
    ///
    /// The corresponding request is no longer valid to manipulate.
    Closed(T),
}
