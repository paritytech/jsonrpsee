// Copyright 2019 Parity Technologies (UK) Ltd.
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
	ws_client::{traits::SubscriptionClient, v2::params::JsonRpcParams, Subscription, WsClientBuilder},
	ws_server::{WsServer, InnerSubSinkParams},
};
use std::net::SocketAddr;
use serde_json::value::Value as JsonValue;

const NUM_SUBSCRIPTION_RESPONSES: usize = 10;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();
	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;
	let mut subscribe_hello: Subscription<String> =
		client.subscribe("subscribe_hello", JsonRpcParams::NoParams, "unsubscribe_hello").await?;

	let param: JsonValue = 3.into();
	let params = JsonRpcParams::Array(vec![param]);
	let mut sub_params = client.subscribe::<Option<char>>("sub_params", params, "unsub_params").await?;
	println!("subscription with params: {:?}", sub_params.next().await);
	let mut i = 0;
	while i <= NUM_SUBSCRIPTION_RESPONSES {
		let r = subscribe_hello.next().await;
		log::debug!("received {:?}", r);
		i += 1;
	}

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let mut server = WsServer::new("127.0.0.1:0").await?;
	let mut subscription = server.register_subscription("subscribe_hello", "unsubscribe_hello").unwrap();
	let mut subscription2 = server.register_subscription_with_params("sub_params", "unsub_params").unwrap();

	std::thread::spawn(move || loop {
		subscription.send(&"hello my friend").unwrap();
		std::thread::sleep(std::time::Duration::from_secs(1));
	});

	std::thread::spawn(move || loop {
		const LETTERS: &'static str = "abcdefghijklmnopqrstuvxyz";
		subscription2.next().and_then(|inner_sub_sink_params: InnerSubSinkParams<usize>| {
			let idx = *inner_sub_sink_params.params();
			let result = LETTERS.chars().nth(idx);
			// inner_sub_sink_params.send(&result);
			result
		});
		std::thread::sleep(std::time::Duration::from_millis(100));
	});

	let addr = server.local_addr();
	tokio::spawn(async move { server.start().await });
	addr
}
