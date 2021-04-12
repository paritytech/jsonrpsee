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
// IN background_task WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use futures::io::{BufReader, BufWriter};
use jsonrpsee_types::error::Error;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::Serialize;
use serde_json::value::to_raw_value;
use soketto::handshake::{server::Response, Server as SokettoServer};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::{
	net::{TcpListener, ToSocketAddrs},
	sync::mpsc,
};
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use tokio_util::compat::TokioAsyncReadCompatExt;

use jsonrpsee_types::v2::error::{METHOD_NOT_FOUND_CODE, METHOD_NOT_FOUND_MSG};
use jsonrpsee_types::v2::{JsonRpcInvalidRequest, JsonRpcRequest, RpcError, RpcParams, TwoPointZero};
use jsonrpsee_types::v2::{JsonRpcNotification, JsonRpcNotificationParams};
use jsonrpsee_utils::server::{send_error, ConnectionId, Methods};

mod module;

pub use module::{RpcContextModule, RpcModule};

type SubscriptionId = u64;

#[derive(Clone)]
pub struct SubscriptionSink {
	method: &'static str,
	subscribers: Arc<Mutex<FxHashMap<(ConnectionId, SubscriptionId), mpsc::UnboundedSender<String>>>>,
}

impl SubscriptionSink {
	pub fn send<T>(&mut self, result: &T) -> anyhow::Result<()>
	where
		T: Serialize,
	{
		let result = to_raw_value(result)?;

		let mut errored = Vec::new();
		let mut subs = self.subscribers.lock();

		for ((conn_id, sub_id), sender) in subs.iter() {
			let msg = serde_json::to_string(&JsonRpcNotification {
				jsonrpc: TwoPointZero,
				method: self.method,
				params: JsonRpcNotificationParams { subscription: *sub_id, result: &*result },
			})?;

			// Log broken connections
			if sender.send(msg).is_err() {
				errored.push((*conn_id, *sub_id));
			}
		}

		// Remove broken connections
		for entry in errored {
			subs.remove(&entry);
		}

		Ok(())
	}
}

pub struct Server {
	root: RpcModule,
	listener: TcpListener,
}

impl Server {
	/// Create a new WebSocket RPC server, bound to the `addr`.
	pub async fn new(addr: impl ToSocketAddrs) -> anyhow::Result<Self> {
		let listener = TcpListener::bind(addr).await?;

		Ok(Server { listener, root: RpcModule::new() })
	}

	/// Register a new RPC method, which responds with a given callback.
	pub fn register_method<F, R>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize,
		F: Fn(RpcParams) -> Result<R, RpcError> + Send + Sync + 'static,
	{
		self.root.register_method(method_name, callback)
	}

	/// Register a new RPC subscription, with subscribe and unsubscribe methods.
	pub fn register_subscription(
		&mut self,
		subscribe_method_name: &'static str,
		unsubscribe_method_name: &'static str,
	) -> Result<SubscriptionSink, Error> {
		self.root.register_subscription(subscribe_method_name, unsubscribe_method_name)
	}

	/// Register all methods from a module on this server.
	pub fn register_module(&mut self, module: RpcModule) -> Result<(), Error> {
		self.root.merge(module)
	}

	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> anyhow::Result<SocketAddr> {
		self.listener.local_addr().map_err(Into::into)
	}

	/// Start responding to connections requests. This will block current thread until the server is stopped.
	pub async fn start(self) {
		let mut incoming = TcpListenerStream::new(self.listener);
		let methods = Arc::new(self.root.into_methods());
		let mut id = 0;

		while let Some(socket) = incoming.next().await {
			if let Ok(socket) = socket {
				socket.set_nodelay(true).unwrap();

				let methods = methods.clone();

				tokio::spawn(async move { background_task(socket, methods, id).await });

				id += 1;
			}
		}
	}
}

async fn background_task(socket: tokio::net::TcpStream, methods: Arc<Methods>, id: ConnectionId) -> anyhow::Result<()> {
	// For each incoming background_task we perform a handshake.
	let mut server = SokettoServer::new(BufReader::new(BufWriter::new(socket.compat())));

	let websocket_key = {
		let req = server.receive_request().await?;
		req.into_key()
	};

	// Here we accept the client unconditionally.
	let accept = Response::Accept { key: &websocket_key, protocol: None };
	server.send_response(&accept).await?;

	// And we can finally transition to a websocket background_task.
	let (mut sender, mut receiver) = server.into_builder().finish();
	let (tx, mut rx) = mpsc::unbounded_channel::<String>();

	tokio::spawn(async move {
		while let Some(response) = rx.recv().await {
			let _ = sender.send_binary_mut(response.into_bytes()).await;
			let _ = sender.flush().await;
		}
	});

	let mut data = Vec::new();

	loop {
		data.clear();

		receiver.receive_data(&mut data).await?;

		match serde_json::from_slice::<JsonRpcRequest>(&data) {
			Ok(req) => {
				let params = RpcParams::new(req.params.map(|params| params.get()));

				if let Some(method) = methods.get(&*req.method) {
					(method)(req.id, params, &tx, id)?;
				} else {
					send_error(req.id, &tx, METHOD_NOT_FOUND_CODE, METHOD_NOT_FOUND_MSG);
				}
			}
			Err(_) => {
				let (id, code, msg) = match serde_json::from_slice::<JsonRpcInvalidRequest>(&data) {
					Ok(req) => (req.id, -32600, "Invalid request"),
					Err(_) => (None, -32700, "Parse error"),
				};

				send_error(id, &tx, code, msg);
			}
		}
	}
}
