#![cfg(test)]

use crate::client::{WsClient, WsConfig, WsSubscription};
use crate::types::jsonrpc::{JsonValue, Params};
use crate::ws::WsServer;

use std::net::SocketAddr;
use std::time::Duration;

use futures::channel::oneshot::{self, Sender};
use futures::future::FutureExt;

pub fn server(server_started: Sender<SocketAddr>) {
	std::thread::spawn(move || {
		let mut rt = tokio::runtime::Runtime::new().unwrap();

		let server = rt.block_on(WsServer::new("127.0.0.1:0")).unwrap();
		let mut sub =
			server.register_subscription("subscribe_hello".to_owned(), "unsubscribe_hello".to_owned()).unwrap();
		server_started.send(*server.local_addr()).unwrap();
		let mut call = server.register_method("say_hello".to_owned()).unwrap();

		rt.block_on(async move {
			loop {
				let hello_fut = async {
					let handle = call.next().await;
					handle.respond(Ok(JsonValue::String("hello".to_owned()))).await.unwrap();
				}
				.fuse();

				let timeout = tokio::time::delay_for(Duration::from_millis(200)).fuse();
				futures::pin_mut!(hello_fut, timeout);

				futures::select! {
					_ = hello_fut => (),
					_ = timeout => {
						sub.send(JsonValue::String("hello from subscription".to_owned())).await.unwrap();
					}
				}
			}
		});
	});
}

#[tokio::test]
async fn subscription_without_polling_doesnt_make_client_unuseable() {
	env_logger::init();
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	server(server_started_tx);
	let server_addr = server_started_rx.await.unwrap();

	let uri = format!("ws://{}", server_addr);
	let client =
		WsClient::new(&uri, WsConfig { subscription_channel_capacity: 4, ..Default::default() }).await.unwrap();
	let mut hello_sub: WsSubscription<JsonValue> =
		client.subscribe("subscribe_hello", Params::None, "unsubscribe_hello").await.unwrap();

	// don't poll the subscription stream for 2 seconds, should be full now.
	std::thread::sleep(Duration::from_secs(2));

	// Capacity is `num_sender` + `capacity`
	for _ in 0..5 {
		assert!(hello_sub.next().await.is_some());
	}

	// NOTE: this is now unuseable and unregistered.
	assert!(hello_sub.next().await.is_none());

	// The client should still be useable => make sure it still works.
	let _hello_req: JsonValue = client.request("say_hello", Params::None).await.unwrap();

	// The same subscription should be possible to register again.
	let mut other_sub: WsSubscription<JsonValue> =
		client.subscribe("subscribe_hello", Params::None, "unsubscribe_hello").await.unwrap();

	other_sub.next().await.unwrap();
}
