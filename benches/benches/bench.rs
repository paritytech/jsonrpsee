use async_std::task::block_on;
use criterion::*;
use futures::channel::oneshot::{self, Sender};
use jsonrpsee_http_client::{HttpClient, HttpConfig};
use jsonrpsee_http_server::HttpServer;
use jsonrpsee_types::jsonrpc::{JsonValue, Params};
use jsonrpsee_ws_client::{WsClient, WsConfig};
use jsonrpsee_ws_server::WsServer;
use std::net::SocketAddr;
use std::sync::Arc;

criterion_group!(benches, http_requests, websocket_requests);
criterion_main!(benches);

fn concurrent_tasks() -> Vec<usize> {
	let cores = num_cpus::get();
	vec![cores / 4, cores / 2, cores, cores * 2, cores * 4]
}

async fn http_server(tx: Sender<SocketAddr>) {
	let server = HttpServer::new("127.0.0.1:0", HttpConfig { max_request_body_size: u32::MAX }).await.unwrap();
	let mut say_hello = server.register_method("say_hello".to_string()).unwrap();
	tx.send(*server.local_addr()).unwrap();
	loop {
		let r = say_hello.next().await;
		r.respond(Ok(JsonValue::String("lo".to_owned()))).await.unwrap();
	}
}

async fn ws_server(tx: Sender<SocketAddr>) {
	let server = WsServer::new("127.0.0.1:0").await.unwrap();
	let mut say_hello = server.register_method("say_hello".to_string()).unwrap();
	tx.send(*server.local_addr()).unwrap();
	loop {
		let r = say_hello.next().await;
		r.respond(Ok(JsonValue::String("lo".to_owned()))).await.unwrap();
	}
}

pub fn http_requests(c: &mut criterion::Criterion) {
	let rt = tokio::runtime::Runtime::new().unwrap();
	let (tx_addr, rx_addr) = oneshot::channel::<SocketAddr>();
	async_std::task::spawn(http_server(tx_addr));
	let server_addr = block_on(rx_addr).unwrap();
	let client = Arc::new(HttpClient::new(&format!("http://{}", server_addr), HttpConfig::default()).unwrap());

	c.bench_function("synchronous http round trip", |b| {
		b.iter(|| {
			rt.block_on(async {
				let _: JsonValue = black_box(client.request("say_hello", Params::None).await.unwrap());
			})
		})
	});

	c.bench_function_over_inputs(
		"concurrent http round trip",
		move |b: &mut Bencher, size: &usize| {
			b.iter(|| {
				let mut tasks = Vec::new();
				for _ in 0..*size {
					let client_rc = client.clone();
					let task = rt.spawn(async move {
						let _: Result<JsonValue, _> = black_box(client_rc.request("say_hello", Params::None)).await;
					});
					tasks.push(task);
				}
				for task in tasks {
					rt.block_on(task).unwrap();
				}
			})
		},
		concurrent_tasks(),
	);
}

pub fn websocket_requests(c: &mut criterion::Criterion) {
	let rt = tokio::runtime::Runtime::new().unwrap();
	let (tx_addr, rx_addr) = oneshot::channel::<SocketAddr>();
	async_std::task::spawn(ws_server(tx_addr));
	let server_addr = block_on(rx_addr).unwrap();
	let client = Arc::new(block_on(WsClient::new(&format!("ws://{}", server_addr), WsConfig::default())).unwrap());

	c.bench_function("synchronous WebSocket round trip", |b| {
		b.iter(|| {
			rt.block_on(async {
				let _: JsonValue = black_box(client.request("say_hello", Params::None).await.unwrap());
			})
		})
	});

	c.bench_function_over_inputs(
		"concurrent WebSocket round trip",
		move |b: &mut Bencher, size: &usize| {
			b.iter(|| {
				let mut tasks = Vec::new();
				for _ in 0..*size {
					let client_rc = client.clone();
					let task = rt.spawn(async move {
						let _: Result<JsonValue, _> = black_box(client_rc.request("say_hello", Params::None)).await;
					});
					tasks.push(task);
				}
				for task in tasks {
					rt.block_on(task).unwrap();
				}
			})
		},
		// TODO(niklasad1): This deadlocks when more than 8 tasks are spawned.
		concurrent_tasks(),
	);
}
