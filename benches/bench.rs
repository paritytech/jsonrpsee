use std::sync::Arc;

use crate::helpers::{ws_handshake, KIB};
use criterion::*;
use futures_util::future::{join_all, FutureExt};
use futures_util::stream::FuturesUnordered;
use helpers::{http_client, ws_client, SUB_METHOD_NAME, UNSUB_METHOD_NAME};
use jsonrpsee::core::client::{ClientT, SubscriptionClientT};
use jsonrpsee::http_client::HeaderMap;
use jsonrpsee::types::params::EmptyParams;
use jsonrpsee::types::{BatchRequestBuilder, Id, ParamsSer, RequestSer, ToRpcParams, UnnamedParamsBuilder};
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
	targets = SyncBencher::http_benches, SyncBencher::websocket_benches
);
criterion_group!(
	name = async_benches;
	config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
	targets = AsyncBencher::http_benches, AsyncBencher::websocket_benches
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
	fn methods(self) -> [&'static str; 3] {
		match self {
			RequestType::Sync => crate::helpers::SYNC_METHODS,
			RequestType::Async => crate::helpers::ASYNC_METHODS,
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
	// Construct the serialized request using the `ParamsSer` directly.
	crit.bench_function("jsonrpsee_types_baseline_params", |b| {
		b.iter(|| {
			let params = &[1_u64.into(), 2_u32.into()];
			let params = ParamsSer::ArrayRef(params);
			let params = serde_json::to_string(&params).unwrap();
			let params = serde_json::value::RawValue::from_string(params).unwrap();

			let request = RequestSer::new(&Id::Number(0), "say_hello", Some(params));
			v2_serialize(request);
		})
	});

	// Construct the serialized request using the `UnnamedParamsBuilder`.
	crit.bench_function("jsonrpsee_types_unnamed_params", |b| {
		b.iter(|| {
			let mut builder = UnnamedParamsBuilder::new();
			builder.insert(1u64).unwrap();
			builder.insert(2u32).unwrap();
			let params = builder.build().to_rpc_params().expect("Valid params");
			let request = RequestSer::new(&Id::Number(0), "say_hello", params);
			v2_serialize(request);
		})
	});
}

trait RequestBencher {
	const REQUEST_TYPE: RequestType;

	fn http_benches(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let (url, _server) = rt.block_on(helpers::http_server(rt.handle().clone()));
		let client = Arc::new(http_client(&url, HeaderMap::new()));
		round_trip(&rt, crit, client.clone(), "http_round_trip", Self::REQUEST_TYPE);
		http_concurrent_conn_calls(&rt, crit, &url, "http_concurrent_conn_calls", Self::REQUEST_TYPE);
		batch_round_trip(&rt, crit, client, "http_batch_requests", Self::REQUEST_TYPE);
		http_custom_headers_round_trip(&rt, crit, &url, "http_custom_headers_round_trip", Self::REQUEST_TYPE);
	}

	fn websocket_benches(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let (url, _server) = rt.block_on(helpers::ws_server(rt.handle().clone()));
		let client = Arc::new(rt.block_on(ws_client(&url)));
		round_trip(&rt, crit, client.clone(), "ws_round_trip", Self::REQUEST_TYPE);
		ws_concurrent_conn_calls(&rt, crit, &url, "ws_concurrent_conn_calls", Self::REQUEST_TYPE);
		ws_concurrent_conn_subs(&rt, crit, &url, "ws_concurrent_conn_subs", Self::REQUEST_TYPE);
		batch_round_trip(&rt, crit, client, "ws_batch_requests", Self::REQUEST_TYPE);
		ws_custom_headers_handshake(&rt, crit, &url, "ws_custom_headers_handshake", Self::REQUEST_TYPE);
	}

	fn subscriptions(crit: &mut Criterion) {
		let rt = TokioRuntime::new().unwrap();
		let (url, _server) = rt.block_on(helpers::ws_server(rt.handle().clone()));
		let client = Arc::new(rt.block_on(ws_client(&url)));
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
				black_box(client.request::<String, EmptyParams>(method, EmptyParams).await.unwrap());
			})
		});
	}
}

fn sub_round_trip(rt: &TokioRuntime, crit: &mut Criterion, client: Arc<impl SubscriptionClientT>, name: &str) {
	let mut group = crit.benchmark_group(name);
	group.bench_function("subscribe", |b| {
		b.to_async(rt).iter_with_large_drop(|| async {
			black_box(
				client.subscribe::<String, EmptyParams>(SUB_METHOD_NAME, EmptyParams, UNSUB_METHOD_NAME).await.unwrap(),
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
							.subscribe::<String, EmptyParams>(SUB_METHOD_NAME, EmptyParams, UNSUB_METHOD_NAME)
							.await
							.unwrap()
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
					client
						.subscribe::<String, EmptyParams>(SUB_METHOD_NAME, EmptyParams, UNSUB_METHOD_NAME)
						.await
						.unwrap()
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
			let mut batch = BatchRequestBuilder::new();
			for _ in 0..*batch_size {
				batch.insert(method, EmptyParams).unwrap();
			}
			group.throughput(Throughput::Elements(*batch_size as u64));
			group.bench_with_input(BenchmarkId::from_parameter(batch_size), batch_size, |b, _| {
				b.to_async(rt).iter(|| async { client.batch_request::<String>(batch.clone()).await.unwrap() })
			});
		}
		group.finish();
	}
}

fn ws_concurrent_conn_calls(rt: &TokioRuntime, crit: &mut Criterion, url: &str, name: &str, request: RequestType) {
	let methods = request.methods();
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
								clients.push(ws_client(url).await);
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
								futs.push(client.request::<String, EmptyParams>(methods[0], EmptyParams));
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

// As this is so slow only fast calls are executed in this benchmark.
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
								clients.push(ws_client(url).await);
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
								let fut = client
									.subscribe::<String, EmptyParams>(SUB_METHOD_NAME, EmptyParams, UNSUB_METHOD_NAME)
									.then(|sub| async move {
										let mut s = sub.unwrap();

										s.next().await.unwrap().unwrap()
									});

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

// As this is so slow only fast calls are executed in this benchmark.
fn http_concurrent_conn_calls(rt: &TokioRuntime, crit: &mut Criterion, url: &str, name: &str, request: RequestType) {
	let method = request.methods()[0];
	let bench_name = format!("{}/{}", name, method);
	let mut group = crit.benchmark_group(request.group_name(&bench_name));
	for conns in [2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
		group.bench_function(format!("{}", conns), |b| {
			b.to_async(rt).iter_with_setup(
				|| (0..conns).map(|_| http_client(url, HeaderMap::new())),
				|clients| async {
					let tasks = clients.map(|client| {
						rt.spawn(async move {
							client.request::<String, EmptyParams>(method, EmptyParams).await.unwrap();
						})
					});
					join_all(tasks).await;
				},
			)
		});
	}
	group.finish();
}

/// Bench `round_trip` with different header sizes.
fn http_custom_headers_round_trip(
	rt: &TokioRuntime,
	crit: &mut Criterion,
	url: &str,
	name: &str,
	request: RequestType,
) {
	let method_name = request.methods()[0];

	for header_size in [0, KIB, 5 * KIB, 25 * KIB, 100 * KIB] {
		let mut headers = HeaderMap::new();
		if header_size != 0 {
			headers.insert("key", "A".repeat(header_size).parse().unwrap());
		}

		let client = Arc::new(http_client(url, headers));
		let bench_name = format!("{}/{}kb", name, header_size / KIB);

		crit.bench_function(&request.group_name(&bench_name), |b| {
			b.to_async(rt).iter(|| async {
				black_box(client.request::<String, EmptyParams>(method_name, EmptyParams).await.unwrap());
			})
		});
	}
}

/// Bench WS handshake with different header sizes.
fn ws_custom_headers_handshake(rt: &TokioRuntime, crit: &mut Criterion, url: &str, name: &str, request: RequestType) {
	let mut group = crit.benchmark_group(request.group_name(name));
	for header_size in [0, KIB, 2 * KIB, 4 * KIB] {
		group.bench_function(format!("{}kb", header_size / KIB), |b| {
			b.to_async(rt).iter(|| async move {
				let mut headers = HeaderMap::new();
				if header_size != 0 {
					headers.insert("key", "A".repeat(header_size).parse().unwrap());
				}

				ws_handshake(url, headers).await;
			})
		});
	}
	group.finish();
}
