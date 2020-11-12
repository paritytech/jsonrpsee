#![cfg(test)]

use crate::http::HttpServer;
use crate::types::jsonrpc::JsonValue;
use futures::channel::oneshot::{self, Sender};
use futures::future::FutureExt;
use futures::{pin_mut, select};
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::{Id, StatusCode};
use std::net::SocketAddr;

async fn server(server_started_tx: Sender<SocketAddr>) {
	let server = HttpServer::new("127.0.0.1:0").await.unwrap();
	let mut hello = server.register_method("say_hello".to_owned()).unwrap();
	let mut add = server.register_method("add".to_owned()).unwrap();
	let mut notif = server.register_notification("notif".to_owned(), false).unwrap();
	server_started_tx.send(*server.local_addr()).unwrap();

	loop {
		let hello_fut = async {
			let handle = hello.next().await;
			handle.respond(Ok(JsonValue::String("hello".to_owned()))).await.unwrap();
		}
		.fuse();

		let add_fut = async {
			let handle = add.next().await;
			let params: Vec<u64> = handle.params().clone().parse().unwrap();
			let sum: u64 = params.iter().sum();
			handle.respond(Ok(JsonValue::Number(sum.into()))).await.unwrap();
		}
		.fuse();

		let notif_fut = async {
			let params = notif.next().await;
			println!("received notification: say_hello params[{:?}]", params);
		}
		.fuse();

		pin_mut!(hello_fut, add_fut, notif_fut);
		select! {
			say_hello = hello_fut => (),
			add = add_fut => (),
			notif = notif_fut => (),
			complete => (),
		};
	}
}

#[tokio::test]
async fn single_method_call_works() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();
	let uri = to_http_uri(server_addr);

	for i in 0..10 {
		let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i);
		let response = http_request(req.into(), uri.clone()).await.unwrap();
		assert_eq!(response.status, StatusCode::OK);
		assert_eq!(response.body, ok_response(JsonValue::String("hello".to_owned()), Id::Num(i)));
	}
}

#[tokio::test]
async fn single_method_call_with_params() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = http_request(req.into(), to_http_uri(server_addr)).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn should_return_method_not_found() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
	let response = http_request(req.into(), to_http_uri(server_addr)).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
	let response = http_request(req.into(), to_http_uri(server_addr)).await.unwrap();
	// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be Null.
	assert_eq!(response.body, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
	let response = http_request(req.into(), to_http_uri(server_addr)).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, invalid_request(Id::Num(1)));
}
