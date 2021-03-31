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

use std::net::SocketAddr;

use jsonrpsee_http_client::HttpClientBuilder;
use jsonrpsee_http_server::HttpServerBuilder;
use jsonrpsee_types::{jsonrpc::Params, traits::Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();

	let server_addr = run_server().await;
	let url = format!("http://{}", server_addr);

	std::thread::sleep(std::time::Duration::from_secs(1));

	let client = HttpClientBuilder::default().build(url)?;
	let response: Result<String, _> = client.request("say_hello", Params::None).await;
	println!("r: {:?}", response);

	Ok(())
}

async fn run_server() -> SocketAddr {
	let mut server = HttpServerBuilder::default().build("127.0.0.1:0".parse().unwrap()).unwrap();
	server.register_method("say_hello", |_| Ok("lo")).unwrap();
	let addr = server.local_addr().unwrap();
	tokio::spawn(async move { server.start().await.unwrap() });
	addr
}
