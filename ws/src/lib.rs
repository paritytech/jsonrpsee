//! Implementation of [`RawClient`](jsonrpsee_core::client::raw::RawClient) and
//! [`RawServer`](jsonrpsee_core::server::raw::RawServer) for HTTP.

#![deny(unsafe_code)]
#![deny(intra_doc_link_resolution_failure)]
#![warn(missing_docs)]

use jsonrpsee_core::client::Client;

pub use crate::client::{WsConnecError, WsNewError, WsRawClient};

// TODO: server

/// Type alias for a [`Client`](jsonrpsee_core::client::Client) that operates on WebSockets.
pub type WsClient = Client<WsRawClient>;

mod client;

/// Returns an object that lets you perform JSON-RPC queries towards the given HTTP server.
pub async fn ws_client(target: &str) -> Result<WsClient, client::WsNewDnsError> {
    WsRawClient::new(target).await
}
