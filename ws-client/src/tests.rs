#![cfg(test)]

use crate::{WsClientBuilder, WsSubscription};
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::{Id, WebSocketTestServer};
use jsonrpsee_types::{
	error::Error,
	traits::{Client, SubscriptionClient},
	v2::{dummy::JsonRpcParams, error::*},
};
use serde::Serialize;
use serde_json::Value as JsonValue;

#[tokio::test]
async fn method_call_works() {
	let result = run_request_with_response(ok_response("hello".into(), Id::Num(0))).await.unwrap();
	assert_eq!(JsonValue::String("hello".into()), result);
}

#[tokio::test]
async fn notif_works() {
	// this empty string shouldn't be read because the server shouldn't respond to notifications.
	let server = WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), String::new()).await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClientBuilder::default().build(&uri).await.unwrap();
	assert!(client.notification("notif", JsonRpcParams::NoParams::<u64>).await.is_ok());
}

#[tokio::test]
async fn response_with_wrong_id() {
	let err = run_request_with_response(ok_response("hello".into(), Id::Num(99))).await.unwrap_err();
	assert!(matches!(err, Error::InvalidRequestId));
}

#[tokio::test]
async fn response_method_not_found() {
	let err = run_request_with_response(method_not_found(Id::Num(0))).await.unwrap_err();
	assert_error_response(err, METHOD_NOT_FOUND_CODE, METHOD_NOT_FOUND_MSG);
}

#[tokio::test]
async fn parse_error_works() {
	let err = run_request_with_response(parse_error(Id::Num(0))).await.unwrap_err();
	assert_error_response(err, PARSE_ERROR_CODE, PARSE_ERROR_MSG);
}

#[tokio::test]
async fn invalid_request_works() {
	let err = run_request_with_response(invalid_request(Id::Num(0_u64))).await.unwrap_err();
	assert_error_response(err, INVALID_REQUEST_CODE, INVALID_REQUEST_MSG);
}

#[tokio::test]
async fn invalid_params_works() {
	let err = run_request_with_response(invalid_params(Id::Num(0_u64))).await.unwrap_err();
	assert_error_response(err, INVALID_PARAMS_CODE, INVALID_PARAMS_MSG);
}

#[tokio::test]
async fn internal_error_works() {
	let err = run_request_with_response(internal_error(Id::Num(0_u64))).await.unwrap_err();
	assert_error_response(err, INTERNAL_ERROR_CODE, INTERNAL_ERROR_MSG);
}

#[tokio::test]
async fn subscription_works() {
	let server = WebSocketTestServer::with_hardcoded_subscription(
		"127.0.0.1:0".parse().unwrap(),
		server_subscription_id_response(Id::Num(0)),
		server_subscription_response(JsonValue::String("hello my friend".to_owned())),
	)
	.await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClientBuilder::default().build(&uri).await.unwrap();
	{
		let mut sub: WsSubscription<String> =
			client.subscribe("subscribe_hello", JsonRpcParams::NoParams::<u64>, "unsubscribe_hello").await.unwrap();
		let response: String = sub.next().await.unwrap().into();
		assert_eq!("hello my friend".to_owned(), response);
	}
}

#[tokio::test]
async fn batch_request_works() {
	let _ = env_logger::try_init();
	let batch_request = vec![
		("say_hello", JsonRpcParams::NoParams),
		("say_goodbye", JsonRpcParams::Array(vec![&0_u64, &1, &2])),
		("get_swag", JsonRpcParams::NoParams),
	];
	let server_response = r#"[{"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","result":"goodbye","id":1}, {"jsonrpc":"2.0","result":"here's your swag","id":2}]"#.to_string();
	let response = run_batch_request_with_response(batch_request, server_response).await.unwrap();
	assert_eq!(response, vec!["hello".to_string(), "goodbye".to_string(), "here's your swag".to_string()]);
}

#[tokio::test]
async fn batch_request_out_of_order_response() {
	let batch_request = vec![
		("say_hello", JsonRpcParams::NoParams),
		("say_goodbye", JsonRpcParams::Array(vec![&0_u64, &1, &2])),
		("get_swag", JsonRpcParams::NoParams),
	];
	let server_response = r#"[{"jsonrpc":"2.0","result":"here's your swag","id":2}, {"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","result":"goodbye","id":1}]"#.to_string();
	let response = run_batch_request_with_response(batch_request, server_response).await.unwrap();
	assert_eq!(response, vec!["hello".to_string(), "goodbye".to_string(), "here's your swag".to_string()]);
}

#[tokio::test]
async fn is_connected_works() {
	let server = WebSocketTestServer::with_hardcoded_response(
		"127.0.0.1:0".parse().unwrap(),
		ok_response(JsonValue::String("foo".into()), Id::Num(99_u64)),
	)
	.await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClientBuilder::default().build(&uri).await.unwrap();
	assert!(client.is_connected());
	client.request::<u64, String>("say_hello", JsonRpcParams::NoParams).await.unwrap_err();
	assert!(!client.is_connected())
}

async fn run_batch_request_with_response<'a, T: Serialize + std::fmt::Debug + Send + Sync>(
	batch: Vec<(&'a str, JsonRpcParams<'a, T>)>,
	response: String,
) -> Result<Vec<String>, Error> {
	let server = WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), response).await;
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClientBuilder::default().build(&uri).await.unwrap();
	client.batch_request(batch).await
}

async fn run_request_with_response(response: String) -> Result<JsonValue, Error> {
	let server = WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), response).await;
	let uri = format!("ws://{}", server.local_addr());
	let client = WsClientBuilder::default().build(&uri).await.unwrap();
	client.request::<u64, JsonValue>("say_hello", JsonRpcParams::NoParams).await
}

fn assert_error_response(response: Error, code: i32, message: &str) {
	todo!();
	/*match response {
		Err(Error::Request(err)) => {
			assert_eq!(err, expected);
		}
		e @ _ => panic!("Expected error: \"{}\", got: {:?}", expected, e),
	};*/
}
