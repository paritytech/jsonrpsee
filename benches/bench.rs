use criterion::*;
use futures_util::future::join_all;
use helpers::{SUB_METHOD_NAME, UNSUB_METHOD_NAME};
use jsonrpsee::{
	http_client::HttpClientBuilder,
	types::traits::SubscriptionClient,
	types::{
		traits::Client,
		v2::{Id, ParamsSer, RequestSer},
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
criterion_group!(subscriptions, AsyncBencher::subscriptions);
criterion_main!(types_benches, sync_benches, async_benches, subscriptions);

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

fn v2_serialize(req: RequestSer<'_>) -> String {
	serde_json::to_string(&req).unwrap()
}

pub fn jsonrpsee_types_v2(crit: &mut Criterion) {
	crit.bench_function("jsonrpsee_types_v2_array_ref", |b| {
		b.iter(|| {
			let params = &[1_u64.into(), 2_u32.into()];
			let params = ParamsSer::ArrayRef(params);
			let request = RequestSer::new(Id::Number(0), "say_hello", params);
			v2_serialize(request);
		})
	});

	crit.bench_function("jsonrpsee_types_v2_vec", |b| {
		b.iter(|| {
			let params = ParamsSer::Array(vec![1_u64.into(), 2_u32.into()]);
			let request = RequestSer::new(Id::Number(0), "say_hello", params);
			v2_serialize(request);
		})
	});
}

trait RequestBencher {
	const REQUEST_TYPE: RequestType;

	fn http_requests(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let (url, _handle) = rt.block_on(helpers::http_server());
		let client = Arc::new(HttpClientBuilder::default().build(&url).unwrap());
		run_round_trip(&rt, crit, client.clone(), "http_round_trip", Self::REQUEST_TYPE);
		run_concurrent_round_trip(&rt, crit, client, "http_concurrent_round_trip", Self::REQUEST_TYPE);
		run_http_concurrent_connections(&rt, crit, &url, "http_concurrent_connections", Self::REQUEST_TYPE);
	}

	fn batched_http_requests(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let (url, _handle) = rt.block_on(helpers::http_server());
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
		run_ws_concurrent_connections(&rt, crit, &url, "ws_concurrent_connections", Self::REQUEST_TYPE);
	}

	fn batched_ws_requests(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let url = rt.block_on(helpers::ws_server());
		let client =
			Arc::new(rt.block_on(WsClientBuilder::default().max_concurrent_requests(1024 * 1024).build(&url)).unwrap());
		run_round_trip_with_batch(&rt, crit, client, "ws batch requests", Self::REQUEST_TYPE);
	}

	fn subscriptions(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let url = rt.block_on(helpers::ws_server());
		let client =
			Arc::new(rt.block_on(WsClientBuilder::default().max_concurrent_requests(1024 * 1024).build(&url)).unwrap());
		run_sub_round_trip(&rt, crit, client, "subscriptions");
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
		b.to_async(rt).iter(|| async {
			black_box(client.request::<String>(request.method_name(), ParamsSer::NoParams).await.unwrap());
		})
	});
}

fn run_sub_round_trip(rt: &TokioRuntime, crit: &mut Criterion, client: Arc<impl SubscriptionClient>, name: &str) {
	let mut group = crit.benchmark_group(name);
	group.bench_function("subscribe", |b| {
		b.to_async(rt).iter_with_large_drop(|| async {
			black_box(
				client.subscribe::<String>(SUB_METHOD_NAME, ParamsSer::NoParams, UNSUB_METHOD_NAME).await.unwrap(),
			);
		})
	});
	group.bench_function("subscribe_response", |b| {
		b.to_async(rt).iter_with_setup(
			|| {
				// We have to use `block_in_place` here since `b.to_async(rt)` automatically enters the
				// runtime context and simply calling `block_on` here will cause the code to panic.
				tokio::task::block_in_place(|| {
					tokio::runtime::Handle::current().block_on(async {
						client
							.subscribe::<String>(SUB_METHOD_NAME, ParamsSer::NoParams, UNSUB_METHOD_NAME)
							.await
							.unwrap()
					})
				})
			},
			|mut sub| async move {
				black_box(sub.next().await.unwrap());
				// Note that this benchmark will include costs for measuring `drop` for subscription,
				// since it's not possible to combine both `iter_with_setup` and `iter_with_large_drop`.
				// To estimate pure cost of method, one should subtract the result of `unsub` bench
				// from this one.
			},
		)
	});
	group.bench_function("unsub", |b| {
		b.iter_with_setup(
			|| {
				rt.block_on(async {
					client.subscribe::<String>(SUB_METHOD_NAME, ParamsSer::NoParams, UNSUB_METHOD_NAME).await.unwrap()
				})
			},
			|sub| {
				// Subscription will be closed inside of the drop impl.
				// Actually, it just sends a notification about object being closed,
				// but it's still important to know that drop impl is not too expensive.
				drop(black_box(sub));
			},
		)
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
		let batch = vec![(request.method_name(), ParamsSer::NoParams); *batch_size];
		group.throughput(Throughput::Elements(*batch_size as u64));
		group.bench_with_input(BenchmarkId::from_parameter(batch_size), batch_size, |b, _| {
			b.to_async(rt).iter(|| async { client.batch_request::<String>(batch.clone()).await.unwrap() })
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
			b.to_async(rt).iter_with_setup(
				|| (0..num_concurrent_tasks).map(|_| client.clone()),
				|clients| async {
					let tasks = clients.map(|client| {
						rt.spawn(async move {
							let _ = black_box(
								client.request::<String>(request.method_name(), ParamsSer::NoParams).await.unwrap(),
							);
						})
					});
					join_all(tasks).await;
				},
			)
		});
	}
	group.finish();
}

fn run_ws_concurrent_connections(rt: &TokioRuntime, crit: &mut Criterion, url: &str, name: &str, request: RequestType) {
	let mut group = crit.benchmark_group(request.group_name(name));
	for conns in [2, 4, 8, 16, 32, 64] {
		group.bench_function(format!("{}", conns), |b| {
			b.to_async(rt).iter_with_setup(
				|| {
					let mut clients = Vec::new();
					// We have to use `block_in_place` here since `b.to_async(rt)` automatically enters the
					// runtime context and simply calling `block_on` here will cause the code to panic.
					tokio::task::block_in_place(|| {
						tokio::runtime::Handle::current().block_on(async {
							for _ in 0..conns {
								clients.push(WsClientBuilder::default().build(url).await.unwrap());
							}
						})
					});

					clients
				},
				|clients| async {
					let tasks = clients.into_iter().map(|client| {
						rt.spawn(async move {
							let _ = black_box(
								client.request::<String>(request.method_name(), ParamsSer::NoParams).await.unwrap(),
							);
						})
					});
					join_all(tasks).await;
				},
			)
		});
	}
	group.finish();
}

fn run_http_concurrent_connections(
	rt: &TokioRuntime,
	crit: &mut Criterion,
	url: &str,
	name: &str,
	request: RequestType,
) {
	let mut group = crit.benchmark_group(request.group_name(name));
	for conns in [2, 4, 8, 16, 32, 64] {
		group.bench_function(format!("{}", conns), |b| {
			b.to_async(rt).iter_with_setup(
				|| (0..conns).map(|_| HttpClientBuilder::default().build(url).unwrap()),
				|clients| async {
					let tasks = clients.map(|client| {
						rt.spawn(async move {
							let _ = black_box(
								client.request::<String>(request.method_name(), ParamsSer::NoParams).await.unwrap(),
							);
						})
					});
					join_all(tasks).await;
				},
			)
		});
	}
	group.finish();
}
