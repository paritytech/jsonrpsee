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

//! Example that shows how to broadcasts the produced values to active all subscriptions using `mpsc channels`.
//!
//! It's possible to use `tokio::sync::broadcast` too but because the Receiver doesn't implement
//! stream thus `mpsc channels` were picked in this example.

use std::{net::SocketAddr, sync::Arc};

use futures::channel::mpsc;
use jsonrpsee::core::client::{Subscription, SubscriptionClientT};
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};
use tokio::sync::Mutex;

const NUM_SUBSCRIPTION_RESPONSES: usize = 5;

/// Sinks that can be shared across threads.
type SharedSinks = Arc<Mutex<Vec<mpsc::UnboundedSender<i32>>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;
	let mut sub: Subscription<i32> = client.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await?;

	let mut i = 0;
	while i <= NUM_SUBSCRIPTION_RESPONSES {
		let r = sub.next().await.unwrap().unwrap();
		tracing::info!("{}", r);
		i += 1;
	}

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = WsServerBuilder::default().build("127.0.0.1:0").await?;
	let sinks = SharedSinks::default();
	let mut module = RpcModule::new(sinks.clone());

	// Produce new items for the server to publish.
	tokio::spawn(produce_items(sinks));

	module.register_subscription("subscribe_hello", "s_hello", "unsubscribe_hello", |_, sink, ctx| {
		let ctx = ctx.clone();
		tokio::spawn(async move {
			let (tx, rx) = mpsc::unbounded();
			ctx.lock().await.push(tx);
			let _ = sink.pipe_from_stream(rx).await;
		});
		Ok(())
	})?;
	let addr = server.local_addr()?;
	server.start(module)?;
	Ok(addr)
}

/// Produce new values that are sent to each active subscription.
async fn produce_items(sinks: SharedSinks) {
	let mut count = 0;
	loop {
		let mut to_remove = Vec::new();

		for (idx, sink) in sinks.lock().await.iter().enumerate() {
			if sink.unbounded_send(count).is_err() {
				to_remove.push(idx);
			}
		}

		// If the channel is closed remove that channel.
		for rm in to_remove {
			sinks.lock().await.remove(rm);
		}

		count += 1;
		tokio::time::sleep(std::time::Duration::from_secs(1)).await;
	}
}
