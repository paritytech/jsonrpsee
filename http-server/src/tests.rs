#![cfg(test)]

use std::net::SocketAddr;

use crate::{HttpServerBuilder, RpcContextModule};
use futures_util::FutureExt;
use jsonrpsee_http_client::v2::params::RpcParams;
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::{Id, StatusCode, TestContext};
use jsonrpsee_test_utils::TimeoutFutureExt;
use jsonrpsee_types::error::CallError;
use serde_json::Value as JsonValue;

async fn server() -> SocketAddr {
	let mut server = HttpServerBuilder::default().build("127.0.0.1:0".parse().unwrap()).unwrap();
	let addr = server.local_addr().unwrap();
	server.register_method("say_hello", |_| Ok("lo")).unwrap();
	server.register_async_method("say_hello_async", |_: RpcParams| async move { Ok("lo") }.boxed()).unwrap();
	server
		.register_method("add", |params| {
			let params: Vec<u64> = params.parse()?;
			let sum: u64 = params.into_iter().sum();
			Ok(sum)
		})
		.unwrap();
	server
		.register_method("multiparam", |params| {
			let params: (String, String, Vec<u8>) = params.parse()?;
			let r = format!("string1={}, string2={}, vec={}", params.0.len(), params.1.len(), params.2.len());
			Ok(r)
		})
		.unwrap();
	server.register_method("notif", |_| Ok("")).unwrap();
	tokio::spawn(async move { server.start().await.unwrap() });
	addr
}

/// Run server with user provided context.
pub async fn server_with_context() -> SocketAddr {
	let mut server = HttpServerBuilder::default().build("127.0.0.1:0".parse().unwrap()).unwrap();

	let ctx = TestContext;
	let mut rpc_ctx = RpcContextModule::new(ctx);

	rpc_ctx
		.register_method("should_err", |_p, ctx| {
			let _ = ctx.err().map_err(|e| CallError::Failed(e.into()))?;
			Ok("err")
		})
		.unwrap();

	rpc_ctx
		.register_method("should_ok", |_p, ctx| {
			let _ = ctx.ok().map_err(|e| CallError::Failed(e.into()))?;
			Ok("ok")
		})
		.unwrap();

	rpc_ctx
		.register_async_method("should_ok_async", |_p, ctx| {
			async move {
				let _ = ctx.ok().map_err(|e| CallError::Failed(e.into()))?;
				Ok("ok")
			}
			.boxed()
		})
		.unwrap();

	let rpc_module = rpc_ctx.into_module();
	server.register_module(rpc_module).unwrap();
	let addr = server.local_addr().unwrap();

	tokio::spawn(async { server.start().with_default_timeout().await.unwrap() });
	addr
}

#[tokio::test]
async fn single_method_call_works() {
	let _ = env_logger::try_init();
	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	for i in 0..10 {
		let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i);
		let response = http_request(req.into(), uri.clone()).with_default_timeout().await.unwrap().unwrap();
		assert_eq!(response.status, StatusCode::OK);
		assert_eq!(response.body, ok_response(JsonValue::String("lo".to_owned()), Id::Num(i)));
	}
}

#[tokio::test]
async fn async_method_call_works() {
	let _ = env_logger::try_init();
	let addr = server().await;
	let uri = to_http_uri(addr);

	for i in 0..10 {
		let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello_async","id":{}}}"#, i);
		let response = http_request(req.into(), uri.clone()).await.unwrap();
		assert_eq!(response.status, StatusCode::OK);
		assert_eq!(response.body, ok_response(JsonValue::String("lo".to_owned()), Id::Num(i)));
	}
}

#[tokio::test]
async fn invalid_single_method_call() {
	let _ = env_logger::try_init();
	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":1, "params": "bar"}"#;
	let response = http_request(req.into(), uri.clone()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, parse_error(Id::Null));
}

#[tokio::test]
async fn single_method_call_with_params() {
	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_multiple_params_of_different_types() {
	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"multiparam", "params":["Hello", "World", [0,1,2,3]],"id":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response(JsonValue::String("string1=5, string2=5, vec=4".into()), Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_faulty_params_returns_err() {
	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":["Invalid"],"id":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, invalid_params(Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_faulty_context() {
	let addr = server_with_context().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"should_err", "params":[],"id":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, invalid_context("RPC context failed", Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_ok_context() {
	let addr = server_with_context().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"should_ok", "params":[],"id":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response("ok".into(), Id::Num(1)));
}

#[tokio::test]
async fn async_method_call_with_ok_context() {
	let addr = server_with_context().await;
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"should_ok_async", "params":[],"id":1}"#;
	let response = http_request(req.into(), uri).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response("ok".into(), Id::Num(1)));
}

#[tokio::test]
async fn valid_batched_method_calls() {
	let _ = env_logger::try_init();

	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"[
		{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1},
		{"jsonrpc":"2.0","method":"add", "params":[3, 4],"id":2},
		{"jsonrpc":"2.0","method":"say_hello","id":3},
		{"jsonrpc":"2.0","method":"add", "params":[5, 6],"id":4}
	]"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(
		response.body,
		r#"[{"jsonrpc":"2.0","result":3,"id":1},{"jsonrpc":"2.0","result":7,"id":2},{"jsonrpc":"2.0","result":"lo","id":3},{"jsonrpc":"2.0","result":11,"id":4}]"#
	);
}

#[tokio::test]
async fn batched_notifications() {
	let _ = env_logger::try_init();

	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"[{"jsonrpc": "2.0", "method": "notif", "params": [1,2,4]},{"jsonrpc": "2.0", "method": "notif", "params": [7]}]"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	// Note: on HTTP acknowledge the notification with an empty response.
	assert_eq!(response.body, "");
}

#[tokio::test]
async fn invalid_batched_method_calls() {
	let _ = env_logger::try_init();

	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	// batch with no requests
	let req = r#"[]"#;
	let response = http_request(req.into(), uri.clone()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, invalid_request(Id::Null));

	// batch with invalid request
	let req = r#"[123]"#;
	let response = http_request(req.into(), uri.clone()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	// Note: according to the spec the `id` should be `null` here, not 123.
	assert_eq!(response.body, invalid_request(Id::Num(123)));

	// batch with invalid request
	let req = r#"[1, 2, 3]"#;
	let response = http_request(req.into(), uri.clone()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	// Note: according to the spec this should return an array of three `Invalid Request`s
	assert_eq!(response.body, parse_error(Id::Null));

	// invalid JSON in batch
	let req = r#"[
		{"jsonrpc": "2.0", "method": "sum", "params": [1,2,4], "id": "1"},
		{"jsonrpc": "2.0", "method"
	  ]"#;
	let response = http_request(req.into(), uri.clone()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, parse_error(Id::Null));
}

#[tokio::test]
async fn should_return_method_not_found() {
	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be Null.
	assert_eq!(response.body, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, invalid_request(Id::Num(1)));
}

#[tokio::test]
async fn notif_works() {
	let addr = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"bar"}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, "");
}
