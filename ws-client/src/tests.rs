#![cfg(test)]

use crate::{WsClient, WsConfig, WsSubscription};
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::{Id, WebSocketTestServer};
use jsonrpsee_types::{
	error::Error,
	jsonrpc,
	traits::{Client, SubscriptionClient},
};

fn assert_error_response(response: Result<jsonrpc::JsonValue, Error>, code: jsonrpc::ErrorCode, message: String) {
	let expected = jsonrpc::Error { code, message, data: None };
	match response {
		Err(Error::Request(err)) => {
			assert_eq!(err, expected);
		}
		e @ _ => panic!("Expected error: \"{}\", got: {:?}", expected, e),
	};
}

#[tokio::test]
async fn method_call_works() {
	let server = WebSocketTestServer::with_hardcoded_response(
		"127.0.0.1:0".parse().unwrap(),
		ok_response("hello".into(), Id::Num(0_u64)),
	)
	.await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClient::new(WsConfig::with_url(&uri)).await.unwrap();
	let response: jsonrpc::JsonValue = client.request("say_hello", jsonrpc::Params::None).await.unwrap();
	let exp = jsonrpc::JsonValue::String("hello".to_string());
	assert_eq!(response, exp);
}

#[tokio::test]
async fn notif_works() {
	// this empty string shouldn't be read because the server shouldn't respond to notifications.
	let server = WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), String::new()).await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClient::new(WsConfig::with_url(&uri)).await.unwrap();
	assert!(client.notification("notif", jsonrpc::Params::None).await.is_ok());
}

#[tokio::test]
async fn method_not_found_works() {
	let server =
		WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), method_not_found(Id::Num(0_u64)))
			.await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClient::new(WsConfig::with_url(&uri)).await.unwrap();
	let response: Result<jsonrpc::JsonValue, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert_error_response(response, jsonrpc::ErrorCode::MethodNotFound, METHOD_NOT_FOUND.into());
}

#[tokio::test]
async fn parse_error_works() {
	let server =
		WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), parse_error(Id::Num(0_u64))).await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClient::new(WsConfig::with_url(&uri)).await.unwrap();
	let response: Result<jsonrpc::JsonValue, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert_error_response(response, jsonrpc::ErrorCode::ParseError, PARSE_ERROR.into());
}

#[tokio::test]
async fn invalid_request_works() {
	let server =
		WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), invalid_request(Id::Num(0_u64)))
			.await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClient::new(WsConfig::with_url(&uri)).await.unwrap();
	let response: Result<jsonrpc::JsonValue, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert_error_response(response, jsonrpc::ErrorCode::InvalidRequest, INVALID_REQUEST.into());
}

#[tokio::test]
async fn invalid_params_works() {
	let server =
		WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), invalid_params(Id::Num(0_u64)))
			.await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClient::new(WsConfig::with_url(&uri)).await.unwrap();
	let response: Result<jsonrpc::JsonValue, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert_error_response(response, jsonrpc::ErrorCode::InvalidParams, INVALID_PARAMS.into());
}

#[tokio::test]
async fn internal_error_works() {
	let server =
		WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), internal_error(Id::Num(0_u64)))
			.await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClient::new(WsConfig::with_url(&uri)).await.unwrap();
	let response: Result<jsonrpc::JsonValue, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert_error_response(response, jsonrpc::ErrorCode::InternalError, INTERNAL_ERROR.into());
}

#[tokio::test]
async fn subscription_works() {
	let server = WebSocketTestServer::with_hardcoded_subscription(
		"127.0.0.1:0".parse().unwrap(),
		server_subscription_id_response(Id::Num(0)),
		server_subscription_response(jsonrpc::JsonValue::String("hello my friend".to_owned())),
	)
	.await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClient::new(WsConfig::with_url(&uri)).await.unwrap();
	{
		let mut sub: WsSubscription<String> =
			client.subscribe("subscribe_hello", jsonrpc::Params::None, "unsubscribe_hello").await.unwrap();
		let response: String = sub.next().await.unwrap().into();
		assert_eq!("hello my friend".to_owned(), response);
	}
}

#[tokio::test]
async fn response_with_wrong_id() {
	let server = WebSocketTestServer::with_hardcoded_response(
		"127.0.0.1:0".parse().unwrap(),
		ok_response(jsonrpc::JsonValue::String("foo".into()), Id::Num(99_u64)),
	)
	.await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClient::new(WsConfig::with_url(&uri)).await.unwrap();
	let err: Result<String, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert!(matches!(err, Err(Error::RestartNeeded(e)) if e.to_string().contains("Invalid request ID")));
}

#[tokio::test]
async fn is_connected_works() {
	let server = WebSocketTestServer::with_hardcoded_response(
		"127.0.0.1:0".parse().unwrap(),
		ok_response(jsonrpc::JsonValue::String("foo".into()), Id::Num(99_u64)),
	)
	.await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClient::new(WsConfig::with_url(&uri)).await.unwrap();
	assert!(client.is_connected());
	client.request::<String, _, _>("say_hello", jsonrpc::Params::None).await.unwrap_err();
	assert!(!client.is_connected())
}
