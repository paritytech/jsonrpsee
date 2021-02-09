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
use rustc_hash::FxHashMap;
use serde::Serialize;
use serde_json::value::RawValue;
use soketto::handshake::{server::Response, Server as SokettoServer};
use std::sync::Arc;
use thiserror::Error;
use tokio::{net::TcpListener, sync::mpsc};
use tokio_stream::{wrappers::TcpListenerStream, StreamExt};
use tokio_util::compat::TokioAsyncReadCompatExt;

use crate::types::{JsonRpcRequest, JsonRpcResponse, TwoPointZero};

type Methods = FxHashMap<
	&'static str,
	Box<dyn Send + Sync + Fn(Option<&RawValue>, &str, &mpsc::UnboundedSender<String>) -> anyhow::Result<()>>,
>;

#[derive(Default)]
pub struct Server {
	methods: Methods,
}

trait RpcResult {
	fn to_json(self, id: Option<&RawValue>) -> anyhow::Result<String>;
}

#[derive(Error, Debug)]
pub enum RpcError {
	#[error("unknown rpc error")]
	Unknown,
}

impl Server {
	pub fn register_method<F, R>(&mut self, method_name: &'static str, callback: F)
	where
		R: Serialize,
		F: Fn(&str) -> Result<R, RpcError> + Send + Sync + 'static, // TODO: figure out correct lifetime here
	{
		self.methods.insert(
			method_name,
			Box::new(move |id, params, tx| {
				let result = callback(params)?;

				let json = serde_json::to_string(&JsonRpcResponse { jsonrpc: TwoPointZero, id, result })?;

				tx.send(json).map_err(Into::into)
			}),
		);
	}

	// TODO: This needs to return the sink channel, and use it to push new messages out.
	pub fn register_subscription(
		&mut self,
		subscribe_method_name: &'static str,
		unsubscribe_method_name: &'static str,
	) {
		self.methods.insert(subscribe_method_name, Box::new(move |id, params, tx| Ok(())));

		self.methods.insert(unsubscribe_method_name, Box::new(move |id, params, tx| Ok(())));
	}

	/// Build the server
	pub async fn start(self, addr: impl AsRef<str>) -> anyhow::Result<()> {
		let addr = addr.as_ref();
		let mut incoming = TcpListenerStream::new(TcpListener::bind(addr).await?);
		let methods = Arc::new(self.methods);

		while let Some(socket) = incoming.next().await {
			if let Ok(socket) = socket {
				socket.set_nodelay(true).unwrap();

				let methods = methods.clone();

				tokio::spawn(async move { background_task(socket, methods).await });
			}
		}

		Ok(())
	}
}

async fn background_task(socket: tokio::net::TcpStream, methods: Arc<Methods>) -> anyhow::Result<()> {
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
				(method)(req.id, req.params.get(), &tx)?;
			}
		}
	}
}
