#![cfg(test)]

use std::net::SocketAddr;

use crate::{HttpServerBuilder, RpcContextModule};
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::{Id, StatusCode, TestContext};
use jsonrpsee_types::error::ServerCallError;
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

/// Run server with user provided context.
pub async fn server_with_context() -> SocketAddr {
	let mut server = HttpServerBuilder::default().build("127.0.0.1:0".parse().unwrap()).unwrap();

	let ctx = TestContext;
	let mut rpc_ctx = RpcContextModule::new(ctx);

	rpc_ctx
		.register_method("should_err", |_p, ctx| {
			let _ = ctx.err().map_err(|e| ServerCallError::ContextFailed(e.into()))?;
			Ok("err")
		})
		.unwrap();

	rpc_ctx
		.register_method("should_ok", |_p, ctx| {
			let _ = ctx.ok().map_err(|e| ServerCallError::ContextFailed(e.into()))?;
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

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = http_request(req.into(), uri).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_faulty_params_returns_err() {
	let addr = server().await;
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":["Invalid"],"id":1}"#;
	let response = http_request(req.into(), uri).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, invalid_params(Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_faulty_context() {
	let addr = server_with_context().await;
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"should_err", "params":[],"id":1}"#;
	let response = http_request(req.into(), uri).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, invalid_context("RPC context failed", Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_ok_context() {
	let addr = server_with_context().await;
	let uri = to_http_uri(addr);

	let req = r#"{"jsonrpc":"2.0","method":"should_ok", "params":[],"id":1}"#;
	let response = http_request(req.into(), uri).await.unwrap();
	assert_eq!(response.status, StatusCode::OK);
	assert_eq!(response.body, ok_response("ok".into(), Id::Num(1)));
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
