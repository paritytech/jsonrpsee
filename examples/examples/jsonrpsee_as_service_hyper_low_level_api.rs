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

use jsonrpsee::core::{async_trait, client::ClientT};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::rpc_params;
use jsonrpsee::server::middleware::rpc::RpcServiceBuilder;
use jsonrpsee::server::{ServerHandle, StopHandle};
use jsonrpsee::types::ErrorObjectOwned;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing_subscriber::util::SubscriberInitExt;

#[rpc(server)]
pub trait Rpc {
	#[method(name = "say_hello")]
	async fn say_hello(&self) -> Result<String, ErrorObjectOwned>;
}

#[async_trait]
impl RpcServer for () {
	async fn say_hello(&self) -> Result<String, ErrorObjectOwned> {
		Ok("lo".to_string())
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()?
		.add_directive("jsonrpsee[method_call{name = \"say_hello\"}]=trace".parse()?);
	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

	// Construct our SocketAddr to listen on...
	let addr = SocketAddr::from(([127, 0, 0, 1], 9944));
	let listener = TcpListener::bind(addr).await?;

	let handle = run_server(listener);
	tokio::spawn(handle.stopped());

	let client = HttpClientBuilder::default().build("http://127.0.0.1:9944").unwrap();

	let x: String = client.request("say_hello", rpc_params!()).await.unwrap();
	tracing::info!("response: {x}");

	Ok(())
}

fn run_server(listener: TcpListener) -> ServerHandle {
	use hyper::server::conn::Http;

	// Maybe we want to be able to stop our server but not added here.
	let (tx, rx) = tokio::sync::watch::channel(());
	let stop_handle = StopHandle::new(rx);
	// This must held to keep the server running
	let server_handle = ServerHandle::new(tx);

	let methods = ().into_rpc();
	let http_middleware = tower::ServiceBuilder::new().layer(CorsLayer::permissive());
	let svc_builder = jsonrpsee::server::Server::builder()
		.set_http_middleware(http_middleware)
		.set_rpc_middleware(RpcServiceBuilder::new().rpc_logger(1024))
		.to_service_builder()
		.max_connections(33);

	tokio::spawn(async move {
		loop {
			let stream = tokio::select! {
				_ = stop_handle.clone().shutdown() => return,
				next = listener.accept() => {
					match next {
						Ok((stream, _)) => stream,
						Err(e) => {
							tracing::warn!("Connection closed: {e}");
							return;
						}
					}
				}
			};

			let svc = svc_builder.build(methods.clone(), stop_handle.clone());

			tokio::task::spawn(async move {
				if let Err(err) = Http::new().serve_connection(stream, svc).await {
					println!("Failed to serve connection: {:?}", err);
				}
			});
		}
	});

	server_handle
}
