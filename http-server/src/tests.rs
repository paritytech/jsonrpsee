#![cfg(test)]

use std::net::SocketAddr;

use crate::HttpServerBuilder;
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::{Id, StatusCode};
use serde_json::Value as JsonValue;

async fn server() -> SocketAddr {
	let mut server = HttpServerBuilder::default().build("127.0.0.1:0".parse().unwrap()).unwrap();
	let addr = server.local_addr().unwrap();
	server.register_method("say_hello", |_| Ok("lo")).unwrap();
	server
		.register_method("add", |params| {
			let params: Vec<u64> = params.parse()?;
			let sum: u64 = params.into_iter().sum();
			Ok(sum)
		})
		.unwrap();
	server.register_method("notif", |_| Ok("")).unwrap();
	tokio::spawn(async move { server.start().await.unwrap() });
	addr
}

#[tokio::test]
async fn single_method_call_works() {
	let _ = env_logger::try_init();
	let addr = server().await;
	let uri = to_http_uri(addr);

	for i in 0..10 {
		let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i);
		let response = http_request(req.into(), uri.clone()).await.unwrap();
		assert_eq!(response.status, StatusCode::OK);
		assert_eq!(response.body, ok_response(JsonValue::String("lo".to_owned()), Id::Num(i)));
	}
}

#[tokio::test]
async fn single_method_call_with_params() {
	let addr = server().await;
	let uri = to_http_uri(addr);

	std::thread::sleep(std::time::Duration::from_secs(2));

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = http_request(req.into(), uri).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

// Batch request example from spec (https://www.jsonrpc.org/specification)
// --> [
//         {"jsonrpc": "2.0", "method": "sum", "params": [1,2,4], "id": "1"},
//         {"jsonrpc": "2.0", "method": "notify_hello", "params": [7]},
//         {"jsonrpc": "2.0", "method": "subtract", "params": [42,23], "id": "2"},
//         {"foo": "boo"},
//         {"jsonrpc": "2.0", "method": "foo.get", "params": {"name": "myself"}, "id": "5"},
//         {"jsonrpc": "2.0", "method": "get_data", "id": "9"}
//     ]
// <-- [
//         {"jsonrpc": "2.0", "result": 7, "id": "1"},
//         {"jsonrpc": "2.0", "result": 19, "id": "2"},
//         {"jsonrpc": "2.0", "error": {"code": -32600, "message": "Invalid Request"}, "id": null},
//         {"jsonrpc": "2.0", "error": {"code": -32601, "message": "Method not found"}, "id": "5"},
//         {"jsonrpc": "2.0", "result": ["hello", 5], "id": "9"}
//     ]
#[tokio::test]
async fn batched_method_calls() {
	let _ = env_logger::try_init();

	let addr = server().await;
	let uri = to_http_uri(addr);

	let req = r#"[
		{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1},
		{"jsonrpc":"2.0","method":"add", "params":[3, 4],"id":2},
		{"jsonrpc":"2.0","method":"say_hello","id":3},
		{"jsonrpc":"2.0","method":"add", "params":[5, 6],"id":4}
	]"#;
	let response = http_request(req.into(), uri).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	log::info!("[test] Response body: {:?}", response.body);
	// TODO: the response should be wrapped in `[]`, but it's a straight up `String`
	assert_eq!(
		response.body,
		r#"[{"jsonrpc":"2.0","result":3,"id":1},{"jsonrpc":"2.0","result":7,"id":2},{"jsonrpc":"2.0","result":"lo","id":3},{"jsonrpc":"2.0","result":11,"id":4}]"#
	);
}

#[tokio::test]
async fn should_return_method_not_found() {
	let addr = server().await;
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
	let response = http_request(req.into(), uri).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
	let addr = server().await;
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
	let response = http_request(req.into(), uri).await.unwrap();
	// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be Null.
	assert_eq!(response.body, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
	let addr = server().await;
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
	let response = http_request(req.into(), uri).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, invalid_request(Id::Num(1)));
}
