/// WebSocket Client.
pub mod client;
/// JSONRPC WebSocket transport.
pub mod jsonrpc_transport;
/// Request manager.
pub mod manager;
/// Stream.
pub mod stream;
/// WebSocket transport.
pub mod transport;

#[cfg(test)]
mod tests;

pub use client::{WsClient, WsConfig};
pub use jsonrpsee_types::client::Subscription as WsSubscription;
