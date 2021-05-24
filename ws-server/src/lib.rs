// Copyright 2019 Parity Technologies (UK) Ltd.
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

extern crate alloc;

use std::fmt::{self, Debug, Display};

mod server;

#[cfg(test)]
mod tests;

pub use jsonrpsee_types::error::Error;
pub use jsonrpsee_utils::server::rpc_module::{Methods, RpcContextModule, RpcModule, SubscriptionSink};
pub use server::Server as WsServer;

#[derive(Debug, thiserror::Error)]
/// Websocket-specific error type.
pub struct WsError(Error);

impl Display for WsError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<WsError> for Error {
	fn from(ws_err: WsError) -> Error {
		ws_err.0
	}
}

impl From<soketto::handshake::Error> for WsError {
	fn from(handshake_err: soketto::handshake::Error) -> WsError {
		WsError(Error::Transport(Box::new(handshake_err)))
	}
}

impl From<soketto::connection::Error> for WsError {
	fn from(conn_err: soketto::connection::Error) -> WsError {
		WsError(Error::Transport(Box::new(conn_err)))
	}
}
