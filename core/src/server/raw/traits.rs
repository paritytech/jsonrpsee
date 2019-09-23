use crate::common;
use futures::prelude::*;
use std::{error, hash::Hash, io, pin::Pin};

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
pub trait RawServer {
    /// Identifier for a request in the context of this server.
    type RequestId: Clone + PartialEq + Eq + Hash + Send + Sync;

    /// Returns the next event that the raw server wants to notify us.
    fn next_request<'a>(&'a mut self)
        -> Pin<Box<dyn Future<Output = RawServerEvent<Self::RequestId>> + Send + 'a>>;

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
    /// support that operation (see [`supports_resuming`](RawServer::supports_resuming)).
    ///
    /// > **Note**: This might not be supported by the underlying implementation. For example, a
    /// >           WebSockets server can support that, but not an HTTP server.
    ///
    /// > **Note**: Just like for [`finish`](RawServer::finish), the returned `Future` shouldn't
    /// >           take too long to complete.
    fn send<'a>(
        &'a mut self,
        request_id: &'a Self::RequestId,
        response: &'a common::Response,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>>;
}

/// Event that the [`RawServer`] can generate.
#[derive(Debug, PartialEq)]
pub enum RawServerEvent<T> {
    /// A new request has arrived on the wire.
    ///
    /// This generates a new "request object" within the state of the [`RawServer`] that is
    /// identified through the returned `id`. You can then use the other methods of the
    /// [`RawServer`] trait in order to manipulate that request.
    Request {
        /// Identifier of the request within the state of the [`RawServer`].
        id: T,
        /// Body of the request.
        request: common::Request,
    },

    /// A request has been cancelled, most likely because the client has closed the connection.
    ///
    /// The corresponding request is no longer valid to manipulate.
    Closed(T),

    /// The server has been closed and will not produce any more request.
    // TODO: define the exact semantics of that; can the implementation panic afterwards? is it
    // even a good idea to have an event?
    ServerClosed,
}
