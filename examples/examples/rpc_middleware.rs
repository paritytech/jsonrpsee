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
//! JSON-RPC method call and batch requests may call the middleware more than once.
//!
//! A typical use-case for this is to implement rate-limiting based on the actual
//! number of JSON-RPC methods calls and a request could potentially be made
//! by HTTP or WebSocket which this middleware is agnostic to.
//!
//! Contrary the HTTP middleware does only apply per HTTP request and
//! may be handy in some scenarios such CORS but if you want to access
//! to the actual JSON-RPC details this is the middleware to use.
//!
//! This example enables the same middleware for both the server and client which
//! can be confusing when one runs this but it is just to demonstrate the API.
//!
//! That the middleware is applied to the server and client in the same way.

use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::middleware::{Batch, Notification, RpcServiceBuilder, RpcServiceT};
use jsonrpsee::rpc_params;
use jsonrpsee::server::{RpcModule, Server};
use jsonrpsee::types::Request;
use jsonrpsee::ws_client::WsClientBuilder;

#[derive(Clone)]
struct IdentityLayer;

impl<S> tower::Layer<S> for IdentityLayer
where
	S: RpcServiceT + Send + Sync + Clone + 'static,
{
	type Service = Identity<S>;

	fn layer(&self, inner: S) -> Self::Service {
		Identity(inner)
	}
}

#[derive(Clone)]
struct Identity<S>(S);

impl<S> RpcServiceT for Identity<S>
where
	S: RpcServiceT + Send + Sync + Clone + 'static,
{
	type Response = S::Response;
	type Error = S::Error;

	fn batch<'a>(&self, batch: Batch<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		self.0.batch(batch)
	}

	fn call<'a>(&self, request: Request<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		self.0.call(request)
	}

	fn notification<'a>(
		&self,
		n: Notification<'a>,
	) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		self.0.notification(n)
	}
}

// It's possible to access the connection ID
// by using the low-level API.
#[derive(Clone)]
pub struct CallsPerConn<S> {
	service: S,
	count: Arc<AtomicUsize>,
	role: &'static str,
}

impl<S> RpcServiceT for CallsPerConn<S>
where
	S: RpcServiceT + Send + Sync + Clone + 'static,
{
	type Error = S::Error;
	type Response = S::Response;

	fn call<'a>(&self, req: Request<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let count = self.count.clone();
		let service = self.service.clone();
		let role = self.role;

		async move {
			let rp = service.call(req).await;
			count.fetch_add(1, Ordering::SeqCst);
			println!("{role} processed calls={} on the connection", count.load(Ordering::SeqCst));
			rp
		}
	}

	fn batch<'a>(&self, batch: Batch<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let len = batch.len();
		self.count.fetch_add(len, Ordering::SeqCst);
		println!("{} processed calls={} on the connection", self.role, self.count.load(Ordering::SeqCst));
		self.service.batch(batch)
	}

	fn notification<'a>(
		&self,
		n: Notification<'a>,
	) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		self.service.notification(n)
	}
}

#[derive(Clone)]
pub struct GlobalCalls<S> {
	service: S,
	count: Arc<AtomicUsize>,
	role: &'static str,
}

impl<S> RpcServiceT for GlobalCalls<S>
where
	S: RpcServiceT + Send + Sync + Clone + 'static,
{
	type Error = S::Error;
	type Response = S::Response;

	fn call<'a>(&self, req: Request<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let count = self.count.clone();
		let service = self.service.clone();
		let role = self.role;

		async move {
			let rp = service.call(req).await;
			count.fetch_add(1, Ordering::SeqCst);
			println!("{role} processed calls={} in total", count.load(Ordering::SeqCst));

			rp
		}
	}

	fn batch<'a>(&self, batch: Batch<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let len = batch.len();
		self.count.fetch_add(len, Ordering::SeqCst);
		println!("{}, processed calls={} in total", self.role, self.count.load(Ordering::SeqCst));
		self.service.batch(batch)
	}

	fn notification<'a>(
		&self,
		n: Notification<'a>,
	) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		self.service.notification(n)
	}
}

#[derive(Clone)]
pub struct Logger<S> {
	service: S,
	role: &'static str,
}

impl<S> RpcServiceT for Logger<S>
where
	S: RpcServiceT + Send + Sync,
{
	type Error = S::Error;
	type Response = S::Response;

	fn call<'a>(&self, req: Request<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		println!("{} logger middleware: method `{}`", self.role, req.method);
		self.service.call(req)
	}

	fn batch<'a>(&self, batch: Batch<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		println!("{} logger middleware: batch {batch}", self.role);
		self.service.batch(batch)
	}
	fn notification<'a>(
		&self,
		n: Notification<'a>,
	) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		self.service.notification(n)
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

	for _ in 0..2 {
		let global_cnt = Arc::new(AtomicUsize::new(0));
		let rpc_middleware = RpcServiceBuilder::new()
			.layer_fn(|service| Logger { service, role: "client" })
			// This state is created per connection.
			.layer_fn(|service| CallsPerConn { service, count: Default::default(), role: "client" })
			// This state is shared by all connections.
			.layer_fn(move |service| GlobalCalls { service, count: global_cnt.clone(), role: "client" });
		let client = WsClientBuilder::new().set_rpc_middleware(rpc_middleware).build(&url).await?;
		let response: String = client.request("say_hello", rpc_params![]).await?;
		println!("response: {:?}", response);
		let _response: Result<String, _> = client.request("unknown_method", rpc_params![]).await;
		let _: String = client.request("say_hello", rpc_params![]).await?;
		let _: String = client.request("thready", rpc_params![4]).await?;

		tokio::time::sleep(std::time::Duration::from_millis(100)).await;
	}

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let global_cnt = Arc::new(AtomicUsize::new(0));

	let rpc_middleware = RpcServiceBuilder::new()
		.layer_fn(|service| Logger { service, role: "server" })
		// This state is created per connection.
		.layer_fn(|service| CallsPerConn { service, count: Default::default(), role: "server" })
		// This state is shared by all connections.
		.layer_fn(move |service| GlobalCalls { service, count: global_cnt.clone(), role: "server" })
		// Optional layer that does not do anything, useful if have an optional layer.
		.option_layer(Some(IdentityLayer));
	let server = Server::builder().set_rpc_middleware(rpc_middleware).build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _, _| "lo")?;
	module.register_method("thready", |params, _, _| {
		let thread_count: usize = params.one().unwrap();
		for _ in 0..thread_count {
			std::thread::spawn(|| std::thread::sleep(std::time::Duration::from_secs(1)));
		}
		""
	})?;
	let addr = server.local_addr()?;
	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
