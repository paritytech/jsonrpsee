#![cfg(test)]

use crate::WsServer;
use futures_channel::oneshot::{self, Sender};
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::{Id, WebSocketTestClient};
use jsonrpsee_types::error::Error;
use serde_json::Value as JsonValue;
use std::net::SocketAddr;

/// Spawns a dummy `JSONRPC v2 WebSocket`
/// It has two hardcoded methods: "say_hello" and "add"
pub async fn server(server_started: Sender<SocketAddr>) {
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
	server_started.send(server.local_addr().unwrap()).unwrap();

	server.start().await;
}

#[tokio::test]
async fn single_method_call_works() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();
	let mut client = WebSocketTestClient::new(server_addr).await.unwrap();

	for i in 0..10 {
		let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i);
		let response = client.send_request_text(req).await.unwrap();

		assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(i)));
	}
}

#[tokio::test]
async fn single_method_call_with_params_works() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();
	let mut client = WebSocketTestClient::new(server_addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_faulty_params_returns_err() {
	let _ = env_logger::try_init();
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();
	let mut client = WebSocketTestClient::new(server_addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":["Invalid"],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, invalid_params(Id::Num(1)));
}

#[tokio::test]
async fn single_method_send_binary() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();
	let mut client = WebSocketTestClient::new(server_addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_binary(req.as_bytes()).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn should_return_method_not_found() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();
	let mut client = WebSocketTestClient::new(server_addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();

	let mut client = WebSocketTestClient::new(server_addr).await.unwrap();
	let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
	let response = client.send_request_text(req).await.unwrap();
	// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be Null.
	assert_eq!(response, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();

	let mut client = WebSocketTestClient::new(server_addr).await.unwrap();
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
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();

	let mut client = WebSocketTestClient::new(server_addr).await.unwrap();
	let invalid_request = r#"{"jsonrpc":"2.0","method":"bar","params":[1,"id":99}"#;
	let response1 = client.send_request_text(invalid_request).await.unwrap();
	assert_eq!(response1, parse_error(Id::Null));
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":33}"#;
	let response2 = client.send_request_text(request).await.unwrap();
	assert_eq!(response2, ok_response(JsonValue::String("hello".to_owned()), Id::Num(33)));
}

#[tokio::test]
async fn invalid_request_should_not_close_connection() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	tokio::spawn(server(server_started_tx));
	let server_addr = server_started_rx.await.unwrap();

	let mut client = WebSocketTestClient::new(server_addr).await.unwrap();
	let req = r#"{"jsonrpc":"2.0","method":"bar","id":1,"is_not_request_object":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, invalid_request(Id::Num(1)));
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":33}"#;
	let response = client.send_request_text(request).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(33)));
}
