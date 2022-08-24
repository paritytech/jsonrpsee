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

//! This example utilizes the `ProxyRequest` layer for redirecting
//! `GET /path` requests to internal RPC methods.
//!
//! The RPC server registers a method named `system_health` which
//! returns `serde_json::Value`. Redirect any `GET /health`
//! requests to the internal method, and return only the method's
//! response in the body (ie, without any jsonRPC 2.0 overhead).
//!
//! # Note
//!
//! This functionality is useful for services which would
//! like to query a certain `URI` path for statistics.

use hyper::{Body, Client, Request};
use std::net::SocketAddr;
use std::time::Duration;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::http_server::middleware::proxy_get_request::ProxyGetRequestLayer;
use jsonrpsee::http_server::{HttpServerBuilder, HttpServerHandle, RpcModule};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let (addr, _handler) = run_server().await?;
	let url = format!("http://{}", addr);

	// Use RPC client to get the response of `say_hello` method.
	let client = HttpClientBuilder::default().build(&url)?;
	let response: String = client.request("say_hello", None).await?;
	println!("[main]: response: {:?}", response);

	// Use hyper client to manually submit a `GET /health` request.
	let http_client = Client::new();
	let uri = format!("http://{}/health", addr);

	let req = Request::builder().method("GET").uri(&uri).body(Body::empty())?;
	println!("[main]: Submit proxy request: {:?}", req);
	let res = http_client.request(req).await?;
	println!("[main]: Received proxy response: {:?}", res);

	// Interpret the response as String.
	let bytes = hyper::body::to_bytes(res.into_body()).await.unwrap();
	let out = String::from_utf8(bytes.to_vec()).unwrap();
	println!("[main]: Interpret proxy response: {:?}", out);
	assert_eq!(out.as_str(), "{\"health\":true}");

	Ok(())
}

async fn run_server() -> anyhow::Result<(SocketAddr, HttpServerHandle)> {
	// Custom tower service to handle the RPC requests
	let service_builder = tower::ServiceBuilder::new()
		// Proxy `GET /health` requests to internal `system_health` method.
		.layer(ProxyGetRequestLayer::new("/health", "system_health")?)
		.timeout(Duration::from_secs(2));

	let server =
		HttpServerBuilder::new().set_middleware(service_builder).build("127.0.0.1:0".parse::<SocketAddr>()?).await?;

	let addr = server.local_addr()?;

	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| Ok("lo")).unwrap();
	module.register_method("system_health", |_, _| Ok(serde_json::json!({ "health": true }))).unwrap();

	let handler = server.start(module)?;

	Ok((addr, handler))
}
