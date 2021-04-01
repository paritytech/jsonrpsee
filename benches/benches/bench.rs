use criterion::*;
use jsonrpsee_http_client::HttpClientBuilder;
use jsonrpsee_types::{jsonrpc::Params, traits::Client};
use jsonrpsee_ws_client::WsClientBuilder;
use std::sync::Arc;
use tokio::runtime::Runtime as TokioRuntime;

mod helpers;

criterion_group!(benches, http_requests, websocket_requests);
criterion_main!(benches);

pub fn http_requests(crit: &mut Criterion) {
	let rt = TokioRuntime::new().unwrap();
	let url = rt.block_on(helpers::http_server());
	let client = Arc::new(HttpClientBuilder::default().build(&url).unwrap());
	run_round_trip(&rt, crit, client.clone(), "http_round_trip");
	run_concurrent_round_trip(&rt, crit, client.clone(), "http_concurrent_round_trip");
}

pub fn websocket_requests(crit: &mut Criterion) {
	let rt = TokioRuntime::new().unwrap();
	let url = rt.block_on(helpers::ws_server());
	let client = Arc::new(rt.block_on(WsClientBuilder::default().build(&url)).unwrap());
	run_round_trip(&rt, crit, client.clone(), "ws_round_trip");
	run_concurrent_round_trip(&rt, crit, client.clone(), "ws_concurrent_round_trip");
}

fn run_round_trip(rt: &TokioRuntime, crit: &mut Criterion, client: Arc<impl Client>, name: &str) {
	crit.bench_function(name, |b| {
		b.iter(|| {
			rt.block_on(async {
				black_box(client.request::<String, _, _>("say_hello", Params::None).await.unwrap());
			})
		})
	});
}

fn run_concurrent_round_trip<C: 'static + Client + Send + Sync>(
	rt: &TokioRuntime,
	crit: &mut Criterion,
	client: Arc<C>,
	name: &str,
) {
	let mut group = crit.benchmark_group(name);
	for num_concurrent_tasks in helpers::concurrent_tasks() {
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
