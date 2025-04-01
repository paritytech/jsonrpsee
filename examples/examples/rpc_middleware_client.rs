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

//! jsonrpsee supports two kinds of middlewares `http_middleware` and `rpc_middleware`.
//!
//! This example demonstrates how to use the `rpc_middleware` which applies for each
//! JSON-RPC method calls, notifications and batch requests.
//!
//! This example demonstrates how to use the `rpc_middleware` for the client
//! and you may benefit specifying the response type to `core::client::MethodResponse`
//! to actually inspect the response instead of using the serialized JSON-RPC response.

use std::net::SocketAddr;
use std::ops::Deref;
use std::sync::{Arc, Mutex};

use futures::FutureExt;
use futures::future::BoxFuture;
use jsonrpsee::core::client::{ClientT, MethodResponse, MethodResponseKind};
use jsonrpsee::core::middleware::{Batch, Notification, RpcServiceBuilder, RpcServiceT};
use jsonrpsee::rpc_params;
use jsonrpsee::server::{RpcModule, Server};
use jsonrpsee::types::{ErrorCode, ErrorObject, Request};
use jsonrpsee::ws_client::WsClientBuilder;

#[derive(Default)]
struct InnerMetrics {
	method_calls_success: usize,
	method_calls_failure: usize,
	notifications: usize,
	batch_calls: usize,
}

#[derive(Clone)]
pub struct Metrics<S> {
	service: S,
	metrics: Arc<Mutex<InnerMetrics>>,
}

impl std::fmt::Debug for InnerMetrics {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("InnerMetrics")
			.field("method_calls_success", &self.method_calls_success)
			.field("method_calls_failure", &self.method_calls_failure)
			.field("notifications", &self.notifications)
			.field("batch_calls", &self.batch_calls)
			.finish()
	}
}

impl<S> Metrics<S> {
	pub fn new(service: S) -> Self {
		Self { service, metrics: Arc::new(Mutex::new(InnerMetrics::default())) }
	}
}

// NOTE: We are MethodResponse as the response type here to be able to inspect the response
// and not just the serialized JSON-RPC response. This is not necessary if you only care about
// the serialized JSON-RPC response.
impl<'a, S> RpcServiceT<'a> for Metrics<S>
where
	S: RpcServiceT<'a, Response = MethodResponse> + Send + Sync + Clone + 'static,
{
	type Future = BoxFuture<'a, Result<Self::Response, Self::Error>>;
	type Error = S::Error;
	type Response = S::Response;

	fn call(&self, req: Request<'a>) -> Self::Future {
		let m = self.metrics.clone();
		let service = self.service.clone();

		async move {
			let rp = service.call(req).await;

			// Access to inner response via the deref implementation.
			match rp.as_ref().map(|r| r.deref()) {
				Ok(MethodResponseKind::MethodCall(r)) => {
					if r.is_success() {
						m.lock().unwrap().method_calls_success += 1;
					} else {
						m.lock().unwrap().method_calls_failure += 1;
					}
				}
				Ok(e) => unreachable!("Unexpected response type {e:?}"),
				Err(e) => {
					m.lock().unwrap().method_calls_failure += 1;
					tracing::error!("Error: {:?}", e);
				}
			}

			rp
		}
		.boxed()
	}

	fn batch(&self, batch: Batch<'a>) -> Self::Future {
		self.metrics.lock().unwrap().batch_calls += 1;
		Box::pin(self.service.batch(batch))
	}

	fn notification(&self, n: Notification<'a>) -> Self::Future {
		self.metrics.lock().unwrap().notifications += 1;
		Box::pin(self.service.notification(n))
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let metrics = Arc::new(Mutex::new(InnerMetrics::default()));

	for _ in 0..2 {
		let metrics = metrics.clone();
		let rpc_middleware =
			RpcServiceBuilder::new().layer_fn(move |s| Metrics { service: s, metrics: metrics.clone() });
		let client = WsClientBuilder::new().set_rpc_middleware(rpc_middleware).build(&url).await?;
		let _: Result<String, _> = client.request("say_hello", rpc_params![]).await;
		let _: Result<String, _> = client.request("unknown_method", rpc_params![]).await;
		let _: Result<String, _> = client.request("thready", rpc_params![4]).await;
		let _: Result<String, _> = client.request("mul", rpc_params![4]).await;
		let _: Result<String, _> = client.request("err", rpc_params![4]).await;

		tokio::time::sleep(std::time::Duration::from_millis(100)).await;
	}

	println!("Metrics: {:?}", metrics.lock().unwrap());

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = Server::builder().build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _, _| "lo")?;
	module.register_method("mul", |params, _, _| {
		let count: usize = params.one().unwrap();
		count * 2
	})?;
	module.register_method("error", |_, _, _| ErrorObject::from(ErrorCode::InternalError))?;
	let addr = server.local_addr()?;
	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
