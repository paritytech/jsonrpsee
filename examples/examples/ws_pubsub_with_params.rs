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
use std::time::Duration;

use futures::{Stream, StreamExt};
use jsonrpsee::core::client::{Subscription, SubscriptionClientT};
use jsonrpsee::core::Serialize;
use jsonrpsee::server::{RpcModule, Server, SubscriptionMessage, TrySendError};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::{rpc_params, PendingSubscriptionSink};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client = WsClientBuilder::default().build(&url).await?;

	// Subscription with a single parameter
	let mut sub_params_one: Subscription<Option<char>> =
		client.subscribe("sub_one_param", rpc_params![3], "unsub_one_param").await?;
	tracing::info!("subscription with one param: {:?}", sub_params_one.next().await);

	// Subscription with multiple parameters
	let mut sub_params_two: Subscription<String> =
		client.subscribe("sub_params_two", rpc_params![2, 5], "unsub_params_two").await?;
	tracing::info!("subscription with two params: {:?}", sub_params_two.next().await);

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	const LETTERS: &str = "abcdefghijklmnopqrstuvxyz";
	let server = Server::builder().set_message_buffer_capacity(10).build("127.0.0.1:0").await?;
	let mut module = RpcModule::new(());
	module
		.register_subscription("sub_one_param", "sub_one_param", "unsub_one_param", |params, pending, _| async move {
			// we are doing this verbose way to get a customized reject error on the subscription.
			let idx = match params.one::<usize>() {
				Ok(p) => p,
				Err(e) => {
					let _ = pending.reject(e).await;
					return Ok(());
				}
			};

			let item = LETTERS.chars().nth(idx);

			let interval = interval(Duration::from_millis(200));
			let stream = IntervalStream::new(interval).map(move |_| item);

			pipe_from_stream_and_drop(pending, stream).await.map_err(Into::into)
		})
		.unwrap();
	module
		.register_subscription("sub_params_two", "params_two", "unsub_params_two", |params, pending, _| async move {
			let (one, two) = params.parse::<(usize, usize)>()?;

			let item = &LETTERS[one..two];
			let interval = interval(Duration::from_millis(200));
			let stream = IntervalStream::new(interval).map(move |_| item);
			pipe_from_stream_and_drop(pending, stream).await.map_err(Into::into)
		})
		.unwrap();

	let addr = server.local_addr()?;
	let handle = server.start(module);

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}

pub async fn pipe_from_stream_and_drop<T: Serialize>(
	pending: PendingSubscriptionSink,
	mut stream: impl Stream<Item = T> + Unpin,
) -> Result<(), anyhow::Error> {
	let mut sink = pending.accept().await?;

	loop {
		tokio::select! {
			_ = sink.closed() => break Err(anyhow::anyhow!("Subscription was closed")),
			maybe_item = stream.next() => {
				let item = match maybe_item {
					Some(item) => item,
					None => break Err(anyhow::anyhow!("Subscription was closed")),
				};
				let msg = SubscriptionMessage::from_json(&item)?;
				match sink.try_send(msg) {
					Ok(_) => (),
					Err(TrySendError::Closed(_)) => break Err(anyhow::anyhow!("Subscription was closed")),
					// channel is full, let's be naive an just drop the message.
					Err(TrySendError::Full(_)) => (),
				}
			}
		}
	}
}
