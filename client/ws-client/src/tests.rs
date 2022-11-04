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
use crate::types::error::{ErrorCode, ErrorObject};

use crate::WsClientBuilder;
use jsonrpsee_core::client::{BatchResponse, ClientT, SubscriptionClientT};
use jsonrpsee_core::client::{IdKind, Subscription};
use jsonrpsee_core::params::BatchRequestBuilder;
use jsonrpsee_core::{rpc_params, DeserializeOwned, Error};
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::mocks::{Id, WebSocketTestServer};
use jsonrpsee_test_utils::TimeoutFutureExt;
use jsonrpsee_types::error::{CallError, ErrorObjectOwned};
use serde_json::Value as JsonValue;

fn init_logger() {
	let _ = tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init();
}

#[tokio::test]
async fn method_call_works() {
	let result = run_request_with_response(ok_response("hello".into(), Id::Num(0)))
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
	assert_eq!("hello", &result);
}

#[tokio::test]
async fn method_call_with_wrong_id_kind() {
	let exp = "id as string";
	let server = WebSocketTestServer::with_hardcoded_response(
		"127.0.0.1:0".parse().unwrap(),
		ok_response(exp.into(), Id::Num(0)),
	)
	.with_default_timeout()
	.await
	.unwrap();
	let uri = format!("ws://{}", server.local_addr());
	let client =
		WsClientBuilder::default().id_format(IdKind::String).build(&uri).with_default_timeout().await.unwrap().unwrap();

	let err: Result<String, Error> = client.request("o", rpc_params![]).with_default_timeout().await.unwrap();
	assert!(matches!(err, Err(Error::RestartNeeded(e)) if e == "Invalid request ID"));
}

#[tokio::test]
async fn method_call_with_id_str() {
	let exp = "id as string";
	let server = WebSocketTestServer::with_hardcoded_response(
		"127.0.0.1:0".parse().unwrap(),
		ok_response(exp.into(), Id::Str("0".into())),
	)
	.with_default_timeout()
	.await
	.unwrap();
	let uri = format!("ws://{}", server.local_addr());
	let client =
		WsClientBuilder::default().id_format(IdKind::String).build(&uri).with_default_timeout().await.unwrap().unwrap();
	let response: String = client.request("o", rpc_params![]).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(&response, exp);
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
	assert!(client.notification("notif", rpc_params![]).with_default_timeout().await.unwrap().is_ok());
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
	assert_error_response(err, ErrorObject::from(ErrorCode::MethodNotFound).into_owned());
}

#[tokio::test]
async fn parse_error_works() {
	let err = run_request_with_response(parse_error(Id::Num(0))).with_default_timeout().await.unwrap().unwrap_err();
	assert_error_response(err, ErrorObject::from(ErrorCode::ParseError).into_owned());
}

#[tokio::test]
async fn invalid_request_works() {
	let err =
		run_request_with_response(invalid_request(Id::Num(0_u64))).with_default_timeout().await.unwrap().unwrap_err();
	assert_error_response(err, ErrorObject::from(ErrorCode::InvalidRequest).into_owned());
}

#[tokio::test]
async fn invalid_params_works() {
	let err =
		run_request_with_response(invalid_params(Id::Num(0_u64))).with_default_timeout().await.unwrap().unwrap_err();
	assert_error_response(err, ErrorObject::from(ErrorCode::InvalidParams).into_owned());
}

#[tokio::test]
async fn internal_error_works() {
	let err =
		run_request_with_response(internal_error(Id::Num(0_u64))).with_default_timeout().await.unwrap().unwrap_err();
	assert_error_response(err, ErrorObject::from(ErrorCode::InternalError).into_owned());
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
			.subscribe("subscribe_hello", rpc_params![], "unsubscribe_hello")
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
	tokio::time::sleep(std::time::Duration::from_secs(2)).await;

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
	let mut batch_request = BatchRequestBuilder::new();
	batch_request.insert("say_hello", rpc_params![]).unwrap();
	batch_request.insert("say_goodbye", rpc_params![0_u64, 1, 2]).unwrap();
	batch_request.insert("get_swag", rpc_params![]).unwrap();
	let server_response = r#"[{"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","result":"goodbye","id":1}, {"jsonrpc":"2.0","result":"here's your swag","id":2}]"#.to_string();
	let batch_response = run_batch_request_with_response::<String>(batch_request, server_response)
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
	assert_eq!(batch_response.num_successful_calls(), 3);
	let results: Vec<String> = batch_response.into_ok().unwrap().collect();
	assert_eq!(results, vec!["hello".to_string(), "goodbye".to_string(), "here's your swag".to_string()]);
}

#[tokio::test]
async fn batch_request_out_of_order_response() {
	let mut batch_request = BatchRequestBuilder::new();
	batch_request.insert("say_hello", rpc_params![]).unwrap();
	batch_request.insert("say_goodbye", rpc_params![0_u64, 1, 2]).unwrap();
	batch_request.insert("get_swag", rpc_params![]).unwrap();
	let server_response = r#"[{"jsonrpc":"2.0","result":"here's your swag","id":2}, {"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","result":"goodbye","id":1}]"#.to_string();
	let res = run_batch_request_with_response::<String>(batch_request, server_response)
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
	assert_eq!(res.num_successful_calls(), 3);
	assert_eq!(res.num_failed_calls(), 0);
	assert_eq!(res.len(), 3);
	let response: Vec<_> = res.into_ok().unwrap().collect();

	assert_eq!(response, vec!["hello".to_string(), "goodbye".to_string(), "here's your swag".to_string()]);
}

#[tokio::test]
async fn batch_request_with_failed_call_works() {
	let mut batch_request = BatchRequestBuilder::new();
	batch_request.insert("say_hello", rpc_params![]).unwrap();
	batch_request.insert("say_goodbye", rpc_params![0_u64, 1, 2]).unwrap();
	batch_request.insert("get_swag", rpc_params![]).unwrap();
	let server_response = r#"[{"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":1}, {"jsonrpc":"2.0","result":"here's your swag","id":2}]"#.to_string();
	let res = run_batch_request_with_response::<String>(batch_request, server_response)
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
	assert_eq!(res.num_successful_calls(), 2);
	assert_eq!(res.num_failed_calls(), 1);
	assert_eq!(res.len(), 3);

	let successful_calls: Vec<_> = res.iter().filter_map(|r| r.as_ref().ok()).collect();
	let failed_calls: Vec<_> = res
		.iter()
		.filter_map(|r| match r {
			Err(e) => Some(e),
			_ => None,
		})
		.collect();

	assert_eq!(successful_calls, vec!["hello", "here's your swag"]);
	assert_eq!(failed_calls, vec![&ErrorObject::from(ErrorCode::MethodNotFound)]);
}

#[tokio::test]
async fn batch_request_with_untagged_enum_works() {
	use serde::Deserialize;

	#[derive(Deserialize, Clone, Debug, PartialEq)]
	#[serde(untagged)]
	enum Custom {
		Text(String),
		Number(u8),
	}

	impl Default for Custom {
		fn default() -> Self {
			Self::Number(0)
		}
	}

	let mut batch_request = BatchRequestBuilder::new();
	batch_request.insert("text", rpc_params![]).unwrap();
	batch_request.insert("binary", rpc_params![0_u64, 1, 2]).unwrap();
	let server_response =
		r#"[{"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","result":13,"id":1}]"#.to_string();
	let res = run_batch_request_with_response::<Custom>(batch_request, server_response)
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
	assert_eq!(res.num_successful_calls(), 2);
	assert_eq!(res.num_failed_calls(), 0);
	assert_eq!(res.len(), 2);
	let response: Vec<_> = res.into_ok().unwrap().collect();

	assert_eq!(response, vec![Custom::Text("hello".to_string()), Custom::Number(13)]);
}

#[tokio::test]
async fn batch_request_with_failed_call_gives_proper_error() {
	let mut batch_request = BatchRequestBuilder::new();
	batch_request.insert("say_hello", rpc_params![]).unwrap();
	batch_request.insert("say_goodbye", rpc_params![0_u64, 1, 2]).unwrap();
	batch_request.insert("get_swag", rpc_params![]).unwrap();
	let server_response = r#"[{"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":1}, {"jsonrpc":"2.0","error":{"code":-32602,"message":"foo"},"id":2}]"#.to_string();
	let res = run_batch_request_with_response::<String>(batch_request, server_response)
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
	let err: Vec<_> = res.into_ok().unwrap_err().collect();
	assert_eq!(err, vec![ErrorObject::from(ErrorCode::MethodNotFound), ErrorObject::borrowed(-32602, &"foo", None)]);
}

#[tokio::test]
async fn is_connected_works() {
	init_logger();

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

	let res: Result<String, Error> = client.request("say_hello", rpc_params![]).with_default_timeout().await.unwrap();
	res.unwrap_err();

	// give the background thread some time to terminate.
	tokio::time::sleep(std::time::Duration::from_millis(100)).await;
	assert!(!client.is_connected())
}

async fn run_batch_request_with_response<T: Send + DeserializeOwned + std::fmt::Debug + 'static>(
	batch: BatchRequestBuilder<'_>,
	response: String,
) -> Result<BatchResponse<T>, Error> {
	let server = WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), response)
		.with_default_timeout()
		.await
		.unwrap();
	let uri = to_ws_uri_string(server.local_addr());
	let client = WsClientBuilder::default().build(&uri).with_default_timeout().await.unwrap().unwrap();
	client.batch_request(batch).with_default_timeout().await.unwrap()
}

async fn run_request_with_response(response: String) -> Result<String, Error> {
	let server = WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), response)
		.with_default_timeout()
		.await
		.unwrap();
	let uri = format!("ws://{}", server.local_addr());
	let client = WsClientBuilder::default().build(&uri).with_default_timeout().await.unwrap().unwrap();
	client.request("say_hello", rpc_params![]).with_default_timeout().await.unwrap()
}

fn assert_error_response(err: Error, exp: ErrorObjectOwned) {
	let exp = CallError::Custom(exp);
	match &err {
		Error::Call(e) => {
			assert_eq!(e.to_string(), exp.to_string());
		}
		e => panic!("Expected error: \"{}\", got: {:?}", err, e),
	};
}

#[tokio::test]
async fn redirections() {
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
	let response: String = client.request("anything", rpc_params![]).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, String::from(expected));
}
