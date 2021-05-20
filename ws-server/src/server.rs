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

use futures_channel::mpsc;
use futures_util::io::{BufReader, BufWriter};
use futures_util::stream::StreamExt;
use serde::Serialize;
use soketto::handshake::{server::Response, Server as SokettoServer};
use std::{net::SocketAddr, sync::Arc};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio_stream::wrappers::TcpListenerStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

use jsonrpsee_types::v2::params::{Id, RpcParams};
use jsonrpsee_types::v2::request::{JsonRpcInvalidRequest, JsonRpcRequest};
use jsonrpsee_types::v2::{error::JsonRpcErrorCode, request::OwnedJsonRpcRequest};
use jsonrpsee_types::{
	error::{CallError, Error},
	traits::AsyncRpcMethod,
};
use jsonrpsee_utils::server::rpc_module::{ConnectionId, MethodSink, RpcModule, SubscriptionSink};
use jsonrpsee_utils::server::{
	helpers::{collect_batch_response, send_error},
	rpc_module::MethodType,
};

pub struct Server {
	root: RpcModule,
	listener: TcpListener,
}

impl Server {
	/// Create a new WebSocket RPC server, bound to the `addr`.
	pub async fn new(addr: impl ToSocketAddrs) -> Result<Self, Error> {
		let listener = TcpListener::bind(addr).await?;

		Ok(Server { listener, root: RpcModule::new() })
	}

	/// Register a new RPC method, which responds with a given callback.
	pub fn register_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize,
		F: Fn(RpcParams) -> Result<R, CallError> + Send + Sync + 'static,
	{
		self.root.register_method(method_name, callback)
	}

	/// Register a new RPC method, which responds with a given callback.
	pub fn register_async_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize + Send + Sync + 'static,
		F: AsyncRpcMethod<R, CallError> + Copy + Send + Sync + 'static,
	{
		self.root.register_async_method(method_name, callback)
	}

	/// Register a new RPC subscription, with subscribe and unsubscribe methods.
	pub fn register_subscription<F>(
		&mut self,
		subscribe_method_name: &'static str,
		unsubscribe_method_name: &'static str,
		callback: F,
	) -> Result<(), Error>
	where
		F: Fn(RpcParams, SubscriptionSink) -> Result<(), Error> + Send + Sync + 'static,
	{
		self.root.register_subscription(subscribe_method_name, unsubscribe_method_name, callback)
	}

	/// Register all methods from a module on this server.
	pub fn register_module(&mut self, module: RpcModule) -> Result<(), Error> {
		self.root.merge(module)
	}

	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.listener.local_addr().map_err(Into::into)
	}

	/// Start responding to connections requests. This will block current thread until the server is stopped.
	pub async fn start(self) {
		let mut incoming = TcpListenerStream::new(self.listener);
		let methods = Arc::new(self.root);
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

async fn background_task(
	socket: tokio::net::TcpStream,
	methods: Arc<RpcModule>,
	conn_id: ConnectionId,
) -> Result<(), Error> {
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
	let (tx, mut rx) = mpsc::unbounded::<String>();

	// Send results back to the client.
	tokio::spawn(async move {
		while let Some(response) = rx.next().await {
			log::debug!("send: {}", response);
			let _ = sender.send_binary_mut(response.into_bytes()).await;
			let _ = sender.flush().await;
		}
	});

	let mut data = Vec::with_capacity(100);

	// Look up the "method" (i.e. function pointer) from the registered methods and run it passing in
	// the params from the request. The result of the computation is sent back over the `tx` channel and
	// the result(s) are collected into a `String` and sent back over the wire.
	//
	// Note: This handler expects method existence to be checked prior to the call and will panic if
	// method does not exist.
	let sync_methods = methods.clone();
	let execute_sync = move |tx: &MethodSink, req: JsonRpcRequest| {
		let method = sync_methods.method(&*req.method).unwrap();
		let params = RpcParams::new(req.params.map(|params| params.get()));
		if let Err(err) = (method)(req.id.clone(), params, &tx, conn_id) {
			log::error!("execution of method call '{}' failed: {:?}, request id={:?}", req.method, err, req.id);
			send_error(req.id, &tx, JsonRpcErrorCode::ServerError(-1).into());
		}
	};

	// Similar to `execute_sync`, but uses an asyncrhonous context.
	// Unfortunately, we have to use owned versions of objects due to heavy lifetime
	// usage in borrowed ones.
	// Probably there is a chance to avoid using the heap here through some `Pin` magic,
	// but several simple attempts to do so were failed.
	//
	// Note: This handler expects method existence to be checked prior to the call and will panic if
	// method does not exist.
	let execute_async = |tx: MethodSink, req: OwnedJsonRpcRequest| {
		let async_methods = methods.clone();
		async move {
			let req = req.borrowed();
			let method = async_methods.async_method(&*req.method).unwrap();
			let params = RpcParams::new(req.params.map(|params| params.get()));
			if let Err(err) = (method)(req.id.clone().into(), params.into(), tx.clone(), conn_id).await {
				log::error!("execution of method call '{}' failed: {:?}, request id={:?}", req.method, err, req.id);
				send_error(req.id, &tx, JsonRpcErrorCode::ServerError(-1).into());
			}
		}
	};

	loop {
		data.clear();

		receiver.receive_data(&mut data).await?;

		// For reasons outlined [here](https://github.com/serde-rs/json/issues/497), `RawValue` can't be used with
		// untagged enums at the moment. This means we can't use an `SingleOrBatch` untagged enum here and have to try
		// each case individually: first the single request case, then the batch case and lastly the error. For the
		// worst case – unparseable input – we make three calls to [`serde_json::from_slice`] which is pretty annoying.
		// Our [issue](https://github.com/paritytech/jsonrpsee/issues/296).
		if let Ok(req) = serde_json::from_slice::<JsonRpcRequest>(&data) {
			log::debug!("recv: {:?}", req);
			match methods.method_type(&*req.method) {
				Some(MethodType::Sync) => execute_sync(&tx, req),
				Some(MethodType::Async) => execute_async(tx.clone(), req.into()).await,
				None => {
					send_error(req.id, &tx, JsonRpcErrorCode::MethodNotFound.into());
				}
			}
		} else if let Ok(batch) = serde_json::from_slice::<Vec<JsonRpcRequest>>(&data) {
			if !batch.is_empty() {
				// Batch responses must be sent back as a single message so we read the results from each request in the
				// batch and read the results off of a new channel, `rx_batch`, and then send the complete batch response
				// back to the client over `tx`.
				let (tx_batch, mut rx_batch) = mpsc::unbounded::<String>();
				for req in batch {
					match methods.method_type(&*req.method) {
						Some(MethodType::Sync) => execute_sync(&tx_batch, req),
						Some(MethodType::Async) => execute_async(tx_batch.clone(), req.into()).await,
						None => {
							send_error(req.id, &tx_batch, JsonRpcErrorCode::MethodNotFound.into());
						}
					}
				}
				// Closes the receiving half of a channel without dropping it. This prevents any further messages from
				// being sent on the channel.
				rx_batch.close();
				let results = collect_batch_response(rx_batch).await;
				if let Err(err) = tx.unbounded_send(results) {
					log::error!("Error sending batch response to the client: {:?}", err)
				}
			} else {
				send_error(Id::Null, &tx, JsonRpcErrorCode::InvalidRequest.into());
			}
		} else {
			let (id, code) = match serde_json::from_slice::<JsonRpcInvalidRequest>(&data) {
				Ok(req) => (req.id, JsonRpcErrorCode::InvalidRequest),
				Err(_) => (Id::Null, JsonRpcErrorCode::ParseError),
			};

			send_error(id, &tx, code.into());
		}
	}
}
