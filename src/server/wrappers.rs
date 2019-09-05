use crate::raw_server::HttpServer;
use crate::server::Server;
use async_std::net::ToSocketAddrs;

/// Starts a [`Server`](../Server) object that servers HTTP.
pub async fn http(addr: impl ToSocketAddrs) -> Server<HttpServer> {
    Server::new(HttpServer::bind(addr).await)
}
