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

//! This examples shows how to use the jsonrpsee::server as
//! a tower service such that one access HTTP related things
//! by launching a hyper::service_fn
//!
//! The typical use-case for this is when one wants to have
//! access to HTTP related things.

use std::error::Error as StdError;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use futures::FutureExt;
use jsonrpsee::core::{async_trait, client::ClientT};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::rpc_params;
use jsonrpsee::server::middleware::rpc::RpcServiceBuilder;
use jsonrpsee::server::{ServerHandle, StopHandle};
use jsonrpsee::types::ErrorObjectOwned;

use hyper::server::conn::AddrStream;
use tower::Service;
use tower_http::cors::CorsLayer;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Default, Clone)]
struct Metrics {
	ws_connections: Arc<AtomicUsize>,
	http_connections: Arc<AtomicUsize>,
}

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

	let handle = run_server();
	tokio::spawn(handle.stopped());

	let client = HttpClientBuilder::default().build("http://127.0.0.1:9944").unwrap();

	let x: String = client.request("say_hello", rpc_params!()).await.unwrap();
	tracing::info!("response: {x}");

	Ok(())
}

fn run_server() -> ServerHandle {
	use hyper::service::{make_service_fn, service_fn};

	// Construct our SocketAddr to listen on...
	let addr = SocketAddr::from(([127, 0, 0, 1], 9944));

	// Maybe we want to be able to stop our server but not added here.
	let (tx, rx) = tokio::sync::watch::channel(());

	let server_handle = ServerHandle::new(tx);
	let stop_handle = StopHandle::new(rx);
	let http_middleware = tower::ServiceBuilder::new().layer(CorsLayer::permissive());
	let svc_builder = jsonrpsee::server::Server::builder()
		.set_http_middleware(http_middleware)
		.set_rpc_middleware(RpcServiceBuilder::new().rpc_logger(1024))
		.to_service_builder()
		.max_connections(33);
	let methods = ().into_rpc();
	let stop_handle2 = stop_handle.clone();
	let metrics = Metrics::default();

	// And a MakeService to handle each connection...
	let make_service = make_service_fn(move |_conn: &AddrStream| {
		// You may use `conn` or the actual HTTP request to get connection related details.

		let stop_handle = stop_handle2.clone();
		let svc_builder = svc_builder.clone();
		let methods = methods.clone();
		let metrics = metrics.clone();

		async move {
			let stop_handle = stop_handle.clone();
			let svc_builder = svc_builder.clone();
			let methods = methods.clone();
			let metrics = metrics.clone();

			Ok::<_, Box<dyn StdError + Send + Sync>>(service_fn(move |req| {
				let metrics = metrics.clone();
				let mut svc = svc_builder.build(methods.clone(), stop_handle.clone());
				async move {
					// You can't determine whether the websocket upgrade handshake failed or not here.
					let is_websocket = jsonrpsee::server::ws::is_upgrade_request(&req);
					let rp = svc.call(req).await;
					if is_websocket {
						metrics.ws_connections.fetch_add(1, Ordering::Relaxed);
					} else {
						metrics.http_connections.fetch_add(1, Ordering::Relaxed);
					}
					rp
				}
				.boxed()
			}))
		}
	});

	let server = hyper::Server::bind(&addr).serve(make_service);

	tokio::spawn(async move {
		let graceful = server.with_graceful_shutdown(async move { stop_handle.shutdown().await });
		graceful.await.unwrap()
	});

	server_handle
}
