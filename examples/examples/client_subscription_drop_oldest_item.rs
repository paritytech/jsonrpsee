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

use std::net::SocketAddr;
use std::time::Duration;

use futures::{Stream, StreamExt};
use jsonrpsee::core::DeserializeOwned;
use jsonrpsee::core::client::{Subscription, SubscriptionClientT};
use jsonrpsee::rpc_params;
use jsonrpsee::server::{RpcModule, Server, SubscriptionMessage};
use jsonrpsee::ws_client::WsClientBuilder;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;

	let sub: Subscription<i32> = client.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await?;

	// drop oldest messages from subscription:
	let mut sub = drop_oldest_when_lagging(sub, 10);

	// Simulate that polling takes a long time.
	tokio::time::sleep(Duration::from_secs(1)).await;

	// The subscription starts from zero but you can
	// notice that many items have been replaced
	// because the subscription wasn't polled.
	for _ in 0..10 {
		match sub.next().await.unwrap() {
			Ok(n) => {
				tracing::info!("recv={n}");
			}
			Err(e) => {
				tracing::info!("{e}");
			}
		};
	}

	Ok(())
}

fn drop_oldest_when_lagging<T: Clone + DeserializeOwned + Send + Sync + 'static>(
	mut sub: Subscription<T>,
	buffer_size: usize,
) -> impl Stream<Item = Result<T, BroadcastStreamRecvError>> {
	let (tx, rx) = tokio::sync::broadcast::channel(buffer_size);

	tokio::spawn(async move {
		// Poll the subscription which ignores errors.
		while let Some(n) = sub.next().await {
			let msg = match n {
				Ok(msg) => msg,
				Err(e) => {
					tracing::error!("Failed to decode the subscription message: {e}");
					continue;
				}
			};

			if tx.send(msg).is_err() {
				return;
			}
		}
	});

	BroadcastStream::new(rx)
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = Server::builder().build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module
		.register_subscription("subscribe_hello", "s_hello", "unsubscribe_hello", |_, pending, _, _| async move {
			let sub = pending.accept().await.unwrap();

			for i in 0..usize::MAX {
				let json = serde_json::value::to_raw_value(&i).unwrap();
				let msg = SubscriptionMessage::from_json(json);
				sub.send(msg).await.unwrap();
				tokio::time::sleep(Duration::from_millis(10)).await;
			}

			Ok(())
		})
		.unwrap();
	let addr = server.local_addr()?;

	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
