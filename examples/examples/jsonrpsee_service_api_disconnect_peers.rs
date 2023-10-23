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
use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicU32, AtomicUsize};
use std::sync::{Arc, Mutex};

use futures::FutureExt;
use jsonrpsee::core::{async_trait, client::ClientT};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::middleware::rpc::*;
use jsonrpsee::server::{ConnectionGuard, RandomIntegerIdProvider, ServiceConfig, ServiceData, StopHandle};
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned, Request};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{rpc_params, BoundedSubscriptions, MethodResponse, MethodSink};

use futures::io::{BufReader, BufWriter};
use hyper::server::conn::AddrStream;
use tokio::sync::{mpsc, OwnedSemaphorePermit};
use tokio_util::compat::TokioAsyncReadCompatExt;
use tracing_subscriber::util::SubscriberInitExt;

struct DummyRateLimit<S> {
	service: S,
	count: Arc<AtomicUsize>,
	state: mpsc::Sender<()>,
}

#[async_trait]
impl<'a, S> RpcServiceT<'a> for DummyRateLimit<S>
where
	S: Send + Sync + RpcServiceT<'a>,
{
	async fn call(&self, req: Request<'a>, ctx: &Context) -> MethodResponse {
		let count = self.count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

		if count > 10 {
			let _ = self.state.try_send(());
			MethodResponse::error(req.id, ErrorObject::borrowed(-32000, "RPC rate limit", None))
		} else {
			self.service.call(req, ctx).await
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
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()?
		.add_directive("jsonrpsee[method_call{name = \"say_hello\"}]=trace".parse()?);
	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

	tokio::spawn(run_server());

	// Make a bunch of requests to be blacklisted by server.
	{
		let client = WsClientBuilder::default().build("ws://127.0.0.1:9944").await.unwrap();
		while client.is_connected() {
			let rp: Result<String, _> = client.request("say_hello", rpc_params!()).await;
			tracing::info!("response: {:?}", rp);
		}
	}

	// After the server has blacklisted the IP address, the connection is denied.
	assert!(WsClientBuilder::default().build("ws://127.0.0.1:9944").await.is_err());

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

	// Blacklisted peers
	let blacklisted_peers = Arc::new(Mutex::new(HashSet::new()));

	// And a MakeService to handle each connection...
	let make_service = make_service_fn(|conn: &AddrStream| {
		// You may use `conn` or the actual HTTP request to deny a certain peer.

		// Connection state.
		let conn_id = conn_id.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
		let remote_addr = conn.remote_addr();
		let stop_handle = stop_handle.clone();
		let conn_permit = Arc::new(conn_guard.try_acquire().unwrap());
		let service_cfg = service_cfg.clone();
		let blacklisted_peers = blacklisted_peers.clone();

		async move {
			let stop_handle = stop_handle.clone();
			let conn_permit = conn_permit.clone();
			let service_cfg = service_cfg.clone();
			let stop_handle = stop_handle.clone();
			let blacklisted_peers = blacklisted_peers.clone();

			Ok::<_, Infallible>(service_fn(move |req| {
				if blacklisted_peers.lock().unwrap().get(&remote_addr.ip()).is_some() {
					return async { Ok::<_, Infallible>(reject(req).await) }.boxed();
				}

				if jsonrpsee::server::is_websocket_request(&req) && service_cfg.enable_ws {
					let service_cfg = service_cfg.clone();
					let stop_handle = stop_handle.clone();
					let conn_permit = conn_permit.clone();
					let blacklisted_peers = blacklisted_peers.clone();

					let (tx, rx_conn) = mpsc::channel(1);
					let rpc_service = RpcServiceBuilder::new().layer_fn(move |service| DummyRateLimit {
						service,
						count: Arc::new(AtomicUsize::new(0)),
						state: tx.clone(),
					});

					async move {
						Ok(websocket_upgrade(
							req,
							service_cfg,
							conn_id,
							rpc_service,
							remote_addr,
							stop_handle,
							conn_permit,
							rx_conn,
							blacklisted_peers,
						)
						.await)
					}
					.boxed()
				} else {
					// TODO: for simplicity in this example we don't about pure HTTP requests
					async move { Ok(echo_http(req).await) }.boxed()
				}
			}))
		}
	});

	// Then bind and serve...
	let server = hyper::Server::bind(&addr).serve(make_service);

	server.await.unwrap();
}

async fn websocket_upgrade<L>(
	req: hyper::Request<hyper::Body>,
	cfg: ServiceConfig,
	conn_id: u32,
	rpc_middleware: RpcServiceBuilder<L>,
	remote_addr: SocketAddr,
	stop_handle: StopHandle,
	conn_permit: Arc<OwnedSemaphorePermit>,
	mut rx_conn: mpsc::Receiver<()>,
	blacklisted_peers: Arc<Mutex<HashSet<IpAddr>>>,
) -> hyper::Response<hyper::Body>
where
	L: for<'a> tower::Layer<RpcService>,
	<L as tower::Layer<RpcService>>::Service: Send + Sync + 'static,
	for<'a> <L as tower::Layer<RpcService>>::Service: RpcServiceT<'a>,
{
	let mut server = soketto::handshake::http::Server::new();

	match server.receive_request(&req) {
		Ok(response) => {
			let (tx, rx) = mpsc::channel::<String>(cfg.message_buffer_capacity as usize);
			let sink = MethodSink::new(tx);

			let ctx = Context {
				transport: TransportProtocol::WebSocket,
				remote_addr: remote_addr,
				conn_id: conn_id as usize,
				headers: req.headers().clone(),
				uri: req.uri().clone(),
			};

			// On each method call the `pending_calls` is cloned
			// then when all pending_calls are dropped
			// a graceful shutdown can has occur.
			let (pending_calls, pending_calls_completed) = mpsc::channel::<()>(1);

			let rpc_service_cfg = RpcServiceCfg::CallsAndSubscriptions {
				bounded_subscriptions: BoundedSubscriptions::new(cfg.max_subscriptions_per_connection),
				id_provider: Arc::new(RandomIntegerIdProvider),
				sink: sink.clone(),
				_pending_calls: pending_calls,
			};

			let rpc_service = RpcService::new(
				cfg.methods.clone(),
				cfg.max_response_body_size as usize,
				conn_id as usize,
				rpc_service_cfg,
			);

			let rpc_service = rpc_middleware.service(rpc_service);

			tokio::spawn(async move {
				let upgraded = match hyper::upgrade::on(req).await {
					Ok(u) => u,
					Err(e) => {
						tracing::debug!("Could not upgrade connection: {}", e);
						return;
					}
				};

				let stream = BufReader::new(BufWriter::new(upgraded.compat()));
				let mut ws_builder = server.into_builder(stream);
				ws_builder.set_max_message_size(cfg.max_response_body_size as usize);
				let (sender, receiver) = ws_builder.finish();

				let svc = ServiceData {
					remote_addr,
					methods: cfg.methods.clone(),
					max_request_body_size: cfg.max_request_body_size,
					max_response_body_size: cfg.max_response_body_size,
					max_subscriptions_per_connection: cfg.max_subscriptions_per_connection,
					ping_config: cfg.ping_config,
					enable_http: cfg.enable_http,
					enable_ws: cfg.enable_ws,
					batch_requests_config: cfg.batch_requests_config,
					id_provider: cfg.id_provider,
					message_buffer_capacity: cfg.message_buffer_capacity,
					conn_id,
					stop_handle,
					conn: conn_permit,
				};

				let params = jsonrpsee::server::ws::BackgroundTaskParams {
					other: svc,
					ws_sender: sender,
					ws_receiver: receiver,
					rpc_service,
					sink,
					rx,
					ctx,
					pending_calls_completed,
				};

				tokio::select! {
					_ = jsonrpsee::server::ws::background_task(params) => (),
					_ = rx_conn.recv() => {
						tracing::warn!("Rate limit middleware blacklist ip={}", remote_addr.ip());
						blacklisted_peers.lock().unwrap().insert(remote_addr.ip());
					}
				}
			});

			response.map(|()| hyper::Body::empty())
		}
		Err(e) => {
			tracing::debug!("Could not upgrade connection: {}", e);
			hyper::Response::new(hyper::Body::from(format!("Could not upgrade connection: {e}")))
		}
	}
}

async fn echo_http(_req: hyper::Request<hyper::Body>) -> hyper::Response<hyper::Body> {
	hyper::Response::builder().status(hyper::StatusCode::OK).body(hyper::Body::empty()).unwrap()
}

async fn reject(_req: hyper::Request<hyper::Body>) -> hyper::Response<hyper::Body> {
	hyper::Response::builder().status(hyper::StatusCode::TOO_MANY_REQUESTS).body(hyper::Body::empty()).unwrap()
}
