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
use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::value::to_raw_value;
use soketto::handshake::{server::Response, Server as SokettoServer};
use std::{net::SocketAddr, sync::Arc};
use std::convert::{TryFrom, TryInto};
use tokio::net::{TcpListener, ToSocketAddrs};
use tokio_stream::wrappers::TcpListenerStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

use jsonrpsee_types::error::{CallError, Error};
use jsonrpsee_types::v2::error::JsonRpcErrorCode;
use jsonrpsee_types::v2::params::{JsonRpcNotificationParams, RpcParams, TwoPointZero};
use jsonrpsee_types::v2::request::{JsonRpcInvalidRequest, JsonRpcNotification, JsonRpcRequest};
use jsonrpsee_utils::server::{collect_batch_response, send_error, ConnectionId, Methods, RpcSender};

mod module;

pub use module::{RpcContextModule, RpcModule};

type SubscriptionId = u64;
type Subscribers<P> = Arc<Mutex<FxHashMap<(ConnectionId, SubscriptionId), InnerSink<P>>>>;

#[derive(Clone)]
pub struct SubscriptionSink<P> {
	method: &'static str,
	subscribers: Subscribers<P>,
}

impl<P> SubscriptionSink<P> {
	/// Send a message on all the subscriptions
	///
	/// If you have subscriptions with params/input you should most likely
	/// call `extract_with_input` to the process the input/params and send out
	/// the result on each subscription individually instead.
	pub fn send_all<T>(&mut self, result: &T) -> anyhow::Result<()>
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
			if sender.sink.unbounded_send(msg).is_err() {
				errored.push((*conn_id, *sub_id));
			}
		}

		// Remove broken connections
		for entry in errored {
			subs.remove(&entry);
		}

		Ok(())
	}

	/// Extract subscriptions with input.
	/// Usually, you want process the input before sending back
	/// data on that subscription.
	pub fn extract_with_input(&self) -> Vec<InnerSinkWithParams<P>> {
		let mut subs = self.subscribers.lock();

		let mut input = Vec::new();
		*subs = std::mem::take(&mut *subs)
			.into_iter()
			.filter_map(|(k, v)| {
				if v.params.is_some() {
					let with_input = v.try_into().expect("is Some checked above; qed");
					input.push(with_input);
					None
				} else {
					Some((k, v))
				}
			})
			.collect();
		input
	}
}

pub struct InnerSink<P> {
	/// Sink.
	sink: mpsc::UnboundedSender<String>,
	/// Params.
	params: Option<P>,
	/// Subscription ID.
	sub_id: SubscriptionId,
	/// Method name
	method: &'static str,
}

pub struct InnerSinkWithParams<P> {
	/// Sink.
	sink: mpsc::UnboundedSender<String>,
	/// Params.
	params: P,
	/// Subscription ID.
	sub_id: SubscriptionId,
	/// Method name
	method: &'static str,
}

impl<P> TryFrom<InnerSink<P>> for InnerSinkWithParams<P> {
	type Error = ();

	fn try_from(other: InnerSink<P>) -> Result<Self, Self::Error> {
		match other.params {
			Some(params) => Ok(InnerSinkWithParams { sink: other.sink, params, sub_id: other.sub_id, method: other.method }),
			None => Err(())
		}
	}
}

impl<P> InnerSinkWithParams<P> {
	/// Send data on a specific subscription
	/// Note: a subscription can be "subscribed" to arbitary number of times with
	/// diffrent input/params.
	pub fn send<T>(&self, result: &T) -> anyhow::Result<()>
	where
		T: Serialize,
	{
		let result = to_raw_value(result)?;
		let msg = serde_json::to_string(&JsonRpcNotification {
			jsonrpc: TwoPointZero,
			method: self.method,
			params: JsonRpcNotificationParams { subscription: self.sub_id, result: &*result },
		})?;

		self.sink.unbounded_send(msg).map_err(|e| anyhow::anyhow!("{:?}", e))
	}

	/// Get the input/params of the subscrption.
	pub fn params(&self) -> &P {
		&self.params
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
	pub fn register_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize,
		F: Fn(RpcParams) -> Result<R, CallError> + Send + Sync + 'static,
	{
		self.root.register_method(method_name, callback)
	}

	/// Register a new RPC subscription, with subscribe and unsubscribe methods.
	pub fn register_subscription<P: DeserializeOwned + Send + Sync + 'static>(
		&mut self,
		subscribe_method_name: &'static str,
		unsubscribe_method_name: &'static str,
	) -> Result<SubscriptionSink<P>, Error> {
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

async fn background_task(
	socket: tokio::net::TcpStream,
	methods: Arc<Methods>,
	conn_id: ConnectionId,
) -> anyhow::Result<()> {
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
	let execute = move |tx: RpcSender, req: JsonRpcRequest| {
		if let Some(method) = methods.get(&*req.method) {
			let params = RpcParams::new(req.params.map(|params| params.get()));
			if let Err(err) = (method)(req.id, params, &tx, conn_id) {
				log::error!("execution of method call '{}' failed: {:?}, request id={:?}", req.method, err, req.id);
				send_error(req.id, &tx, JsonRpcErrorCode::ServerError(-1).into());
			}
		} else {
			send_error(req.id, &tx, JsonRpcErrorCode::MethodNotFound.into());
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
			execute(&tx, req);
		} else if let Ok(batch) = serde_json::from_slice::<Vec<JsonRpcRequest>>(&data) {
			if !batch.is_empty() {
				// Batch responses must be sent back as a single message so we read the results from each request in the
				// batch and read the results off of a new channel, `rx_batch`, and then send the complete batch response
				// back to the client over `tx`.
				let (tx_batch, mut rx_batch) = mpsc::unbounded::<String>();
				for req in batch {
					execute(&tx_batch, req);
				}
				// Closes the receiving half of a channel without dropping it. This prevents any further messages from
				// being sent on the channel.
				rx_batch.close();
				let results = collect_batch_response(rx_batch).await;
				if let Err(err) = tx.unbounded_send(results) {
					log::error!("Error sending batch response to the client: {:?}", err)
				}
			} else {
				send_error(None, &tx, JsonRpcErrorCode::InvalidRequest.into());
			}
		} else {
			let (id, code) = match serde_json::from_slice::<JsonRpcInvalidRequest>(&data) {
				Ok(req) => (req.id, JsonRpcErrorCode::InvalidRequest),
				Err(_) => (None, JsonRpcErrorCode::ParseError),
			};

			send_error(id, &tx, code.into());
		}
	}
}
