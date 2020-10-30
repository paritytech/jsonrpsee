use async_std::task::block_on;
use criterion::*;
use futures::channel::oneshot::{self, Sender};
use jsonrpsee::client::{HttpClient, WsClient};
use jsonrpsee::http::HttpServer;
use jsonrpsee::types::jsonrpc::{JsonValue, Params};
use jsonrpsee::ws::WsServer;
use std::net::SocketAddr;

criterion_group!(benches, http, ws);
criterion_main!(benches);

async fn http_server(tx: Sender<SocketAddr>) {
	let server = HttpServer::new("127.0.0.1:0").await.unwrap();
	let mut say_hello = server.register_method("say_hello".to_string()).unwrap();
	tx.send(*server.local_addr()).unwrap();
	loop {
		let r = say_hello.next().await;
		r.respond(Ok(JsonValue::String("lo".to_owned()))).await.unwrap();
	}
}

async fn ws_server(tx: Sender<SocketAddr>) {
	let server = WsServer::new("127.0.0.1:0").await.unwrap();
	let mut say_hello = server.register_method("say_hello".to_string()).unwrap();
	tx.send(*server.local_addr()).unwrap();
	loop {
		let r = say_hello.next().await;
		r.respond(Ok(JsonValue::String("lo".to_owned()))).await.unwrap();
	}
}

pub fn http(c: &mut criterion::Criterion) {
	c.bench_function("http 100 requests", |b| {
		let (tx_addr, rx_addr) = oneshot::channel::<SocketAddr>();
		async_std::task::spawn(http_server(tx_addr));
		let server_addr = block_on(rx_addr).unwrap();
		let client = HttpClient::new(&format!("http://{}", server_addr));

		b.iter(|| {
			block_on(async {
				for _ in 0..100 {
					let _: JsonValue = black_box(client.request("say_hello", Params::None).await.unwrap());
				}
			})
		})
	});
}

pub fn ws(c: &mut criterion::Criterion) {
	c.bench_function("ws 100 request", |b| {
		let (tx_addr, rx_addr) = oneshot::channel::<SocketAddr>();
		async_std::task::spawn(ws_server(tx_addr));
		let server_addr = block_on(rx_addr).unwrap();
		let client = block_on(WsClient::new(&format!("ws://{}", server_addr))).unwrap();

		b.iter(|| {
			block_on(async {
				for _ in 0..100 {
					let _: JsonValue = black_box(client.request("say_hello", Params::None).await.unwrap());
				}
			})
		})
	});
}
