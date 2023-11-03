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

use std::collections::HashSet;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};

use futures::FutureExt;
use jsonrpsee::core::{async_trait, client::ClientT};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::middleware::rpc::*;
use jsonrpsee::server::{http, ws, ConnectionGuard, ServerHandle, ServiceData, StopHandle};
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned, Request};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{rpc_params, MethodResponse};
use tokio::sync::Mutex as AsyncMutex;

use hyper::server::conn::AddrStream;
use tokio::sync::mpsc;
use tracing_subscriber::util::SubscriberInitExt;

struct DummyRateLimit<S> {
	service: S,
	count: Arc<AsyncMutex<usize>>,
	state: mpsc::Sender<()>,
}

#[async_trait]
impl<'a, S> RpcServiceT<'a> for DummyRateLimit<S>
where
	S: Send + Sync + RpcServiceT<'a>,
{
	async fn call(&self, req: Request<'a>, t: TransportProtocol) -> MethodResponse {
		let mut lock = self.count.lock().await;

		if *lock >= 10 {
			let _ = self.state.try_send(());
			MethodResponse::error(req.id, ErrorObject::borrowed(-32000, "RPC rate limit", None))
		} else {
			let rp = self.service.call(req, t).await;
			*lock = *lock + 1;
			rp
		}
	}
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
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()?;
	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

	// Make a bunch of WebSocket calls to be blacklisted by server.
	{
		let mut i = 0;
		let handle = run_server();

		let client = WsClientBuilder::default().build("ws://127.0.0.1:9944").await.unwrap();
		while client.is_connected() {
			let rp: Result<String, _> = client.request("say_hello", rpc_params!()).await;
			if rp.is_ok() {
				i += 1;
			}
		}

		// After the server has blacklisted the IP address, the connection is denied.
		assert!(WsClientBuilder::default().build("ws://127.0.0.1:9944").await.is_err());
		tracing::info!("WS client made {i} successful calls before getting blacklisted");

		handle.stop().unwrap();
		handle.stopped().await;
	}

	// Make a bunch of HTTP calls to be blacklisted by server.
	{
		let mut i = 0;
		let handle = run_server();

		let client = HttpClientBuilder::default().build("http://127.0.0.1:9944").unwrap();
		while let Ok(_) = client.request::<String, _>("say_hello", rpc_params![]).await {
			i += 1;
		}
		tracing::info!("HTTP client made {i} successful calls before getting blacklisted");

		handle.stop().unwrap();
		handle.stopped().await;
	}

	Ok(())
}

fn run_server() -> ServerHandle {
	use hyper::service::{make_service_fn, service_fn};

	// Construct our SocketAddr to listen on...
	let addr = SocketAddr::from(([127, 0, 0, 1], 9944));

	// Maybe we want to be able to stop our server but not added here.
	let (tx, rx) = tokio::sync::watch::channel(());

	let stop_handle = StopHandle::new(rx);
	let server_handle = ServerHandle::new(tx);

	let service_cfg = jsonrpsee::server::Server::builder().to_service(().into_rpc());
	let conn_guard = Arc::new(ConnectionGuard::new(service_cfg.settings.max_connections as usize));
	let conn_id = Arc::new(AtomicU32::new(0));
	let http_rate_limit = Arc::new(AsyncMutex::new(0));

	// Blacklisted peers
	let blacklisted_peers = Arc::new(Mutex::new(HashSet::new()));
	let stop_handle2 = stop_handle.clone();

	// And a MakeService to handle each connection...
	let make_service = make_service_fn(move |conn: &AddrStream| {
		// You may use `conn` or the actual HTTP request to deny a certain peer.

		// Connection state.
		let conn_id = conn_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
		let remote_addr = conn.remote_addr();
		let stop_handle = stop_handle2.clone();
		let conn_guard = conn_guard.clone();
		let service_cfg = service_cfg.clone();
		let blacklisted_peers = blacklisted_peers.clone();
		let http_rate_limit = http_rate_limit.clone();

		async move {
			let stop_handle = stop_handle.clone();
			let conn_guard = conn_guard.clone();
			let service_cfg = service_cfg.clone();
			let stop_handle = stop_handle.clone();
			let blacklisted_peers = blacklisted_peers.clone();
			let http_rate_limit = http_rate_limit.clone();

			Ok::<_, Infallible>(service_fn(move |req| {
				// Connection number limit exceeded.
				let Some(conn_permit) = conn_guard.try_acquire() else {
					return async { Ok::<_, Infallible>(http::response::too_many_requests()) }.boxed();
				};

				// The IP addr was blacklisted.
				if blacklisted_peers.lock().unwrap().get(&remote_addr.ip()).is_some() {
					return async { Ok(http::response::denied()) }.boxed();
				}

				if ws::is_upgrade_request(&req) && service_cfg.settings.enable_ws {
					let service_cfg = service_cfg.clone();
					let stop_handle = stop_handle.clone();
					let blacklisted_peers = blacklisted_peers.clone();

					let (tx, mut disconnect) = mpsc::channel(1);
					let rpc_service = RpcServiceBuilder::new().layer_fn(move |service| DummyRateLimit {
						service,
						count: Default::default(),
						state: tx.clone(),
					});

					let svc = ServiceData {
						cfg: service_cfg.settings,
						conn_id,
						stop_handle,
						conn_permit: Arc::new(conn_permit),
						methods: service_cfg.methods.clone(),
					};

					// Establishes the websocket connection
					// and if the `DummyRateLimit` middleware triggers the hard limit
					// then the connection is closed i.e, the `conn_fut` is dropped.
					async move {
						match ws::connect(req, svc, rpc_service).await {
							Ok((rp, conn_fut)) => {
								tokio::spawn(async move {
									tokio::select! {
										_ = conn_fut => (),
										_ = disconnect.recv() => {
											blacklisted_peers.lock().unwrap().insert(remote_addr.ip());
										},
									}
								});
								Ok(rp)
							}
							Err(rp) => Ok(rp),
						}
					}
					.boxed()
				} else if !ws::is_upgrade_request(&req) && service_cfg.settings.enable_http {
					let svc = ServiceData {
						cfg: service_cfg.settings.clone(),
						conn_id,
						stop_handle: stop_handle.clone(),
						conn_permit: Arc::new(conn_permit),
						methods: service_cfg.methods.clone(),
					};

					// In this example it doesn't make sense to disconnect the peer in the middleware rate limit
					// is triggered as it will just reply to HTTP request i.e, the "mpsc::Sender disconnect"
					// is not used.
					let (tx, _disconnect) = mpsc::channel(1);

					let http_rate_limit = http_rate_limit.clone();

					let rpc_service = RpcServiceBuilder::new().layer_fn(move |service| DummyRateLimit {
						service,
						count: http_rate_limit.clone(),
						state: tx.clone(),
					});

					// There is another API for making call with just a service as well.
					//
					// See [`jsonrpsee::server::http::call_with_service`]
					async move { http::call_with_service_builder(req, svc, rpc_service).map(Ok).await }.boxed()
				} else {
					async { Ok(http::response::denied()) }.boxed()
				}
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
