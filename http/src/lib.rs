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

//! Implementation of [`TransportClient`](jsonrpsee_core::client::raw::TransportClient) and
//! [`TransportServer`](jsonrpsee_core::server::raw::TransportServer) for HTTP.

#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

use jsonrpsee_core::client::Client;
use jsonrpsee_core::server::{raw::TransportServer, Server};
use jsonrpsee_server_utils as server_utils;
use server_utils::access_control::AccessControl;
use std::{error, net::SocketAddr};

pub use crate::client::{HttpTransportClient, RequestError};
pub use crate::server::HttpTransportServer;

pub use server_utils::access_control;

/// Type alias for a [`Client`](jsonrpsee_core::client::Client) that operates on HTTP.
pub type HttpClient = Client<HttpTransportClient>;
/// Type alias for a [`Server`](jsonrpsee_core::server::Server) that operates on HTTP.
pub type HttpServer =
    Server<HttpTransportServer, <HttpTransportServer as TransportServer>::RequestId>;

mod client;
mod server;

/// Starts a [`Server`](../Server) object that serves HTTP.
pub async fn http_server(
    addr: &SocketAddr,
) -> Result<HttpServer, Box<dyn error::Error + Send + Sync>> {
    Ok(Server::new(HttpTransportServer::bind(addr).await?))
}

/// Starts a [`Server`](../Server) object that serves HTTP with a whitlist access control.
pub async fn http_server_with_acl(
    addr: &SocketAddr,
    access_control: AccessControl,
) -> Result<HttpServer, Box<dyn error::Error + Send + Sync>> {
    Ok(Server::new(
        HttpTransportServer::bind_with_acl(addr, access_control).await?,
    ))
}

/// Returns an object that lets you perform JSON-RPC queries towards the given HTTP server.
pub fn http_client(addr: &str) -> HttpClient {
    HttpTransportClient::new(addr)
}
