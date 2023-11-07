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
use std::pin::Pin;
use std::sync::Arc;

use futures::Future;
use jsonrpsee::core::{async_trait, client::ClientT};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::rpc_params;
use jsonrpsee::server::middleware::rpc::{RpcService, RpcServiceT};
use jsonrpsee::server::{ConnectionGuard, ServerHandle, StopHandle, TowerService};
use jsonrpsee::types::ErrorObjectOwned;
use tokio::net::TcpListener;

use tower::Service;
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

	let http_middleware = tower::ServiceBuilder::new().layer(CorsLayer::permissive());
	let cfg = jsonrpsee::server::Server::builder().set_http_middleware(http_middleware).to_service(().into_rpc());
	let conn_guard = Arc::new(ConnectionGuard::new(cfg.settings.max_connections as usize));
	let mut conn_id = 0;

	tokio::spawn(async move {
		loop {
			let (stream, _) = listener.accept().await.unwrap();

			let conn_permit = conn_guard.try_acquire().unwrap();

			let svc = TowerService::new(
				cfg.settings.clone(),
				cfg.methods.clone(),
				cfg.rpc_middleware.clone(),
				Arc::new(conn_permit),
				stop_handle.clone(),
				conn_id,
			);

			let svc = Svc { svc, builder: cfg.http_middleware.clone() };

			tokio::task::spawn(async move {
				if let Err(err) = Http::new().serve_connection(stream, svc).await {
					println!("Failed to serve connection: {:?}", err);
				}
			});
			conn_id += 1;
		}
	});

	server_handle
}

struct Svc<RpcMiddleware, HttpMiddleware> {
	svc: TowerService<RpcMiddleware>,
	builder: tower::ServiceBuilder<HttpMiddleware>,
}

impl<RpcMiddleware, HttpMiddleware> Service<hyper::Request<hyper::Body>> for Svc<RpcMiddleware, HttpMiddleware>
where
	RpcMiddleware: tower::Layer<RpcService> + Clone + Send + 'static,
	for<'a> <RpcMiddleware as tower::Layer<RpcService>>::Service: RpcServiceT<'a>,
	HttpMiddleware: tower::Layer<TowerService<RpcMiddleware>> + Send + 'static,
	<HttpMiddleware as tower::Layer<TowerService<RpcMiddleware>>>::Service: Send
		+ Service<
			hyper::Request<hyper::Body>,
			Response = hyper::Response<hyper::Body>,
			Error = Box<(dyn StdError + Send + Sync + 'static)>,
		>,
	<<HttpMiddleware as tower::Layer<TowerService<RpcMiddleware>>>::Service as Service<hyper::Request<hyper::Body>>>::Future:
		Send,
{
	type Response = hyper::Response<hyper::Body>;
	type Error = Box<(dyn StdError + Send + Sync + 'static)>;
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

	fn call(&mut self, req: hyper::Request<hyper::Body>) -> Self::Future {
		let m = self.svc.clone();
		Box::pin(self.builder.service(m).call(req))
	}

	fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
		std::task::Poll::Ready(Ok(()))
	}
}
