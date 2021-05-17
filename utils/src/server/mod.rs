//! Shared modules for the JSON-RPC servers.

/// Helpers.
pub mod helpers;
/// Abstract JSON-RPC modules that can be used to register methods on a server.
pub mod rpc_module;

/// Sender.
pub type RpcSender<'a> = &'a futures_channel::mpsc::UnboundedSender<String>;
/// RPC ID.
pub type RpcId<'a> = Option<&'a serde_json::value::RawValue>;
