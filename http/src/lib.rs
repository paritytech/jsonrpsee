#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

use crate::client::HttpClient;
use crate::server::HttpServer;
use async_std::net::ToSocketAddrs;
use jsonrpsee_core::client::Client;
use jsonrpsee_core::server::{raw::RawServer, Server};

pub mod client;
pub mod server;

/// Starts a [`Server`](../Server) object that serves HTTP.
pub async fn http_server(addr: impl ToSocketAddrs) -> Server<HttpServer, <HttpServer as RawServer>::RequestId> {
    Server::new(HttpServer::bind(addr).await)
}

/// Returns an object that lets you perform JSON-RPC queries towards the given HTTP server.
pub fn http_client(addr: &str) -> Client<HttpClient> {
    Client::new(HttpClient::new(addr))
}
