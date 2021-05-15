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
	ws_client::{traits::SubscriptionClient, v2::params::JsonRpcParams, WsClientBuilder},
	ws_server::WsServer,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();
	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;

	// Subscription with a single parameter
	let params = JsonRpcParams::Array(vec![3.into()]);
	let mut sub_params_one = client.subscribe::<Option<char>>("sub_one_param", params, "unsub_one_param").await?;
	println!("subscription with one param: {:?}", sub_params_one.next().await);

	// Subscription with multiple parameters
	let params = JsonRpcParams::Array(vec![2.into(), 5.into()]);
	let mut sub_params_two = client.subscribe::<String>("sub_params_two", params, "unsub_params_two").await?;
	println!("subscription with two params: {:?}", sub_params_two.next().await);

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	const LETTERS: &'static str = "abcdefghijklmnopqrstuvxyz";
	let mut server = WsServer::new("127.0.0.1:0").await?;
	let one_param = server.register_subscription_with_params("sub_one_param", "unsub_one_param").unwrap();
	let two_params = server.register_subscription_with_params("sub_params_two", "unsub_params_two").unwrap();

	std::thread::spawn(move || loop {
		one_param.next().and_then(|inner_sub_sink_params| {
			let idx = *inner_sub_sink_params.params();
			let result = LETTERS.chars().nth(idx);
			let _ = inner_sub_sink_params.send(&result);
			// TODO: why do I need to return something here? Returning "".into() works just as well...
			result
		});
		std::thread::sleep(std::time::Duration::from_millis(50));
	});

	std::thread::spawn(move || loop {
		two_params.next().and_then(|inner_sub_sink_params| {
			let params: &Vec<usize> = inner_sub_sink_params.params();
			// Validate your params here: check len, check > 0 etc
			let result = LETTERS[params[0]..params[1]].to_string();
			let _ = inner_sub_sink_params.send(&result);
			// TODO: why do I need to return something here? Returning `Option::<char>::None` works just as well...
			Some(result)
		});
		std::thread::sleep(std::time::Duration::from_millis(100));
	});

	let addr = server.local_addr();
	tokio::spawn(async move { server.start().await });
	addr
}
