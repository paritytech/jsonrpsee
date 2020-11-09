use crate::client::HttpClient;
use crate::types::client::Error;
use crate::types::jsonrpc::{self, ErrorCode, JsonValue, Params};

use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::types::Id;

#[tokio::test]
async fn method_call_works() {
	let result = run_request_with_response(ok_response("hello".into(), Id::Num(0))).await.unwrap();
	assert_eq!(JsonValue::String("hello".into()), result);
}

#[tokio::test]
async fn notification_works() {
	let server_addr = http_server_with_hardcoded_response(String::new()).await;
	let uri = format!("http://{}", server_addr);
	let client = HttpClient::new(&uri, Default::default());
	client
		.notification("i_dont_care_about_the_response_because_the_server_should_not_respond", Params::None)
		.await
		.unwrap();
}

#[tokio::test]
async fn response_with_wrong_id() {
	let err = run_request_with_response(ok_response("hello".into(), Id::Num(99))).await.unwrap_err();
	assert!(matches!(err, Error::InvalidRequestId(_mismatch)));
}

#[tokio::test]
async fn response_method_not_found() {
	let err = run_request_with_response(method_not_found(Id::Num(0))).await.unwrap_err();
	assert_jsonrpc_error_response(err, ErrorCode::MethodNotFound, METHOD_NOT_FOUND.into());
}

#[tokio::test]
async fn response_parse_error() {
	let err = run_request_with_response(parse_error(Id::Num(0))).await.unwrap_err();
	assert_jsonrpc_error_response(err, ErrorCode::ParseError, PARSE_ERROR.into());
}

#[tokio::test]
async fn invalid_request_works() {
	let err = run_request_with_response(invalid_request(Id::Num(0_u64))).await.unwrap_err();
	assert_jsonrpc_error_response(err, ErrorCode::InvalidRequest, INVALID_REQUEST.into());
}

#[tokio::test]
async fn invalid_params_works() {
	let err = run_request_with_response(invalid_params(Id::Num(0_u64))).await.unwrap_err();
	assert_jsonrpc_error_response(err, ErrorCode::InvalidParams, INVALID_PARAMS.into());
}

#[tokio::test]
async fn internal_error_works() {
	let err = run_request_with_response(internal_error(Id::Num(0_u64))).await.unwrap_err();
	assert_jsonrpc_error_response(err, ErrorCode::InternalError, INTERNAL_ERROR.into());
}

async fn run_request_with_response(response: String) -> Result<JsonValue, Error> {
	let server_addr = http_server_with_hardcoded_response(response).await;
	let uri = format!("http://{}", server_addr);
	let client = HttpClient::new(&uri, Default::default());
	client.request("say_hello", Params::None).await
}

fn assert_jsonrpc_error_response(response: Error, code: ErrorCode, message: String) {
	let expected = jsonrpc::Error { code, message, data: None };
	match response {
		Error::Request(err) => {
			assert_eq!(err, expected);
		}
		e @ _ => panic!("Expected error: \"{}\", got: {:?}", expected, e),
	};
}
