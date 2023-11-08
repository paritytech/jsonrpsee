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

//! This examples shows how to use the `jsonrpsee::server`` as
//! a tower service such that it's possible to get access
//! HTTP related things by launching a `hyper::service_fn`.
//!
//! The typical use-case for this is when one wants to have
//! access to HTTP related things.

use std::error::Error as StdError;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use futures::FutureExt;
use hyper::header::AUTHORIZATION;
use hyper::server::conn::AddrStream;
use hyper::HeaderMap;
use jsonrpsee::core::async_trait;
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::middleware::rpc::{RpcServiceBuilder, RpcServiceT, TransportProtocol};
use jsonrpsee::server::{stop_channel, ServerHandle};
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned, Request};
use jsonrpsee::ws_client::HeaderValue;
use jsonrpsee::MethodResponse;
use tower::Service;
use tower_http::cors::CorsLayer;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Default, Clone)]
struct Metrics {
	ws_connections: Arc<AtomicUsize>,
	http_connections: Arc<AtomicUsize>,
}

struct AuthorizationMiddleware<S> {
	headers: HeaderMap,
	inner: S,
}

#[async_trait]
impl<'a, S> RpcServiceT<'a> for AuthorizationMiddleware<S>
where
	S: Send + Sync + RpcServiceT<'a>,
{
	async fn call(&self, req: Request<'a>, label: TransportProtocol) -> MethodResponse {
		if req.method_name() == "trusted_call" {
			let Some(Ok(_)) = self.headers.get(AUTHORIZATION).map(|auth| auth.to_str()) else {
				return MethodResponse::error(req.id, ErrorObject::borrowed(-32000, "Authorization failed", None));
			};

			// In this example for simplicity, the authorization value is not checked
			// and used because it's just a toy example.

			self.inner.call(req, label).await
		} else {
			self.inner.call(req, label).await
		}
	}
}

#[rpc(server, client)]
pub trait Rpc {
	#[method(name = "trusted_call")]
	async fn trusted_call(&self) -> Result<String, ErrorObjectOwned>;
}

#[async_trait]
impl RpcServer for () {
	async fn trusted_call(&self) -> Result<String, ErrorObjectOwned> {
		Ok("mysecret".to_string())
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let filter = tracing_subscriber::EnvFilter::try_from_default_env()?;
	tracing_subscriber::FmtSubscriber::builder().with_env_filter(filter).finish().try_init()?;

	let handle = run_server();
	tokio::spawn(handle.stopped());

	{
		let client = HttpClientBuilder::default().build("http://127.0.0.1:9944").unwrap();

		// Fails because the authorization header is missing.
		let x = client.trusted_call().await.unwrap_err();
		tracing::info!("response: {x}");
	}

	{
		let mut headers = HeaderMap::new();
		headers.insert(AUTHORIZATION, HeaderValue::from_static("don't care in this example"));

		let client = HttpClientBuilder::default().set_headers(headers).build("http://127.0.0.1:9944").unwrap();

		let x = client.trusted_call().await.unwrap();
		tracing::info!("response: {x}");
	}

	Ok(())
}

fn run_server() -> ServerHandle {
	use hyper::service::{make_service_fn, service_fn};

	let addr = SocketAddr::from(([127, 0, 0, 1], 9944));
	// Each RPC call/connection get its own `stop_handle`
	// to able to determine whether the server has been stopped or not.
	//
	// To keep the server running the `server_handle`
	// must be kept and it can also be used to stop the server.
	let (stop_handle, server_handle) = stop_channel();

	let svc_builder = jsonrpsee::server::Server::builder()
		.set_http_middleware(tower::ServiceBuilder::new().layer(CorsLayer::permissive()))
		.max_connections(33)
		.to_service_builder();

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
				let svc_builder = svc_builder.clone();
				let methods = methods.clone();
				let stop_handle = stop_handle.clone();

				async move {
					let headers = req.headers().clone();

					// NOTE, the rpc middleware must be initialized here to be able to created once per connection
					// with data from the connection such as the headers in this example
					let rpc_middleware = RpcServiceBuilder::new()
						.rpc_logger(1024)
						.layer_fn(move |service| AuthorizationMiddleware { inner: service, headers: headers.clone() });

					let mut svc = svc_builder.set_rpc_middleware(rpc_middleware).build(methods, stop_handle);

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
