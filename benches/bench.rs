use criterion::*;
use jsonrpsee::{
	http_client::{
		traits::Client,
		v2::{JsonRpcCallSer, JsonRpcParams},
		HttpClientBuilder,
	},
	ws_client::WsClientBuilder,
};
use std::sync::Arc;
use tokio::runtime::Runtime as TokioRuntime;

mod helpers;

criterion_group!(benches, http_requests, websocket_requests, jsonrpsee_types_v2);
criterion_main!(benches);

fn v2_serialize(req: JsonRpcCallSer<u64>) -> String {
	serde_json::to_string(&req).unwrap()
}

pub fn jsonrpsee_types_v2(crit: &mut Criterion) {
	crit.bench_function("jsonrpsee_types_v2", |b| {
		b.iter(|| {
			let params = JsonRpcParams::Array(&[1_u64, 2]);
			let request = JsonRpcCallSer::new(0, "say_hello", params);
			v2_serialize(request);
		})
	});
}

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
	let client =
		Arc::new(rt.block_on(WsClientBuilder::default().max_concurrent_requests(1024 * 1024).build(&url)).unwrap());
	run_round_trip(&rt, crit, client.clone(), "ws_round_trip");
	run_concurrent_round_trip(&rt, crit, client.clone(), "ws_concurrent_round_trip");
}

fn run_round_trip(rt: &TokioRuntime, crit: &mut Criterion, client: Arc<impl Client>, name: &str) {
	crit.bench_function(name, |b| {
		b.iter(|| {
			rt.block_on(async {
				black_box(client.request::<u64, String>("say_hello", JsonRpcParams::NoParams).await.unwrap());
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
						let _ = black_box(
							client_rc.request::<u64, String>("say_hello", JsonRpcParams::NoParams).await.unwrap(),
						);
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
