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

criterion_group!(types_benches, jsonrpsee_types_v2);
criterion_group!(
	sync_benches,
	SyncBencher::http_requests,
	SyncBencher::batched_http_requests,
	SyncBencher::websocket_requests
);
criterion_group!(
	async_benches,
	AsyncBencher::http_requests,
	AsyncBencher::batched_http_requests,
	AsyncBencher::websocket_requests
);
criterion_main!(types_benches, sync_benches, async_benches);

#[derive(Debug, Clone, Copy)]
enum RequestType {
	Sync,
	Async,
}

impl RequestType {
	fn method_name(self) -> &'static str {
		match self {
			RequestType::Sync => crate::helpers::SYNC_METHOD_NAME,
			RequestType::Async => crate::helpers::ASYNC_METHOD_NAME,
		}
	}

	fn group_name(self, name: &str) -> String {
		let request_type_name = match self {
			RequestType::Sync => "sync",
			RequestType::Async => "async",
		};
		format!("{}/{}", request_type_name, name)
	}
}

fn v2_serialize(req: JsonRpcCallSer<'_>) -> String {
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

trait RequestBencher {
	const REQUEST_TYPE: RequestType;

	fn http_requests(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let url = rt.block_on(helpers::http_server());
		let client = Arc::new(HttpClientBuilder::default().build(&url).unwrap());
		run_round_trip(&rt, crit, client.clone(), "http_round_trip", Self::REQUEST_TYPE);
		run_concurrent_round_trip(&rt, crit, client, "http_concurrent_round_trip", Self::REQUEST_TYPE);
	}

	fn batched_http_requests(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let url = rt.block_on(helpers::http_server());
		let client = Arc::new(HttpClientBuilder::default().build(&url).unwrap());
		run_round_trip_with_batch(&rt, crit, client, "http batch requests", Self::REQUEST_TYPE);
	}

	fn websocket_requests(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let url = rt.block_on(helpers::ws_server());
		let client =
			Arc::new(rt.block_on(WsClientBuilder::default().max_concurrent_requests(1024 * 1024).build(&url)).unwrap());
		run_round_trip(&rt, crit, client.clone(), "ws_round_trip", Self::REQUEST_TYPE);
		run_concurrent_round_trip(&rt, crit, client, "ws_concurrent_round_trip", Self::REQUEST_TYPE);
	}

	fn batched_ws_requests(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let url = rt.block_on(helpers::ws_server());
		let client =
			Arc::new(rt.block_on(WsClientBuilder::default().max_concurrent_requests(1024 * 1024).build(&url)).unwrap());
		run_round_trip_with_batch(&rt, crit, client, "ws batch requests", Self::REQUEST_TYPE);
	}
}

pub struct SyncBencher;

impl RequestBencher for SyncBencher {
	const REQUEST_TYPE: RequestType = RequestType::Sync;
}
pub struct AsyncBencher;

impl RequestBencher for AsyncBencher {
	const REQUEST_TYPE: RequestType = RequestType::Async;
}

fn run_round_trip(rt: &TokioRuntime, crit: &mut Criterion, client: Arc<impl Client>, name: &str, request: RequestType) {
	crit.bench_function(&request.group_name(name), |b| {
		b.iter(|| {
			rt.block_on(async {
				black_box(client.request::<String>(request.method_name(), JsonRpcParams::NoParams).await.unwrap());
			})
		})
	});
}

/// Benchmark http batch requests over batch sizes of 2, 5, 10, 50 and 100 RPCs in each batch.
fn run_round_trip_with_batch(
	rt: &TokioRuntime,
	crit: &mut Criterion,
	client: Arc<impl Client>,
	name: &str,
	request: RequestType,
) {
	let mut group = crit.benchmark_group(request.group_name(name));
	for batch_size in [2, 5, 10, 50, 100usize].iter() {
		let batch = vec![(request.method_name(), JsonRpcParams::NoParams); *batch_size];
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
	request: RequestType,
) {
	let mut group = crit.benchmark_group(request.group_name(name));
	for num_concurrent_tasks in helpers::concurrent_tasks() {
		group.bench_function(format!("{}", num_concurrent_tasks), |b| {
			b.iter(|| {
				let mut tasks = Vec::new();
				for _ in 0..num_concurrent_tasks {
					let client_rc = client.clone();
					let task = rt.spawn(async move {
						let _ = black_box(
							client_rc.request::<String>(request.method_name(), JsonRpcParams::NoParams).await.unwrap(),
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
