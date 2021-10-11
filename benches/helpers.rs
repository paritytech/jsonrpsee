use futures_channel::oneshot;
use jsonrpsee::{
	http_server::HttpServerBuilder,
	ws_server::{RpcModule, WsServerBuilder},
};

pub(crate) const SYNC_METHOD_NAME: &str = "say_hello";
pub(crate) const ASYNC_METHOD_NAME: &str = "say_hello_async";
pub(crate) const SUB_METHOD_NAME: &str = "sub";
pub(crate) const UNSUB_METHOD_NAME: &str = "unsub";

/// Run jsonrpsee HTTP server for benchmarks.
pub async fn http_server() -> String {
	let (server_started_tx, server_started_rx) = oneshot::channel();
	tokio::spawn(async move {
		let server =
			HttpServerBuilder::default().max_request_body_size(u32::MAX).build("127.0.0.1:0".parse().unwrap()).unwrap();
		let mut module = RpcModule::new(());
		module.register_method(SYNC_METHOD_NAME, |_, _| Ok("lo")).unwrap();
		module.register_async_method(ASYNC_METHOD_NAME, |_, _| async { Ok("lo") }).unwrap();
		server_started_tx.send(server.local_addr().unwrap()).unwrap();
		server.start(module).await
	});
	format!("http://{}", server_started_rx.await.unwrap())
}

/// Run jsonrpsee WebSocket server for benchmarks.
pub async fn ws_server() -> String {
	let (server_started_tx, server_started_rx) = oneshot::channel();
	tokio::spawn(async move {
		let server = WsServerBuilder::default().build("127.0.0.1:0").await.unwrap();
		let mut module = RpcModule::new(());
		module.register_method(SYNC_METHOD_NAME, |_, _| Ok("lo")).unwrap();
		module.register_async_method(ASYNC_METHOD_NAME, |_, _| async { Ok("lo") }).unwrap();
		module
			.register_subscription(SUB_METHOD_NAME, UNSUB_METHOD_NAME, |_params, mut sink, _ctx| {
				let x = "Hello";
				tokio::spawn(async move { sink.send(&x) });
				Ok(())
			})
			.unwrap();

		server_started_tx.send(server.local_addr().unwrap()).unwrap();
		server.start(module).unwrap()
	});
	format!("ws://{}", server_started_rx.await.unwrap())
}

/// Get number of concurrent tasks based on the num_cpus.
pub fn concurrent_tasks() -> Vec<usize> {
	let cores = num_cpus::get();
	vec![cores / 4, cores / 2, cores, cores * 2, cores * 4]
}
