//! Shared helpers for JSON-RPC Servers.

use jsonrpsee_types::v2::error::{INTERNAL_ERROR_CODE, INTERNAL_ERROR_MSG};
use jsonrpsee_types::v2::{JsonRpcError, JsonRpcErrorParams, JsonRpcResponse, RpcParams, TwoPointZero};
use rustc_hash::FxHashMap;
use serde::Serialize;
use serde_json::value::RawValue;
use tokio::sync::mpsc;

/// Connection ID.
pub type ConnectionId = usize;
/// Sender.
pub type RpcSender<'a> = &'a mpsc::UnboundedSender<String>;
/// RPC ID.
pub type RpcId<'a> = Option<&'a RawValue>;
/// Method registered in the server.
pub type Method = Box<dyn Send + Sync + Fn(RpcId, RpcParams, RpcSender, ConnectionId) -> anyhow::Result<()>>;
/// Methods registered in the Server.
pub type Methods = FxHashMap<&'static str, Method>;

/// Helper for sending JSON-RPC responses to the client
pub fn send_response(id: RpcId, tx: RpcSender, result: impl Serialize) {
	let json = match serde_json::to_string(&JsonRpcResponse { jsonrpc: TwoPointZero, id, result }) {
		Ok(json) => json,
		Err(err) => {
			log::error!("Error serializing response: {:?}", err);

			return send_error(id, tx, INTERNAL_ERROR_CODE, INTERNAL_ERROR_MSG);
		}
	};

	if let Err(err) = tx.send(json) {
		log::error!("Error sending response to the client: {:?}", err)
	}
}

/// Helper for sending JSON-RPC errors to the client
pub fn send_error(id: RpcId, tx: RpcSender, code: i32, message: &str) {
	let json = match serde_json::to_string(&JsonRpcError {
		jsonrpc: TwoPointZero,
		error: JsonRpcErrorParams { code, message },
		id,
	}) {
		Ok(json) => json,
		Err(err) => {
			log::error!("Error serializing error message: {:?}", err);

			return;
		}
	};

	if let Err(err) = tx.send(json) {
		log::error!("Error sending response to the client: {:?}", err)
	}
}
