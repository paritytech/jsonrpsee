use std::sync::Arc;

use criterion::*;
use futures_util::future::{join_all, FutureExt};
use futures_util::stream::FuturesUnordered;
use helpers::{SUB_METHOD_NAME, UNSUB_METHOD_NAME};
use jsonrpsee::core::client::{ClientT, SubscriptionClientT};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::types::{Id, ParamsSer, RequestSer};
use jsonrpsee::ws_client::WsClientBuilder;
use pprof::criterion::{Output, PProfProfiler};
use tokio::runtime::Runtime as TokioRuntime;

mod helpers;

criterion_group!(
	name = types_benches;
	config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
	targets = jsonrpsee_types_v2
);
criterion_group!(
	name = sync_benches;
	config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
	targets = SyncBencher::http_requests, SyncBencher::batched_http_requests, SyncBencher::websocket_requests, SyncBencher::batched_ws_requests
);
criterion_group!(
	name = async_benches;
	config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
	targets = AsyncBencher::http_requests, AsyncBencher::batched_http_requests, AsyncBencher::websocket_requests, AsyncBencher::batched_ws_requests
);
criterion_group!(
	name = subscriptions;
	config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
	targets = AsyncBencher::subscriptions
);
criterion_main!(types_benches, sync_benches, async_benches, subscriptions);

#[derive(Debug, Clone, Copy)]
enum RequestType {
	Sync,
	Async,
}

impl RequestType {
	fn methods(self) -> Vec<&'static str> {
		match self {
			RequestType::Sync => crate::helpers::SYNC_METHODS.to_vec(),
			RequestType::Async => crate::helpers::ASYNC_METHODS.to_vec(),
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
			let request = RequestSer::new(&Id::Number(0), "say_hello", Some(params));
			v2_serialize(request);
		})
	});

	crit.bench_function("jsonrpsee_types_v2_vec", |b| {
		b.iter(|| {
			let params = ParamsSer::Array(vec![1_u64.into(), 2_u32.into()]);
			let request = RequestSer::new(&Id::Number(0), "say_hello", Some(params));
			v2_serialize(request);
		})
	});
}

trait RequestBencher {
	const REQUEST_TYPE: RequestType;

	fn http_requests(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let (url, _server) = rt.block_on(helpers::http_server(rt.handle().clone()));
		let client = Arc::new(HttpClientBuilder::default().max_request_body_size(u32::MAX).max_concurrent_requests(1024 * 1024).build(&url).unwrap());
		round_trip(&rt, crit, client.clone(), "http_round_trip", Self::REQUEST_TYPE);
		http_concurrent_conn_calls(&rt, crit, &url, "http_concurrent_conn_calls", Self::REQUEST_TYPE);
	}

	fn batched_http_requests(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let (url, _server) = rt.block_on(helpers::http_server(rt.handle().clone()));
		let client = Arc::new(HttpClientBuilder::default().max_request_body_size(u32::MAX).max_concurrent_requests(1024 * 1024).build(&url).unwrap());
		batch_round_trip(&rt, crit, client, "http_batch_requests", Self::REQUEST_TYPE);
	}

	fn websocket_requests(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let (url, _server) = rt.block_on(helpers::ws_server(rt.handle().clone()));
		let client =
			Arc::new(rt.block_on(WsClientBuilder::default().max_request_body_size(u32::MAX).max_concurrent_requests(1024 * 1024).build(&url)).unwrap());
		round_trip(&rt, crit, client.clone(), "ws_round_trip", Self::REQUEST_TYPE);
		ws_concurrent_conn_calls(&rt, crit, &url, "ws_concurrent_conn_calls", Self::REQUEST_TYPE);
		ws_concurrent_conn_subs(&rt, crit, &url, "ws_concurrent_conn_subs", Self::REQUEST_TYPE);
	}

	fn batched_ws_requests(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let (url, _server) = rt.block_on(helpers::ws_server(rt.handle().clone()));
		let client = Arc::new(rt.block_on(WsClientBuilder::default().max_request_body_size(u32::MAX).max_concurrent_requests(1024 * 1024).build(&url)).unwrap());
		batch_round_trip(&rt, crit, client, "ws_batch_requests", Self::REQUEST_TYPE);
	}

	fn subscriptions(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let (url, _server) = rt.block_on(helpers::ws_server(rt.handle().clone()));
		let client = Arc::new(rt.block_on(WsClientBuilder::default().max_request_body_size(u32::MAX).max_concurrent_requests(1024 * 1024).build(&url)).unwrap());
		sub_round_trip(&rt, crit, client, "subscriptions");
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

fn round_trip(rt: &TokioRuntime, crit: &mut Criterion, client: Arc<impl ClientT>, name: &str, request: RequestType) {
	for method in request.methods() {
		let bench_name = format!("{}/{}", name, method);
		crit.bench_function(&request.group_name(&bench_name), |b| {
			b.to_async(rt).iter(|| async {
				black_box(client.request::<String>(method, None).await.unwrap());
			})
		});
	}	
}

fn sub_round_trip(rt: &TokioRuntime, crit: &mut Criterion, client: Arc<impl SubscriptionClientT>, name: &str) {
	let mut group = crit.benchmark_group(name);
	group.bench_function("subscribe", |b| {
		b.to_async(rt).iter_with_large_drop(|| async {
			black_box(client.subscribe::<String>(SUB_METHOD_NAME, None, UNSUB_METHOD_NAME).await.unwrap());
		})
	});
	group.bench_function("subscribe_response", |b| {
		b.to_async(rt).iter_with_setup(
			|| {
				// We have to use `block_in_place` here since `b.to_async(rt)` automatically enters the
				// runtime context and simply calling `block_on` here will cause the code to panic.
				tokio::task::block_in_place(|| {
					tokio::runtime::Handle::current().block_on(async {
						client.subscribe::<String>(SUB_METHOD_NAME, None, UNSUB_METHOD_NAME).await.unwrap()
					})
				})
			},
			|mut sub| async move {
				black_box(sub.next().await.transpose().unwrap());
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
					client.subscribe::<String>(SUB_METHOD_NAME, None, UNSUB_METHOD_NAME).await.unwrap()
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

/// Benchmark http_batch_requests over batch sizes of 2, 5, 10, 50 and 100 RPCs in each batch.
fn batch_round_trip(
	rt: &TokioRuntime,
	crit: &mut Criterion,
	client: Arc<impl ClientT>,
	name: &str,
	request: RequestType,
) {

	for method in request.methods() {
		let bench_name = format!("{}/{}", name, method);
		let mut group = crit.benchmark_group(request.group_name(&bench_name));
		for batch_size in [2, 5, 10, 50, 100usize].iter() {
			let batch = vec![(method, None); *batch_size];
			group.throughput(Throughput::Elements(*batch_size as u64));
			group.bench_with_input(BenchmarkId::from_parameter(batch_size), batch_size, |b, _| {
				b.to_async(rt).iter(|| async { client.batch_request::<String>(batch.clone()).await.unwrap() })
			});
		}
		group.finish();
	}
}

fn ws_concurrent_conn_calls(rt: &TokioRuntime, crit: &mut Criterion, url: &str, name: &str, request: RequestType) {
	let method = request.methods()[0];
	let bench_name = format!("{}/{}", name, method);
	let mut group = crit.benchmark_group(request.group_name(&bench_name));
	for conns in [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
		group.bench_function(format!("{}", conns), |b| {
			b.to_async(rt).iter_with_setup(
				|| {
					let mut clients = Vec::new();
					// We have to use `block_in_place` here since `b.to_async(rt)` automatically enters the
					// runtime context and simply calling `block_on` here will cause the code to panic.
					tokio::task::block_in_place(|| {
						tokio::runtime::Handle::current().block_on(async {
							for _ in 0..conns {
								clients.push(WsClientBuilder::default().max_request_body_size(u32::MAX).build(url).await.unwrap());
							}
						})
					});

					clients
				},
				|clients| async {
					let tasks = clients.into_iter().map(|client| {
						rt.spawn(async move {
							let futs = FuturesUnordered::new();

							for _ in 0..10 {
								futs.push(client.request::<String>(method, None));
							}

							join_all(futs).await;
						})
					});
					join_all(tasks).await;
				},
			)
		});
	}
	group.finish();
}

fn ws_concurrent_conn_subs(rt: &TokioRuntime, crit: &mut Criterion, url: &str, name: &str, request: RequestType) {
	let mut group = crit.benchmark_group(request.group_name(name));
	for conns in [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
		group.bench_function(format!("{}", conns), |b| {
			b.to_async(rt).iter_with_setup(
				|| {
					let mut clients = Vec::new();
					// We have to use `block_in_place` here since `b.to_async(rt)` automatically enters the
					// runtime context and simply calling `block_on` here will cause the code to panic.
					tokio::task::block_in_place(|| {
						tokio::runtime::Handle::current().block_on(async {
							for _ in 0..conns {
								clients.push(WsClientBuilder::default().max_request_body_size(u32::MAX).build(url).await.unwrap());
							}
						})
					});

					clients
				},
				|clients| async {
					let tasks = clients.into_iter().map(|client| {
						rt.spawn(async move {
							let futs = FuturesUnordered::new();

							for _ in 0..10 {
								let fut = client.subscribe::<String>(SUB_METHOD_NAME, None, UNSUB_METHOD_NAME).then(
									|sub| async move {
										let mut s = sub.unwrap();
										let res = s.next().await.unwrap().unwrap();
										res
									},
								);

								futs.push(Box::pin(fut));
							}

							join_all(futs).await;
						})
					});
					join_all(tasks).await;
				},
			)
		});
	}
	group.finish();
}

fn http_concurrent_conn_calls(rt: &TokioRuntime, crit: &mut Criterion, url: &str, name: &str, request: RequestType) {
	let method = request.methods()[0];
	let bench_name = format!("{}/{}", name, method);
	let mut group = crit.benchmark_group(request.group_name(&bench_name));
	for conns in [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
		group.bench_function(format!("{}", conns), |b| {
			b.to_async(rt).iter_with_setup(
				|| (0..conns).map(|_| HttpClientBuilder::default().max_request_body_size(u32::MAX).build(url).unwrap()),
				|clients| async {
					let tasks = clients.map(|client| {
						rt.spawn(async move {
							client.request::<String>(method, None).await.unwrap();
						})
					});
					join_all(tasks).await;
				},
			)
		});
	}
	group.finish();
}
