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

//! Example that shows how to broadcasts the produced values to all active subscriptions using `tokio::sync::broadcast`.

use std::net::SocketAddr;
use std::time::Duration;

use futures::future;
use futures::StreamExt;
use jsonrpsee::core::client::{Subscription, SubscriptionClientT};
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};

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
	let (tx, rx) = async_broadcast::broadcast(16);

	tokio::spawn(produce_items(tx.clone()));

	module.register_subscription("subscribe_hello", "s_hello", "unsubscribe_hello", move |_, sink, _| {
		let rx = rx.clone();

		tokio::spawn(async move {
			let _ = sink.pipe_from_stream(rx).await;
		});
		Ok(())
	})?;
	let addr = server.local_addr()?;
	server.start(module)?;
	Ok(addr)
}

// Naive example that broadcasts the produced values to all subscribers.
async fn produce_items(tx: async_broadcast::Sender<i32>) {
	let mut i = 0;
	while let Ok(_) = tx.broadcast(i).await {
		i += 1;
		tokio::time::sleep(Duration::from_secs(1)).await;
	}
}
