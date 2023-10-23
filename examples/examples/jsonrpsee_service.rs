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

use std::error::Error as StdError;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use jsonrpsee::core::{async_trait, client::ClientT};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::rpc_params;
use jsonrpsee::server::middleware::rpc::RpcServiceBuilder;
use jsonrpsee::server::{ConnectionGuard, StopHandle, TowerService};
use jsonrpsee::types::ErrorObjectOwned;

use hyper::server::conn::AddrStream;
use tower::Service;
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

	tokio::spawn(run_server());

	let client = HttpClientBuilder::default().build("http://127.0.0.1:9944").unwrap();

	let x: String = client.request("say_hello", rpc_params!()).await.unwrap();
	tracing::info!("response: {x}");

	Ok(())
}

async fn run_server() {
	use hyper::service::{make_service_fn, service_fn};

	// Construct our SocketAddr to listen on...
	let addr = SocketAddr::from(([127, 0, 0, 1], 9944));

	// Maybe we want to be able to stop our server but not added here.
	let (_tx, rx) = tokio::sync::watch::channel(());

	let stop_handle = StopHandle::new(rx);
	let conn_guard = ConnectionGuard::new(100);
	let service_cfg = jsonrpsee::server::Server::builder().to_service(().into_rpc());
	let conn_id = Arc::new(AtomicU32::new(0));

	// And a MakeService to handle each connection...
	let make_service = make_service_fn(|conn: &AddrStream| {
		// You may use `conn` or the actual HTTP request to deny a certain peer.

		// Connection state.
		let conn_id = conn_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
		let remote_addr = conn.remote_addr();
		let stop_handle = stop_handle.clone();
		let conn_permit = Arc::new(conn_guard.try_acquire().unwrap());
		let service_cfg = service_cfg.clone();

		async move {
			let stop_handle = stop_handle.clone();
			let conn_permit = conn_permit.clone();
			let service_cfg = service_cfg.clone();

			Ok::<_, Box<dyn StdError + Send + Sync>>(service_fn(move |req| {
				let mut svc = TowerService::new(
					service_cfg.clone(),
					RpcServiceBuilder::new(),
					remote_addr,
					conn_permit.clone(),
					stop_handle.clone(),
					conn_id,
				);

				async move { svc.call(req).await }
			}))
		}
	});

	// Then bind and serve...
	let server = hyper::Server::bind(&addr).serve(make_service);

	server.await.unwrap();
}
