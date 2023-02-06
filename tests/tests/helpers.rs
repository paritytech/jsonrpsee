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

use futures::{SinkExt, StreamExt};
use jsonrpsee::core::server::host_filtering::AllowHosts;
use jsonrpsee::core::{Error, SubscriptionClosed};
use jsonrpsee::server::middleware::proxy_get_request::ProxyGetRequestLayer;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use jsonrpsee::types::error::{ErrorObject, SUBSCRIPTION_CLOSED_WITH_ERROR};
use jsonrpsee::types::ErrorObjectOwned;
use jsonrpsee::RpcModule;
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;
use tower_http::cors::CorsLayer;

#[allow(dead_code)]
pub async fn server_with_subscription_and_handle() -> (SocketAddr, ServerHandle) {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();

	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| Ok("hello")).unwrap();

	module
		.register_subscription("subscribe_hello", "subscribe_hello", "unsubscribe_hello", |_, pending, _| async move {
			let interval = interval(Duration::from_millis(50));
			let stream = IntervalStream::new(interval).map(move |_| &"hello from subscription");

			let mut sink = pending.accept().await?;
			sink.pipe_from_stream(|_, next| next, stream).await;

			Ok(())
		})
		.unwrap();

	module
		.register_subscription("subscribe_foo", "subscribe_foo", "unsubscribe_foo", |_, pending, _| async {
			let interval = interval(Duration::from_millis(100));
			let stream = IntervalStream::new(interval).map(move |_| 1337_usize);

			let mut sink = pending.accept().await?;
			sink.pipe_from_stream(|_, next| next, stream).await;

			Ok(())
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
						let _ = pending.reject(ErrorObjectOwned::from(e)).await;
						return Ok(());
					}
				};

				let wrapping_counter = futures::stream::iter((count..).cycle());
				let interval = interval(Duration::from_millis(100));
				let stream = IntervalStream::new(interval).zip(wrapping_counter).map(move |(_, c)| c);

				let mut sink = pending.accept().await?;
				sink.pipe_from_stream(|_, next| next, stream).await;

				Ok(())
			},
		)
		.unwrap();

	module
		.register_subscription("subscribe_noop", "subscribe_noop", "unsubscribe_noop", |_, pending, _| async {
			let sink = pending.accept().await.unwrap();
			tokio::time::sleep(Duration::from_secs(1)).await;
			let err = ErrorObject::owned(
				SUBSCRIPTION_CLOSED_WITH_ERROR,
				"Server closed the stream because it was lazy",
				None::<()>,
			);
			sink.close(err).await;

			Ok(())
		})
		.unwrap();

	module
		.register_subscription("subscribe_5_ints", "n", "unsubscribe_5_ints", |_, pending, _| async move {
			let interval = interval(Duration::from_millis(50));
			let stream = IntervalStream::new(interval).zip(futures::stream::iter(1..=5)).map(|(_, c)| c);

			let mut sink = pending.accept().await?;
			match sink.pipe_from_stream(|_, next| next, stream).await {
				SubscriptionClosed::Success => {
					sink.close(SubscriptionClosed::Success).await;
				}
				_ => unreachable!(),
			}

			Ok(())
		})
		.unwrap();

	module
		.register_subscription("can_reuse_subscription", "n", "u_can_reuse_subscription", |_, pending, _| async move {
			let stream1 = IntervalStream::new(interval(Duration::from_millis(50)))
				.zip(futures::stream::iter(1..=5))
				.map(|(_, c)| c);
			let stream2 = IntervalStream::new(interval(Duration::from_millis(50)))
				.zip(futures::stream::iter(6..=10))
				.map(|(_, c)| c);

			let mut sink = pending.accept().await?;
			let result = sink.pipe_from_stream(|_, next| next, stream1).await;
			assert!(matches!(result, SubscriptionClosed::Success));

			match sink.pipe_from_stream(|_, next| next, stream2).await {
				SubscriptionClosed::Success => {
					sink.close(SubscriptionClosed::Success).await;
				}
				_ => unreachable!(),
			}

			Ok(())
		})
		.unwrap();

	module
		.register_subscription(
			"subscribe_with_err_on_stream",
			"n",
			"unsubscribe_with_err_on_stream",
			move |_, pending, _| async {
				let err: &'static str = "error on the stream";

				// Create stream that produce an error which will cancel the subscription.
				let stream = futures::stream::iter(vec![Ok(1_u32), Err(err), Ok(2), Ok(3)]);

				let mut sink = pending.accept().await?;
				match sink.pipe_from_try_stream(|_, next| next, stream).await {
					SubscriptionClosed::Failed(e) => {
						sink.close(e).await;
					}
					_ => unreachable!(),
				};

				Ok(())
			},
		)
		.unwrap();

	module
		.register_subscription(
			"subscribe_with_backpressure_aggregation",
			"n",
			"unsubscribe_with_backpressure_aggregation",
			move |_, pending, _| async {
				let sink = pending.accept().await?;
				let mut n = sink.build_message(&1).unwrap();

				loop {
					tokio::select! {
						biased;
						_ = sink.closed() => {
							// User closed connection.
							println!("User closed");
							break;
						},
						res = sink.send(n) => {
							// n back to 1 when message sends
							if res.is_err() {
								break;
							}
							n = sink.build_message(&1).unwrap();
						},
						else => {
							// Every time sending is busy, increment n.
							n = sink.build_message(&2).unwrap();
						}
					}
				}
				Ok(())
			})
			.unwrap();

	let addr = server.local_addr().unwrap();
	let server_handle = server.start(module).unwrap();

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
	module.register_method("say_hello", |_, _| Ok("hello")).unwrap();

	module
		.register_async_method("slow_hello", |_, _| async {
			tokio::time::sleep(std::time::Duration::from_secs(1)).await;
			Result::<_, Error>::Ok("hello")
		})
		.unwrap();

	module.register_async_method("err", |_, _| async { Err::<(), _>(Error::Custom("err".to_string())) }).unwrap();

	let addr = server.local_addr().unwrap();

	let server_handle = server.start(module).unwrap();

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

			let mut sink = pending.accept().await?;
			sink.pipe_from_stream(|_, next| next, stream).await;

			let send_back = std::sync::Arc::make_mut(&mut tx);
			send_back.send(()).await.unwrap();

			Ok(())
		})
		.unwrap();
	let handle = server.start(module).unwrap();

	tokio::spawn(handle.stopped());

	addr
}

#[allow(dead_code)]
pub async fn server_with_health_api() -> (SocketAddr, ServerHandle) {
	server_with_access_control(AllowHosts::Any, CorsLayer::new()).await
}

pub async fn server_with_access_control(allowed_hosts: AllowHosts, cors: CorsLayer) -> (SocketAddr, ServerHandle) {
	let middleware = tower::ServiceBuilder::new()
		// Proxy `GET /health` requests to internal `system_health` method.
		.layer(ProxyGetRequestLayer::new("/health", "system_health").unwrap())
		// Add `CORS` layer.
		.layer(cors);

	let server = ServerBuilder::default()
		.set_host_filtering(allowed_hosts)
		.set_middleware(middleware)
		.build("127.0.0.1:0")
		.await
		.unwrap();
	let mut module = RpcModule::new(());
	let addr = server.local_addr().unwrap();
	module.register_method("say_hello", |_, _| Ok("hello")).unwrap();
	module.register_method("notif", |_, _| Ok("")).unwrap();

	module.register_method("system_health", |_, _| Ok(serde_json::json!({ "health": true }))).unwrap();

	let handle = server.start(module).unwrap();
	(addr, handle)
}

pub fn init_logger() {
	let _ = tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init();
}
