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
	types::{traits::SubscriptionClient, Error, Subscription},
	ws_client::WsClientBuilder,
	ws_server::{RpcModule, WsServerBuilder},
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	const NUM_SUBSCRIPTION_RESPONSES: usize = 5;
	env_logger::init();
	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;
	let mut subscribe_hello: Subscription<String> =
		client.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await?;

	let mut i = 0;
	while i <= NUM_SUBSCRIPTION_RESPONSES {
		let r = subscribe_hello.next().await;
		println!("received {:?}", r);
		i += 1;
	}

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = WsServerBuilder::default().build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_subscription("subscribe_hello", "unsubscribe_hello", |_, mut sink, _| {
		std::thread::spawn(move || loop {
			if let Err(Error::SubscriptionClosed(_)) = sink.send(&"hello my friend") {
				return;
			}
			std::thread::sleep(std::time::Duration::from_secs(1));
		});
		Ok(())
	})?;
	let addr = server.local_addr()?;
	server.start(module)?;
	Ok(addr)
}
