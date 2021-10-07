use futures_channel::oneshot;
use jsonrpsee::{
	http_server::HttpServerBuilder,
	ws_server::{RpcModule, WsServerBuilder},
};
use tokio::runtime::Runtime;

pub(crate) const SYNC_METHOD_NAME: &str = "say_hello";
pub(crate) const ASYNC_METHOD_NAME: &str = "say_hello_async";
pub(crate) const SUB_METHOD_NAME: &str = "sub";
pub(crate) const UNSUB_METHOD_NAME: &str = "unsub";

/// Run jsonrpsee HTTP server for benchmarks.
/*#[cfg(not(feature = "jsonrpc-crate-servers"))]
pub async fn http_server(rt: &Runtime) -> String {
	let server =
		HttpServerBuilder::default().max_request_body_size(u32::MAX).build("127.0.0.1:0".parse().unwrap()).unwrap();
	let mut module = RpcModule::new(());
	module.register_method(SYNC_METHOD_NAME, |_, _| Ok("lo")).unwrap();
	module.register_async_method(ASYNC_METHOD_NAME, |_, _| async { Ok("lo") }).unwrap();
	let addr = server.local_addr().unwrap();
	rt.spawn(async move { server.start(module).await });
	format!("http://{}", addr)
}

/// Run jsonrpsee WebSocket server for benchmarks.
#[cfg(not(feature = "jsonrpc-crate-servers"))]
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
		server.start(module).await
	});
	format!("ws://{}", server_started_rx.await.unwrap())
}*/

/// Run jsonrpc HTTP server for benchmarks.
pub async fn http_server(handle: tokio::runtime::Handle) -> (String, jsonrpc_http_server::Server) {
	use jsonrpc_http_server::jsonrpc_core::*;
	use jsonrpc_http_server::*;

	let mut io = IoHandler::new();
	io.add_sync_method(SYNC_METHOD_NAME, |_| Ok(Value::String("lo".to_string())));
	io.add_method(ASYNC_METHOD_NAME, |_| async { Ok(Value::String("lo".to_string())) });

	let server = ServerBuilder::new(io)
		.max_request_body_size(usize::MAX)
		.event_loop_executor(handle)
		.start_http(&"127.0.0.1:0".parse().unwrap())
		.expect("Server must start with no issues");

	let addr = *server.address();
	(format!("http://{}", addr), server)
}

/// Run jsonrpc WebSocket server for benchmarks.
pub async fn ws_server(handle: tokio::runtime::Handle) -> (String, jsonrpc_ws_server::Server) {
	use jsonrpc_ws_server::jsonrpc_core::*;
	use jsonrpc_ws_server::*;

	let mut io = IoHandler::new();
	io.add_sync_method(SYNC_METHOD_NAME, |_| Ok(Value::String("lo".to_string())));
	io.add_method(ASYNC_METHOD_NAME, |_| async { Ok(Value::String("lo".to_string())) });

	let server = ServerBuilder::new(io)
		.max_payload(usize::MAX)
		.event_loop_executor(handle)
		.start(&"127.0.0.1:0".parse().unwrap())
		.expect("Server must start with no issues");

	let addr = *server.addr();
	(format!("ws://{}", addr), server)
}

/// Get number of concurrent tasks based on the num_cpus.
pub fn concurrent_tasks() -> Vec<usize> {
	let cores = num_cpus::get();
	vec![cores / 4, cores / 2, cores /*cores * 2, cores * 4*/]
}
