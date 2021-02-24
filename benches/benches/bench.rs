use criterion::*;
use futures::channel::oneshot::{self, Sender};
use jsonrpsee_http_server::HttpServer;
use jsonrpsee_types::{
	http::HttpConfig,
	jsonrpc::{JsonValue, Params},
};
use jsonrpsee_ws_server::WsServer;
use std::net::SocketAddr;

criterion_group!(benches, /*http_requests,*/ websocket_requests);
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
	let mut server = WsServer::new("127.0.0.1:0").await.unwrap();

	tx.send(server.local_addr().unwrap()).unwrap();

	server.register_method("say_hello", |_| Ok("lo")).unwrap();

	server.start().await;
}

pub fn http_requests(c: &mut criterion::Criterion) {
	let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
	let (tx_addr, rx_addr) = oneshot::channel::<SocketAddr>();
	rt.spawn(http_server(tx_addr));
	let server_addr = rt.block_on(rx_addr).unwrap();
	let client = jsonrpsee_client::http(&format!("http://{}", server_addr));

	c.bench_function("synchronous_http_round_trip", |b| {
		b.iter(|| {
			rt.block_on(async {
				let _: JsonValue = black_box(client.request("say_hello", Params::None).await.unwrap());
			})
		})
	});

	// c.bench_function_over_inputs(
	//     "concurrent_http_round_trip",
	//     move |b: &mut Bencher, size: &usize| {
	//         b.iter(|| {
	//             let mut tasks = Vec::new();
	//             for _ in 0..*size {
	//                 let client_rc = client.clone();
	//                 let task = rt.spawn(async move {
	//                     let _: Result<JsonValue, _> = black_box(client_rc.request("say_hello", Params::None)).await;
	//                 });
	//                 tasks.push(task);
	//             }
	//             for task in tasks {
	//                 rt.block_on(task).unwrap();
	//             }
	//         })
	//     },
	//     concurrent_tasks(),
	// );
}

pub fn websocket_requests(c: &mut criterion::Criterion) {
	let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
	let (tx_addr, rx_addr) = oneshot::channel::<SocketAddr>();
	rt.spawn(ws_server(tx_addr));
	let server_addr = rt.block_on(rx_addr).unwrap();
	let url = format!("ws://{}", server_addr);
	let client = rt.block_on(jsonrpsee_client::ws(&url));

	c.bench_function("synchronous_websocket_round_trip", |b| {
		b.iter(|| {
			rt.block_on(async {
				let _: JsonValue = black_box(client.request("say_hello", Params::None).await.unwrap());
			})
		})
	});

	c.bench_function_over_inputs(
	    "concurrent_websocket_round_trip",
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
