// Copyright 2024 Parity Technologies (UK) Ltd.
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
//! in jsonrpsee and inject a `mpsc::Sender<()>` into the
//! request extensions to be able to close the connection from
//! a subscription handler.

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use futures::FutureExt;
use jsonrpsee::core::{async_trait, SubscriptionResult};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::{
	http, serve_with_graceful_shutdown, stop_channel, ws, ConnectionGuard, ConnectionState, HttpRequest,
	RpcServiceBuilder, ServerConfig, ServerHandle, StopHandle,
};
use jsonrpsee::types::ErrorObjectOwned;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{Extensions, Methods, PendingSubscriptionSink};
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tracing_subscriber::util::SubscriberInitExt;

#[rpc(server, client)]
pub trait Rpc {
	#[method(name = "closeConn", with_extensions)]
	async fn close_conn(&self) -> Result<(), ErrorObjectOwned>;

	#[subscription(name = "subscribeCloseConn", item = String, with_extensions)]
	async fn close_conn_from_sub(&self) -> SubscriptionResult;
}

#[async_trait]
impl RpcServer for () {
	async fn close_conn(&self, ext: &Extensions) -> Result<(), ErrorObjectOwned> {
		let tx = ext.get::<mpsc::Sender<()>>().unwrap();
		tx.send(()).await.unwrap();

		Ok(())
	}

	async fn close_conn_from_sub(&self, _pending: PendingSubscriptionSink, ext: &Extensions) -> SubscriptionResult {
		let tx = ext.get::<mpsc::Sender<()>>().unwrap();
		tx.send(()).await.unwrap();

		Ok(())
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()?;
	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

	let handle = run_server().await?;

	{
		let client = WsClientBuilder::default().build("ws://127.0.0.1:9944").await?;
		let _ = client.close_conn().await;
		client.on_disconnect().await;
		eprintln!("Connection closed from RPC call");
	}

	{
		let client = WsClientBuilder::default().build("ws://127.0.0.1:9944").await?;
		let _ = client.close_conn_from_sub().await;
		client.on_disconnect().await;
		eprintln!("Connection closed from RPC subscription");
	}

	let _ = handle.stop();
	handle.stopped().await;

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
	}

	let per_conn = PerConnection {
		methods: ().into_rpc().into(),
		stop_handle: stop_handle.clone(),
		conn_id: Default::default(),
		conn_guard: ConnectionGuard::new(100),
	};

	tokio::spawn(async move {
		loop {
			// The `tokio::select!` macro is used to wait for either of the
			// listeners to accept a new connection or for the server to be
			// stopped.
			let (sock, _) = tokio::select! {
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
			let svc = tower::service_fn(move |mut req: HttpRequest<hyper::body::Incoming>| {
				let PerConnection { methods, stop_handle, conn_guard, conn_id } = per_conn.clone();
				let (tx, mut disconnect) = mpsc::channel::<()>(1);

				// Insert the `tx` into the request extensions to be able to close the connection
				// from method or subscription handlers.
				req.extensions_mut().insert(tx.clone());

				// jsonrpsee expects a `conn permit` for each connection.
				//
				// This may be omitted if don't want to limit the number of connections
				// to the server.
				let Some(conn_permit) = conn_guard.try_acquire() else {
					return async { Ok::<_, Infallible>(http::response::too_many_requests()) }.boxed();
				};

				let conn = ConnectionState::new(stop_handle, conn_id.fetch_add(1, Ordering::Relaxed), conn_permit);

				if ws::is_upgrade_request(&req) {
					let rpc_service = RpcServiceBuilder::new();

					// Establishes the websocket connection
					// then the connection is closed i.e, the `conn_fut` is dropped.
					async move {
						match ws::connect(req, ServerConfig::default(), methods, conn, rpc_service).await {
							Ok((rp, conn_fut)) => {
								tokio::spawn(async move {
									tokio::select! {
										_ = conn_fut => (),
										_ = disconnect.recv() => {
											eprintln!("Server closed connection");
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
					// There is another API for making call with just a service as well.
					//
					// See [`jsonrpsee::server::http::call_with_service`]
					async move {
						tokio::select! {
							// Rpc call finished successfully.
							res = http::call_with_service_builder(req, ServerConfig::default(), conn, methods, RpcServiceBuilder::new()) => Ok(res),
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
