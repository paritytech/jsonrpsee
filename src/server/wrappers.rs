use crate::raw_server::HttpServer;
use crate::server::Server;

/// Starts a [`Server`](../Server) object that servers HTTP.
pub fn http() -> Server<HttpServer> {
    Server::new(HttpServer::bind("0.0.0.0:8000"))
}
