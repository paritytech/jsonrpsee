// Copyright 2019-2021 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::server::rpc_module::MethodSink;
use futures_channel::mpsc;
use futures_util::stream::StreamExt;
use jsonrpsee_types::error::{CallError, Error};
use jsonrpsee_types::v2::{
	error::{CALL_EXECUTION_FAILED_CODE, UNKNOWN_ERROR_CODE},
	ErrorCode, ErrorObject, Id, InvalidRequest, Response, RpcError, TwoPointZero,
};
use serde::Serialize;

/// Helper for sending JSON-RPC responses to the client
pub fn send_response(id: Id, tx: &MethodSink, result: impl Serialize) {
	let json = match serde_json::to_string(&Response { jsonrpc: TwoPointZero, id: id.clone(), result }) {
		Ok(json) => json,
		Err(err) => {
			tracing::error!("Error serializing response: {:?}", err);

			return send_error(id, tx, ErrorCode::InternalError.into());
		}
	};

	tracing::debug!(tx_method_call_len = json.len());
	tracing::debug!(tx_method_call = ?json);
	if let Err(err) = tx.unbounded_send(json) {
		tracing::error!("Error sending response to the client: {:?}", err)
	}
}

/// Helper for sending JSON-RPC errors to the client
pub fn send_error(id: Id, tx: &MethodSink, error: ErrorObject) {
	let json = match serde_json::to_string(&RpcError { jsonrpc: TwoPointZero, error, id }) {
		Ok(json) => json,
		Err(err) => {
			tracing::error!("Error serializing error message: {:?}", err);

			return;
		}
	};

	if let Err(err) = tx.unbounded_send(json) {
		tracing::error!("Could not send error response to the client: {:?}", err)
	}
}

/// Helper for sending the general purpose `Error` as a JSON-RPC errors to the client
pub fn send_call_error(id: Id, tx: &MethodSink, err: Error) {
	let (code, message, data) = match err {
		Error::Call(CallError::InvalidParams(e)) => (ErrorCode::InvalidParams, e.to_string(), None),
		Error::Call(CallError::Failed(e)) => (ErrorCode::ServerError(CALL_EXECUTION_FAILED_CODE), e.to_string(), None),
		Error::Call(CallError::Custom { code, message, data }) => (code.into(), message, data),
		// This should normally not happen because the most common use case is to
		// return `Error::Call` in `register_async_method`.
		e => (ErrorCode::ServerError(UNKNOWN_ERROR_CODE), e.to_string(), None),
	};

	let err = ErrorObject { code, message: &message, data: data.as_deref() };

	send_error(id, tx, err)
}

/// Figure out if this is a sufficiently complete request that we can extract an [`Id`] out of, or just plain
/// unparseable garbage.
pub fn prepare_error(data: &[u8]) -> (Id<'_>, ErrorCode) {
	match serde_json::from_slice::<InvalidRequest>(data) {
		Ok(InvalidRequest { id }) => (id, ErrorCode::InvalidRequest),
		Err(_) => (Id::Null, ErrorCode::ParseError),
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
