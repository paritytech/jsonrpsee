//! Lower-level API for servers that receive JSON payloads.
//!
//! A "raw server" is a server that produces JSON payloads and can send back JSON payloads in
//! return. This module isn't concerned with concepts such as a "batch", "successes" and "failures".
//! All it does is accept and send back JSON data.
//!
//! ## Example usage
//!
//! ```
//! use jsonrpsee_core::server::raw::{RawServer, RawServerEvent};
//! use jsonrpsee_core::common::{Error, Request, Response, Version};
//!
//! async fn run_server(server: &mut impl RawServer) {
//!     // Note that this implementation is a bit naive, as no request will be accepted by the
//!     // server while `request_to_response` is running. This is fine as long as building the
//!     // response is instantaneous (which is the case in this exampe), but probably isn't for
//!     // actual real-world usages.
//!     loop {
//!         match server.next_request().await {
//!             RawServerEvent::ServerClosed => break,
//!             RawServerEvent::Closed(_) => {},
//!             RawServerEvent::Request { id, request } => {
//!                 let response = request_to_response(&request).await;
//!                 let _ = server.finish(&id, Some(&response)).await;
//!             },
//!         }
//!     }
//! }
//!
//! async fn request_to_response(rq: &Request) -> Response {
//!     // ... to be implemented ...
//!     Response::from(Error::method_not_found(), Version::V2)
//! }
//! ```
//!

pub use self::join::{join, Join, JoinRequestId};
pub use self::traits::{RawServer, RawServerEvent};

mod join;
mod traits;
