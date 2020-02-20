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

//! Implementation of [`TransportClient`](core::client::raw::TransportClient) and
//! [`TransportServer`](core::server::raw::TransportServer) for HTTP.

use crate::core::client::RawClient;
use crate::core::server::{raw::TransportServer, RawServer};

use crate::server_utils::access_control::AccessControl;
use std::{error, net::SocketAddr};

pub use crate::http::client::{HttpTransportClient, RequestError};
pub use crate::http::server::HttpTransportServer;

pub use crate::server_utils::access_control;

/// Type alias for a [`RawClient`](core::client::RawClient) that operates on HTTP.
pub type HttpRawClient = RawClient<HttpTransportClient>;
/// Type alias for a [`RawServer`](core::server::RawServer) that operates on HTTP.
pub type HttpRawServer =
    RawServer<HttpTransportServer, <HttpTransportServer as TransportServer>::RequestId>;

mod client;
mod server;

/// Starts a [`RawServer`](../RawServer) object that serves HTTP.
pub async fn http_raw_server(
    addr: &SocketAddr,
) -> Result<HttpRawServer, Box<dyn error::Error + Send + Sync>> {
    Ok(RawServer::new(HttpTransportServer::bind(addr).await?))
}

/// Starts a [`RawServer`](../RawServer) object that serves HTTP with a whitlist access control.
pub async fn http_raw_server_with_acl(
    addr: &SocketAddr,
    access_control: AccessControl,
) -> Result<HttpRawServer, Box<dyn error::Error + Send + Sync>> {
    Ok(RawServer::new(
        HttpTransportServer::bind_with_acl(addr, access_control).await?,
    ))
}

/// Returns an object that lets you perform JSON-RPC queries towards the given HTTP server.
pub fn http_raw_client(addr: &str) -> HttpRawClient {
    RawClient::new(HttpTransportClient::new(addr))
}
