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

//! Example that shows how to broadcast to all active subscriptions using `tokio::sync::broadcast`.

use std::net::SocketAddr;

use futures::future;
use futures::StreamExt;
use jsonrpsee::core::client::{Subscription, SubscriptionClientT};
use jsonrpsee::core::error::SubscriptionClosed;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

const NUM_SUBSCRIPTION_RESPONSES: usize = 5;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client1 = WsClientBuilder::default().build(&url).await?;
	let client2 = WsClientBuilder::default().build(&url).await?;
	let sub1: Subscription<i32> = client1.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await?;
	let sub2: Subscription<i32> = client2.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await?;

	let fut1 = sub1.take(NUM_SUBSCRIPTION_RESPONSES).for_each(|r| async move { tracing::info!("sub1 rx: {:?}", r) });
	let fut2 = sub2.take(NUM_SUBSCRIPTION_RESPONSES).for_each(|r| async move { tracing::info!("sub2 rx: {:?}", r) });

	future::join(fut1, fut2).await;

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = WsServerBuilder::default().build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	let (tx, _rx) = broadcast::channel(16);
	let tx2 = tx.clone();

	std::thread::spawn(move || produce_items(tx2));

	module.register_subscription("subscribe_hello", "s_hello", "unsubscribe_hello", move |_, pending, _| {
		let rx = BroadcastStream::new(tx.clone().subscribe());
		let mut sink = match pending.accept() {
			Some(sink) => sink,
			_ => return,
		};

		tokio::spawn(async move {
			sink.pipe_from_try_stream(rx, |reason, sink| match reason {
				SubscriptionClosed::Success => {
					sink.close(SubscriptionClosed::Success);
				}
				SubscriptionClosed::RemotePeerAborted => (),
				SubscriptionClosed::Failed(err) => {
					sink.close(err);
				}
			})
			.await;
		});
	})?;
	let addr = server.local_addr()?;
	server.start(module)?;
	Ok(addr)
}

// Naive example that broadcasts the produced values to all active subscribers.
fn produce_items(tx: broadcast::Sender<usize>) {
	for c in 1..=100 {
		std::thread::sleep(std::time::Duration::from_secs(1));

		// This might fail if no receivers are alive, could occur if no subscriptions are active...
		// Also be aware that this will succeed when at least one receiver is alive
		// Thus, clients connecting at different point in time will not receive
		// the items sent before the subscription got established.
		let _ = tx.send(c);
	}
}
