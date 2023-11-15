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

use std::net::SocketAddr;
use std::time::Duration;

use fast_socks5::client::Socks5Stream;
use fast_socks5::server;
use futures::{AsyncRead, AsyncWrite, SinkExt, Stream, StreamExt};
use jsonrpsee::core::Error;
use jsonrpsee::server::middleware::http::ProxyGetRequestLayer;
use jsonrpsee::server::{
	PendingSubscriptionSink, RpcModule, Server, ServerBuilder, ServerHandle, SubscriptionMessage, TrySendError,
};
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned};
use jsonrpsee::SubscriptionCloseResponse;
use pin_project::pin_project;
use serde::Serialize;
use tokio::net::TcpStream;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use tower_http::cors::CorsLayer;

#[allow(dead_code)]
pub async fn server_with_subscription_and_handle() -> (SocketAddr, ServerHandle) {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();

	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| "hello").unwrap();

	module
		.register_subscription("subscribe_hello", "subscribe_hello", "unsubscribe_hello", |_, pending, _| async move {
			let interval = interval(Duration::from_millis(50));
			let stream = IntervalStream::new(interval).map(move |_| &"hello from subscription");
			pipe_from_stream_and_drop(pending, stream).await.map_err(Into::into)
		})
		.unwrap();

	module
		.register_subscription("subscribe_foo", "subscribe_foo", "unsubscribe_foo", |_, pending, _| async {
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
			|params, pending, _| async move {
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
		.register_subscription("subscribe_noop", "subscribe_noop", "unsubscribe_noop", |_, pending, _| async {
			let _sink = pending.accept().await?;
			tokio::time::sleep(Duration::from_secs(1)).await;
			Err(Error::Custom("Server closed the stream because it was lazy".to_string()).into())
		})
		.unwrap();

	module
		.register_subscription("subscribe_5_ints", "n", "unsubscribe_5_ints", |_, pending, _| async move {
			let interval = interval(Duration::from_millis(50));
			let stream = IntervalStream::new(interval).zip(futures::stream::iter(1..=5)).map(|(_, c)| c);
			pipe_from_stream_and_drop(pending, stream).await.map_err(Into::into)
		})
		.unwrap();

	module
		.register_subscription("subscribe_option", "n", "unsubscribe_option", |_, pending, _| async move {
			let _ = pending.accept().await;
			SubscriptionCloseResponse::None
		})
		.unwrap();

	module
		.register_subscription("subscribe_unit", "n", "unubscribe_unit", |_, pending, _| async move {
			let _sink = pending.accept().await?;
			tokio::time::sleep(std::time::Duration::from_millis(100)).await;
			Ok(())
		})
		.unwrap();

	let addr = server.local_addr().unwrap();
	let server_handle = server.start(module);

	(addr, server_handle)
}

#[allow(dead_code)]
pub async fn server_with_subscription() -> SocketAddr {
	let (addr, handle) = server_with_subscription_and_handle().await;

	tokio::spawn(handle.stopped());

	addr
}

#[allow(dead_code)]
pub async fn server() -> SocketAddr {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| "hello").unwrap();

	module
		.register_async_method("slow_hello", |_, _| async {
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

	module.register_async_method::<Result<(), CustomError>, _, _>("err", |_, _| async { Err(CustomError) }).unwrap();

	let addr = server.local_addr().unwrap();

	let server_handle = server.start(module);

	tokio::spawn(server_handle.stopped());

	addr
}

/// Yields one item then sleeps for an hour.
#[allow(dead_code)]
pub async fn server_with_sleeping_subscription(tx: futures::channel::mpsc::Sender<()>) -> SocketAddr {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr = server.local_addr().unwrap();

	let mut module = RpcModule::new(tx);

	module
		.register_subscription("subscribe_sleep", "n", "unsubscribe_sleep", |_, pending, mut tx| async move {
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

#[allow(dead_code)]
pub async fn server_with_health_api() -> (SocketAddr, ServerHandle) {
	server_with_cors(CorsLayer::new()).await
}

pub async fn server_with_cors(cors: CorsLayer) -> (SocketAddr, ServerHandle) {
	let middleware = tower::ServiceBuilder::new()
		// Proxy `GET /health` requests to internal `system_health` method.
		.layer(ProxyGetRequestLayer::new("/health", "system_health").unwrap())
		// Add `CORS` layer.
		.layer(cors);

	let server = Server::builder().set_http_middleware(middleware).build("127.0.0.1:0").await.unwrap();
	let mut module = RpcModule::new(());
	let addr = server.local_addr().unwrap();
	module.register_method("say_hello", |_, _| "hello").unwrap();
	module.register_method("notif", |_, _| "").unwrap();

	module.register_method("system_health", |_, _| serde_json::json!({ "health": true })).unwrap();

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

#[allow(dead_code)]
pub async fn socks_server_no_auth() -> SocketAddr {
	let mut config = server::Config::default();
	config.set_dns_resolve(false);
	let config = std::sync::Arc::new(config);

	let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
	let proxy_addr = listener.local_addr().unwrap();

	spawn_socks_server(listener, config).await;

	proxy_addr
}

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[pin_project(project = DataStreamProj)]
#[allow(dead_code)]
pub enum DataStream<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + std::marker::Unpin> {
	Socks5(#[pin] Socks5Stream<T>),
}

impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + std::marker::Unpin> AsyncRead for DataStream<T> {
	fn poll_read(
		self: std::pin::Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
		buf: &mut [u8],
	) -> std::task::Poll<std::io::Result<usize>> {
		match self.project() {
			DataStreamProj::Socks5(s) => {
				let compat = s.compat();
				futures_util::pin_mut!(compat);
				AsyncRead::poll_read(compat, cx, buf)
			}
		}
	}
}

impl<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + std::marker::Unpin> AsyncWrite for DataStream<T> {
	fn poll_write(
		self: std::pin::Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
		buf: &[u8],
	) -> std::task::Poll<std::io::Result<usize>> {
		match self.project() {
			DataStreamProj::Socks5(s) => {
				let compat = s.compat_write();
				futures_util::pin_mut!(compat);
				AsyncWrite::poll_write(compat, cx, buf)
			}
		}
	}

	fn poll_flush(
		self: std::pin::Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> std::task::Poll<std::io::Result<()>> {
		match self.project() {
			DataStreamProj::Socks5(s) => {
				let compat = s.compat_write();
				futures_util::pin_mut!(compat);
				AsyncWrite::poll_flush(compat, cx)
			}
		}
	}

	fn poll_close(
		self: std::pin::Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> std::task::Poll<std::io::Result<()>> {
		match self.project() {
			DataStreamProj::Socks5(s) => {
				let compat = s.compat_write();
				futures_util::pin_mut!(compat);
				AsyncWrite::poll_close(compat, cx)
			}
		}
	}
}
