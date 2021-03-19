use super::{JsonRpcError, JsonRpcErrorParams, JsonRpcResponse, RpcId, RpcSender, TwoPointZero};
use serde::Serialize;

// Private helper for sending JSON-RPC responses to the client
pub fn send_response(id: RpcId, tx: RpcSender, result: impl Serialize) {
	let json = match serde_json::to_string(&JsonRpcResponse { jsonrpc: TwoPointZero, id, result }) {
		Ok(json) => json,
		Err(err) => {
			log::error!("Error serializing response: {:?}", err);

			return send_error(id, tx, -32603, "Internal error");
		}
	};

	if let Err(err) = tx.send(json) {
		log::error!("Error sending response to the client: {:?}", err)
	}
}

// Private helper for sending JSON-RPC errors to the client
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
