//! Traits for implementing request-making capabilities.

use crate::common;
use futures::prelude::*;
use std::{error, pin::Pin};

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
pub trait RawClient {
    /// Error that can happen during a request.
    type Error: error::Error;

    /// Sends out out a request. Returns a `Future` that finishes when the request has been
    /// successfully sent.
    fn send_request<'a>(
        &'a mut self,
        request: common::Request,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>;

    /// Returns a `Future` resolving when the server sent us something back.
    fn next_response<'a>(&'a mut self)
        -> Pin<Box<dyn Future<Output = Result<common::Response, Self::Error>> + Send + 'a>>;
}
