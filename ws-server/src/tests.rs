#![cfg(test)]

use crate::{RpcContextModule, WsServer};
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::{Id, TestContext, WebSocketTestClient};
use jsonrpsee_types::error::{CallError, Error};
use serde_json::Value as JsonValue;
use std::fmt;
use std::net::SocketAddr;

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
/// It has two hardcoded methods: "say_hello" and "add"
pub async fn server() -> SocketAddr {
	let mut server = WsServer::new("127.0.0.1:0").await.unwrap();

	server
		.register_method("say_hello", |_| {
			log::debug!("server respond to hello");
			Ok("hello")
		})
		.unwrap();
	server
		.register_method("add", |params| {
			let params: Vec<u64> = params.parse()?;
			let sum: u64 = params.into_iter().sum();
			Ok(sum)
		})
		.unwrap();
	server.register_method("invalid_params", |_params| Err::<(), _>(CallError::InvalidParams)).unwrap();
	server.register_method("call_fail", |_params| Err::<(), _>(CallError::Failed(Box::new(MyAppError)))).unwrap();
	server
		.register_method("sleep_for", |params| {
			let sleep: Vec<u64> = params.parse()?;
			std::thread::sleep(std::time::Duration::from_millis(sleep[0]));
			Ok("Yawn!")
		})
		.unwrap();

	let addr = server.local_addr().unwrap();

	tokio::spawn(async { server.start().await });
	addr
}

/// Run server with user provided context.
pub async fn server_with_context() -> SocketAddr {
	let mut server = WsServer::new("127.0.0.1:0").await.unwrap();

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

	let rpc_module = rpc_ctx.into_module();
	server.register_module(rpc_module).unwrap();
	let addr = server.local_addr().unwrap();

	tokio::spawn(async { server.start().await });
	addr
}

#[tokio::test]
async fn single_method_calls_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	for i in 0..10 {
		let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i);
		let response = client.send_request_text(req).await.unwrap();

		assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(i)));
	}
}

#[tokio::test]
async fn slow_method_calls_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"sleep_for","params":[1000],"id":123}"#;
	let response = client.send_request_text(req).await.unwrap();

	assert_eq!(response, ok_response(JsonValue::String("Yawn!".to_owned()), Id::Num(123)));
}

#[tokio::test]
async fn batch_method_call_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let mut batch = Vec::new();
	batch.push(r#"{"jsonrpc":"2.0","method":"sleep_for","params":[1000],"id":123}"#.to_string());
	for i in 1..4 {
		batch.push(format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i));
	}
	let batch = format!("[{}]", batch.join(","));
	let response = client.send_request_text(batch).await.unwrap();
	assert_eq!(
		response,
		r#"[{"jsonrpc":"2.0","result":"Yawn!","id":123},{"jsonrpc":"2.0","result":"hello","id":1},{"jsonrpc":"2.0","result":"hello","id":2},{"jsonrpc":"2.0","result":"hello","id":3}]"#
	);
}

#[tokio::test]
async fn batch_method_call_where_some_calls_fail() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let mut batch = Vec::new();
	batch.push(r#"{"jsonrpc":"2.0","method":"say_hello","id":1}"#);
	batch.push(r#"{"jsonrpc":"2.0","method":"call_fail","id":2}"#);
	batch.push(r#"{"jsonrpc":"2.0","method":"add","params":[34, 45],"id":3}"#);
	let batch = format!("[{}]", batch.join(","));

	let response = client.send_request_text(batch).await.unwrap();

	assert_eq!(
		response,
		r#"[{"jsonrpc":"2.0","result":"hello","id":1},{"jsonrpc":"2.0","error":{"code":-32000,"message":"MyAppError"},"id":2},{"jsonrpc":"2.0","result":79,"id":3}]"#
	);
}

#[tokio::test]
async fn single_method_call_with_params_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_faulty_params_returns_err() {
	let _ = env_logger::try_init();
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":["Invalid"],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, invalid_params(Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_faulty_context() {
	let addr = server_with_context().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"should_err", "params":[],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, invalid_context("RPC context failed", Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_ok_context() {
	let addr = server_with_context().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"should_ok", "params":[],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, ok_response("ok".into(), Id::Num(1)));
}

#[tokio::test]
async fn single_method_send_binary() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_binary(req.as_bytes()).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn should_return_method_not_found() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
	let response = client.send_request_text(req).await.unwrap();
	// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be Null.
	assert_eq!(response, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, invalid_request(Id::Num(1)));
}

#[tokio::test]
async fn register_methods_works() {
	let mut server = WsServer::new("127.0.0.1:0").await.unwrap();
	assert!(server.register_method("say_hello", |_| Ok("lo")).is_ok());
	assert!(server.register_method("say_hello", |_| Ok("lo")).is_err());
	assert!(server.register_subscription("subscribe_hello", "unsubscribe_hello").is_ok());
	assert!(server.register_subscription("subscribe_hello_again", "unsubscribe_hello").is_err());
	assert!(
		server.register_method("subscribe_hello_again", |_| Ok("lo")).is_ok(),
		"Failed register_subscription should not have side-effects"
	);
}

#[tokio::test]
async fn register_same_subscribe_unsubscribe_is_err() {
	let mut server = WsServer::new("127.0.0.1:0").await.unwrap();
	assert!(matches!(
		server.register_subscription("subscribe_hello", "subscribe_hello"),
		Err(Error::SubscriptionNameConflict(_))
	));
}

#[tokio::test]
async fn parse_error_request_should_not_close_connection() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let invalid_request = r#"{"jsonrpc":"2.0","method":"bar","params":[1,"id":99}"#;
	let response1 = client.send_request_text(invalid_request).await.unwrap();
	assert_eq!(response1, parse_error(Id::Null));
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":33}"#;
	let response2 = client.send_request_text(request).await.unwrap();
	assert_eq!(response2, ok_response(JsonValue::String("hello".to_owned()), Id::Num(33)));
}

#[tokio::test]
async fn invalid_request_should_not_close_connection() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, invalid_request(Id::Num(1)));
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":33}"#;
	let response = client.send_request_text(request).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(33)));
}

#[tokio::test]
async fn valid_request_that_fails_to_execute_should_not_close_connection() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	// Good request, executes fine
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":33}"#;
	let response = client.send_request_text(request).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(33)));

	// Good request, but causes error.
	let req = r#"{"jsonrpc":"2.0","method":"call_fail","params":[],"id":123}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, r#"{"jsonrpc":"2.0","error":{"code":-32000,"message":"MyAppError"},"id":123}"#);

	// Connection is still good.
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":333}"#;
	let response = client.send_request_text(request).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(333)));
}
