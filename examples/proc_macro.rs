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
	http_client::{traits::Client, HttpClientBuilder},
	http_server::HttpServerBuilder,
};
use std::net::SocketAddr;

jsonrpsee::proc_macros::rpc_client_api! {
	RpcApi {
		#[rpc(method = "state_getPairs")]
		fn storage_pairs() -> Vec<u8>;
	}
}

jsonrpsee::proc_macros::rpc_client_api! {
	Registrar {
		#[rpc(method = "say_hello")]
		fn register_para(foo: i32, bar: String);
	}
}

jsonrpsee::proc_macros::rpc_client_api! {
	ManyReturnTypes<A: Send + Sync, B: Send + Sync> {
		#[rpc(method = "say_hello")]
		fn a() -> A;
		fn b() -> B;
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();

	let server_addr = run_server().await?;
	let url = format!("http://{}", server_addr);

	let client = HttpClientBuilder::default().build(url)?;
	let response: Vec<u8> = RpcApi::storage_pairs(&client).await.unwrap();
	println!("r: {:?}", response);

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let mut server = HttpServerBuilder::default().build("127.0.0.1:0".parse()?)?;
	server.register_method("state_getPairs", |_| Ok(vec![1, 2, 3]))?;
	let addr = server.local_addr();
	tokio::spawn(async move { server.start().await });
	addr
}
