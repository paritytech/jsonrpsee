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
// IN background_task WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

#![cfg(test)]
use std::fmt;
use std::net::SocketAddr;
use std::time::Duration;

use crate::types::error::CallError;
use crate::types::{Response, SubscriptionId};
use crate::{future::ServerHandle, RpcModule, WsServerBuilder};
use anyhow::anyhow;
use futures_util::future::join;
use jsonrpsee_core::{traits::IdProvider, DeserializeOwned, Error};
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::mocks::{Id, TestContext, WebSocketTestClient, WebSocketTestError};
use jsonrpsee_test_utils::TimeoutFutureExt;
use serde_json::Value as JsonValue;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn init_logger() {
	let _ = FmtSubscriber::builder().with_env_filter(EnvFilter::from_default_env()).try_init();
}

fn deser_call<T: DeserializeOwned>(raw: String) -> T {
	let out: Response<T> = serde_json::from_str(&raw).unwrap();
	out.result
}

/// Applications can/should provide their own error.
#[derive(Debug)]
struct MyAppError;
impl fmt::Display for MyAppError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "MyAppError")
	}
}
impl std::error::Error for MyAppError {}

/// Spawns a dummy `JSONRPC v2 WebSocket`
async fn server() -> SocketAddr {
	server_with_handles().await.0
}

/// Spawns a dummy `JSONRPC v2 WebSocket`
/// It has the following methods:
///     sync methods: `say_hello` and `add`
///     async: `say_hello_async` and `add_sync`
///     other: `invalid_params` (always returns `CallError::InvalidParams`),
///            `call_fail` (always returns `CallError::Failed`),
///            `sleep_for`
///            `subscribe_hello` (starts a subscription that doesn't send anything)
///
/// Returns the address together with handle for the server.
async fn server_with_handles() -> (SocketAddr, ServerHandle) {
	let server = WsServerBuilder::default().build("127.0.0.1:0").with_default_timeout().await.unwrap().unwrap();
	let mut module = RpcModule::new(());
	module
		.register_method("say_hello", |_, _| {
			tracing::debug!("server respond to hello");
			Ok("hello")
		})
		.unwrap();
	module
		.register_method("add", |params, _| {
			let params: Vec<u64> = params.parse()?;
			let sum: u64 = params.into_iter().sum();
			Ok(sum)
		})
		.unwrap();
	module
		.register_async_method("say_hello_async", |_, _| {
			async move {
				tracing::debug!("server respond to hello");
				// Call some async function inside.
				futures_util::future::ready(()).await;
				Ok("hello")
			}
		})
		.unwrap();
	module
		.register_async_method("add_async", |params, _| async move {
			let params: Vec<u64> = params.parse()?;
			let sum: u64 = params.into_iter().sum();
			Ok(sum)
		})
		.unwrap();
	module
		.register_method("invalid_params", |_params, _| Err::<(), _>(CallError::InvalidParams(anyhow!("buh!")).into()))
		.unwrap();
	module.register_method("call_fail", |_params, _| Err::<(), _>(Error::to_call_error(MyAppError))).unwrap();
	module
		.register_method("sleep_for", |params, _| {
			let sleep: Vec<u64> = params.parse()?;
			std::thread::sleep(std::time::Duration::from_millis(sleep[0]));
			Ok("Yawn!")
		})
		.unwrap();
	module
		.register_subscription("subscribe_hello", "subscribe_hello", "unsubscribe_hello", |_, pending, _| {
			let sink = pending.accept()?;
			std::thread::spawn(move || loop {
				let _ = &sink;
				std::thread::sleep(std::time::Duration::from_secs(30));
			});
			Ok(())
		})
		.unwrap();

	let addr = server.local_addr().unwrap();

	let server_handle = server.start(module).unwrap();
	(addr, server_handle)
}

/// Run server with user provided context.
async fn server_with_context() -> SocketAddr {
	let server = WsServerBuilder::default().build("127.0.0.1:0").with_default_timeout().await.unwrap().unwrap();

	let ctx = TestContext;
	let mut rpc_module = RpcModule::new(ctx);

	rpc_module
		.register_method("should_err", |_p, ctx| {
			let _ = ctx.err().map_err(CallError::Failed)?;
			Ok("err")
		})
		.unwrap();

	rpc_module
		.register_method("should_ok", |_p, ctx| {
			let _ = ctx.ok().map_err(CallError::Failed)?;
			Ok("ok")
		})
		.unwrap();

	rpc_module
		.register_async_method("should_ok_async", |_p, ctx| async move {
			let _ = ctx.ok().map_err(CallError::Failed)?;
			// Call some async function inside.
			Ok(futures_util::future::ready("ok!").await)
		})
		.unwrap();

	rpc_module
		.register_async_method("err_async", |_p, ctx| async move {
			let _ = ctx.ok().map_err(CallError::Failed)?;
			// Async work that returns an error
			futures_util::future::err::<(), _>(anyhow!("nah").into()).await
		})
		.unwrap();

	let addr = server.local_addr().unwrap();

	server.start(rpc_module).unwrap();
	addr
}

#[tokio::test]
async fn can_set_the_max_request_body_size() {
	init_logger();

	let addr = "127.0.0.1:0";
	// Rejects all requests larger than 100 bytes
	let server = WsServerBuilder::default().max_request_body_size(100).build(addr).await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("anything", |_p, _cx| Ok("a".repeat(100))).unwrap();
	let addr = server.local_addr().unwrap();
	let handle = server.start(module).unwrap();

	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	// Invalid: too long
	let req = format!(r#"{{"jsonrpc":"2.0", "method":{}, "id":1}}"#, "a".repeat(100));
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, oversized_request());

	// Max request body size should not override the max response body size
	let req = r#"{"jsonrpc":"2.0", "method":"anything", "id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::String("a".repeat(100)), Id::Num(1)));

	handle.stop().unwrap();
}

#[tokio::test]
async fn can_set_the_max_response_body_size() {
	init_logger();

	let addr = "127.0.0.1:0";
	// Set the max response body size to 100 bytes
	let server = WsServerBuilder::default().max_response_body_size(100).build(addr).await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("anything", |_p, _cx| Ok("a".repeat(101))).unwrap();
	let addr = server.local_addr().unwrap();
	let handle = server.start(module).unwrap();

	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	// Oversized response.
	let req = r#"{"jsonrpc":"2.0", "method":"anything", "id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, oversized_response(Id::Num(1), 100));

	handle.stop().unwrap();
}

#[tokio::test]
async fn can_set_max_connections() {
	let addr = "127.0.0.1:0";
	// Server that accepts max 2 connections
	let server = WsServerBuilder::default().max_connections(2).build(addr).await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("anything", |_p, _cx| Ok(())).unwrap();
	let addr = server.local_addr().unwrap();

	let handle = server.start(module).unwrap();

	let conn1 = WebSocketTestClient::new(addr).await;
	let conn2 = WebSocketTestClient::new(addr).await;
	let conn3 = WebSocketTestClient::new(addr).await;
	assert!(conn1.is_ok());
	assert!(conn2.is_ok());
	// Third connection is rejected
	assert!(conn3.is_err());
	if !matches!(conn3, Err(WebSocketTestError::RejectedWithStatusCode(429))) {
		panic!("Expected RejectedWithStatusCode(429), got: {:#?}", conn3);
	}

	// Decrement connection count
	drop(conn2);
	// Can connect again
	let conn4 = WebSocketTestClient::new(addr).await;
	assert!(conn4.is_ok());

	handle.stop().unwrap();
}

#[tokio::test]
async fn single_method_calls_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	for i in 0..10 {
		let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i);
		let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();

		assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(i)));
	}
}

#[tokio::test]
async fn async_method_calls_works() {
	init_logger();
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	for i in 0..10 {
		let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello_async","id":{}}}"#, i);
		let response = client.send_request_text(req).await.unwrap();

		assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(i)));
	}
}

#[tokio::test]
async fn slow_method_calls_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"sleep_for","params":[1000],"id":123}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();

	assert_eq!(response, ok_response(JsonValue::String("Yawn!".to_owned()), Id::Num(123)));
}

#[tokio::test]
async fn batch_method_call_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let mut batch = vec![r#"{"jsonrpc":"2.0","method":"sleep_for","params":[1000],"id":123}"#.to_string()];
	for i in 1..4 {
		batch.push(format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i));
	}
	let batch = format!("[{}]", batch.join(","));
	let response = client.send_request_text(batch).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(
		response,
		r#"[{"jsonrpc":"2.0","result":"Yawn!","id":123},{"jsonrpc":"2.0","result":"hello","id":1},{"jsonrpc":"2.0","result":"hello","id":2},{"jsonrpc":"2.0","result":"hello","id":3}]"#
	);
}

#[tokio::test]
async fn batch_method_call_where_some_calls_fail() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let batch = vec![
		r#"{"jsonrpc":"2.0","method":"say_hello","id":1}"#,
		r#"{"jsonrpc":"2.0","method":"call_fail","id":2}"#,
		r#"{"jsonrpc":"2.0","method":"add","params":[34, 45],"id":3}"#,
	];
	let batch = format!("[{}]", batch.join(","));

	let response = client.send_request_text(batch).with_default_timeout().await.unwrap().unwrap();

	assert_eq!(
		response,
		r#"[{"jsonrpc":"2.0","result":"hello","id":1},{"jsonrpc":"2.0","error":{"code":-32000,"message":"MyAppError"},"id":2},{"jsonrpc":"2.0","result":79,"id":3}]"#
	);
}

#[tokio::test]
async fn garbage_request_fails() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"dsdfs fsdsfds"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"{ "#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"         {"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"{}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"{sds}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"["#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"[dsds]"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#" [{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}]"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"[]"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, invalid_request(Id::Null));

	let req = r#"[{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));
}

#[tokio::test]
async fn single_method_call_with_params_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_faulty_params_returns_err() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();
	let expected = r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"invalid type: string \"should be a number\", expected u64 at line 1 column 21"},"id":1}"#;

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":["should be a number"],"id":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, expected);
}

#[tokio::test]
async fn single_method_call_with_faulty_context() {
	let addr = server_with_context().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"should_err", "params":[],"id":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, call_execution_failed("RPC context failed", Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_ok_context() {
	let addr = server_with_context().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"should_ok", "params":[],"id":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response("ok".into(), Id::Num(1)));
}

#[tokio::test]
async fn async_method_call_with_ok_context() {
	let addr = server_with_context().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"should_ok_async", "params":[],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, ok_response("ok!".into(), Id::Num(1)));
}

#[tokio::test]
async fn async_method_call_with_params() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add_async", "params":[1, 2],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn async_method_call_that_fails() {
	let addr = server_with_context().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"err_async", "params":[],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, call_execution_failed("nah", Id::Num(1)));
}

#[tokio::test]
async fn single_method_send_binary() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_binary(req.as_bytes()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn should_return_method_not_found() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be
	// Null.
	assert_eq!(response, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, invalid_request(Id::Num(1)));
}

#[tokio::test]
async fn register_methods_works() {
	let mut module = RpcModule::new(());
	assert!(module.register_method("say_hello", |_, _| Ok("lo")).is_ok());
	assert!(module.register_method("say_hello", |_, _| Ok("lo")).is_err());
	assert!(module
		.register_subscription("subscribe_hello", "subscribe_hello", "unsubscribe_hello", |_, _, _| Ok(()))
		.is_ok());
	assert!(module
		.register_subscription("subscribe_hello_again", "subscribe_hello_again", "unsubscribe_hello", |_, _, _| Ok(()))
		.is_err());
	assert!(
		module.register_method("subscribe_hello_again", |_, _| Ok("lo")).is_ok(),
		"Failed register_subscription should not have side-effects"
	);
}

#[tokio::test]
async fn register_same_subscribe_unsubscribe_is_err() {
	let mut module = RpcModule::new(());
	assert!(matches!(
		module.register_subscription("subscribe_hello", "subscribe_hello", "subscribe_hello", |_, _, _| Ok(())),
		Err(Error::SubscriptionNameConflict(_))
	));
}

#[tokio::test]
async fn parse_error_request_should_not_close_connection() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let invalid_request = r#"{"jsonrpc":"2.0","method":"bar","params":[1,"id":99}"#;
	let response1 = client.send_request_text(invalid_request).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response1, parse_error(Id::Null));
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":33}"#;
	let response2 = client.send_request_text(request).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response2, ok_response(JsonValue::String("hello".to_owned()), Id::Num(33)));
}

#[tokio::test]
async fn invalid_request_should_not_close_connection() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, invalid_request(Id::Num(1)));
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":33}"#;
	let response = client.send_request_text(request).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(33)));
}

#[tokio::test]
async fn valid_request_that_fails_to_execute_should_not_close_connection() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	// Good request, executes fine
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":33}"#;
	let response = client.send_request_text(request).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(33)));

	// Good request, but causes error.
	let req = r#"{"jsonrpc":"2.0","method":"call_fail","params":[],"id":123}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, r#"{"jsonrpc":"2.0","error":{"code":-32000,"message":"MyAppError"},"id":123}"#);

	// Connection is still good.
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":333}"#;
	let response = client.send_request_text(request).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(333)));
}

#[tokio::test]
async fn can_register_modules() {
	let cx = String::new();
	let mut mod1 = RpcModule::new(cx);

	let cx2 = Vec::<u8>::new();
	let mut mod2 = RpcModule::new(cx2);

	assert_eq!(mod1.method_names().count(), 0);
	assert_eq!(mod2.method_names().count(), 0);
	mod1.register_method("bla", |_, cx| Ok(format!("Gave me {}", cx))).unwrap();
	mod1.register_method("bla2", |_, cx| Ok(format!("Gave me {}", cx))).unwrap();
	mod2.register_method("yada", |_, cx| Ok(format!("Gave me {:?}", cx))).unwrap();

	// Won't register, name clashes
	mod2.register_method("bla", |_, cx| Ok(format!("Gave me {:?}", cx))).unwrap();

	assert_eq!(mod1.method_names().count(), 2);
	let err = mod1.merge(mod2).unwrap_err();
	let _expected_err = Error::MethodAlreadyRegistered(String::from("bla"));
	assert!(matches!(err, _expected_err));
	assert_eq!(mod1.method_names().count(), 2);
}

#[tokio::test]
async fn stop_works() {
	init_logger();
	let (_addr, server_handle) = server_with_handles().with_default_timeout().await.unwrap();
	server_handle.clone().stop().unwrap().with_default_timeout().await.unwrap();

	// After that we should be able to wait for task handle to finish.
	// First `unwrap` is timeout, second is `JoinHandle`'s one.

	// After server was stopped, attempt to stop it again should result in an error.
	assert!(matches!(server_handle.stop(), Err(Error::AlreadyStopped)));
}

#[tokio::test]
async fn run_forever() {
	const TIMEOUT: Duration = Duration::from_millis(200);

	init_logger();
	let (_addr, server_handle) = server_with_handles().with_default_timeout().await.unwrap();

	assert!(matches!(server_handle.with_timeout(TIMEOUT).await, Err(_timeout_err)));

	let (_addr, server_handle) = server_with_handles().with_default_timeout().await.unwrap();

	// Send the shutdown request from one handle and await the server on the second one.
	join(server_handle.clone().stop().unwrap(), server_handle).with_timeout(TIMEOUT).await.unwrap();
}

#[tokio::test]
async fn unsubscribe_twice_should_indicate_error() {
	init_logger();
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let sub_call = call("subscribe_hello", Vec::<()>::new(), Id::Num(0));
	let sub_id: u64 = deser_call(client.send_request_text(sub_call).await.unwrap());

	let unsub_call = call("unsubscribe_hello", vec![sub_id], Id::Num(1));
	let unsub_1: bool = deser_call(client.send_request_text(unsub_call).await.unwrap());
	assert!(unsub_1);

	let unsub_call = call("unsubscribe_hello", vec![sub_id], Id::Num(2));
	let unsub_2: bool = deser_call(client.send_request_text(unsub_call).await.unwrap());

	assert!(!unsub_2);
}

#[tokio::test]
async fn unsubscribe_wrong_sub_id_type() {
	init_logger();
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let unsub: bool =
		deser_call(client.send_request_text(call("unsubscribe_hello", vec![13.99_f64], Id::Num(0))).await.unwrap());
	assert!(!unsub);
}

#[tokio::test]
async fn custom_subscription_id_works() {
	#[derive(Debug, Clone)]
	struct HardcodedSubscriptionId;

	impl IdProvider for HardcodedSubscriptionId {
		fn next_id(&self) -> SubscriptionId<'static> {
			"0xdeadbeef".to_string().into()
		}
	}

	init_logger();
	let server = WsServerBuilder::default()
		.set_id_provider(HardcodedSubscriptionId)
		.build("127.0.0.1:0")
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
	let addr = server.local_addr().unwrap();
	let mut module = RpcModule::new(());
	module
		.register_subscription("subscribe_hello", "subscribe_hello", "unsubscribe_hello", |_, pending, _| {
			let sink = pending.accept()?;
			std::thread::spawn(move || loop {
				let _ = &sink;
				std::thread::sleep(std::time::Duration::from_secs(30));
			});
			Ok(())
		})
		.unwrap();
	server.start(module).unwrap();

	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let sub = client.send_request_text(call("subscribe_hello", Vec::<()>::new(), Id::Num(0))).await.unwrap();
	assert_eq!(&sub, r#"{"jsonrpc":"2.0","result":"0xdeadbeef","id":0}"#);
	let unsub = client.send_request_text(call("unsubscribe_hello", vec!["0xdeadbeef"], Id::Num(1))).await.unwrap();
	assert_eq!(&unsub, r#"{"jsonrpc":"2.0","result":true,"id":1}"#);
}
