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
use jsonrpsee_types::jsonrpc::SubscriptionId;
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::Serialize;
use serde_json::value::RawValue;
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

use crate::types::{JsonRpcNotification, JsonRpcNotificationParams, JsonRpcRequest, JsonRpcResponse, TwoPointZero};

type ConnectionId = usize;

type RpcSender<'a> = &'a mpsc::UnboundedSender<String>;
type RpcId<'a> = Option<&'a RawValue>;
type RpcParams<'a> = &'a str;
type Methods =
	FxHashMap<&'static str, Box<dyn Send + Sync + Fn(RpcId, RpcParams, RpcSender, ConnectionId) -> anyhow::Result<()>>>;

pub struct Server {
	methods: Methods,
	listener: TcpListener,
}

trait RpcResult {
	fn to_json(self, id: Option<&RawValue>) -> anyhow::Result<String>;
}

#[derive(Error, Debug)]
pub enum RpcError {
	#[error("unknown rpc error")]
	Unknown,
}

#[derive(Clone)]
pub struct SubsciptionSink {
	method: &'static str,
	subscribers: Arc<Mutex<FxHashMap<(ConnectionId, u64), mpsc::UnboundedSender<String>>>>,
}

impl SubsciptionSink {
	pub fn send<T>(&mut self, result: &T) -> anyhow::Result<()>
	where
		T: Serialize,
	{
		for ((_, sub_id), sender) in self.subscribers.lock().iter() {
			let msg = serde_json::to_string(&JsonRpcNotification {
				jsonrpc: TwoPointZero,
				method: self.method,
				params: JsonRpcNotificationParams { subscription: *sub_id, result },
			})?;

			sender.send(msg)?;
		}

		Ok(())
	}
}

impl Server {
	pub async fn new(addr: impl ToSocketAddrs) -> anyhow::Result<Self> {
		let listener = TcpListener::bind(addr).await?;

		Ok(Server { listener, methods: Default::default() })
	}

	pub fn register_method<F, R>(&mut self, method_name: &'static str, callback: F)
	where
		R: Serialize,
		F: Fn(&str) -> Result<R, RpcError> + Send + Sync + 'static, // TODO: figure out correct lifetime here
	{
		self.methods.insert(
			method_name,
			Box::new(move |id, params, tx, _| {
				let result = callback(params)?;

				let json = serde_json::to_string(&JsonRpcResponse { jsonrpc: TwoPointZero, id, result })?;

				tx.send(json).map_err(Into::into)
			}),
		);
	}

	pub fn register_subscription<T>(
		&mut self,
		subscribe_method_name: &'static str,
		unsubscribe_method_name: &'static str,
	) -> SubsciptionSink {
		let subscribers = Arc::new(Mutex::new(FxHashMap::default()));

		{
			let subscribers = subscribers.clone();
			self.methods.insert(
				subscribe_method_name,
				Box::new(move |id, _, tx, conn| {
					let sub_id = {
						const JS_NUM_MASK: u64 = !0 >> 11;

						let sub_id = rand::random::<u64>() & JS_NUM_MASK;

						subscribers.lock().insert((conn, sub_id), tx.clone());

						sub_id
					};

					let json = serde_json::to_string(&JsonRpcResponse { jsonrpc: TwoPointZero, id, result: sub_id })?;

					tx.send(json).map_err(Into::into)
				}),
			);
		}

		{
			let subscribers = subscribers.clone();
			self.methods.insert(
				unsubscribe_method_name,
				Box::new(move |id, params, tx, conn| {
					let [sub_id]: [u64; 1] = serde_json::from_str(params)?;

					subscribers.lock().remove(&(conn, sub_id));

					let json = serde_json::to_string(&JsonRpcResponse {
						jsonrpc: TwoPointZero,
						id,
						result: SubscriptionId::Num(0),
					})?;

					tx.send(json).map_err(Into::into)
				}),
			);
		}

		SubsciptionSink { method: subscribe_method_name, subscribers }
	}

	pub fn local_addr(&self) -> anyhow::Result<SocketAddr> {
		self.listener.local_addr().map_err(Into::into)
	}

	pub async fn start(self) {
		let mut incoming = TcpListenerStream::new(self.listener);
		let methods = Arc::new(self.methods);
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

		let req: Result<JsonRpcRequest, _> = serde_json::from_slice(&data);

		if let Ok(req) = req {
			if let Some(method) = methods.get(&*req.method) {
				(method)(req.id, req.params.get(), &tx, id)?;
			}
		}
	}
}
