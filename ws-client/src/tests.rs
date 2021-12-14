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
use crate::types::error::Error;
use crate::types::error_response::{ErrorCode, ErrorObject, ErrorResponse};
use crate::types::{ParamsSer, Subscription};
use crate::WsClientBuilder;
use jsonrpsee_core::rpc_params;
use jsonrpsee_core::traits::{Client, SubscriptionClient};
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::mocks::{Id, WebSocketTestServer};
use jsonrpsee_test_utils::TimeoutFutureExt;
use serde_json::Value as JsonValue;

#[tokio::test]
async fn method_call_works() {
	let result = run_request_with_response(ok_response("hello".into(), Id::Num(0)))
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
	assert_eq!(JsonValue::String("hello".into()), result);
}

#[tokio::test]
async fn notif_works() {
	// this empty string shouldn't be read because the server shouldn't respond to notifications.
	let server = WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), String::new())
		.with_default_timeout()
		.await
		.unwrap();
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClientBuilder::default().build(&uri).with_default_timeout().await.unwrap().unwrap();
	assert!(client.notification("notif", None).with_default_timeout().await.unwrap().is_ok());
}

#[tokio::test]
async fn response_with_wrong_id() {
	let err = run_request_with_response(ok_response("hello".into(), Id::Num(99)))
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap_err();
	assert!(matches!(err, Error::RestartNeeded(_)));
}

#[tokio::test]
async fn response_method_not_found() {
	let err =
		run_request_with_response(method_not_found(Id::Num(0))).with_default_timeout().await.unwrap().unwrap_err();
	assert_error_response(err, ErrorCode::MethodNotFound.into());
}

#[tokio::test]
async fn parse_error_works() {
	let err = run_request_with_response(parse_error(Id::Num(0))).with_default_timeout().await.unwrap().unwrap_err();
	assert_error_response(err, ErrorCode::ParseError.into());
}

#[tokio::test]
async fn invalid_request_works() {
	let err =
		run_request_with_response(invalid_request(Id::Num(0_u64))).with_default_timeout().await.unwrap().unwrap_err();
	assert_error_response(err, ErrorCode::InvalidRequest.into());
}

#[tokio::test]
async fn invalid_params_works() {
	let err =
		run_request_with_response(invalid_params(Id::Num(0_u64))).with_default_timeout().await.unwrap().unwrap_err();
	assert_error_response(err, ErrorCode::InvalidParams.into());
}

#[tokio::test]
async fn internal_error_works() {
	let err =
		run_request_with_response(internal_error(Id::Num(0_u64))).with_default_timeout().await.unwrap().unwrap_err();
	assert_error_response(err, ErrorCode::InternalError.into());
}

#[tokio::test]
async fn subscription_works() {
	let server = WebSocketTestServer::with_hardcoded_subscription(
		"127.0.0.1:0".parse().unwrap(),
		server_subscription_id_response(Id::Num(0)),
		server_subscription_response(JsonValue::String("hello my friend".to_owned())),
	)
	.with_default_timeout()
	.await
	.unwrap();
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClientBuilder::default().build(&uri).with_default_timeout().await.unwrap().unwrap();
	{
		let mut sub: Subscription<String> = client
			.subscribe("subscribe_hello", None, "unsubscribe_hello")
			.with_default_timeout()
			.await
			.unwrap()
			.unwrap();
		let response: String = sub.next().with_default_timeout().await.unwrap().unwrap().unwrap();
		assert_eq!("hello my friend".to_owned(), response);
	}
}

#[tokio::test]
async fn notification_handler_works() {
	let server = WebSocketTestServer::with_hardcoded_notification(
		"127.0.0.1:0".parse().unwrap(),
		server_notification("test", "server originated notification works".into()),
	)
	.with_default_timeout()
	.await
	.unwrap();

	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClientBuilder::default().build(&uri).with_default_timeout().await.unwrap().unwrap();
	{
		let mut nh: Subscription<String> =
			client.subscribe_to_method("test").with_default_timeout().await.unwrap().unwrap();
		let response: String = nh.next().with_default_timeout().await.unwrap().unwrap().unwrap();
		assert_eq!("server originated notification works".to_owned(), response);
	}
}

#[tokio::test]
async fn notification_without_polling_doesnt_make_client_unuseable() {
	let server = WebSocketTestServer::with_hardcoded_notification(
		"127.0.0.1:0".parse().unwrap(),
		server_notification("test", "server originated notification".into()),
	)
	.with_default_timeout()
	.await
	.unwrap();

	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClientBuilder::default()
		.max_notifs_per_subscription(4)
		.build(&uri)
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
	let mut nh: Subscription<String> =
		client.subscribe_to_method("test").with_default_timeout().await.unwrap().unwrap();

	// don't poll the notification stream for 2 seconds, should be full now.
	std::thread::sleep(std::time::Duration::from_secs(2));

	// Capacity is `num_sender` + `capacity`
	for _ in 0..5 {
		assert!(nh.next().with_default_timeout().await.unwrap().unwrap().is_ok());
	}

	// NOTE: this is now unuseable and unregistered.
	assert!(nh.next().with_default_timeout().await.unwrap().is_none());

	// The same subscription should be possible to register again.
	let mut other_nh: Subscription<String> =
		client.subscribe_to_method("test").with_default_timeout().await.unwrap().unwrap();

	// check that the new subscription works.
	assert!(other_nh.next().with_default_timeout().await.unwrap().unwrap().is_ok());
	assert!(client.is_connected());
}

#[tokio::test]
async fn batch_request_works() {
	let batch_request = vec![("say_hello", None), ("say_goodbye", rpc_params![0_u64, 1, 2]), ("get_swag", None)];
	let server_response = r#"[{"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","result":"goodbye","id":1}, {"jsonrpc":"2.0","result":"here's your swag","id":2}]"#.to_string();
	let response =
		run_batch_request_with_response(batch_request, server_response).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, vec!["hello".to_string(), "goodbye".to_string(), "here's your swag".to_string()]);
}

#[tokio::test]
async fn batch_request_out_of_order_response() {
	let batch_request = vec![("say_hello", None), ("say_goodbye", rpc_params![0_u64, 1, 2]), ("get_swag", None)];
	let server_response = r#"[{"jsonrpc":"2.0","result":"here's your swag","id":2}, {"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","result":"goodbye","id":1}]"#.to_string();
	let response =
		run_batch_request_with_response(batch_request, server_response).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, vec!["hello".to_string(), "goodbye".to_string(), "here's your swag".to_string()]);
}

#[tokio::test]
async fn is_connected_works() {
	let server = WebSocketTestServer::with_hardcoded_response(
		"127.0.0.1:0".parse().unwrap(),
		ok_response(JsonValue::String("foo".into()), Id::Num(99_u64)),
	)
	.with_default_timeout()
	.await
	.unwrap();
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClientBuilder::default().build(&uri).with_default_timeout().await.unwrap().unwrap();
	assert!(client.is_connected());
	client.request::<String>("say_hello", None).with_default_timeout().await.unwrap().unwrap_err();
	// give the background thread some time to terminate.
	std::thread::sleep(std::time::Duration::from_millis(100));
	assert!(!client.is_connected())
}

async fn run_batch_request_with_response<'a>(
	batch: Vec<(&'a str, Option<ParamsSer<'a>>)>,
	response: String,
) -> Result<Vec<String>, Error> {
	let server = WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), response)
		.with_default_timeout()
		.await
		.unwrap();
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClientBuilder::default().build(&uri).with_default_timeout().await.unwrap().unwrap();
	client.batch_request(batch).with_default_timeout().await.unwrap()
}

async fn run_request_with_response(response: String) -> Result<JsonValue, Error> {
	let server = WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), response)
		.with_default_timeout()
		.await
		.unwrap();
	let uri = format!("ws://{}", server.local_addr());
	let client = WsClientBuilder::default().build(&uri).with_default_timeout().await.unwrap().unwrap();
	client.request("say_hello", None).with_default_timeout().await.unwrap()
}

fn assert_error_response(err: Error, exp: ErrorObject) {
	match &err {
		Error::Request(e) => {
			let this: ErrorResponse = serde_json::from_str(e).unwrap();
			assert_eq!(this.error, exp);
		}
		e => panic!("Expected error: \"{}\", got: {:?}", err, e),
	};
}

#[tokio::test]
async fn redirections() {
	let _ = env_logger::try_init();
	let expected = "abc 123";
	let server = WebSocketTestServer::with_hardcoded_response(
		"127.0.0.1:0".parse().unwrap(),
		ok_response(expected.into(), Id::Num(0)),
	)
	.with_default_timeout()
	.await
	.unwrap();

	let server_url = format!("ws://{}", server.local_addr());
	let redirect_url = jsonrpsee_test_utils::mocks::ws_server_with_redirect(server_url);

	// The client will first connect to a server that only performs re-directions and finally
	// redirect to another server to complete the handshake.
	let client = WsClientBuilder::default().build(&redirect_url).with_default_timeout().await;
	// It's an ok client
	let client = match client {
		Ok(Ok(client)) => client,
		Ok(Err(e)) => panic!("WsClient builder failed with: {:?}", e),
		Err(e) => panic!("WsClient builder timed out with: {:?}", e),
	};
	// It's connected
	assert!(client.is_connected());
	// It works
	let response = client.request::<String>("anything", None).with_default_timeout().await.unwrap();
	assert_eq!(response.unwrap(), String::from(expected));
}
