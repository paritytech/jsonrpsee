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

//! jsonrpsee-server middleware

use std::net::SocketAddr;

use hyper::Body;
use jsonrpsee_core::server::MethodResponse;
use jsonrpsee_types::Request;

use self::rpc::RpcService;

/// HTTP related middleware.
pub mod http;
/// JSON-RPC specific middleware.
pub mod rpc;

/// Represent a single connection.
#[async_trait::async_trait]
pub trait ConnectionManager {
	/// Callback that is invoked every time a new connection is created.
	///
	/// You should only return `Some(response)` if you want refuse a peer.
	async fn on_connect(
		&self,
		req: hyper::Request<Body>,
		remote_addr: SocketAddr,
		conn_id: u32,
	) -> Option<hyper::Response<Body>>;

	/// Middleware that runs on every RPC call.
	async fn on_call<'a, S>(&self, req: Request<'a>, service: S) -> MethodResponse;

	/// Callback that is invoked once a peer disconnects.
	fn on_disconnect(&self);
}
