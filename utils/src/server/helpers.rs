use crate::server::rpc_module::MethodSink;
use futures_channel::mpsc;
use futures_util::stream::StreamExt;
use jsonrpsee_types::v2::error::{JsonRpcError, JsonRpcErrorCode, JsonRpcErrorObject};
use jsonrpsee_types::v2::params::{Id, TwoPointZero};
use jsonrpsee_types::v2::response::JsonRpcResponse;
use jsonrpsee_types::v2::request::JsonRpcInvalidRequest;
use serde::Serialize;

/// Helper for sending JSON-RPC responses to the client
pub fn send_response(id: Id, tx: &MethodSink, result: impl Serialize) {
	let json = match serde_json::to_string(&JsonRpcResponse { jsonrpc: TwoPointZero, id: id.clone(), result }) {
		Ok(json) => json,
		Err(err) => {
			log::error!("Error serializing response: {:?}", err);

			return send_error(id, tx, JsonRpcErrorCode::InternalError.into());
		}
	};

	if let Err(err) = tx.unbounded_send(json) {
		log::error!("Error sending response to the client: {:?}", err)
	}
}

/// Helper for sending JSON-RPC errors to the client
pub fn send_error(id: Id, tx: &MethodSink, error: JsonRpcErrorObject) {
	let json = match serde_json::to_string(&JsonRpcError { jsonrpc: TwoPointZero, error, id }) {
		Ok(json) => json,
		Err(err) => {
			log::error!("Error serializing error message: {:?}", err);

			return;
		}
	};

	if let Err(err) = tx.unbounded_send(json) {
		log::error!("Error sending response to the client: {:?}", err)
	}
}


/// Figure out if this is a sufficiently complete request that we can extract an [`Id`] out of, or just plain
/// unparseable garbage.
pub fn prepare_error<'a>(data: &'a Vec<u8>) -> (Id<'a>, JsonRpcErrorCode) {
	match serde_json::from_slice::<JsonRpcInvalidRequest>(&data) {
		Ok(JsonRpcInvalidRequest { id }) => (id, JsonRpcErrorCode::InvalidRequest),
		Err(_) => (Id::Null, JsonRpcErrorCode::ParseError),
	}
}

/// Read all the results of all method calls in a batch request from the ['Stream']. Format the result into a single
/// `String` appropriately wrapped in `[`/`]`.
pub async fn collect_batch_response(rx: mpsc::UnboundedReceiver<String>) -> String {
	let mut buf = String::with_capacity(2048);
	buf.push('[');
	let mut buf = rx
		.fold(buf, |mut acc, response| async move {
			acc.push_str(&response);
			acc.push(',');
			acc
		})
		.await;
	// Remove trailing comma
	buf.pop();
	buf.push(']');
	buf
}
