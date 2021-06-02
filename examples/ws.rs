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
	ws_client::{traits::Client, v2::params::JsonRpcParams, WsClientBuilder},
	ws_server::{RpcModule, WsServerBuilder},
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();
	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;
	let response: String = client.request("say_hello", JsonRpcParams::NoParams).await?;
	println!("r: {:?}", response);

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let mut server = WsServerBuilder::default().build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| Ok("lo"))?;
	server.register_module(module).unwrap();
	let addr = server.local_addr()?;
	tokio::spawn(async move { server.start().await });
	Ok(addr)
}
