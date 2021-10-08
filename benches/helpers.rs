use std::sync::atomic::AtomicU64;

pub(crate) const SYNC_METHOD_NAME: &str = "say_hello";
pub(crate) const ASYNC_METHOD_NAME: &str = "say_hello_async";
pub(crate) const SUB_METHOD_NAME: &str = "sub";
pub(crate) const UNSUB_METHOD_NAME: &str = "unsub";

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
	use jsonrpc_pubsub::{PubSubHandler, Session, Subscriber, SubscriptionId};
	use jsonrpc_ws_server::jsonrpc_core::*;
	use jsonrpc_ws_server::*;
	use std::sync::atomic::Ordering;

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
	.start(&"127.0.0.1:0".parse().unwrap())
	.expect("Server must start with no issues");

	let addr = *server.addr();
	(format!("ws://{}", addr), server)
}

/// Get number of concurrent tasks based on the num_cpus.
pub fn concurrent_tasks() -> Vec<usize> {
	let cores = num_cpus::get();
	vec![cores / 4, cores / 2, cores, cores * 2, cores * 4]
}
