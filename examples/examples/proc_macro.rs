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

use jsonrpsee::core::{async_trait, client::Subscription, Error};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::{PendingSubscriptionSink, ServerBuilder};
use jsonrpsee::types::SubscriptionResult;
use jsonrpsee::ws_client::WsClientBuilder;

type ExampleHash = [u8; 32];
type ExampleStorageKey = Vec<u8>;

#[rpc(client, server)]
pub trait DupOverride {
	#[subscription(name = "subscribeOne" => "override", item = u8)]
	async fn one(&self) -> jsonrpsee::core::SubscriptionResult;
	/*#[subscription(name = "subscribeTwo" => "override", item = u8)]
	async fn two(&self) -> jsonrpsee::core::SubscriptionResult;*/
}

#[rpc(server, client, namespace = "state")]
pub trait Rpc<Hash: Clone, StorageKey>
where
	Hash: std::fmt::Debug,
{
	/// Async method call example.
	#[method(name = "getKeys")]
	async fn storage_keys(&self, storage_key: StorageKey, hash: Option<Hash>) -> Result<Vec<StorageKey>, Error>;

	/// Subscription that takes a `StorageKey` as input and produces a `Vec<Hash>`.
	#[subscription(name = "subscribeStorage" => "override", item = Vec<Hash>)]
	async fn subscribe_storage(&self, keys: Option<Vec<StorageKey>>) -> SubscriptionResult;
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

	// Note that the server's subscription method must return `SubscriptionResult`.
	async fn subscribe_storage(
		&self,
		pending: PendingSubscriptionSink,
		_keys: Option<Vec<ExampleStorageKey>>,
	) -> SubscriptionResult {
		let sink = pending.accept().await?;
		let msg = sink.build_message(&vec![[0; 32]]).unwrap();
		sink.send(msg).await.unwrap();

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
	assert_eq!(client.storage_keys(vec![1, 2, 3, 4], None::<ExampleHash>).await.unwrap(), vec![vec![1, 2, 3, 4]]);

	let mut sub: Subscription<Vec<ExampleHash>> =
		RpcClient::<ExampleHash, ExampleStorageKey>::subscribe_storage(&client, None).await.unwrap();
	assert_eq!(Some(vec![[0; 32]]), sub.next().await.transpose().unwrap());

	sub.unsubscribe().await.unwrap();

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = ServerBuilder::default().build("127.0.0.1:0").await?;

	let addr = server.local_addr()?;
	let handle = server.start(RpcServerImpl.into_rpc())?;

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
