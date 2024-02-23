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

use jsonrpsee::core::{async_trait, client::Subscription};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::{PendingSubscriptionSink, Server, SubscriptionMessage};
use jsonrpsee::types::ErrorObjectOwned;
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::ConnectionId;

#[rpc(server, client)]
pub trait Rpc {
	/// Raw method with connection ID.
	#[method(name = "connectionIdMethod", raw_method)]
	async fn raw_method(&self, first_param: usize, second_param: u16) -> Result<usize, ErrorObjectOwned>;

	/// Normal method call example.
	#[method(name = "normalMethod")]
	fn normal_method(&self, first_param: usize, second_param: u16) -> Result<usize, ErrorObjectOwned>;

	/// Subscriptions expose the connection ID on the subscription sink.
	#[subscription(name = "subscribeSync" => "sync", item = usize)]
	fn sub(&self, first_param: usize);
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn raw_method(
		&self,
		connection_id: ConnectionId,
		_first_param: usize,
		_second_param: u16,
	) -> Result<usize, ErrorObjectOwned> {
		// Return the connection ID from which this method was called.
		Ok(connection_id)
	}

	fn normal_method(&self, _first_param: usize, _second_param: u16) -> Result<usize, ErrorObjectOwned> {
		// The normal method does not have access to the connection ID.
		Ok(usize::MAX)
	}

	fn sub(&self, pending: PendingSubscriptionSink, _first_param: usize) {
		tokio::spawn(async move {
			// The connection ID can be obtained before or after accepting the subscription
			let pending_connection_id = pending.connection_id();
			let sink = pending.accept().await.unwrap();
			let sink_connection_id = sink.connection_id();

			assert_eq!(pending_connection_id, sink_connection_id);

			let msg = SubscriptionMessage::from_json(&sink_connection_id).unwrap();
			sink.send(msg).await.unwrap();
		});
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
	let connection_id_first = client.raw_method(1, 2).await.unwrap();

	// Second call from the same connection ID.
	assert_eq!(client.raw_method(1, 2).await.unwrap(), connection_id_first);

	// Second client will increment the connection ID.
	let client_second = WsClientBuilder::default().build(&url).await?;
	let connection_id_second = client_second.raw_method(1, 2).await.unwrap();
	assert_ne!(connection_id_first, connection_id_second);

	let mut sub: Subscription<usize> = RpcClient::sub(&client, 0).await.unwrap();
	assert_eq!(connection_id_first, sub.next().await.transpose().unwrap().unwrap());

	let mut sub: Subscription<usize> = RpcClient::sub(&client_second, 0).await.unwrap();
	assert_eq!(connection_id_second, sub.next().await.transpose().unwrap().unwrap());

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	let server = Server::builder().build("127.0.0.1:0").await?;

	let addr = server.local_addr()?;
	let handle = server.start(RpcServerImpl.into_rpc());

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}
