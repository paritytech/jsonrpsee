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

//! Lower-level API for servers that receive JSON payloads.
//!
//! A "raw server" is a server that produces JSON payloads and can send back JSON payloads in
//! return. This module isn't concerned with concepts such as a "batch", "successes" and "failures".
//! All it does is accept and send back JSON data.
//!
//! ## Example usage
//!
//! ```
//! use jsonrpsee::common::{Error, Request, Response, Version};
//! use jsonrpsee::transport::server::{TransportServer, TransportServerEvent};
//!
//! async fn run_server(server: &mut impl TransportServer) {
//!     // Note that this implementation is a bit naive, as no request will be accepted by the
//!     // server while `request_to_response` is running. This is fine as long as building the
//!     // response is instantaneous (which is the case in this exampe), but probably isn't for
//!     // actual real-world usages.
//!     loop {
//!         match server.next_request().await {
//!             TransportServerEvent::Closed(_) => {},
//!             TransportServerEvent::Request { id, request } => {
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
pub use self::traits::{TransportServer, TransportServerEvent};

mod join;
mod traits;
