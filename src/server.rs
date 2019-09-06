//!
//!
//! A [`Server`](crate::server::Server) can be seen as a collection of requests and subscriptions.
//! Calling [`next_request`](crate::server::Server::next_request) returns a `Future` that returns
//! the next incoming request from a client.
//!
//! When a request arrives, can choose to:
//! 
//! - Answer the request immediately.
//! - Turn the request into a subscription.
//! - Ignore this request and process it later. This can only be done for requests that have an ID,
//! and not for notifications.
//!
//! ## About batches
//!
//! If a client sends [a batch](https://www.jsonrpc.org/specification#batch) of requests and/or
//! notification, the `Server` automatically splits each element of the batch. The batch is later
//! properly recomposed when the answer is sent back.
//!
//! ## Example usage
//!
//! TODO: write example
//!

pub use self::params::{ServerRequestParams, Iter as ServerRequestParamsIter, ParamKey as ServerRequestParamsKey};
pub use self::run::run;
pub use self::server::{Server, ServerRq};
pub use self::wrappers::http;

pub mod raw;

mod params;
mod run;
mod server;
mod wrappers;

