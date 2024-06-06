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

use jsonrpsee::core::client::ClientT;
use jsonrpsee::server::{serve_with_graceful_shutdown, stop_channel, ServerHandle};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{rpc_params, RpcModule};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()?
		.add_directive("jsonrpsee[method_call{name = \"say_hello\"}]=trace".parse()?)
		.add_directive("jsonrpsee-client=trace".parse()?);

	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

	let (_server_hdl, addrs) = run_server().await?;
	let url_v4 = format!("ws://{}", addrs.v4);
	let url_v6 = format!("ws://{}", addrs.v6);

	let client_v4 = WsClientBuilder::default().build(&url_v4).await?;
	let client_v6 = WsClientBuilder::default().build(&url_v6).await?;

	let response_v4: String = client_v4.request("say_hello", rpc_params![]).await?;
	let response_v6: String = client_v6.request("say_hello", rpc_params![]).await?;

	tracing::info!("response V4: {:?}", response_v4);
	tracing::info!("response V6: {:?}", response_v6);

	Ok(())
}

async fn run_server() -> anyhow::Result<(ServerHandle, Addrs)> {
	let port = 9944;
	// V4 address
	let v4_addr = SocketAddr::from(([127, 0, 0, 1], port));
	// V6 address
	let v6_addr = SocketAddr::new("::1".parse().unwrap(), port);

	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _, _| "lo")?;

	// Bind to both IPv4 and IPv6 addresses.
	let listener_v4 = TcpListener::bind(&v4_addr).await?;
	let listener_v6 = TcpListener::bind(&v6_addr).await?;

	// Each RPC call/connection get its own `stop_handle`
	// to able to determine whether the server has been stopped or not.
	//
	// To keep the server running the `server_handle`
	// must be kept and it can also be used to stop the server.
	let (stop_hdl, server_hdl) = stop_channel();

	// Create and finalize a server configuration from a TowerServiceBuilder
	// given an RpcModule and the stop handle.
	let svc = jsonrpsee::server::Server::builder().to_service_builder().build(module, stop_hdl.clone());

	tokio::spawn(async move {
		loop {
			// The `tokio::select!` macro is used to wait for either of the
			// listeners to accept a new connection or for the server to be
			// stopped.
			let stream = tokio::select! {
				res = listener_v4.accept() => {
					match res {
						Ok((stream, _remote_addr)) => stream,
						Err(e) => {
							tracing::error!("failed to accept v4 connection: {:?}", e);
							continue;
						}
					}
				}
				res = listener_v6.accept() => {
					match res {
						Ok((stream, _remote_addr)) => stream,
						Err(e) => {
							tracing::error!("failed to accept v6 connection: {:?}", e);
							continue;
						}
					}
				}
				_ = stop_hdl.clone().shutdown() => break,
			};

			// Spawn a new task to serve each respective (Hyper) connection.
			tokio::spawn(serve_with_graceful_shutdown(stream, svc.clone(), stop_hdl.clone().shutdown()));
		}
	});

	Ok((server_hdl, Addrs { v4: v4_addr, v6: v6_addr }))
}

struct Addrs {
	v4: SocketAddr,
	v6: SocketAddr,
}
