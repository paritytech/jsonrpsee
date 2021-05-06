use criterion::*;
use jsonrpsee::{
	http_client::{
		traits::Client,
		v2::params::{Id, JsonRpcParams},
		v2::request::JsonRpcCallSer,
		HttpClientBuilder,
	},
	ws_client::WsClientBuilder,
};
use std::sync::Arc;
use tokio::runtime::Runtime as TokioRuntime;

mod helpers;

criterion_group!(benches, http_requests, batched_http_requests, websocket_requests, jsonrpsee_types_v2);
criterion_main!(benches);

fn v2_serialize<'a>(req: JsonRpcCallSer<'a>) -> String {
	serde_json::to_string(&req).unwrap()
}

pub fn jsonrpsee_types_v2(crit: &mut Criterion) {
	crit.bench_function("jsonrpsee_types_v2_array_ref", |b| {
		b.iter(|| {
			let params = &[1_u64.into(), 2_u32.into()];
			let params = JsonRpcParams::ArrayRef(params);
			let request = JsonRpcCallSer::new(Id::Number(0), "say_hello", params);
			v2_serialize(request);
		})
	});

	crit.bench_function("jsonrpsee_types_v2_vec", |b| {
		b.iter(|| {
			let params = JsonRpcParams::Array(vec![1_u64.into(), 2_u32.into()]);
			let request = JsonRpcCallSer::new(Id::Number(0), "say_hello", params);
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

pub fn batched_http_requests(crit: &mut Criterion) {
	let rt = TokioRuntime::new().unwrap();
	let url = rt.block_on(helpers::http_server());
	let client = Arc::new(HttpClientBuilder::default().build(&url).unwrap());
	run_round_trip_with_batch(&rt, crit, client.clone(), "http batch requests");
}

pub fn websocket_requests(crit: &mut Criterion) {
	let rt = TokioRuntime::new().unwrap();
	let url = rt.block_on(helpers::ws_server());
	let client =
		Arc::new(rt.block_on(WsClientBuilder::default().max_concurrent_requests(1024 * 1024).build(&url)).unwrap());
	run_round_trip(&rt, crit, client.clone(), "ws_round_trip");
	run_concurrent_round_trip(&rt, crit, client.clone(), "ws_concurrent_round_trip");
}

pub fn batched_ws_requests(crit: &mut Criterion) {
	let rt = TokioRuntime::new().unwrap();
	let url = rt.block_on(helpers::ws_server());
	let client =
		Arc::new(rt.block_on(WsClientBuilder::default().max_concurrent_requests(1024 * 1024).build(&url)).unwrap());
	run_round_trip_with_batch(&rt, crit, client.clone(), "ws batch requests");
}

fn run_round_trip(rt: &TokioRuntime, crit: &mut Criterion, client: Arc<impl Client>, name: &str) {
	crit.bench_function(name, |b| {
		b.iter(|| {
			rt.block_on(async {
				black_box(client.request::<String>("say_hello", JsonRpcParams::NoParams).await.unwrap());
			})
		})
	});
}

/// Benchmark http batch requests over batch sizes of 2, 5, 10, 50 and 100 RPCs in each batch.
fn run_round_trip_with_batch(rt: &TokioRuntime, crit: &mut Criterion, client: Arc<impl Client>, name: &str) {
	let mut group = crit.benchmark_group(name);
	for batch_size in [2, 5, 10, 50, 100usize].iter() {
		let batch = vec![("say_hello", JsonRpcParams::NoParams); *batch_size];
		group.throughput(Throughput::Elements(*batch_size as u64));
		group.bench_with_input(BenchmarkId::from_parameter(batch_size), batch_size, |b, _| {
			b.iter(|| rt.block_on(async { client.batch_request::<String>(batch.clone()).await.unwrap() }))
		});
	}
	group.finish();
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
						let _ =
							black_box(client_rc.request::<String>("say_hello", JsonRpcParams::NoParams).await.unwrap());
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
