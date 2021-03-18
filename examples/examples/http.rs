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

use futures::channel::oneshot::{self, Sender};
use jsonrpsee_http_client::{HttpClient, HttpConfig};
use jsonrpsee_http_server::HttpServer;
use jsonrpsee_types::{
	jsonrpc::{JsonValue, Params},
	traits::Client,
};

const SOCK_ADDR: &str = "127.0.0.1:9933";
const SERVER_URI: &str = "http://localhost:9933";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();

	let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
	let _server = tokio::spawn(async move {
		run_server(server_started_tx, SOCK_ADDR).await;
	});

	std::thread::sleep(std::time::Duration::from_secs(1));

	let client = HttpClient::new(SERVER_URI, HttpConfig::default())?;
	let response: Result<String, _> = client.request("say_hello", Params::None).await;
	println!("r: {:?}", response);

	Ok(())
}

async fn run_server(server_started_tx: Sender<()>, addr: &str) {
	let mut server = HttpServer::new(&addr.parse().unwrap()).await.unwrap();
	server.register_method("say_hello", |_| Ok("lo")).unwrap();
	server.start().await;
}
