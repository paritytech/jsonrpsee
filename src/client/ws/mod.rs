/// Client.
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

pub use client::{Client, Config, Subscription};
