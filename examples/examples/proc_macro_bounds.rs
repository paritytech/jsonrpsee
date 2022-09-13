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

use jsonrpsee::core::{async_trait, Error};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::ServerBuilder;
use jsonrpsee::ws_client::WsClientBuilder;

type ExampleHash = [u8; 32];

pub trait Config {
	type Hash: Send + Sync + 'static;
}

impl Config for ExampleHash {
	type Hash = Self;
}

/// The RPC macro requires `DeserializeOwned` for output types for the client implementation, while the
/// server implementation requires output types to be bounded by `Serialize`.
///
/// In this example, we don't want the `Conf` to be bounded by default to
/// `Conf : Send + Sync + 'static + jsonrpsee::core::DeserializeOwned` for client implementation and
/// `Conf : Send + Sync + 'static + jsonrpsee::core::Serialize` for server implementation.
///
/// Explicitly, specify client and server bounds to handle the `Serialize` and `DeserializeOwned` cases
/// just for the `Conf::hash` part.
#[rpc(server, client, namespace = "foo", client_bounds(T::Hash: jsonrpsee::core::DeserializeOwned), server_bounds(T::Hash: jsonrpsee::core::Serialize))]
pub trait Rpc<T: Config> {
	#[method(name = "bar")]
	fn method(&self) -> Result<T::Hash, Error>;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer<ExampleHash> for RpcServerImpl {
	fn method(&self) -> Result<<ExampleHash as Config>::Hash, Error> {
		Ok([0u8; 32])
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
	assert_eq!(RpcClient::<ExampleHash>::method(&client).await.unwrap(), [0u8; 32]);

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = ServerBuilder::default().build("127.0.0.1:0").await?;

	let addr = server.local_addr()?;
	let handle = server.start(RpcServerImpl.into_rpc())?;

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(async move { handle.stopped().await });

	Ok(addr)
}
