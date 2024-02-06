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

use jsonrpsee::core::client::ClientT;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::Server;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{rpc_params, ResponsePayload};

#[rpc(client, server, namespace = "state")]
pub trait Rpc {
	/// Async method call example.
	#[method(name = "getKeys")]
	fn storage_keys(&self) -> ResponsePayload<'static, String>;
}

pub struct RpcServerImpl;

impl RpcServer for RpcServerImpl {
	fn storage_keys(&self) -> ResponsePayload<'static, String> {
		let (rp, rp_future) = ResponsePayload::success("ehheeheh".to_string()).notify_on_completion();

		tokio::spawn(async move {
			rp_future.await.unwrap();
			println!("Method response to `state_getKeys` finished");
		});

		rp
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let server_addr = run_server().await?;
	let url = format!("ws://{}", server_addr);

	let client = WsClientBuilder::default().build(&url).await?;
	assert_eq!("ehheeheh".to_string(), client.request::<String, _>("state_getKeys", rpc_params![]).await.unwrap());

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = Server::builder().build("127.0.0.1:0").await?;

	let addr = server.local_addr()?;
	let handle = server.start(RpcServerImpl.into_rpc());

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
