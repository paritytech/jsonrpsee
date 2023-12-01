// Copyright 2019-2022 Parity Technologies (UK) Ltd.
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

//! This example shows how to configure `host filtering` by tower middleware on the jsonrpsee server.
//!
//! The server whitelist's only `example.com` and any call from localhost will be
//! rejected both by HTTP and WebSocket transports.

use std::net::SocketAddr;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use jsonrpsee::server::middleware::http::HostFilterLayer;
use jsonrpsee::server::{RpcModule, Server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let url = format!("http://{}", addr);

	// Use RPC client to get the response of `say_hello` method.
	let client = HttpClientBuilder::default().build(&url)?;
	// This call will be denied because only `example.com` URIs/hosts are allowed by the host filter.
	let response = client.request::<String, _>("say_hello", rpc_params![]).await.unwrap_err();
	println!("[main]: response: {}", response);

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	// Custom tower service to handle the RPC requests
	let service_builder = tower::ServiceBuilder::new()
		// For this example we only want to permit requests from `example.com`
		// all other request are denied.
		//
		// `HostFilerLayer::new` only fails on invalid URIs..
		.layer(HostFilterLayer::new(["example.com"]).unwrap());

	let server =
		Server::builder().set_http_middleware(service_builder).build("127.0.0.1:0".parse::<SocketAddr>()?).await?;

	let addr = server.local_addr()?;

	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| "lo").unwrap();

	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
