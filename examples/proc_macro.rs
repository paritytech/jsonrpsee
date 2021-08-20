// Copyright 2019 Parity Technologies (UK) Ltd.
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

use jsonrpsee::{
	proc_macros::rpc,
	types::{async_trait, error::Error},
	ws_client::WsClientBuilder,
	ws_server::{SubscriptionSink, WsServerBuilder},
};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use std::net::SocketAddr;

#[rpc(server, client, namespace = "state")]
pub trait Rpc<
	Hash: DeserializeOwned + Serialize + Send + Sync + 'static,
	Prefix: DeserializeOwned + Serialize + Send + Sync + 'static,
>
{
	/// Async method call example.
	#[method(name = "getPairs")]
	async fn storage_pairs(&self, prefix: Prefix, hash: Hash) -> Result<Vec<usize>, Error>;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer<Vec<u8>, usize> for RpcServerImpl {
	async fn storage_pairs(&self, _prefix: usize, _hash: Vec<u8>) -> Result<Vec<usize>, Error> {
		Ok(vec![1, 2, 3, 4])
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();

	let server_addr = run_server().await?;
	let url = format!("ws://{}", server_addr);

	let client = WsClientBuilder::default().build(&url).await?;
	assert_eq!(client.storage_pairs(10, vec![1, 2, 3, 4]).await.unwrap(), vec![1, 2, 3, 4]);

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = WsServerBuilder::default().build("127.0.0.1:0").await?;

	let addr = server.local_addr()?;
	tokio::spawn(async move { server.start(RpcServerImpl.into_rpc()).await });
	Ok(addr)
}
