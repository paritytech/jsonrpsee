//! Shared modules for the JSON-RPC servers.

/// Helpers.
pub mod helpers;
/// JSON-RPC "modules" groups sets of methods that belong together and handles method/subscription registration.
pub mod rpc_module;

/// Sender.
pub type RpcSender<'a> = &'a futures_channel::mpsc::UnboundedSender<String>;
/// RPC ID.
pub type RpcId<'a> = Option<&'a serde_json::value::RawValue>;
