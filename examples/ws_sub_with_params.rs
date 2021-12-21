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

use jsonrpsee::core::client::SubscriptionClientT;
use jsonrpsee::rpc_params;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;

	// Subscription with a single parameter
	let mut sub_params_one =
		client.subscribe::<Option<char>>("sub_one_param", rpc_params![3], "unsub_one_param").await?;
	tracing::info!("subscription with one param: {:?}", sub_params_one.next().await);

	// Subscription with multiple parameters
	let mut sub_params_two =
		client.subscribe::<String>("sub_params_two", rpc_params![2, 5], "unsub_params_two").await?;
	tracing::info!("subscription with two params: {:?}", sub_params_two.next().await);

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	const LETTERS: &str = "abcdefghijklmnopqrstuvxyz";
	let server = WsServerBuilder::default().build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module
		.register_subscription("sub_one_param", "sub_one_param", "unsub_one_param", |params, mut sink, _| {
			let idx: usize = params.one()?;
			std::thread::spawn(move || loop {
				let _ = sink.send(&LETTERS.chars().nth(idx));
				std::thread::sleep(std::time::Duration::from_millis(50));
			});
			Ok(())
		})
		.unwrap();
	module
		.register_subscription("sub_params_two", "params_two", "unsub_params_two", |params, mut sink, _| {
			let (one, two): (usize, usize) = params.parse()?;
			std::thread::spawn(move || loop {
				let _ = sink.send(&LETTERS[one..two].to_string());
				std::thread::sleep(std::time::Duration::from_millis(100));
			});
			Ok(())
		})
		.unwrap();

	let addr = server.local_addr()?;
	server.start(module)?;
	Ok(addr)
}
