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
use std::time::Instant;

use jsonrpsee::core::{client::Client, middleware};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};

#[derive(Clone)]
struct Timings;

impl middleware::Middleware for Timings {
	type Instant = Instant;

	fn on_request(&self) -> Self::Instant {
		Instant::now()
	}

	fn on_call(&self, name: &str) {
		println!("[Middleware::on_call] '{}'", name);
	}

	fn on_result(&self, name: &str, succeess: bool, started_at: Self::Instant) {
		println!("[Middleware::on_result] '{}', worked? {}, time elapsed {:?}", name, succeess, started_at.elapsed());
	}

	fn on_response(&self, started_at: Self::Instant) {
		println!("[Middleware::on_response] time elapsed {:?}", started_at.elapsed());
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
	println!("response: {:?}", response);
	let _response: Result<String, _> = client.request("unknown_method", None).await;
	let _ = client.request::<String>("say_hello", None).await?;

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = WsServerBuilder::new().set_middleware(Timings).build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| Ok("lo"))?;
	let addr = server.local_addr()?;
	server.start(module)?;
	Ok(addr)
}
