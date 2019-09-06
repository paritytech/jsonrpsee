use crate::server::{Server, raw::HttpServer};
use async_std::net::ToSocketAddrs;

/// Starts a [`Server`](../Server) object that serves HTTP.
pub async fn http(addr: impl ToSocketAddrs) -> Server<HttpServer> {
    Server::new(HttpServer::bind(addr).await)
}
