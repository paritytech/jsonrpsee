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
//! The particular example disconnects peers that
//! makes more than ten RPC calls and bans the IP addr.
//!
//! NOTE:
//!
//! Enabling tower middleware in this example doesn't work,
//! to do so then the low-level API in hyper must be used.
//!
//! See <https://docs.rs/hyper/latest/hyper/server/conn/index.html>
//! for further information regarding the "low-level API" in hyper.

use std::collections::HashSet;
use std::convert::Infallible;
use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

use futures::future::BoxFuture;
use futures::FutureExt;
use jsonrpsee::core::async_trait;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::middleware::rpc::RpcServiceT;
use jsonrpsee::server::{
	http, serve_with_graceful_shutdown, stop_channel, ws, ConnectionGuard, ConnectionState, RpcServiceBuilder,
	ServerConfig, ServerHandle, StopHandle,
};
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned, Request};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{MethodResponse, Methods};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::Mutex as AsyncMutex;
use tracing_subscriber::util::SubscriberInitExt;

/// This is just a counter to limit
/// the number of calls per connection.
/// Once the limit has been exceeded
/// all future calls are rejected.
#[derive(Clone)]
struct CallLimit<S> {
	service: S,
	count: Arc<AsyncMutex<usize>>,
	state: mpsc::Sender<()>,
}

impl<'a, S> RpcServiceT<'a> for CallLimit<S>
where
	S: Send + Sync + RpcServiceT<'a> + Clone + 'static,
{
	type Future = BoxFuture<'a, MethodResponse>;

	fn call(&self, req: Request<'a>) -> Self::Future {
		let count = self.count.clone();
		let state = self.state.clone();
		let service = self.service.clone();

		async move {
			let mut lock = count.lock().await;

			if *lock >= 10 {
				let _ = state.try_send(());
				MethodResponse::error(req.id, ErrorObject::borrowed(-32000, "RPC rate limit", None))
			} else {
				let rp = service.call(req).await;
				*lock += 1;
				rp
			}
		}
		.boxed()
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
		let handle = run_server().await?;

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
		let handle = run_server().await?;

		let client = HttpClientBuilder::default().build("http://127.0.0.1:9944").unwrap();
		while client.say_hello().await.is_ok() {
			i += 1;
		}
		tracing::info!("HTTP client made {i} successful calls before getting blacklisted");

		handle.stop().unwrap();
		handle.stopped().await;
	}

	Ok(())
}

async fn run_server() -> anyhow::Result<ServerHandle> {
	// Construct our SocketAddr to listen on...
	let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 9944))).await?;

	// Each RPC call/connection get its own `stop_handle`
	// to able to determine whether the server has been stopped or not.
	//
	// To keep the server running the `server_handle`
	// must be kept and it can also be used to stop the server.
	let (stop_handle, server_handle) = stop_channel();

	// This state is cloned for every connection
	// all these types based on Arcs and it should
	// be relatively cheap to clone them.
	//
	// Make sure that nothing expensive is cloned here
	// when doing this or use an `Arc`.
	#[derive(Clone)]
	struct PerConnection {
		methods: Methods,
		stop_handle: StopHandle,
		conn_id: Arc<AtomicU32>,
		conn_guard: ConnectionGuard,
		blacklisted_peers: Arc<Mutex<HashSet<IpAddr>>>,
		// HTTP rate limit that is shared by all connections.
		//
		// This is just a toy-example and one not should "limit" HTTP connections
		// like this because the actual IP addr of each request is not checked.
		//
		// Because it's possible to blacklist a peer which has only made one or
		// a few calls.
		global_http_rate_limit: Arc<AsyncMutex<usize>>,
	}

	let per_conn = PerConnection {
		methods: ().into_rpc().into(),
		stop_handle: stop_handle.clone(),
		conn_id: Default::default(),
		conn_guard: ConnectionGuard::new(100),
		blacklisted_peers: Default::default(),
		global_http_rate_limit: Default::default(),
	};

	tokio::spawn(async move {
		loop {
			// The `tokio::select!` macro is used to wait for either of the
			// listeners to accept a new connection or for the server to be
			// stopped.
			let (sock, remote_addr) = tokio::select! {
				res = listener.accept() => {
					match res {
						Ok(sock) => sock,
						Err(e) => {
							tracing::error!("failed to accept v4 connection: {:?}", e);
							continue;
						}
					}
				}
				_ = per_conn.stop_handle.clone().shutdown() => break,
			};
			let per_conn = per_conn.clone();

			// Create a service handler.
			let stop_handle2 = per_conn.stop_handle.clone();
			let per_conn = per_conn.clone();
			let svc = tower::service_fn(move |req| {
				let PerConnection {
					methods,
					stop_handle,
					conn_guard,
					conn_id,
					blacklisted_peers,
					global_http_rate_limit,
				} = per_conn.clone();

				// jsonrpsee expects a `conn permit` for each connection.
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
					let (tx, mut disconnect) = mpsc::channel(1);
					let rpc_service = RpcServiceBuilder::new().layer_fn(move |service| CallLimit {
						service,
						count: Default::default(),
						state: tx.clone(),
					});

					let conn = ConnectionState::new(stop_handle, conn_id.fetch_add(1, Ordering::Relaxed), conn_permit);

					// Establishes the websocket connection
					// and if the `CallLimit` middleware triggers the hard limit
					// then the connection is closed i.e, the `conn_fut` is dropped.
					async move {
						match ws::connect(req, ServerConfig::default(), methods, conn, rpc_service).await {
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

					let rpc_service = RpcServiceBuilder::new().layer_fn(move |service| CallLimit {
						service,
						count: global_http_rate_limit.clone(),
						state: tx.clone(),
					});

					let server_cfg = ServerConfig::default();
					let conn = ConnectionState::new(stop_handle, conn_id.fetch_add(1, Ordering::Relaxed), conn_permit);

					// There is another API for making call with just a service as well.
					//
					// See [`jsonrpsee::server::http::call_with_service`]
					async move {
						tokio::select! {
							// Rpc call finished successfully.
							res = http::call_with_service_builder(req, server_cfg, conn, methods, rpc_service) => Ok(res),
							// Deny the call if the call limit is exceeded.
							_ = disconnect.recv() => Ok(http::response::denied()),
						}
					}
					.boxed()
				} else {
					async { Ok(http::response::denied()) }.boxed()
				}
			});

			// Upgrade the connection to a HTTP service.
			tokio::spawn(serve_with_graceful_shutdown(sock, svc, stop_handle2.shutdown()));
		}
	});

	Ok(server_handle)
}
