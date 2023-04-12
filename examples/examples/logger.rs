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

use jsonrpsee::core::client::ClientT;
use jsonrpsee::server::logger::{self, HttpRequest, MethodKind, Params, TransportProtocol};
use jsonrpsee::server::ServerBuilder;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{rpc_params, RpcModule};

#[derive(Clone)]
struct Timings;

impl logger::Logger for Timings {
	type Instant = Instant;

	fn on_connect(&self, remote_addr: SocketAddr, request: &HttpRequest, _t: TransportProtocol) {
		println!("[Logger::on_connect] remote_addr {:?}, headers: {:?}", remote_addr, request);
	}

	fn on_request(&self, _t: TransportProtocol) -> Self::Instant {
		println!("[Logger::on_request]");
		Instant::now()
	}

	fn on_call(&self, name: &str, params: Params, kind: MethodKind, _t: TransportProtocol) {
		println!("[Logger::on_call] method: '{}', params: {:?}, kind: {}", name, params, kind);
	}

	fn on_result(&self, name: &str, succeess: bool, started_at: Self::Instant, _t: TransportProtocol) {
		println!("[Logger::on_result] '{}', worked? {}, time elapsed {:?}", name, succeess, started_at.elapsed());
	}

	fn on_response(&self, result: &str, started_at: Self::Instant, _t: TransportProtocol) {
		println!("[Logger::on_response] result: {}, time elapsed {:?}", result, started_at.elapsed());
	}

	fn on_disconnect(&self, remote_addr: SocketAddr, _t: TransportProtocol) {
		println!("[Logger::on_disconnect] remote_addr: {:?}", remote_addr);
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
	let response: String = client.request("say_hello", rpc_params![]).await?;
	println!("response: {:?}", response);
	let _response: Result<String, _> = client.request("unknown_method", rpc_params![]).await;
	let _: String = client.request("say_hello", rpc_params![]).await?;

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = ServerBuilder::new().set_logger(Timings).build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| "lo")?;
	let addr = server.local_addr()?;

	let handle = server.start(module)?;

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
