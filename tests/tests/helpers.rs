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

use futures::{SinkExt, Stream, StreamExt};
use jsonrpsee::core::Error;
use jsonrpsee::server::middleware::http::ProxyGetRequestLayer;
use jsonrpsee::server::{
	PendingSubscriptionSink, RpcModule, Server, ServerBuilder, ServerHandle, SubscriptionMessage, TrySendError,
};
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned};
use jsonrpsee::SubscriptionCloseResponse;
use serde::Serialize;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
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
