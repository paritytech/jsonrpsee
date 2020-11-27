use jsonrpsee::http::{HttpConfig, HttpServer};
use jsonrpsee::types::jsonrpc::JsonValue;
use jsonrpsee::ws::WsServer;

use std::net::SocketAddr;
use std::time::Duration;

use futures::channel::oneshot::Sender;
use futures::future::FutureExt;

pub fn websocket_server(server_started: Sender<SocketAddr>) {
	std::thread::spawn(move || {
		let mut rt = tokio::runtime::Runtime::new().unwrap();

		let server = rt.block_on(WsServer::new("127.0.0.1:0")).unwrap();
		let mut sub_hello =
			server.register_subscription("subscribe_hello".to_owned(), "unsubscribe_hello".to_owned()).unwrap();
		let mut sub_foo =
			server.register_subscription("subscribe_foo".to_owned(), "unsubscribe_foo".to_owned()).unwrap();
		server_started.send(*server.local_addr()).unwrap();
		let mut call = server.register_method("say_hello".to_owned()).unwrap();

		rt.block_on(async move {
			loop {
				let hello_fut = async {
					let handle = call.next().await;
					handle.respond(Ok(JsonValue::String("hello".to_owned()))).await.unwrap();
				}
				.fuse();

				let timeout = tokio::time::delay_for(Duration::from_millis(100)).fuse();
				futures::pin_mut!(hello_fut, timeout);

				futures::select! {
					_ = hello_fut => (),
					_ = timeout => {
				  sub_hello.send(JsonValue::String("hello from subscription".to_owned())).await.unwrap();
				  sub_foo.send(JsonValue::Number(1337_u64.into())).await.unwrap();
					}
				}
			}
		});
	});
}

pub fn http_server(server_started: Sender<SocketAddr>) {
	std::thread::spawn(move || {
		let mut rt = tokio::runtime::Runtime::new().unwrap();

		let server = rt.block_on(HttpServer::new("127.0.0.1:0", HttpConfig::default())).unwrap();
		server_started.send(*server.local_addr()).unwrap();
		let mut call = server.register_method("say_hello".to_owned()).unwrap();

		rt.block_on(async move {
			loop {
				let handle = call.next().await;
				handle.respond(Ok(JsonValue::String("hello".to_owned()))).await.unwrap();
			}
		});
	});
}
