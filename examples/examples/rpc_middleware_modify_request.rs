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

use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::middleware::{Batch, BatchEntry, Notification, RpcServiceBuilder, RpcServiceT};
use jsonrpsee::server::Server;
use jsonrpsee::types::Request;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{RpcModule, rpc_params};
use std::borrow::Cow as StdCow;
use std::net::SocketAddr;

fn modify_method_call(req: &mut Request<'_>) {
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
}

fn modify_notif(n: &mut Notification<'_>) {
	// Example how to modify the params in the notification.
	if n.method == "say_hello" {
		// It's a bit awkward to create new params in the request
		// but this shows how to do it.
		let raw_value = serde_json::value::to_raw_value("myparams").unwrap();
		n.params = Some(StdCow::Owned(raw_value));
	}
	// Re-direct all notifs that isn't `say_hello` to `say_goodbye`
	else if n.method != "say_hello" {
		n.method = "say_goodbye".into();
	}
}

#[derive(Clone)]
pub struct ModifyRequestIf<S>(S);

impl<S> RpcServiceT for ModifyRequestIf<S>
where
	S: Send + Sync + RpcServiceT,
{
	type Error = S::Error;
	type Response = S::Response;

	fn call<'a>(&self, mut req: Request<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		modify_method_call(&mut req);
		self.0.call(req)
	}

	fn batch<'a>(&self, mut batch: Batch<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		for call in batch.as_mut_batch_entries() {
			match call {
				Ok(BatchEntry::Call(call)) => {
					modify_method_call(call);
				}
				Ok(BatchEntry::Notification(n)) => {
					modify_notif(n);
				}
				// Invalid request, we don't care about it.
				Err(_err) => {}
			}
		}

		self.0.batch(batch)
	}

	fn notification<'a>(
		&self,
		mut n: Notification<'a>,
	) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		modify_notif(&mut n);
		self.0.notification(n)
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
	let rpc_middleware = RpcServiceBuilder::new().layer_fn(ModifyRequestIf);
	let server = Server::builder().set_rpc_middleware(rpc_middleware).build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("say_hello", |_, _, _| "lo")?;
	module.register_method("say_goodbye", |_, _, _| "goodbye")?;
	let addr = server.local_addr()?;

	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
