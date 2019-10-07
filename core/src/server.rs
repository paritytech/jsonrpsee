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
