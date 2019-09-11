//! Traits for implementing request-making capabilities.

use crate::common;
use futures::prelude::*;
use std::{error, pin::Pin};

/// Objects that can act as clients.
pub trait RawClient {
    /// Error that can happen during a request.
    type Error: error::Error;

    /// Starts a request. Returns a `Future` that finishes when the request succeeds or fails.
    fn request<'a>(
        &'a mut self,
        request: common::Request,
    ) -> Pin<Box<dyn Future<Output = Result<common::Response, Self::Error>> + Send + 'a>>;
}
