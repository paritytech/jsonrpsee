use jsonrpsee_types::{
	error::Error,
	jsonrpc::{self, ErrorCode, JsonValue, Params},
};

use crate::Subscription;
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::{Id, WebSocketTestServer};

#[tokio::test]
async fn http_method_call_works() {
	let result = http_run_request_with_response(ok_response("hello".into(), Id::Num(0))).await.unwrap();
	assert_eq!(JsonValue::String("hello".into()), result);
}

#[tokio::test]
async fn http_notification_works() {
	let server_addr = http_server_with_hardcoded_response(String::new()).await;
	let uri = format!("http://{}", server_addr);
	let client = crate::http(&uri);
	client
		.notification("i_dont_care_about_the_response_because_the_server_should_not_respond", Params::None)
		.await
		.unwrap();
}

#[tokio::test]
async fn http_response_with_wrong_id() {
	let err = http_run_request_with_response(ok_response("hello".into(), Id::Num(99))).await.unwrap_err();
	assert!(matches!(err, Error::TransportError(e) if e.to_string().contains("background task closed")));
}

#[tokio::test]
async fn http_response_method_not_found() {
	let err = http_run_request_with_response(method_not_found(Id::Num(0))).await;
	assert_jsonrpc_error_response(err, ErrorCode::MethodNotFound, METHOD_NOT_FOUND.into());
}

#[tokio::test]
async fn http_response_parse_error() {
	let err = http_run_request_with_response(parse_error(Id::Num(0))).await;
	assert_jsonrpc_error_response(err, ErrorCode::ParseError, PARSE_ERROR.into());
}

#[tokio::test]
async fn http_invalid_request_works() {
	let err = http_run_request_with_response(invalid_request(Id::Num(0_u64))).await;
	assert_jsonrpc_error_response(err, ErrorCode::InvalidRequest, INVALID_REQUEST.into());
}

#[tokio::test]
async fn http_invalid_params_works() {
	let err = http_run_request_with_response(invalid_params(Id::Num(0_u64))).await;
	assert_jsonrpc_error_response(err, ErrorCode::InvalidParams, INVALID_PARAMS.into());
}

#[tokio::test]
async fn http_internal_error_works() {
	let err = http_run_request_with_response(internal_error(Id::Num(0_u64))).await;
	assert_jsonrpc_error_response(err, ErrorCode::InternalError, INTERNAL_ERROR.into());
}

async fn http_run_request_with_response(response: String) -> Result<JsonValue, Error> {
	let server_addr = http_server_with_hardcoded_response(response).await;
	let uri = format!("http://{}", server_addr);
	let client = crate::http(&uri);
	client.request("say_hello", Params::None).await
}

#[tokio::test]
async fn ws_method_call_works() {
	let server = WebSocketTestServer::with_hardcoded_response(
		"127.0.0.1:0".parse().unwrap(),
		ok_response("hello".into(), Id::Num(0_u64)),
	);
	let uri = to_ws_uri_string(server.local_addr());
	let client = crate::ws(&uri).await;
	let response: String = client.request("say_hello", jsonrpc::Params::None).await.unwrap();
	assert_eq!(&response, "hello");
}

#[tokio::test]
async fn ws_notif_works() {
	// this empty string shouldn't be read because the server shouldn't respond to notifications.
	let server = WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), String::new());
	let uri = to_ws_uri_string(server.local_addr());
	let client = crate::ws(&uri).await;
	assert!(client.notification("notif", jsonrpc::Params::None).await.is_ok());
}

#[tokio::test]
async fn ws_method_not_found_works() {
	let server =
		WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), method_not_found(Id::Num(0_u64)));
	let uri = to_ws_uri_string(server.local_addr());
	let client = crate::ws(&uri).await;
	let response: Result<jsonrpc::JsonValue, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert_jsonrpc_error_response(response, jsonrpc::ErrorCode::MethodNotFound, METHOD_NOT_FOUND.into());
}

#[tokio::test]
async fn ws_parse_error_works() {
	let server =
		WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), parse_error(Id::Num(0_u64)));
	let uri = to_ws_uri_string(server.local_addr());
	let client = crate::ws(&uri).await;
	let response: Result<jsonrpc::JsonValue, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert_jsonrpc_error_response(response, jsonrpc::ErrorCode::ParseError, PARSE_ERROR.into());
}

#[tokio::test]
async fn ws_invalid_request_works() {
	let server =
		WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), invalid_request(Id::Num(0_u64)));
	let uri = to_ws_uri_string(server.local_addr());
	let client = crate::ws(&uri).await;
	let response: Result<jsonrpc::JsonValue, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert_jsonrpc_error_response(response, jsonrpc::ErrorCode::InvalidRequest, INVALID_REQUEST.into());
}

#[tokio::test]
async fn ws_invalid_params_works() {
	let server =
		WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), invalid_params(Id::Num(0_u64)));
	let uri = to_ws_uri_string(server.local_addr());
	let client = crate::ws(&uri).await;
	let response: Result<jsonrpc::JsonValue, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert_jsonrpc_error_response(response, jsonrpc::ErrorCode::InvalidParams, INVALID_PARAMS.into());
}

#[tokio::test]
async fn ws_internal_error_works() {
	let server =
		WebSocketTestServer::with_hardcoded_response("127.0.0.1:0".parse().unwrap(), internal_error(Id::Num(0_u64)));
	let uri = to_ws_uri_string(server.local_addr());
	let client = crate::ws(&uri).await;
	let response: Result<jsonrpc::JsonValue, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert_jsonrpc_error_response(response, jsonrpc::ErrorCode::InternalError, INTERNAL_ERROR.into());
}

#[tokio::test]
async fn ws_subscription_works() {
	let server = WebSocketTestServer::with_hardcoded_subscription(
		"127.0.0.1:0".parse().unwrap(),
		server_subscription_id_response(Id::Num(0)),
		server_subscription_response(jsonrpc::JsonValue::String("hello my friend".to_owned())),
	);
	let uri = to_ws_uri_string(server.local_addr());
	let client = crate::ws(&uri).await;
	{
		let mut sub: Subscription<String> =
			client.subscribe("subscribe_hello", jsonrpc::Params::None, "unsubscribe_hello").await.unwrap();
		let response: String = sub.next().await.unwrap().into();
		assert_eq!("hello my friend".to_owned(), response);
	}
}

// TODO(niklasad1): This test fails sometimes when the a task on `tokio` is supposed to be dropped but doesn't sometimes.
// It appears similar to https://github.com/tokio-rs/tokio/issues/3493 rarely but should be fixed on futures 0.3.13.
//
// So I suspect there's some bug in tokio that prevents that task to dropped when there are tasks in the queue of something
// but I haven't debugged it properly quite hard with the raw pointers/generator stuff in tokio/futures.
//
// To workaround this I changed the `WebSocketServer` to use `async-std` instead and I haven't seen any deadlocks yet.
#[tokio::test]
async fn ws_response_with_wrong_id() {
	let server = WebSocketTestServer::with_hardcoded_response(
		"127.0.0.1:0".parse().unwrap(),
		ok_response("hello".into(), Id::Num(99)),
	);
	let uri = to_ws_uri_string(server.local_addr());
	let client = crate::ws(&uri).await;
	let err: Result<jsonrpc::JsonValue, Error> = client.request("say_hello", jsonrpc::Params::None).await;
	assert!(matches!(err, Err(Error::TransportError(e)) if e.to_string().contains("background task closed")));
}

fn assert_jsonrpc_error_response(
	response: Result<jsonrpc::JsonValue, Error>,
	code: jsonrpc::ErrorCode,
	message: String,
) {
	let expected = jsonrpc::Error { code, message, data: None };
	match response {
		Err(Error::Request(err)) => {
			assert_eq!(err, expected);
		}
		e => panic!("Expected error: \"{}\", got: {:?}", expected, e),
	};
}
