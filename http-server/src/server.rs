// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

use crate::transport::HttpTransportServer;

use hyper::server::{conn::AddrIncoming, Builder as HyperBuilder};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body as HyperBody, Error as HyperError, Response as HyperResponse, Server as HyperServer};
use jsonrpsee_types::error::Error;
use jsonrpsee_types::jsonrpc_v2::{ConnectionId, Methods, RpcError, RpcId, RpcParams, RpcSender};
use jsonrpsee_types::jsonrpc_v2::{JsonRpcError, JsonRpcErrorParams};
use jsonrpsee_types::jsonrpc_v2::{JsonRpcInvalidRequest, JsonRpcRequest, JsonRpcResponse, TwoPointZero};
use jsonrpsee_types::jsonrpc_v2::{JsonRpcNotification, JsonRpcNotificationParams};
use parking_lot::Mutex;
use serde::Serialize;
use std::{
	collections::{HashMap, HashSet},
	error,
	net::SocketAddr,
	sync::{atomic, Arc},
};
use tokio::{
	net::{TcpListener, ToSocketAddrs},
	sync::mpsc,
};

pub struct Server {
	/// Local socket address of the transport server.
	listener: HyperBuilder<AddrIncoming>,
}

impl Server {
	/// ...
	pub async fn new(addr: &SocketAddr) -> anyhow::Result<Self> {
		// TODO: use create the TCP socket manually to more fine-grained settings.
		let listener = hyper::Server::try_bind(&addr)?.tcp_nodelay(true);
		Ok(Self { listener })
	}

	/// Register a new RPC method, which responds with a given callback.
	pub fn register_method<F, R>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize,
		F: Fn(RpcParams) -> Result<R, RpcError> + Send + Sync + 'static,
	{
		//self.root.register_method(method_name, callback)
		Ok(())
	}

	/// Start responding to connections requests. This will block current thread until the server is stopped.
	pub async fn start(self) {
		let make_svc = make_service_fn(|_| async {
			Ok::<_, HyperError>(service_fn(|req| async move {
				log::info!("recv: {:?}", req);
				Ok::<_, HyperError>(crate::transport::response::bad_request(""))
			}))
		});

		let server = self.listener.serve(make_svc);
		let _ = server.await;
	}
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task() {}
