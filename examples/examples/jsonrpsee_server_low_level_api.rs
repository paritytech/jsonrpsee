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

//! This example shows how to use the low-level server API
//! in jsonrpsee.
//!
//! The particular example disconnects peers that exceeds
//! the rate limit more than ten times and bans IP addr.
//!
//! NOTE:
//!
//! Enabling tower middleware in this example doesn't work,
//! to do so then the low-level API in hyper must be used.

use std::collections::HashSet;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use futures::FutureExt;
use jsonrpsee::core::async_trait;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::middleware::rpc::{RpcServiceT, TransportProtocol};
use jsonrpsee::server::{
	http, ws, ConnectionGuard, Params, PingConfig, RandomIntegerIdProvider, RpcServiceBuilder, ServerHandle, Settings,
	StopHandle,
};
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned, Request};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{MethodResponse, Methods};
use tokio::sync::{Mutex as AsyncMutex, OwnedSemaphorePermit};

use hyper::server::conn::AddrStream;
use tokio::sync::mpsc;
use tracing_subscriber::util::SubscriberInitExt;

/// This is just a counter to limit
/// the number of calls per connection.
/// Once the limit has been exceeded
/// all future calls are rejected.
struct CallLimit<S> {
	service: S,
	count: Arc<AsyncMutex<usize>>,
	state: mpsc::Sender<()>,
}

#[async_trait]
impl<'a, S> RpcServiceT<'a> for CallLimit<S>
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

#[rpc(server, client)]
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
			let rp: Result<String, _> = client.say_hello().await;
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
		while let Ok(_) = client.say_hello().await {
			i += 1;
		}
		tracing::info!("HTTP client made {i} successful calls before getting blacklisted");

		handle.stop().unwrap();
		handle.stopped().await;
	}

	Ok(())
}

fn to_jsonrpsee_params(
	methods: impl Into<Methods>,
	stop_handle: StopHandle,
	conn_id: u32,
	conn_permit: OwnedSemaphorePermit,
) -> Params {
	let cfg = Settings {
		max_connections: 100,
		max_request_body_size: 1024 * 1024,
		max_response_body_size: 1024 * 1024,
		max_subscriptions_per_connection: 128,
		message_buffer_capacity: 1024,
		enable_http: true,
		enable_ws: true,
		tokio_runtime: None,
		ping_config: PingConfig::WithoutInactivityCheck(Duration::from_secs(30)),
		batch_requests_config: jsonrpsee::server::BatchRequestConfig::Disabled,
		id_provider: Arc::new(RandomIntegerIdProvider),
	};

	Params { methods: methods.into(), stop_handle, conn_id, conn_permit: Arc::new(conn_permit), cfg }
}

fn run_server() -> ServerHandle {
	use hyper::service::{make_service_fn, service_fn};

	// Construct our SocketAddr to listen on...
	let addr = SocketAddr::from(([127, 0, 0, 1], 9944));

	// Maybe we want to be able to stop our server but not added here.
	let (tx, rx) = tokio::sync::watch::channel(());

	let stop_handle = StopHandle::new(rx);
	let server_handle = ServerHandle::new(tx);
	let methods = ().into_rpc();
	let conn_guard = ConnectionGuard::new(100);
	let conn_id = Arc::new(AtomicU32::new(0));
	let http_rate_limit = Arc::new(AsyncMutex::new(0));

	// Blacklisted peers
	let blacklisted_peers = Arc::new(Mutex::new(HashSet::new()));
	let stop_handle2 = stop_handle.clone();

	// And a MakeService to handle each connection...
	let make_service = make_service_fn(move |conn: &AddrStream| {
		// Connection state.
		let conn_id = conn_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
		let remote_addr = conn.remote_addr();
		let stop_handle = stop_handle2.clone();
		let conn_guard = conn_guard.clone();
		let methods = methods.clone();
		let blacklisted_peers = blacklisted_peers.clone();
		// HTTP rate limit that is shared by all connections.
		//
		// This is just a toy-example and one not should "limit" HTTP connections
		// like this because the actual IP addr of each request is not checked.
		//
		// Because it's possible to blacklist a peer which has only made one or
		// a few calls.
		let global_http_rate_limit = http_rate_limit.clone();

		async move {
			let stop_handle = stop_handle.clone();
			let conn_guard = conn_guard.clone();
			let methods = methods.clone();
			let stop_handle = stop_handle.clone();
			let blacklisted_peers = blacklisted_peers.clone();
			let global_http_rate_limit = global_http_rate_limit.clone();

			Ok::<_, Infallible>(service_fn(move |req| {
				// jsonrpsee::server::Params expects a `conn permit` for connection.
				//
				// This may be omitted if don't want to limit the number of connections
				// to the server.
				let Some(conn_permit) = conn_guard.try_acquire() else {
					return async { Ok::<_, Infallible>(http::response::too_many_requests()) }.boxed();
				};

				// The IP addr was blacklisted.
				if blacklisted_peers.lock().unwrap().get(&remote_addr.ip()).is_some() {
					return async { Ok(http::response::denied()) }.boxed();
				}

				if ws::is_upgrade_request(&req) {
					let methods = methods.clone();
					let stop_handle = stop_handle.clone();
					let blacklisted_peers = blacklisted_peers.clone();

					let (tx, mut disconnect) = mpsc::channel(1);
					let rpc_service = RpcServiceBuilder::new().layer_fn(move |service| CallLimit {
						service,
						count: Default::default(),
						state: tx.clone(),
					});

					let params = to_jsonrpsee_params(methods.clone(), stop_handle.clone(), conn_id, conn_permit);

					// Establishes the websocket connection
					// and if the `CallLimit` middleware triggers the hard limit
					// then the connection is closed i.e, the `conn_fut` is dropped.
					async move {
						match ws::connect(req, params, rpc_service).await {
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
				} else if !ws::is_upgrade_request(&req) {
					let (tx, mut disconnect) = mpsc::channel(1);

					let global_http_rate_limit = global_http_rate_limit.clone();

					let rpc_service = RpcServiceBuilder::new().layer_fn(move |service| CallLimit {
						service,
						count: global_http_rate_limit.clone(),
						state: tx.clone(),
					});

					let params = to_jsonrpsee_params(methods.clone(), stop_handle.clone(), conn_id, conn_permit);

					// There is another API for making call with just a service as well.
					//
					// See [`jsonrpsee::server::http::call_with_service`]
					async move {
						tokio::select! {
							// Rpc call finished successfully.
							res = http::call_with_service_builder(req, params, rpc_service) => Ok(res),
							// Deny the call if the call limit is exceeded.
							_ = disconnect.recv() => Ok(http::response::denied()),
						}
					}
					.boxed()
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
