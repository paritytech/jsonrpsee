use crate::client::{raw::http::WithServer, raw::HttpClientPool, Client};
use crate::server::{raw::HttpServer, raw::RawServer, Server};
use async_std::net::ToSocketAddrs;

/// Starts a [`Server`](../Server) object that serves HTTP.
pub async fn http_server(addr: impl ToSocketAddrs) -> Server<HttpServer, <HttpServer as RawServer>::RequestId> {
    Server::new(HttpServer::bind(addr).await)
}

lazy_static::lazy_static! {
    static ref HTTP_POOL: HttpClientPool = HttpClientPool::new().unwrap();      // TODO: don't unwrap
}

/// Returns an object that lets you perform JSON-RPC queries towards the given HTTP server.
// TODO: static addr :(
pub fn http_client(addr: &str) -> Client<WithServer<'static, 'static>> {
    Client::new(HTTP_POOL.with_server(addr.to_string()))
}
