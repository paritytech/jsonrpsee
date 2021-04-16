use crate::v2::error::{
	INTERNAL_ERROR_CODE, INTERNAL_ERROR_MSG, INVALID_PARAMS_CODE, INVALID_PARAMS_MSG, INVALID_REQUEST_CODE,
	INVALID_REQUEST_MSG, METHOD_NOT_FOUND_CODE, METHOD_NOT_FOUND_MSG, PARSE_ERROR_CODE, PARSE_ERROR_MSG,
};
use crate::v2::JsonRpcParams;
use crate::{traits::Client, Error, HttpClientBuilder, JsonValue};
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
	let client = HttpClientBuilder::default().build(&uri).unwrap();
	client
		.notification("i_dont_care_about_the_response_because_the_server_should_not_respond", JsonRpcParams::NoParams)
		.await
		.unwrap();
}

#[tokio::test]
async fn response_with_wrong_id() {
	let err = run_request_with_response(ok_response("hello".into(), Id::Num(99))).await.unwrap_err();
	assert!(matches!(err, Error::InvalidRequestId));
}

#[tokio::test]
async fn response_method_not_found() {
	let err = run_request_with_response(method_not_found(Id::Num(0))).await.unwrap_err();
	assert_jsonrpc_error_response(err, METHOD_NOT_FOUND_CODE, METHOD_NOT_FOUND_MSG);
}

#[tokio::test]
async fn response_parse_error() {
	let err = run_request_with_response(parse_error(Id::Num(0))).await.unwrap_err();
	assert_jsonrpc_error_response(err, PARSE_ERROR_CODE, PARSE_ERROR_MSG);
}

#[tokio::test]
async fn invalid_request_works() {
	let err = run_request_with_response(invalid_request(Id::Num(0_u64))).await.unwrap_err();
	assert_jsonrpc_error_response(err, INVALID_REQUEST_CODE, INVALID_REQUEST_MSG);
}

#[tokio::test]
async fn invalid_params_works() {
	let err = run_request_with_response(invalid_params(Id::Num(0_u64))).await.unwrap_err();
	assert_jsonrpc_error_response(err, INVALID_PARAMS_CODE, INVALID_PARAMS_MSG);
}

#[tokio::test]
async fn internal_error_works() {
	let err = run_request_with_response(internal_error(Id::Num(0_u64))).await.unwrap_err();
	assert_jsonrpc_error_response(err, INTERNAL_ERROR_CODE, INTERNAL_ERROR_MSG);
}

#[tokio::test]
async fn subscription_response_to_request() {
	let req = r#"{"jsonrpc":"2.0","method":"subscribe_hello","params":{"subscription":"3px4FrtxSYQ1zBKW154NoVnrDhrq764yQNCXEgZyM6Mu","result":"hello my friend"}}"#.to_string();
	let err = run_request_with_response(req).await.unwrap_err();
	assert!(matches!(err, Error::ParseError(_)));
}

#[tokio::test]
async fn batch_request_works() {
	let batch_request = vec![
		("say_hello", JsonRpcParams::NoParams),
		("say_goodbye", JsonRpcParams::Array(vec![0_u64.into(), 1.into(), 2.into()])),
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
		("say_goodbye", JsonRpcParams::Array(vec![0_u64.into(), 1.into(), 2.into()])),
		("get_swag", JsonRpcParams::NoParams),
	];
	let server_response = r#"[{"jsonrpc":"2.0","result":"here's your swag","id":2}, {"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","result":"goodbye","id":1}]"#.to_string();
	let response = run_batch_request_with_response(batch_request, server_response).await.unwrap();
	assert_eq!(response, vec!["hello".to_string(), "goodbye".to_string(), "here's your swag".to_string()]);
}

async fn run_batch_request_with_response<'a>(
	batch: Vec<(&'a str, JsonRpcParams<'a>)>,
	response: String,
) -> Result<Vec<String>, Error> {
	let server_addr = http_server_with_hardcoded_response(response).await;
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&uri).unwrap();
	client.batch_request(batch).await
}

async fn run_request_with_response(response: String) -> Result<JsonValue, Error> {
	let server_addr = http_server_with_hardcoded_response(response).await;
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&uri).unwrap();
	client.request("say_hello", JsonRpcParams::NoParams).await
}

fn assert_jsonrpc_error_response(error: Error, code: i32, message: &str) {
	match &error {
		Error::Request(err) => {
			assert_eq!(err.inner.code.code(), code);
			assert_eq!(&err.inner.message, message);
		}
		e @ _ => panic!("Expected error: \"{}\", got: {:?}", error, e),
	};
}
