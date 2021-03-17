use futures::channel::oneshot;
use jsonrpsee_http_server::{HttpConfig, HttpServer};
use jsonrpsee_ws_server::WsServer;

/// Run jsonrpsee HTTP server for benchmarks.
pub async fn http_server() -> String {
	let (server_started_tx, server_started_rx) = oneshot::channel();
	tokio::spawn(async move {
		let server = HttpServer::new("127.0.0.1:0", HttpConfig { max_request_body_size: u32::MAX }).await.unwrap();
		let mut say_hello = server.register_method("say_hello".to_string()).unwrap();
		server_started_tx.send(*server.local_addr()).unwrap();
		loop {
			let r = say_hello.next().await;
			r.respond(Ok("lo".into())).await.unwrap();
		}
	});
	format!("http://{}", server_started_rx.await.unwrap())
}

/// Run jsonrpsee WebSocket server for benchmarks.
pub async fn ws_server() -> String {
	let (server_started_tx, server_started_rx) = oneshot::channel();
	tokio::spawn(async move {
		let mut server = WsServer::new("127.0.0.1:0").await.unwrap();
		server.register_method("say_hello", |_| Ok("lo")).unwrap();
		server_started_tx.send(server.local_addr().unwrap()).unwrap();
		server.start().await
	});
	format!("ws://{}", server_started_rx.await.unwrap())
}

/// Get number of concurrent tasks based on the num_cpus.
pub fn concurrent_tasks() -> Vec<usize> {
	let cores = num_cpus::get();
	vec![cores / 4, cores / 2, cores, cores * 2, cores * 4]
}
