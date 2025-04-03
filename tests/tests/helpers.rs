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

#![cfg(test)]
#![allow(dead_code)]

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use std::time::Duration;

use fast_socks5::client::Socks5Stream;
use fast_socks5::server;
use futures::{SinkExt, Stream, StreamExt};
use jsonrpsee::core::middleware::{Batch, Notification, RpcServiceBuilder, RpcServiceT};
use jsonrpsee::server::middleware::http::ProxyGetRequestLayer;
use jsonrpsee::server::{
	ConnectionGuard, PendingSubscriptionSink, RpcModule, Server, ServerBuilder, ServerHandle, SubscriptionMessage,
	TrySendError, serve_with_graceful_shutdown, stop_channel,
};
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned};
use jsonrpsee::{Methods, SubscriptionCloseResponse};
use serde::Serialize;
use tokio::net::TcpStream;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use tower::Service;
use tower_http::cors::CorsLayer;

pub async fn server_with_subscription_and_handle() -> (SocketAddr, ServerHandle) {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();

	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _, _| "hello").unwrap();

	module
		.register_subscription(
			"subscribe_hello",
			"subscribe_hello",
			"unsubscribe_hello",
			|_, pending, _, _| async move {
				let interval = interval(Duration::from_millis(50));
				let stream = IntervalStream::new(interval).map(move |_| &"hello from subscription");
				pipe_from_stream_and_drop(pending, stream).await.map_err(Into::into)
			},
		)
		.unwrap();

	module
		.register_subscription("subscribe_foo", "subscribe_foo", "unsubscribe_foo", |_, pending, _, _| async {
			let interval = interval(Duration::from_millis(100));
			let stream = IntervalStream::new(interval).map(move |_| 1337_usize);
			pipe_from_stream_and_drop(pending, stream).await.map_err(Into::into)
		})
		.unwrap();

	module
		.register_subscription(
			"subscribe_add_one",
			"subscribe_add_one",
			"unsubscribe_add_one",
			|params, pending, _, _| async move {
				let count = match params.one::<usize>().map(|c| c.wrapping_add(1)) {
					Ok(count) => count,
					Err(e) => {
						let _ = pending.reject(e).await;
						return Ok(());
					}
				};

				let wrapping_counter = futures::stream::iter((count..).cycle());
				let interval = interval(Duration::from_millis(100));
				let stream = IntervalStream::new(interval).zip(wrapping_counter).map(move |(_, c)| c);
				pipe_from_stream_and_drop(pending, stream).await.map_err(Into::into)
			},
		)
		.unwrap();

	module
		.register_subscription("subscribe_noop", "subscribe_noop", "unsubscribe_noop", |_, pending, _, _| async {
			let _sink = pending.accept().await?;
			tokio::time::sleep(Duration::from_secs(1)).await;
			Err("Server closed the stream because it was lazy".to_string().into())
		})
		.unwrap();

	module
		.register_subscription("subscribe_5_ints", "n", "unsubscribe_5_ints", |_, pending, _, _| async move {
			let interval = interval(Duration::from_millis(50));
			let stream = IntervalStream::new(interval).zip(futures::stream::iter(1..=5)).map(|(_, c)| c);
			pipe_from_stream_and_drop(pending, stream).await.map_err(Into::into)
		})
		.unwrap();

	module
		.register_subscription("subscribe_option", "n", "unsubscribe_option", |_, pending, _, _| async move {
			let _ = pending.accept().await;
			SubscriptionCloseResponse::None
		})
		.unwrap();

	module
		.register_subscription("subscribe_unit", "n", "unubscribe_unit", |_, pending, _, _| async move {
			let _sink = pending.accept().await?;
			tokio::time::sleep(std::time::Duration::from_millis(100)).await;
			Ok(())
		})
		.unwrap();

	let addr = server.local_addr().unwrap();
	let server_handle = server.start(module);

	(addr, server_handle)
}

pub async fn server_with_subscription() -> SocketAddr {
	let (addr, handle) = server_with_subscription_and_handle().await;
	tokio::spawn(handle.stopped());
	addr
}

pub async fn server() -> SocketAddr {
	#[derive(Debug, Clone)]
	struct ConnectionDetails<S> {
		inner: S,
		connection_id: u32,
	}

	impl<S> RpcServiceT for ConnectionDetails<S>
	where
		S: RpcServiceT,
	{
		type Error = S::Error;
		type Response = S::Response;

		fn call<'a>(
			&self,
			mut request: jsonrpsee::types::Request<'a>,
		) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a  {
			request.extensions_mut().insert(self.connection_id);
			self.inner.call(request)
		}

		fn batch<'a>(
			&self,
			batch: Batch<'a>,
		) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a  {
			self.inner.batch(batch)
		}

		fn notification<'a>(
			&self,
			n: Notification<'a>,
		) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a  {
			self.inner.notification(n)
		}
	}

	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _, _| "hello").unwrap();
	module.register_method("get_connection_id", |_, _, ext| *ext.get::<u32>().unwrap()).unwrap();
	module
		.register_method("get_available_connections", |_, _, ext| {
			ext.get::<ConnectionGuard>().unwrap().available_connections()
		})
		.unwrap();
	module
		.register_method("get_max_connections", |_, _, ext| ext.get::<ConnectionGuard>().unwrap().max_connections())
		.unwrap();

	module
		.register_async_method("slow_hello", |_, _, _| async {
			tokio::time::sleep(std::time::Duration::from_secs(1)).await;
			"hello"
		})
		.unwrap();

	struct CustomError;

	impl From<CustomError> for ErrorObjectOwned {
		fn from(_: CustomError) -> Self {
			ErrorObject::owned(-32001, "err", None::<()>)
		}
	}

	module.register_async_method::<Result<(), CustomError>, _, _>("err", |_, _, _| async { Err(CustomError) }).unwrap();

	let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
	let addr = listener.local_addr().unwrap();
	let (stop_hdl, server_hdl) = stop_channel();
	let methods: Methods = module.into();

	let methods2 = methods.clone();
	tokio::spawn(async move {
		let conn_id = Arc::new(AtomicU32::new(0));
		// Create and finalize a server configuration from a TowerServiceBuilder
		// given an RpcModule and the stop handle.
		let svc_builder = jsonrpsee::server::Server::builder().to_service_builder();

		loop {
			let stream = tokio::select! {
				res = listener.accept() => {
					match res {
						Ok((stream, _remote_addr)) => stream,
						Err(e) => {
							tracing::error!("failed to accept v4 connection: {:?}", e);
							continue;
						}
					}
				}
				_ = stop_hdl.clone().shutdown() => break,
			};

			let methods2 = methods2.clone();
			let stop_hdl2 = stop_hdl.clone();
			let svc_builder2 = svc_builder.clone();
			let conn_id2 = conn_id.clone();
			let svc = tower::service_fn(move |req: hyper::Request<hyper::body::Incoming>| {
				let connection_id = conn_id2.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
				let rpc_middleware = RpcServiceBuilder::default()
					.layer_fn(move |service| ConnectionDetails { inner: service, connection_id });

				// Start a new service with our own connection ID.
				let mut tower_service = svc_builder2
					.clone()
					.set_rpc_middleware(rpc_middleware)
					.connection_id(connection_id)
					.build(methods2.clone(), stop_hdl2.clone());

				async move { tower_service.call(req).await }
			});

			// Spawn a new task to serve each respective (Hyper) connection.
			tokio::spawn(serve_with_graceful_shutdown(stream, svc, stop_hdl.clone().shutdown()));
		}
	});

	tokio::spawn(server_hdl.stopped());
	addr
}

/// Yields one item then sleeps for an hour.
pub async fn server_with_sleeping_subscription(tx: futures::channel::mpsc::Sender<()>) -> SocketAddr {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr = server.local_addr().unwrap();

	let mut module = RpcModule::new(tx);

	module
		.register_subscription("subscribe_sleep", "n", "unsubscribe_sleep", |_, pending, mut tx, _| async move {
			let interval = interval(Duration::from_secs(60 * 60));
			let stream = IntervalStream::new(interval).zip(futures::stream::iter(1..=5)).map(|(_, c)| c);

			let res = pipe_from_stream_and_drop(pending, stream).await;

			let send_back = std::sync::Arc::make_mut(&mut tx);
			send_back.send(()).await.unwrap();

			res.map_err(Into::into)
		})
		.unwrap();
	let handle = server.start(module);

	tokio::spawn(handle.stopped());

	addr
}

pub async fn server_with_health_api() -> (SocketAddr, ServerHandle) {
	server_with_cors(CorsLayer::new()).await
}

pub async fn server_with_cors(cors: CorsLayer) -> (SocketAddr, ServerHandle) {
	let middleware = tower::ServiceBuilder::new()
		// Proxy `GET /health` requests to internal `system_health` method.
		.layer(ProxyGetRequestLayer::new([("/health", "system_health")]).unwrap())
		// Add `CORS` layer.
		.layer(cors);

	let server = Server::builder().set_http_middleware(middleware).build("127.0.0.1:0").await.unwrap();
	let mut module = RpcModule::new(());
	let addr = server.local_addr().unwrap();
	module.register_method("say_hello", |_, _, _| "hello").unwrap();
	module.register_method("notif", |_, _, _| "").unwrap();

	module.register_method("system_health", |_, _, _| serde_json::json!({ "health": true })).unwrap();

	let handle = server.start(module);
	(addr, handle)
}

pub fn init_logger() {
	let _ = tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init();
}

pub async fn pipe_from_stream_and_drop<T: Serialize>(
	pending: PendingSubscriptionSink,
	mut stream: impl Stream<Item = T> + Unpin,
) -> Result<(), anyhow::Error> {
	let mut sink = pending.accept().await?;

	loop {
		tokio::select! {
			_ = sink.closed() => return Err(anyhow::anyhow!("Subscription was closed")),
			maybe_item = stream.next() => {
				let item = match maybe_item {
					Some(item) => item,
					None => return Err(anyhow::anyhow!("Subscription executed successful")),
				};
				let msg = SubscriptionMessage::from_json(&item)?;

				match sink.try_send(msg) {
					Ok(_) => (),
					Err(TrySendError::Closed(_)) => return Err(anyhow::anyhow!("Subscription executed successful")),
					// channel is full, let's be naive an just drop the message.
					Err(TrySendError::Full(_)) => (),
				}
			}
		}
	}
}

pub async fn socks_server_no_auth() -> SocketAddr {
	let mut config = server::Config::default();
	config.set_dns_resolve(false);
	let config = std::sync::Arc::new(config);

	let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
	let proxy_addr = listener.local_addr().unwrap();

	spawn_socks_server(listener, config).await;

	proxy_addr
}

pub async fn spawn_socks_server(listener: tokio::net::TcpListener, config: std::sync::Arc<server::Config>) {
	let addr = listener.local_addr().unwrap();
	tokio::spawn(async move {
		loop {
			let (stream, _) = listener.accept().await.unwrap();
			let mut socks5_socket = server::Socks5Socket::new(stream, config.clone());
			socks5_socket.set_reply_ip(addr.ip());

			socks5_socket.upgrade_to_socks5().await.unwrap();
		}
	});
}

pub async fn connect_over_socks_stream(server_addr: SocketAddr) -> Socks5Stream<TcpStream> {
	let target_addr = server_addr.ip().to_string();
	let target_port = server_addr.port();

	let socks_server = socks_server_no_auth().await;

	fast_socks5::client::Socks5Stream::connect(
		socks_server,
		target_addr,
		target_port,
		fast_socks5::client::Config::default(),
	)
	.await
	.unwrap()
}
