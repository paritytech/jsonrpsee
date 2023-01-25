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

//! Example that shows how to broadcast to all active subscriptions using `tokio::sync::broadcast`.

use std::net::SocketAddr;

use anyhow::anyhow;
use futures::future;
use futures::StreamExt;
use jsonrpsee::core::client::{Subscription, SubscriptionClientT};
use jsonrpsee::core::server::rpc_module::TrySendError;
use jsonrpsee::rpc_params;
use jsonrpsee::server::{RpcModule, ServerBuilder};
use jsonrpsee::ws_client::WsClientBuilder;
use jsonrpsee::PendingSubscriptionSink;
use tokio::sync::broadcast;
use tokio::time::interval;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::wrappers::IntervalStream;

const NUM_SUBSCRIPTION_RESPONSES: usize = 5;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let addr = run_server().await?;
	let url = format!("ws://{}", addr);

	let client1 = WsClientBuilder::default().build(&url).await?;
	let client2 = WsClientBuilder::default().build(&url).await?;
	let sub1: Subscription<i32> = client1.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await?;
	let sub2: Subscription<i32> = client2.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello").await?;

	let fut1 = sub1.take(NUM_SUBSCRIPTION_RESPONSES).for_each(|r| async move { tracing::info!("sub1 rx: {:?}", r) });
	let fut2 = sub2.take(NUM_SUBSCRIPTION_RESPONSES).for_each(|r| async move { tracing::info!("sub2 rx: {:?}", r) });

	future::join(fut1, fut2).await;

	Ok(())
}

async fn run_server() -> anyhow::Result<SocketAddr> {
	// let's configure the server only hold 5 messages in memory.
	let server = ServerBuilder::default().set_buffer_size(5).build("127.0.0.1:0").await?;
	let (tx, _rx) = broadcast::channel::<usize>(16);

	let mut module = RpcModule::new(tx.clone());

	std::thread::spawn(move || produce_items(tx));

	module
		.register_subscription("subscribe_hello", "s_hello", "unsubscribe_hello", |_, pending, tx| async move {
			let rx = tx.subscribe();
			let stream = BroadcastStream::new(rx);
			pipe_from_stream_with_bounded_buffer(pending, stream).await?;

			Ok(())
		})
		.unwrap();
	let addr = server.local_addr()?;
	let handle = server.start(module)?;

	// In this example we don't care about doing shutdown so let's it run forever.
	// You may use the `ServerHandle` to shut it down or manage it yourself.
	tokio::spawn(handle.stopped());

	Ok(addr)
}

async fn pipe_from_stream_with_bounded_buffer(
	pending: PendingSubscriptionSink,
	mut stream: BroadcastStream<usize>,
) -> anyhow::Result<()> {
	let mut sink = pending.accept().await.map_err(|e| anyhow!("{:?}", e))?;
	let mut bounded_buffer = bounded_vec_deque::BoundedVecDeque::new(10);

	// This is not recommended approach it would be better to have a queue that returns a future once
	// a element is ready to consumed in the bounded_buffer. This might waste CPU with the timeouts
	// as it's no guarantee that buffer has elements.
	let mut timeout = IntervalStream::new(interval(std::time::Duration::from_millis(100)));

	loop {
		tokio::select! {
			// subscription was closed, no point of reading more messages from the stream.
			_ = sink.closed() => {
				return Ok(());
			}
			// stream yielded a new element, try to send it.
			Some(Ok(v)) = stream.next() => {
				let msg = sink.build_message(&v).expect("usize serialize is infalliable; qed");
				match sink.try_send(msg) {
					// message was sent successfully.
					Ok(()) => (),
					// the connection is closed.
					Err(TrySendError::Closed(_msg_unset_dont_care)) => {
						tracing::warn!("The connection closed; closing subscription");
						return Ok(());
					}
					// the channel was full let's buffer the ten most recent messages.
					Err(TrySendError::Full(unsent_msg)) => {
						if let Some(msg) = bounded_buffer.push_back(unsent_msg) {
							tracing::warn!("Dropping the oldest message: {:?}", msg);
						}
					}
				};
			}
			// try to pop to oldest element for our bounded buffer and send it.
			_ = timeout.next() => {
				if let Some(msg) = bounded_buffer.pop_front() {
					match sink.try_send(msg) {
						// message was sent successfully.
						Ok(()) => (),
						// the connection is closed.
						Err(TrySendError::Closed(_msg_unset_dont_care)) => {
							tracing::warn!("The connection closed; closing subscription");
							return Ok(());
						}
						// the channel was full, let's insert back the pop:ed element.
						Err(TrySendError::Full(unsent_msg)) => {
							bounded_buffer.push_front(unsent_msg).expect("pop:ed an element must be space in; qed");
						}
					};
				}
			}
			else => return Ok(()),
		}
	}
}

// Naive example that broadcasts the produced values to all active subscribers.
fn produce_items(tx: broadcast::Sender<usize>) {
	for c in 1..=100 {
		std::thread::sleep(std::time::Duration::from_millis(1));

		// This might fail if no receivers are alive, could occur if no subscriptions are active...
		// Also be aware that this will succeed when at least one receiver is alive
		// Thus, clients connecting at different point in time will not receive
		// the items sent before the subscription got established.
		let _ = tx.send(c);
	}
}
