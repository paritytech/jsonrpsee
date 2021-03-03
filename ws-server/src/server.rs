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
use serde::{Deserialize, Serialize};
use serde_json::value::{to_raw_value, RawValue};
use soketto::handshake::{server::Response, Server as SokettoServer};
use std::net::SocketAddr;
use std::sync::Arc;
use thiserror::Error;
use tokio::{
	net::{TcpListener, ToSocketAddrs},
	sync::mpsc,
};
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::types::{ConnectionId, Methods, RpcId, RpcMethod, RpcSender};
use crate::types::{JsonRpcError, JsonRpcErrorParams};
use crate::types::{JsonRpcInvalidRequest, JsonRpcRequest, JsonRpcResponse, TwoPointZero};
use crate::types::{JsonRpcNotification, JsonRpcNotificationParams};

mod module;

pub use module::RpcModule;

type SubscriptionId = u64;

trait RpcResult {
	fn to_json(self, id: Option<&RawValue>) -> anyhow::Result<String>;
}

#[derive(Error, Debug)]
pub enum RpcError {
	#[error("unknown rpc error")]
	Unknown,
	#[error("invalid params")]
	InvalidParams,
}

/// Parameters sent with the RPC request
#[derive(Clone, Copy)]
pub struct RpcParams<'a>(Option<&'a str>);

impl<'a> RpcParams<'a> {
	/// Attempt to parse all parameters as array or map into type T
	pub fn parse<T>(self) -> Result<T, RpcError>
	where
		T: Deserialize<'a>,
	{
		match self.0 {
			None => Err(RpcError::InvalidParams),
			Some(params) => serde_json::from_str(params).map_err(|_| RpcError::InvalidParams),
		}
	}

	/// Attempt to parse only the first parameter from an array into type T
	pub fn one<T>(self) -> Result<T, RpcError>
	where
		T: Deserialize<'a>,
	{
		self.parse::<[T; 1]>().map(|[res]| res)
	}
}

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

// Private helper for sending JSON-RPC responses to the client
fn send_response(id: RpcId, tx: RpcSender, result: impl Serialize) {
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
fn send_error(id: RpcId, tx: RpcSender, code: i32, message: &str) {
	let json = match serde_json::to_string(&JsonRpcError {
		jsonrpc: TwoPointZero,
		error: JsonRpcErrorParams { code, message },
		id: id,
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
		let methods = Arc::new(self.root.into_map());
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
				let params = RpcParams(req.params.map(|params| params.get()));

				if let Some(method) = methods.get(&*req.method) {
					(method)(req.id, params, &tx, id)?;
				} else {
					send_error(req.id, &tx, -32601, "Method not found");
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
