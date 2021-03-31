use async_std::task::block_on;
use criterion::*;
use jsonrpsee_http_client::HttpClientBuilder;
use jsonrpsee_http_server::HttpServerBuilder;
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
	let mut server = HttpServerBuilder::default().build("127.0.0.1:0".parse().unwrap()).unwrap();
	server.register_method("say_hello", |_| Ok("lo")).unwrap();
	let addr = server.local_addr().unwrap();
	tokio::spawn(async move { server.start().await.unwrap() });
	addr
}

async fn ws_server() -> SocketAddr {
	let mut server = WsServer::new("127.0.0.1:0").await.unwrap();
	let addr = server.local_addr().unwrap();
	server.register_method("say_hello", |_| Ok("lo")).unwrap();
	tokio::spawn(async move { server.start().await });
	addr
}

pub fn http_requests(c: &mut criterion::Criterion) {
	let rt = tokio::runtime::Runtime::new().unwrap();
	let server_addr = rt.block_on(http_server());
	let url = format!("http://{}", server_addr);
	let client = Arc::new(HttpClientBuilder::default().build(&url).unwrap());

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
	let addr = rt.block_on(ws_server());
	let url = format!("ws://{}", addr);
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
