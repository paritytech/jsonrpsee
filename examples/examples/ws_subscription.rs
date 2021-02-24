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
use jsonrpsee_client::Subscription;
use jsonrpsee_types::jsonrpc::{JsonValue, Params};
use jsonrpsee_ws_server::WsServer;
use tokio::task;

const SOCK_ADDR: &str = "127.0.0.1:9966";
const SERVER_URI: &str = "ws://localhost:9966";
const NUM_SUBSCRIPTION_RESPONSES: usize = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();

	let (server_started_tx, server_started_rx) = oneshot::channel::<()>();
	let _server = task::spawn(async move {
		run_server(server_started_tx, SOCK_ADDR).await;
	});

	server_started_rx.await?;
	let client = jsonrpsee_client::ws(SERVER_URI).await;
	let mut subscribe_hello: Subscription<JsonValue> =
		client.subscribe("subscribe_hello", Params::None, "unsubscribe_hello").await?;

	let mut i = 0;
	while i <= NUM_SUBSCRIPTION_RESPONSES {
		let r = subscribe_hello.next().await;
		log::debug!("received {:?}", r);
		i += 1;
	}

	Ok(())
}

async fn run_server(server_started_tx: Sender<()>, url: &str) {
	let mut server = WsServer::new(url).await.unwrap();

	let mut subscription = server.register_subscription("subscribe_hello", "unsubscribe_hello").unwrap();

	std::thread::spawn(move || loop {
		subscription.send(&"hello my friend").unwrap();
		std::thread::sleep(std::time::Duration::from_secs(1));
	});

	server_started_tx.send(()).unwrap();

	server.start().await;
}
