use async_std::task::block_on;
use criterion::*;
use futures::channel::oneshot::{self, Sender};
use jsonrpsee_http_client::{HttpClient, HttpConfig};
use jsonrpsee_http_server::HttpServer;
use jsonrpsee_types::{jsonrpc::Params, traits::Client};
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

async fn http_server() -> SocketAddr {
	let mut server =
		HttpServer::new(&"127.0.0.1:0".parse().unwrap(), HttpConfig::default(), Default::default()).await.unwrap();
	server.register_method("say_hello", |_| Ok("lo")).unwrap();
	server.start().await.unwrap()
}

async fn ws_server(tx: Sender<SocketAddr>) {
	let mut server = WsServer::new("127.0.0.1:0").await.unwrap();
	tx.send(server.local_addr().unwrap()).unwrap();
	server.register_method("say_hello", |_| Ok("lo")).unwrap();
	server.start().await;
}

pub fn http_requests(c: &mut criterion::Criterion) {
	let rt = tokio::runtime::Runtime::new().unwrap();
	let server_addr = rt.block_on(http_server());
	let client = Arc::new(HttpClient::new(&format!("http://{}", server_addr), HttpConfig::default()).unwrap());

	c.bench_function("synchronous_http_round_trip", |b| {
		b.iter(|| {
			rt.block_on(async {
				black_box(client.request::<String, _, _>("say_hello", Params::None).await.unwrap());
			})
		})
	});

	let mut group = c.benchmark_group("concurrent_http_round_trip");

	for num_concurrent_tasks in concurrent_tasks() {
		group.bench_function(format!("{}", num_concurrent_tasks), |b| {
			b.iter(|| {
				let mut tasks = Vec::new();
				for _ in 0..num_concurrent_tasks {
					let client_rc = client.clone();
					let task = rt.spawn(async move {
						let _ = black_box(client_rc.request::<String, _, _>("say_hello", Params::None)).await;
					});
					tasks.push(task);
				}
				for task in tasks {
					rt.block_on(task).unwrap();
				}
			})
		});
	}
	group.finish();
}

pub fn websocket_requests(c: &mut criterion::Criterion) {
	let rt = tokio::runtime::Runtime::new().unwrap();
	let (tx_addr, rx_addr) = oneshot::channel::<SocketAddr>();
	rt.spawn(ws_server(tx_addr));
	let server_addr = block_on(rx_addr).unwrap();
	let url = format!("ws://{}", server_addr);
	let config = WsConfig::with_url(&url);
	let client = Arc::new(block_on(WsClient::new(config)).unwrap());

	c.bench_function("synchronous_websocket_round_trip", |b| {
		b.iter(|| {
			rt.block_on(async {
				black_box(client.request::<String, _, _>("say_hello", Params::None).await.unwrap());
			})
		})
	});

	let mut group = c.benchmark_group("concurrent_websocket_round_trip");

	for num_concurrent_tasks in concurrent_tasks() {
		group.bench_function(format!("{}", num_concurrent_tasks), |b| {
			b.iter(|| {
				let mut tasks = Vec::new();
				for _ in 0..num_concurrent_tasks {
					let client_rc = client.clone();
					let task = rt.spawn(async move {
						let _ = black_box(client_rc.request::<String, _, _>("say_hello", Params::None)).await;
					});
					tasks.push(task);
				}
				for task in tasks {
					rt.block_on(task).unwrap();
				}
			})
		});
	}
	group.finish();
}
