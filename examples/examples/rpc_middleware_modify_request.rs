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

use jsonrpsee::core::{async_trait, client::ClientT};
use jsonrpsee::server::middleware::rpc::{RpcServiceBuilder, RpcServiceT, TransportProtocol};
use jsonrpsee::server::Server;
use jsonrpsee::types::Request;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{rpc_params, MethodResponse, RpcModule};
use std::borrow::Cow as StdCow;
use std::net::SocketAddr;

#[derive(Clone)]
pub struct ModifyRequestIf<S>(S);

#[async_trait]
impl<'a, S> RpcServiceT<'a> for ModifyRequestIf<S>
where
	S: Send + Sync + RpcServiceT<'a>,
{
	async fn call(&self, mut req: Request<'a>, t: TransportProtocol) -> MethodResponse {
		// Example how to modify the params in the call.
		if req.method == "say_hello" {
			// It's a bit awkward to create new params in the request
			// but this shows how to do it.
			let raw_value = serde_json::value::to_raw_value("myparams").unwrap();
			req.params = Some(StdCow::Owned(raw_value));
		}
		// Re-direct all calls that isn't `say_hello` to `say_goodbye`
		else if req.method != "say_hello" {
			req.method = "say_goodbye".into();
		}

		self.0.call(req, t).await
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

	let client = WsClientBuilder::default().build(&url).await?;
	let _response: String = client.request("say_hello", rpc_params![]).await?;
	let _response: Result<String, _> = client.request("unknown_method", rpc_params![]).await;
	let _: String = client.request("say_hello", rpc_params![]).await?;

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let rpc_middleware = RpcServiceBuilder::new().layer_fn(|service| ModifyRequestIf(service));
	let server = Server::builder().set_rpc_middleware(rpc_middleware).build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _| "lo")?;
	module.register_method("say_goodbye", |_, _| "goodbye")?;
	let addr = server.local_addr()?;

	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
