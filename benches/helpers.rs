pub(crate) const SYNC_METHOD_NAME: &str = "say_hello";
pub(crate) const ASYNC_METHOD_NAME: &str = "say_hello_async";
pub(crate) const SUB_METHOD_NAME: &str = "sub";
pub(crate) const UNSUB_METHOD_NAME: &str = "unsub";

/// Run jsonrpc HTTP server for benchmarks.
#[cfg(feature = "jsonrpc-crate")]
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
#[cfg(feature = "jsonrpc-crate")]
pub async fn ws_server(handle: tokio::runtime::Handle) -> (String, jsonrpc_ws_server::Server) {
	use std::sync::atomic::{AtomicU64, Ordering};

	use jsonrpc_pubsub::{PubSubHandler, Session, Subscriber, SubscriptionId};
	use jsonrpc_ws_server::jsonrpc_core::*;
	use jsonrpc_ws_server::*;

	const ID: AtomicU64 = AtomicU64::new(0);

	let handle2 = handle.clone();

	let mut io = PubSubHandler::new(MetaIoHandler::default());
	io.add_sync_method(SYNC_METHOD_NAME, |_| Ok(Value::String("lo".to_string())));
	io.add_method(ASYNC_METHOD_NAME, |_| async { Ok(Value::String("lo".to_string())) });
	io.add_subscription(
		SUB_METHOD_NAME,
		(SUB_METHOD_NAME, move |_params: Params, _, subscriber: Subscriber| {
			handle2.spawn(async move {
				let id = ID.fetch_add(1, Ordering::Relaxed);
				let sink = subscriber.assign_id(SubscriptionId::Number(id)).unwrap();
				// NOTE(niklasad1): the way jsonrpc works this is the only way to get this working
				// -> jsonrpc responds to the request in background so not possible to know when the request
				// has been answered, so this benchmark is bad.
				tokio::time::sleep(std::time::Duration::from_millis(100)).await;
				let mut map = serde_json::Map::new();
				map.insert("subscription".into(), id.into());
				map.insert("result".into(), "hello".into());
				let _ = sink.notify(Params::Map(map));
			});
		}),
		(UNSUB_METHOD_NAME, |_id: SubscriptionId, _| futures::future::ok(Value::Bool(true))),
	);

	let server = ServerBuilder::with_meta_extractor(io, |context: &RequestContext| {
		std::sync::Arc::new(Session::new(context.sender().clone()))
	})
	.event_loop_executor(handle)
	.max_connections(10 * 1024)
	.start(&"127.0.0.1:0".parse().unwrap())
	.expect("Server must start with no issues");

	let addr = *server.addr();
	(format!("ws://{}", addr), server)
}

/// Run jsonrpsee HTTP server for benchmarks.
#[cfg(not(feature = "jsonrpc-crate"))]
pub async fn http_server(handle: tokio::runtime::Handle) -> (String, jsonrpsee::http_server::HttpServerHandle) {
	use jsonrpsee::http_server::{HttpServerBuilder, RpcModule};

	let server = HttpServerBuilder::default()
		.max_request_body_size(u32::MAX)
		.custom_tokio_runtime(handle)
		.build("127.0.0.1:0")
		.unwrap();
	let mut module = RpcModule::new(());
	module.register_method(SYNC_METHOD_NAME, |_, _| Ok("lo")).unwrap();
	module.register_async_method(ASYNC_METHOD_NAME, |_, _| async { Ok("lo") }).unwrap();
	let addr = server.local_addr().unwrap();
	let handle = server.start(module).unwrap();
	(format!("http://{}", addr), handle)
}

/// Run jsonrpsee WebSocket server for benchmarks.
#[cfg(not(feature = "jsonrpc-crate"))]
pub async fn ws_server(handle: tokio::runtime::Handle) -> (String, jsonrpsee::ws_server::WsServerHandle) {
	use jsonrpsee::ws_server::{RpcModule, WsServerBuilder};

	let server = WsServerBuilder::default()
		.max_request_body_size(u32::MAX)
		.max_connections(10 * 1024)
		.custom_tokio_runtime(handle)
		.build("127.0.0.1:0")
		.await
		.unwrap();
	let mut module = RpcModule::new(());
	module.register_method(SYNC_METHOD_NAME, |_, _| Ok("lo")).unwrap();
	module.register_async_method(ASYNC_METHOD_NAME, |_, _| async { Ok("lo") }).unwrap();
	module
		.register_subscription(SUB_METHOD_NAME, SUB_METHOD_NAME, UNSUB_METHOD_NAME, |_params, mut sink, _ctx| {
			let x = "Hello";
			tokio::spawn(async move { sink.send(&x) });
			Ok(())
		})
		.unwrap();
	let addr = format!("ws://{}", server.local_addr().unwrap());
	let handle = server.start(module).unwrap();
	(addr, handle)
}
