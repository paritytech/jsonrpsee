use jsonrpsee::http_client::{HttpClient, HttpClientBuilder};
use jsonrpsee::ws_client::{WsClient, WsClientBuilder};

pub(crate) const SYNC_FAST_CALL: &str = "fast_call";
pub(crate) const ASYNC_FAST_CALL: &str = "fast_async";
pub(crate) const SYNC_MEM_CALL: &str = "memory_intense";
pub(crate) const ASYNC_MEM_CALL: &str = "memory_intense_async";
pub(crate) const SYNC_SLOW_CALL: &str = "slow_call";
pub(crate) const ASYNC_SLOW_CALL: &str = "slow_call_async";
pub(crate) const SUB_METHOD_NAME: &str = "sub";
pub(crate) const UNSUB_METHOD_NAME: &str = "unsub";

pub(crate) const SYNC_METHODS: [&str; 3] = [SYNC_FAST_CALL, SYNC_MEM_CALL, SYNC_SLOW_CALL];
pub(crate) const ASYNC_METHODS: [&str; 3] = [SYNC_FAST_CALL, SYNC_MEM_CALL, SYNC_SLOW_CALL];

/// Run jsonrpc HTTP server for benchmarks.
#[cfg(feature = "jsonrpc-crate")]
pub async fn http_server(handle: tokio::runtime::Handle) -> (String, jsonrpc_http_server::Server) {
	use jsonrpc_http_server::jsonrpc_core::*;
	use jsonrpc_http_server::*;

	let mut io = IoHandler::new();
	io.add_sync_method(SYNC_FAST_CALL, |_| Ok(Value::String("lo".to_string())));
	io.add_method(ASYNC_FAST_CALL, |_| async { Ok(Value::String("lo".to_string())) });
	io.add_sync_method(SYNC_MEM_CALL, |_| Ok(Value::String("A".repeat(1 * 1024 * 1024))));
	io.add_method(ASYNC_MEM_CALL, |_| async { Ok(Value::String("A".repeat(1 * 1024 * 1024))) });
	io.add_sync_method(SYNC_SLOW_CALL, |_| {
		std::thread::sleep(std::time::Duration::from_millis(1));
		Ok(Value::String("slow call".to_string()))
	});
	io.add_method(ASYNC_SLOW_CALL, |_| async {
		tokio::time::sleep(std::time::Duration::from_millis(1)).await;
		Ok(Value::String("slow call async".to_string()))
	});

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
	io.add_sync_method(SYNC_FAST_CALL, |_| Ok(Value::String("lo".to_string())));
	io.add_method(ASYNC_FAST_CALL, |_| async { Ok(Value::String("lo".to_string())) });
	io.add_sync_method(SYNC_MEM_CALL, |_| Ok(Value::String("A".repeat(1 * 1024 * 1024))));
	io.add_method(ASYNC_MEM_CALL, |_| async { Ok(Value::String("A".repeat(1 * 1024 * 1024))) });
	io.add_sync_method(SYNC_SLOW_CALL, |_| {
		std::thread::sleep(std::time::Duration::from_millis(1));
		Ok(Value::String("slow call".to_string()))
	});
	io.add_method(ASYNC_SLOW_CALL, |_| async {
		tokio::time::sleep(std::time::Duration::from_millis(1)).await;
		Ok(Value::String("slow call async".to_string()))
	});

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
	.max_payload(100 * 1024 * 1024)
	.max_in_buffer_capacity(100 * 1024 * 1024)
	.max_out_buffer_capacity(100 * 1024 * 1024)
	.start(&"127.0.0.1:0".parse().unwrap())
	.expect("Server must start with no issues");

	let addr = *server.addr();
	(format!("ws://{}", addr), server)
}

/// Run jsonrpsee HTTP server for benchmarks.
#[cfg(not(feature = "jsonrpc-crate"))]
pub async fn http_server(handle: tokio::runtime::Handle) -> (String, jsonrpsee::http_server::HttpServerHandle) {
	use jsonrpsee::http_server::HttpServerBuilder;

	let server = HttpServerBuilder::default()
		.max_request_body_size(u32::MAX)
		.max_response_body_size(u32::MAX)
		.custom_tokio_runtime(handle)
		.build("127.0.0.1:0")
		.await
		.unwrap();

	let module = gen_rpc_module();

	let addr = server.local_addr().unwrap();
	let handle = server.start(module).unwrap();
	(format!("http://{}", addr), handle)
}

/// Run jsonrpsee WebSocket server for benchmarks.
#[cfg(not(feature = "jsonrpc-crate"))]
pub async fn ws_server(handle: tokio::runtime::Handle) -> (String, jsonrpsee::ws_server::WsServerHandle) {
	use jsonrpsee::ws_server::WsServerBuilder;

	let server = WsServerBuilder::default()
		.max_request_body_size(u32::MAX)
		.max_response_body_size(u32::MAX)
		.max_connections(10 * 1024)
		.custom_tokio_runtime(handle)
		.build("127.0.0.1:0")
		.await
		.unwrap();

	let mut module = gen_rpc_module();

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

#[cfg(not(feature = "jsonrpc-crate"))]
fn gen_rpc_module() -> jsonrpsee::RpcModule<()> {
	let mut module = jsonrpsee::RpcModule::new(());

	module.register_method(SYNC_FAST_CALL, |_, _| Ok("lo")).unwrap();
	module.register_async_method(ASYNC_FAST_CALL, |_, _| async { Ok("lo") }).unwrap();

	module.register_method(SYNC_MEM_CALL, |_, _| Ok("A".repeat(1024 * 1024))).unwrap();

	module.register_async_method(ASYNC_MEM_CALL, |_, _| async move { Ok("A".repeat(1024 * 1024)) }).unwrap();

	module
		.register_method(SYNC_SLOW_CALL, |_, _| {
			std::thread::sleep(std::time::Duration::from_millis(1));
			Ok("slow call")
		})
		.unwrap();

	module
		.register_async_method(ASYNC_SLOW_CALL, |_, _| async move {
			tokio::time::sleep(std::time::Duration::from_millis(1)).await;
			Ok("slow call async")
		})
		.unwrap();

	module
}

pub(crate) fn http_client(url: &str) -> HttpClient {
	HttpClientBuilder::default()
		.max_request_body_size(u32::MAX)
		.max_concurrent_requests(1024 * 1024)
		.build(url)
		.unwrap()
}

pub(crate) async fn ws_client(url: &str) -> WsClient {
	WsClientBuilder::default()
		.max_request_body_size(u32::MAX)
		.max_concurrent_requests(1024 * 1024)
		.build(url)
		.await
		.unwrap()
}
