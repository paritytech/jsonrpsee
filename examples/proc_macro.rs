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

use jsonrpsee::{
	proc_macros::rpc,
	types::{async_trait, error::Error, Subscription},
	ws_client::WsClientBuilder,
	ws_server::{SubscriptionSink, WsServerBuilder, WsStopHandle},
	RpcModule,
};
use std::net::SocketAddr;

type ExampleHash = [u8; 32];
type ExampleStorageKey = Vec<u8>;

#[rpc(server, client, namespace = "state")]
pub trait Rpc<Hash: Clone, StorageKey>
where
	Hash: std::fmt::Debug,
{
	/// Async method call example.
	#[method(name = "getKeys")]
	async fn storage_keys(&self, storage_key: StorageKey, hash: Option<Hash>) -> Result<Vec<StorageKey>, Error>;

	/// Subscription that takes a `StorageKey` as input and produces a `Vec<Hash>`.
	#[subscription(name = "subscribeStorage", item = Vec<Hash>)]
	fn subscribe_storage(&self, keys: Option<Vec<StorageKey>>) -> Result<(), Error>;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer<ExampleHash, ExampleStorageKey> for RpcServerImpl {
	async fn storage_keys(
		&self,
		storage_key: ExampleStorageKey,
		_hash: Option<ExampleHash>,
	) -> Result<Vec<ExampleStorageKey>, Error> {
		Ok(vec![storage_key])
	}

	fn subscribe_storage(
		&self,
		mut sink: SubscriptionSink,
		_keys: Option<Vec<ExampleStorageKey>>,
	) -> Result<(), Error> {
		sink.send(&vec![[0; 32]])
	}
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();

	let (server_addr, _handle) = run_server().await?;
	let url = format!("ws://{}", server_addr);

	let client = WsClientBuilder::default().build(&url).await?;
	assert_eq!(client.storage_keys(vec![1, 2, 3, 4], None::<ExampleHash>).await.unwrap(), vec![vec![1, 2, 3, 4]]);

	let mut sub: Subscription<Vec<ExampleHash>> =
		RpcClient::<ExampleHash, ExampleStorageKey>::subscribe_storage(&client, None).await.unwrap();
	assert_eq!(Some(vec![[0; 32]]), sub.next().await.unwrap());

	Ok(())
}

async fn run_server() -> anyhow::Result<(SocketAddr, WsStopHandle)> {
	let server = WsServerBuilder::default().build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module.register_method("state_getPairs", |_, _| Ok(vec![1, 2, 3]))?;

	let addr = server.local_addr()?;
	let handle = server.start(module)?;
	Ok((addr, handle))
}
