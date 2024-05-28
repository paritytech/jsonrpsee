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

use std::net::SocketAddr;
use std::sync::atomic::AtomicU32;
use std::sync::Arc;

use futures::future::{self, Either};
use hyper_util::rt::{TokioExecutor, TokioIo};
use jsonrpsee::core::async_trait;
use jsonrpsee::core::SubscriptionResult;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::middleware::rpc::RpcServiceT;
use jsonrpsee::server::{stop_channel, PendingSubscriptionSink, RpcServiceBuilder, SubscriptionMessage};
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::Extensions;
use tokio::net::TcpListener;
use tower::Service;

#[derive(Debug, Clone)]
struct ConnectionDetails<S> {
	inner: S,
	connection_id: u32,
}

impl<'a, S> RpcServiceT<'a> for ConnectionDetails<S>
where
	S: RpcServiceT<'a>,
{
	type Future = S::Future;

	fn call(&self, mut request: jsonrpsee::types::Request<'a>) -> Self::Future {
		request.extensions_mut().insert(self.connection_id);
		self.inner.call(request)
	}
}

#[rpc(server, client)]
pub trait Rpc {
	/// method with connection ID.
	#[method(name = "connectionIdMethod", with_extensions)]
	async fn method(&self, first_param: usize, second_param: u16) -> Result<u32, ErrorObjectOwned>;

	#[subscription(name = "subscribeConnectionId", item = u32, with_extensions)]
	async fn sub(&self) -> SubscriptionResult;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn method(&self, ext: &Extensions, _first_param: usize, _second_param: u16) -> Result<u32, ErrorObjectOwned> {
		ext.get::<u32>().cloned().ok_or_else(|| ErrorObject::owned(0, "No connection details found", None::<()>))
	}

	async fn sub(&self, pending: PendingSubscriptionSink, ext: &Extensions) -> SubscriptionResult {
		let sink = pending.accept().await?;
		let conn_id = ext
			.get::<u32>()
			.cloned()
			.ok_or_else(|| ErrorObject::owned(0, "No connection details found", None::<()>))?;
		sink.send(SubscriptionMessage::from_json(&conn_id).unwrap()).await?;
		Ok(())
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let server_addr = run_server().await?;
	let url = format!("ws://{}", server_addr);

	let client = WsClientBuilder::default().build(&url).await?;
	let connection_id_first = client.method(1, 2).await.unwrap();

	// Second call from the same connection ID.
	assert_eq!(client.method(1, 2).await.unwrap(), connection_id_first);

	// Second client will increment the connection ID.
	let client2 = WsClientBuilder::default().build(&url).await?;
	let connection_id_second = client2.method(1, 2).await.unwrap();
	assert_ne!(connection_id_first, connection_id_second);

	let mut sub = client.sub().await.unwrap();
	assert_eq!(connection_id_first, sub.next().await.transpose().unwrap().unwrap());

	let mut sub = client2.sub().await.unwrap();
	assert_eq!(connection_id_second, sub.next().await.transpose().unwrap().unwrap());

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).await?;
	let addr = listener.local_addr()?;

	let (stop_hdl, server_hdl) = stop_channel();

	tokio::spawn(async move {
		let conn_id = Arc::new(AtomicU32::new(0));
		// Create and finalize a server configuration from a TowerServiceBuilder
		// given an RpcModule and the stop handle.
		let svc_builder = jsonrpsee::server::Server::builder().to_service_builder();
		let methods = RpcServerImpl.into_rpc();

		loop {
			let stream = tokio::select! {
				res = listener.accept() => {
					match res {
						Ok((stream, _remote_addr)) => stream,
						Err(e) => {
							tracing::error!("failed to accept v4 connection: {:?}", e);
							continue;
						}
					}
				}
				_ = stop_hdl.clone().shutdown() => break,
			};

			let methods2 = methods.clone();
			let stop_hdl2 = stop_hdl.clone();
			let svc_builder2 = svc_builder.clone();
			let conn_id2 = conn_id.clone();
			let svc = hyper::service::service_fn(move |req: hyper::Request<hyper::body::Incoming>| {
				let connection_id = conn_id2.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
				let rpc_middleware = RpcServiceBuilder::default()
					.layer_fn(move |service| ConnectionDetails { inner: service, connection_id });

				// Start a new service with our own connection ID.
				let mut tower_service = svc_builder2
					.clone()
					.set_rpc_middleware(rpc_middleware)
					.connection_id(connection_id)
					.build(methods2.clone(), stop_hdl2.clone());

				async move { tower_service.call(req).await.map_err(|e| anyhow::anyhow!("{:?}", e)) }
			});

			let stop_hdl2 = stop_hdl.clone();
			// Spawn a new task to serve each respective (Hyper) connection.
			tokio::spawn(async move {
				let builder = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());
				let conn = builder.serve_connection_with_upgrades(TokioIo::new(stream), svc);
				let stopped = stop_hdl2.shutdown();

				// Pin the future so that it can be polled.
				tokio::pin!(stopped, conn);

				let res = match future::select(conn, stopped).await {
					// Return the connection if not stopped.
					Either::Left((conn, _)) => conn,
					// If the server is stopped, we should gracefully shutdown
					// the connection and poll it until it finishes.
					Either::Right((_, mut conn)) => {
						conn.as_mut().graceful_shutdown();
						conn.await
					}
				};

				// Log any errors that might have occurred.
				if let Err(err) = res {
					tracing::error!(err=?err, "HTTP connection failed");
				}
			});
		}
	});

	tokio::spawn(server_hdl.stopped());

	Ok(addr)
}
