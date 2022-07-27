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

use hyper::Server;
use std::iter::once;
use std::net::SocketAddr;
use std::time::Instant;
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::middleware::{self, Headers, Params};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::http_server::{HttpServerBuilder, RpcModule};

#[derive(Clone)]
struct Timings;

impl middleware::HttpMiddleware for Timings {
	type Instant = Instant;

	fn on_request(&self, remote_addr: SocketAddr, headers: &Headers) -> Self::Instant {
		println!("[Middleware::on_request] remote_addr {}, headers: {:?}", remote_addr, headers);
		Instant::now()
	}

	fn on_call(&self, name: &str, params: Params, kind: middleware::MethodKind) {
		println!("[Middleware::on_call] method: '{}', params: {:?}, kind: {}", name, params, kind);
	}

	fn on_result(&self, name: &str, succeess: bool, started_at: Self::Instant) {
		println!("[Middleware::on_result] '{}', worked? {}, time elapsed {:?}", name, succeess, started_at.elapsed());
	}

	fn on_response(&self, result: &str, started_at: Self::Instant) {
		println!("[Middleware::on_response] result: {}, time elapsed {:?}", result, started_at.elapsed());
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let url = format!("http://{}", addr);
	println!("[main]: URL {:?}", url);

	let client = HttpClientBuilder::default().build(&url)?;
	let response: String = client.request("say_hello", None).await?;
	println!("[main]: response: {:?}", response);
	let _response: Result<String, _> = client.request("unknown_method", None).await;
	let _ = client.request::<String>("say_hello", None).await?;

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| Ok("lo"))?;

	println!("[run_server]: Creating RPC service");
	let rpc_service = HttpServerBuilder::new().set_middleware(Timings).pre_build().to_service(module);

	println!("[run_server]: Tower builder");
	let service_builder = tower::ServiceBuilder::new()
		// Mark the `Authorization` request header as sensitive so it doesn't show in logs
		.layer(SetSensitiveRequestHeadersLayer::new(once(hyper::header::AUTHORIZATION)))
		.service(rpc_service);

	let addr = SocketAddr::from(([127, 0, 0, 1], 9935));

	println!("[run_server]: Bind server");

	tokio::spawn(async move { Server::bind(&addr).serve(service_builder) });

	Ok(addr)
}
