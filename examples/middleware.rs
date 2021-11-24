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
	types::traits::Client,
	utils::server::middleware,
	ws_client::WsClientBuilder,
	ws_server::{RpcModule, WsServerBuilder},
};
use std::net::SocketAddr;
use std::sync::atomic;

#[derive(Default)]
struct ManInTheMiddle {
	when: atomic::AtomicU64,
}

impl Clone for ManInTheMiddle {
	fn clone(&self) -> Self {
		ManInTheMiddle { when: atomic::AtomicU64::new(self.when.load(atomic::Ordering::SeqCst)) }
	}
}

impl middleware::Middleware for ManInTheMiddle {
	type Instant = u64;
	fn on_request(&self) -> Self::Instant {
		self.when.fetch_add(1, atomic::Ordering::SeqCst)
	}

	fn on_call(&self, name: &str) {
		println!("They called '{}'", name);
	}

	fn on_result(&self, name: &str, succeess: bool, started_at: Self::Instant) {
		println!("call={}, worked? {}, when? {}", name, succeess, started_at);
	}

	fn on_response(&self, started_at: Self::Instant) {
		println!("Response started_at={}", started_at);
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;
	let response: String = client.request("say_hello", None).await?;
	tracing::info!("response: {:?}", response);
	// TODO: This prints `They called 'blabla'` but nothing more. I expected the `on_response` callback to be called too?
	let _response: Result<String, _> = client.request("blabla", None).await;
	// TODO: Something's odd here. This prints:
	/*
	They called 'say_hello'
	call=say_hello, worked? true, when? 1
	Response started_at=1
	Error: Parse error: invalid type: string "lo", expected unit

	Caused by:
		invalid type: string "lo", expected unit
	 */
	let _ = client.request("say_hello", None).await?;

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let m = ManInTheMiddle::default();
	let server = WsServerBuilder::default().with_middleware(m).build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| Ok("lo"))?;
	let addr = server.local_addr()?;
	server.start(module)?;
	Ok(addr)
}
