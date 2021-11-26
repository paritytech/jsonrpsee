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

use jsonrpsee::{
	rpc_params,
	types::{traits::SubscriptionClient, Error, JsonValue, Subscription},
	ws_client::*,
	ws_server::{RpcModule, WsServerBuilder},
};
use std::net::SocketAddr;

fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();

	rt.block_on(async {
		let addr = run_server().await.unwrap();
		let ws_addr = format!("ws://{}", addr);

		run_clients(true, ws_addr.clone()).await;
		run_clients(false, ws_addr.clone()).await;
	});

	drop(rt);

	Ok(())
}

async fn run_clients(graceful_shutdown: bool, addr: String) {
	let mut tasks = Vec::new();

	for _ in 0..100 {
		let addr = addr.clone();
		tasks.push(tokio::spawn(async move {
			let (sender, receiver) = WsTransportClientBuilder::default().build(&addr).await.unwrap();
			let client = ClientBuilder::default().build(sender, receiver);
			let mut sub: Subscription<JsonValue> =
				client.subscribe("state_subscribeStorage", rpc_params![], "state_unsubscribeStorage").await.unwrap();
			let n = sub.next().await.unwrap().unwrap();
			tracing::debug!("{:?}", n);

			if !graceful_shutdown {
				drop(client);
			}
		}));
	}

	futures::future::join_all(tasks).await;
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = WsServerBuilder::default().build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_subscription(
		"state_subscribeStorage",
		"s_hello",
		"state_unsubscribeStorage",
		|_, mut sink, _| {
			tokio::spawn(async move {
				let mut i = 0_u64;
				loop {
					if let Err(Error::SubscriptionClosed(_)) = sink.send(&i) {
						return;
					}
					i = i.saturating_add(1);
					tokio::time::sleep(std::time::Duration::from_secs(1)).await;
				}
			});
			Ok(())
		},
	)?;
	let addr = server.local_addr()?;
	server.start(module)?;
	Ok(addr)
}
