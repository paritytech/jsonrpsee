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

mod helpers;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use helpers::init_logger;
use jsonrpsee::core::{async_trait, client::ClientT, Error};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::middleware::rpc::{RpcServiceBuilder, RpcServiceT, TransportProtocol};
use jsonrpsee::server::{Server, ServerHandle};
use jsonrpsee::types::{ErrorObject, ErrorObjectOwned, Id, Request};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::RpcModule;
use jsonrpsee::{rpc_params, MethodResponse};
use tokio::time::sleep;

#[derive(Default, Clone)]
struct Counter {
	/// (Number of started requests, number of finished requests)
	requests: (u32, u32),
	/// Mapping method names to (number of calls, ids of successfully completed calls)
	calls: HashMap<String, (u32, Vec<Id<'static>>)>,
}

#[derive(Clone)]
pub struct CounterMiddleware<S> {
	service: S,
	counter: Arc<Mutex<Counter>>,
}

#[async_trait]
impl<'a, S> RpcServiceT<'a> for CounterMiddleware<S>
where
	S: RpcServiceT<'a> + Send + Sync,
{
	async fn call(&self, request: Request<'a>, transport: TransportProtocol) -> MethodResponse {
		let name = request.method.to_string();
		let id = request.id.clone();

		{
			let mut n = self.counter.lock().unwrap();
			n.requests.0 += 1;
			let entry = n.calls.entry(name.clone()).or_insert((0, Vec::new()));
			entry.0 += 1;
		}

		let rp = self.service.call(request, transport).await;

		{
			let mut n = self.counter.lock().unwrap();
			n.requests.1 += 1;
			if rp.is_success() {
				n.calls.get_mut(&name).unwrap().1.push(id.into_owned());
			}
		}

		rp
	}
}

fn test_module() -> RpcModule<()> {
	#[rpc(server)]
	pub trait Rpc {
		#[method(name = "say_hello")]
		async fn hello(&self) -> String {
			sleep(Duration::from_millis(50)).await;
			"hello".to_string()
		}

		#[method(name = "err")]
		async fn err(&self) -> Result<String, ErrorObjectOwned> {
			Err(ErrorObject::owned(1, "err", None::<()>))
		}
	}

	impl RpcServer for () {}

	().into_rpc()
}

async fn websocket_server(
	module: RpcModule<()>,
	counter: Arc<Mutex<Counter>>,
) -> Result<(SocketAddr, ServerHandle), Error> {
	let rpc_middleware =
		RpcServiceBuilder::new().layer_fn(move |service| CounterMiddleware { service, counter: counter.clone() });
	let server = Server::builder().set_rpc_middleware(rpc_middleware).build("127.0.0.1:0").await?;

	let addr = server.local_addr()?;
	let handle = server.start(module);

	Ok((addr, handle))
}

async fn http_server(module: RpcModule<()>, counter: Arc<Mutex<Counter>>) -> Result<(SocketAddr, ServerHandle), Error> {
	let rpc_middleware =
		RpcServiceBuilder::new().layer_fn(move |service| CounterMiddleware { service, counter: counter.clone() });
	let server = Server::builder().set_rpc_middleware(rpc_middleware).build("127.0.0.1:0").await?;

	let addr = server.local_addr()?;
	let handle = server.start(module);

	Ok((addr, handle))
}

#[tokio::test]
async fn ws_server_logger() {
	init_logger();

	let counter: Arc<Mutex<Counter>> = Default::default();
	let (server_addr, server_handle) = websocket_server(test_module(), counter.clone()).await.unwrap();

	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");

	let res: Result<String, Error> = client.request("unknown_method", rpc_params![]).await;
	assert!(res.is_err());

	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");
	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");

	let res: Result<String, Error> = client.request("unknown_method", rpc_params![]).await;
	assert!(res.is_err());

	let res: Result<String, Error> = client.request("err", rpc_params![]).await;
	assert!(res.is_err());

	{
		let inner = counter.lock().unwrap();

		assert_eq!(inner.requests, (6, 6));
		assert_eq!(inner.calls["say_hello"], (3, vec![Id::Number(0), Id::Number(2), Id::Number(3)]));
		assert_eq!(inner.calls["err"], (1, vec![]));
		assert_eq!(inner.calls["unknown_method"], (2, vec![]));
	}

	server_handle.stop().unwrap();
	server_handle.stopped().await;
}

#[tokio::test]
async fn http_server_logger() {
	init_logger();

	let counter: Arc<Mutex<Counter>> = Default::default();
	let (server_addr, server_handle) = http_server(test_module(), counter.clone()).await.unwrap();

	let server_url = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&server_url).unwrap();

	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");

	let res: Result<String, Error> = client.request("unknown_method", rpc_params![]).await;
	assert!(res.is_err());

	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");
	let res: String = client.request("say_hello", rpc_params![]).await.unwrap();
	assert_eq!(res, "hello");

	let res: Result<String, Error> = client.request("unknown_method", rpc_params![]).await;
	assert!(res.is_err());

	let res: Result<String, Error> = client.request("err", rpc_params![]).await;
	assert!(res.is_err());

	{
		let inner = counter.lock().unwrap();
		assert_eq!(inner.requests, (6, 6));
		assert_eq!(inner.calls["say_hello"], (3, vec![Id::Number(0), Id::Number(2), Id::Number(3)]));
		assert_eq!(inner.calls["unknown_method"], (2, vec![]));
		assert_eq!(inner.calls["err"], (1, vec![]));
	}

	server_handle.stop().unwrap();
	server_handle.stopped().await;
}
