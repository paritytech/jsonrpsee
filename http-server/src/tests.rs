#![cfg(test)]

use crate::{HttpConfig, HttpServer};
use futures::channel::oneshot::{self, Sender};
use futures::future::FutureExt;
use futures::{pin_mut, select};
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::{Id, StatusCode};
use jsonrpsee_types::jsonrpc::JsonValue;

const SOCK_ADDR: &str = "127.0.0.1:9933";

async fn server() {
	let mut server =
		HttpServer::new(&SOCK_ADDR.parse().unwrap(), HttpConfig::default(), Default::default()).await.unwrap();
	server.register_method("say_hello", |_| Ok("lo")).unwrap();
	server
		.register_method("add", |params| {
			let params: Vec<u64> = params.parse()?;
			let sum: u64 = params.into_iter().sum();
			Ok(sum)
		})
		.unwrap();
	server.register_method("notif", |_| Ok("")).unwrap();
	server.start().await.unwrap();
}

#[tokio::test]
async fn single_method_call_works() {
	let _ = env_logger::try_init();
	tokio::spawn(async { server().await });
	let uri = to_http_uri(SOCK_ADDR.parse().unwrap());

	std::thread::sleep(std::time::Duration::from_secs(3));

	for i in 0..10 {
		let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i);
		let response = http_request(req.into(), uri.clone()).await.unwrap();
		assert_eq!(response.status, StatusCode::OK);
		assert_eq!(response.body, ok_response(JsonValue::String("hello".to_owned()), Id::Num(i)));
	}
}

#[tokio::test]
async fn single_method_call_with_params() {
	tokio::spawn(server());

	std::thread::sleep(std::time::Duration::from_secs(2));

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = http_request(req.into(), SOCK_ADDR.parse().unwrap()).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn should_return_method_not_found() {
	tokio::spawn(server());
	std::thread::sleep(std::time::Duration::from_secs(2));

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
	let response = http_request(req.into(), to_http_uri(SOCK_ADDR.parse().unwrap())).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
	tokio::spawn(server());
	std::thread::sleep(std::time::Duration::from_secs(2));

	let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
	let response = http_request(req.into(), to_http_uri(SOCK_ADDR.parse().unwrap())).await.unwrap();
	// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be Null.
	assert_eq!(response.body, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
	tokio::spawn(server());
	std::thread::sleep(std::time::Duration::from_secs(2));

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
	let response = http_request(req.into(), to_http_uri(SOCK_ADDR.parse().unwrap())).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, invalid_request(Id::Num(1)));
}
