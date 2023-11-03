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

//! jsonrpsee supports two kinds of middlewares `http_middleware` and `rpc_middleware`.
//!
//! This example demonstrates how to use the `http_middleware` which applies for each
//! HTTP request.
//!
//! A typical use-case for this it to apply a specific CORS policy which applies both
//! for HTTP and WebSocket.
//!

use hyper::body::Bytes;
use hyper::http::HeaderValue;
use hyper::Method;
use jsonrpsee::rpc_params;
use std::iter::once;
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tower_http::sensitive_headers::SetSensitiveRequestHeadersLayer;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;

use jsonrpsee::core::client::ClientT;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::server::{RpcModule, Server};
use jsonrpsee::ws_client::WsClientBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;

	// WebSocket.
	{
		let client = WsClientBuilder::default().build(format!("ws://{}", addr)).await?;
		let response: String = client.request("say_hello", rpc_params![]).await?;
		println!("[main]: ws response: {:?}", response);
		let _response: Result<String, _> = client.request("unknown_method", rpc_params![]).await;
		let _ = client.request::<String, _>("say_hello", rpc_params![]).await?;
	}

	tokio::time::sleep(std::time::Duration::from_secs(1)).await;

	// HTTP.
	{
		let client = HttpClientBuilder::default().build(format!("http://{}", addr))?;
		let response: String = client.request("say_hello", rpc_params![]).await?;
		println!("[main]: http response: {:?}", response);
		let _response: Result<String, _> = client.request("unknown_method", rpc_params![]).await;
		let _ = client.request::<String, _>("say_hello", rpc_params![]).await?;
	}

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let cors = CorsLayer::new()
		// Allow `POST` when accessing the resource
		.allow_methods([Method::POST])
		// Allow requests from any origin
		.allow_origin(HeaderValue::from_str("http://example.com").unwrap())
		.allow_headers([hyper::header::CONTENT_TYPE]);

	// Custom tower service to handle the RPC requests
	let service_builder = tower::ServiceBuilder::new()
		// Add high level tracing/logging to all requests
		.layer(
			TraceLayer::new_for_http()
				.on_request(
					|request: &hyper::Request<hyper::Body>, _span: &tracing::Span| tracing::info!(request = ?request, "on_request"),
				)
				.on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
					tracing::info!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
				})
				.make_span_with(DefaultMakeSpan::new().include_headers(true))
				.on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),
		)
		// Mark the `Authorization` request header as sensitive so it doesn't show in logs
		.layer(SetSensitiveRequestHeadersLayer::new(once(hyper::header::AUTHORIZATION)))
		.layer(cors)
		.timeout(Duration::from_secs(2));

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
