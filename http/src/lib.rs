//! Implementation of [`RawClient`](jsonrpsee_core::client::raw::RawClient) and
//! [`RawServer`](jsonrpsee_core::server::raw::RawServer) for HTTP.

#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

use jsonrpsee_core::client::Client;
use jsonrpsee_core::server::{raw::RawServer, Server};
use std::{error, net::SocketAddr};

pub use crate::client::HttpRawClient;
pub use crate::server::HttpRawServer;

/// Type alias for a [`Client`](jsonrpsee_core::client::Client) that operates on HTTP.
pub type HttpClient = Client<HttpRawClient>;
/// Type alias for a [`Server`](jsonrpsee_core::server::Server) that operates on HTTP.
pub type HttpServer = Server<HttpRawServer, <HttpRawServer as RawServer>::RequestId>;

mod client;
mod server;

/// Starts a [`Server`](../Server) object that serves HTTP.
pub async fn http_server(
    addr: &SocketAddr,
) -> Result<HttpServer, Box<dyn error::Error + Send + Sync>> {
    Ok(Server::new(HttpRawServer::bind(addr).await?))
}

/// Starts a [`Server`](../Server) object that serves HTTP with a whitlist access control.
pub async fn http_server_with_acl(
    addr: &SocketAddr,
    allowed_hosts: Vec<SocketAddr>,
) -> Result<HttpServer, Box<dyn error::Error + Send + Sync>> {
    Ok(Server::new(HttpRawServer::bind_with_acl(addr, allowed_hosts).await?))
}

/// Returns an object that lets you perform JSON-RPC queries towards the given HTTP server.
pub fn http_client(addr: &str) -> HttpClient {
    HttpRawClient::new(addr)
}
