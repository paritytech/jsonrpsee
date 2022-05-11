// Copyright 2019-2021 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#![cfg(test)]

use std::net::SocketAddr;
use std::time::Duration;

use crate::types::error::CallError;
use crate::{server::ServerHandle, HttpServerBuilder, RpcModule};
use jsonrpsee_core::Error;
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::mocks::{Id, StatusCode, TestContext};
use jsonrpsee_test_utils::TimeoutFutureExt;
use serde_json::Value as JsonValue;

async fn server() -> (SocketAddr, ServerHandle) {
	let server = HttpServerBuilder::default().health_api("health", "system_health").build("127.0.0.1:0").await.unwrap();
	let ctx = TestContext;
	let mut module = RpcModule::new(ctx);
	let addr = server.local_addr().unwrap();
	module.register_method("say_hello", |_, _| Ok("lo")).unwrap();
	module.register_async_method("say_hello_async", |_, _| async move { Ok("lo") }).unwrap();
	module
		.register_method("add", |params, _| {
			let params: Vec<u64> = params.parse()?;
			let sum: u64 = params.into_iter().sum();
			Ok(sum)
		})
		.unwrap();
	module
		.register_method("multiparam", |params, _| {
			let params: (String, String, Vec<u8>) = params.parse()?;
			let r = format!("string1={}, string2={}, vec={}", params.0.len(), params.1.len(), params.2.len());
			Ok(r)
		})
		.unwrap();
	module.register_method("notif", |_, _| Ok("")).unwrap();
	module
		.register_method("should_err", |_, ctx| {
			let _ = ctx.err().map_err(CallError::Failed)?;
			Ok("err")
		})
		.unwrap();

	module
		.register_method("should_ok", |_, ctx| {
			let _ = ctx.ok().map_err(CallError::Failed)?;
			Ok("ok")
		})
		.unwrap();
	module
		.register_async_method("should_ok_async", |_p, ctx| async move {
			let _ = ctx.ok().map_err(CallError::Failed)?;
			Ok("ok")
		})
		.unwrap();
	module.register_method("system_health", |_, _| Ok("i'm health")).unwrap();

	let server_handle = server.start(module).unwrap();
	(addr, server_handle)
}

#[tokio::test]
async fn single_method_call_works() {
	let _ = env_logger::try_init();
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
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
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
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
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":1, "params": "bar"}"#;
	let response = http_request(req.into(), uri.clone()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, parse_error(Id::Null));
}

#[tokio::test]
async fn single_method_call_with_params() {
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_multiple_params_of_different_types() {
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"multiparam", "params":["Hello", "World", [0,1,2,3]],"id":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response(JsonValue::String("string1=5, string2=5, vec=4".into()), Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_faulty_params_returns_err() {
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);
	let expected = r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"invalid type: string \"this should be a number\", expected u64 at line 1 column 26"},"id":1}"#;

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":["this should be a number"],"id":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, expected);
}

#[tokio::test]
async fn single_method_call_with_faulty_context() {
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"should_err","params":[],"id":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, call_execution_failed("RPC context failed", Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_ok_context() {
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"should_ok", "params":[],"id":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response("ok".into(), Id::Num(1)));
}

#[tokio::test]
async fn async_method_call_with_ok_context() {
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"should_ok_async", "params":[],"id":1}"#;
	let response = http_request(req.into(), uri).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response("ok".into(), Id::Num(1)));
}

#[tokio::test]
async fn valid_batched_method_calls() {
	let _ = env_logger::try_init();

	let (addr, _handle) = server().with_default_timeout().await.unwrap();
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

	let (addr, _handle) = server().with_default_timeout().await.unwrap();
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

	let (addr, _handle) = server().with_default_timeout().await.unwrap();
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
async fn garbage_request_fails() {
	let (addr, _handle) = server().await;
	let uri = to_http_uri(addr);

	let req = r#"dsdfs fsdsfds"#;
	let response = http_request(req.into(), uri.clone()).await.unwrap();
	assert_eq!(response.body, parse_error(Id::Null));

	let req = r#"{ "#;
	let response = http_request(req.into(), uri.clone()).await.unwrap();
	assert_eq!(response.body, parse_error(Id::Null));

	let req = r#"         {"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = http_request(req.into(), uri.clone()).await.unwrap();
	assert_eq!(response.body, parse_error(Id::Null));

	let req = r#"{}"#;
	let response = http_request(req.into(), uri.clone()).await.unwrap();
	assert_eq!(response.body, parse_error(Id::Null));

	let req = r#"{sds}"#;
	let response = http_request(req.into(), uri.clone()).await.unwrap();
	assert_eq!(response.body, parse_error(Id::Null));

	let req = r#"["#;
	let response = http_request(req.into(), uri.clone()).await.unwrap();
	assert_eq!(response.body, parse_error(Id::Null));

	let req = r#"[dsds]"#;
	let response = http_request(req.into(), uri.clone()).await.unwrap();
	assert_eq!(response.body, parse_error(Id::Null));

	let req = r#" [{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}]"#;
	let response = http_request(req.into(), uri.clone()).await.unwrap();
	assert_eq!(response.body, parse_error(Id::Null));

	let req = r#"[]"#;
	let response = http_request(req.into(), uri.clone()).await.unwrap();
	assert_eq!(response.body, invalid_request(Id::Null));

	let req = r#"[{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = http_request(req.into(), uri.clone()).await.unwrap();
	assert_eq!(response.body, parse_error(Id::Null));
}

#[tokio::test]
async fn should_return_method_not_found() {
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be
	// Null.
	assert_eq!(response.body, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, invalid_request(Id::Num(1)));
}

#[tokio::test]
async fn notif_works() {
	let (addr, _handle) = server().with_default_timeout().await.unwrap();
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"bar"}"#;
	let response = http_request(req.into(), uri).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, "");
}

#[tokio::test]
async fn can_register_modules() {
	let cx = String::new();
	let mut mod1 = RpcModule::new(cx);

	let cx2 = Vec::<u8>::new();
	let mut mod2 = RpcModule::new(cx2);

	assert_eq!(mod1.method_names().count(), 0);
	mod1.register_method("bla", |_, cx| Ok(format!("Gave me {}", cx))).unwrap();
	mod1.register_method("bla2", |_, cx| Ok(format!("Gave me {}", cx))).unwrap();
	mod2.register_method("yada", |_, cx| Ok(format!("Gave me {:?}", cx))).unwrap();

	// Won't register, name clashes
	mod2.register_method("bla", |_, cx| Ok(format!("Gave me {:?}", cx))).unwrap();

	assert_eq!(mod1.method_names().count(), 2);

	let err = mod1.merge(mod2).unwrap_err();

	let expected_err = Error::MethodAlreadyRegistered(String::from("bla"));
	assert_eq!(err.to_string(), expected_err.to_string());
	assert_eq!(mod1.method_names().count(), 2);
}

#[tokio::test]
async fn stop_works() {
	let _ = env_logger::try_init();
	let (_addr, server_handle) = server().with_default_timeout().await.unwrap();
	assert!(matches!(server_handle.stop().unwrap().await, Ok(_)));
}

#[tokio::test]
async fn run_forever() {
	const TIMEOUT: Duration = Duration::from_millis(200);

	let _ = env_logger::try_init();
	let (_addr, server_handle) = server().with_default_timeout().await.unwrap();

	assert!(matches!(server_handle.with_timeout(TIMEOUT).await, Err(_timeout_err)));

	let (_addr, server_handle) = server().await;
	server_handle.handle.as_ref().unwrap().abort();

	// Cancelled task is still considered to be finished without errors.
	// A subject to change.
	server_handle.with_timeout(TIMEOUT).await.unwrap();

	let (_addr, mut server_handle) = server().with_default_timeout().await.unwrap();
	server_handle.handle.take();
	server_handle.with_timeout(TIMEOUT).await.unwrap();
}

#[tokio::test]
async fn can_set_the_max_request_body_size() {
	let addr = "127.0.0.1:0";
	// Rejects all requests larger than 100 bytes
	let server = HttpServerBuilder::default().max_request_body_size(100).build(addr).await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("anything", |_p, _cx| Ok("a".repeat(100))).unwrap();
	let addr = server.local_addr().unwrap();
	let uri = to_http_uri(addr);
	let handle = server.start(module).unwrap();

	// Invalid: too long
	let req = format!(r#"{{"jsonrpc":"2.0", "method":{}, "id":1}}"#, "a".repeat(100));
	let response = http_request(req.into(), uri.clone()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.body, oversized_request());

	// Max request body size should not override the max response size
	let req = r#"{"jsonrpc":"2.0", "method":"anything", "id":1}"#;
	let response = http_request(req.into(), uri.clone()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.body, ok_response(JsonValue::String("a".repeat(100)), Id::Num(1)));

	handle.stop().unwrap();
}

#[tokio::test]
async fn can_set_the_max_response_size() {
	let addr = "127.0.0.1:0";
	// Set the max response size to 100 bytes
	let server = HttpServerBuilder::default().max_response_body_size(100).build(addr).await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("anything", |_p, _cx| Ok("a".repeat(101))).unwrap();
	let addr = server.local_addr().unwrap();
	let uri = to_http_uri(addr);
	let handle = server.start(module).unwrap();

	// Oversized response.
	let req = r#"{"jsonrpc":"2.0", "method":"anything", "id":1}"#;
	let response = http_request(req.into(), uri.clone()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.body, oversized_response(Id::Num(1), 100));

	handle.stop().unwrap();
}

#[tokio::test]
async fn disabled_batches() {
	let addr = "127.0.0.1:0";
	// Disable batches support.
	let server = HttpServerBuilder::default().batch_requests_supported(false).build(addr).await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("should_ok", |_, _ctx| Ok("ok")).unwrap();
	let addr = server.local_addr().unwrap();
	let uri = to_http_uri(addr);
	let handle = server.start(module).unwrap();

	// Send a valid batch.
	let req = r#"[
		{"jsonrpc":"2.0","method":"should_ok", "params":[],"id":1},
		{"jsonrpc":"2.0","method":"should_ok", "params":[],"id":2}
	]"#;
	let response = http_request(req.into(), uri.clone()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response.body, batches_not_supported());

	handle.stop().unwrap();
}
