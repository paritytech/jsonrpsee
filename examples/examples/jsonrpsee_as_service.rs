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

//! This example shows how to use the `jsonrpsee::server` as
//! a tower service such that it's possible to get access
//! HTTP related things by launching a `hyper::service_fn`.
//!
//! The typical use-case for this is when one wants to have
//! access to HTTP related things.

use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use futures::FutureExt;
use hyper::HeaderMap;
use hyper::header::AUTHORIZATION;
use jsonrpsee::core::middleware::{Batch, BatchEntry, Notification, RpcServiceBuilder, RpcServiceT};
use jsonrpsee::core::{BoxError, async_trait};
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::{
	ServerConfig, ServerHandle, StopHandle, TowerServiceBuilder, serve_with_graceful_shutdown, stop_channel,
};
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned, Id, Request};
use jsonrpsee::ws_client::{HeaderValue, WsClientBuilder};
use jsonrpsee::{MethodResponse, Methods};
use tokio::net::TcpListener;
use tower::Service;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Default, Clone, Debug)]
struct Metrics {
	opened_ws_connections: Arc<AtomicUsize>,
	closed_ws_connections: Arc<AtomicUsize>,
	http_calls: Arc<AtomicUsize>,
	success_http_calls: Arc<AtomicUsize>,
}

fn auth_reject_error() -> ErrorObjectOwned {
	ErrorObject::owned(-32999, "HTTP Authorization header is missing", None::<()>)
}

#[derive(Clone)]
struct AuthorizationMiddleware<S> {
	headers: HeaderMap,
	inner: S,
	#[allow(unused)]
	transport_label: &'static str,
}

impl<S> AuthorizationMiddleware<S> {
	/// Authorize the request by checking the `Authorization` header.
	///
	///
	/// In this example for simplicity, the authorization value is not checked
	// and used because it's just a toy example.
	fn auth_method_call(&self, req: &Request<'_>) -> bool {
		if req.method_name() == "trusted_call" {
			let Some(Ok(_)) = self.headers.get(AUTHORIZATION).map(|auth| auth.to_str()) else { return false };
		}

		true
	}

	/// Authorize the notification by checking the `Authorization` header.
	///
	/// Because notifications are not expected to return a response, we
	/// return a `MethodResponse` by injecting an error into the extensions
	/// which could be read by other middleware or the server.
	fn auth_notif(&self, notif: &Notification<'_>) -> bool {
		if notif.method_name() == "trusted_call" {
			let Some(Ok(_)) = self.headers.get(AUTHORIZATION).map(|auth| auth.to_str()) else { return false };
		}

		true
	}
}

impl<S> RpcServiceT for AuthorizationMiddleware<S>
where
	S: Send + Clone + Sync + RpcServiceT<Response = MethodResponse, Error = Infallible> + 'static,
	S::Error: Into<BoxError> + Send + 'static,
	S::Response: Send,
{
	type Error = S::Error;
	type Response = S::Response;

	fn call<'a>(&self, req: Request<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let this = self.clone();
		let auth_ok = this.auth_method_call(&req);

		async move {
			// If the authorization header is missing, it's recommended to
			// to return the response as MethodResponse::error instead of
			// returning an error from the service.
			//
			// This way the error is returned as a JSON-RPC error
			if !auth_ok {
				return Ok(MethodResponse::error(req.id, auth_reject_error()));
			}
			this.inner.call(req).await
		}
	}

	fn batch<'a>(&self, mut batch: Batch<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		// Check the authorization header for each entry in the batch.
		// If any entry is unauthorized, return an error.
		//
		// It might be better to return an error for each entry
		// instead of returning an error for the whole batch.
		for entry in batch.as_mut_batch_entries() {
			match entry {
				Ok(BatchEntry::Call(req)) => {
					if self.auth_method_call(req) {
						return async { Ok(MethodResponse::error(Id::Null, auth_reject_error())) }.boxed();
					}
				}
				Ok(BatchEntry::Notification(notif)) => {
					if self.auth_notif(notif) {
						return async { Ok(MethodResponse::error(Id::Null, auth_reject_error())) }.boxed();
					}
				}
				// Ignore parse error.
				Err(_err) => {}
			}
		}

		self.inner.batch(batch).boxed()
	}

	fn notification<'a>(
		&self,
		n: Notification<'a>,
	) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		self.inner.notification(n)
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

	let metrics = Metrics::default();

	let handle = run_server(metrics.clone()).await?;
	tokio::spawn(handle.stopped());

	{
		let client = HttpClient::builder().build("http://127.0.0.1:9944").unwrap();

		// Fails because the authorization header is missing.
		let x = client.trusted_call().await.unwrap_err();
		tracing::info!("response: {x}");
	}

	{
		let client = WsClientBuilder::default().build("ws://127.0.0.1:9944").await.unwrap();

		// Fails because the authorization header is missing.
		let x = client.trusted_call().await.unwrap_err();
		tracing::info!("response: {x}");
	}

	{
		let mut headers = HeaderMap::new();
		headers.insert(AUTHORIZATION, HeaderValue::from_static("don't care in this example"));

		let client = HttpClient::builder().set_headers(headers).build("http://127.0.0.1:9944").unwrap();

		let x = client.trusted_call().await.unwrap();
		tracing::info!("response: {x}");
	}

	tracing::info!("{:?}", metrics);

	Ok(())
}

async fn run_server(metrics: Metrics) -> anyhow::Result<ServerHandle> {
	let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 9944))).await?;

	// This state is cloned for every connection
	// all these types based on Arcs and it should
	// be relatively cheap to clone them.
	//
	// Make sure that nothing expensive is cloned here
	// when doing this or use an `Arc`.
	#[derive(Clone)]
	struct PerConnection<RpcMiddleware, HttpMiddleware> {
		methods: Methods,
		stop_handle: StopHandle,
		metrics: Metrics,
		svc_builder: TowerServiceBuilder<RpcMiddleware, HttpMiddleware>,
	}

	// Each RPC call/connection get its own `stop_handle`
	// to able to determine whether the server has been stopped or not.
	//
	// To keep the server running the `server_handle`
	// must be kept and it can also be used to stop the server.
	let (stop_handle, server_handle) = stop_channel();

	let per_conn = PerConnection {
		methods: ().into_rpc().into(),
		stop_handle: stop_handle.clone(),
		metrics,
		svc_builder: jsonrpsee::server::Server::builder()
			.set_config(ServerConfig::builder().max_connections(33).build())
			.set_http_middleware(tower::ServiceBuilder::new().layer(CorsLayer::permissive()))
			.to_service_builder(),
	};

	tokio::spawn(async move {
		loop {
			// The `tokio::select!` macro is used to wait for either of the
			// listeners to accept a new connection or for the server to be
			// stopped.
			let sock = tokio::select! {
				res = listener.accept() => {
					match res {
						Ok((stream, _remote_addr)) => stream,
						Err(e) => {
							tracing::error!("failed to accept v4 connection: {:?}", e);
							continue;
						}
					}
				}
				_ = per_conn.stop_handle.clone().shutdown() => break,
			};
			let per_conn2 = per_conn.clone();

			let svc = tower::service_fn(move |req: hyper::Request<hyper::body::Incoming>| {
				let is_websocket = jsonrpsee::server::ws::is_upgrade_request(&req);
				let transport_label = if is_websocket { "ws" } else { "http" };
				let PerConnection { methods, stop_handle, metrics, svc_builder } = per_conn2.clone();

				let http_middleware = tower::ServiceBuilder::new().layer(CompressionLayer::new());
				// NOTE, the rpc middleware must be initialized here to be able to created once per connection
				// with data from the connection such as the headers in this example
				let headers = req.headers().clone();
				let rpc_middleware = RpcServiceBuilder::new().rpc_logger(1024).layer_fn(move |service| {
					AuthorizationMiddleware { inner: service, headers: headers.clone(), transport_label }
				});

				let mut svc = svc_builder
					.set_http_middleware(http_middleware)
					.set_rpc_middleware(rpc_middleware)
					.build(methods, stop_handle);

				if is_websocket {
					// Utilize the session close future to know when the actual WebSocket
					// session was closed.
					let session_close = svc.on_session_closed();

					// A little bit weird API but the response to HTTP request must be returned below
					// and we spawn a task to register when the session is closed.
					tokio::spawn(async move {
						session_close.await;
						tracing::info!("Closed WebSocket connection");
						metrics.closed_ws_connections.fetch_add(1, Ordering::Relaxed);
					});

					async move {
						tracing::info!("Opened WebSocket connection");
						metrics.opened_ws_connections.fetch_add(1, Ordering::Relaxed);
						svc.call(req).await
					}
					.boxed()
				} else {
					// HTTP.
					async move {
						tracing::info!("Opened HTTP connection");
						metrics.http_calls.fetch_add(1, Ordering::Relaxed);
						let rp = svc.call(req).await;

						if rp.is_ok() {
							metrics.success_http_calls.fetch_add(1, Ordering::Relaxed);
						}

						tracing::info!("Closed HTTP connection");
						rp
					}
					.boxed()
				}
			});

			tokio::spawn(serve_with_graceful_shutdown(sock, svc, stop_handle.clone().shutdown()));
		}
	});

	Ok(server_handle)
}
