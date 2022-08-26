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

use crate::types::error::{ErrorCode, ErrorObject};

use crate::HttpClientBuilder;
use jsonrpsee_core::client::{ClientT, IdKind};
use jsonrpsee_core::rpc_params;
use jsonrpsee_core::Error;
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::mocks::Id;
use jsonrpsee_test_utils::TimeoutFutureExt;
use jsonrpsee_types::error::{CallError, ErrorObjectOwned};
use jsonrpsee_types::BatchParamsBuilder;
use serde_json::value::RawValue;

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
	let server_addr =
		http_server_with_hardcoded_response(ok_response(exp.into(), Id::Num(0))).with_default_timeout().await.unwrap();
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().id_format(IdKind::String).build(&uri).unwrap();
	let res: Result<String, Error> = client.request("o", rpc_params![]).with_default_timeout().await.unwrap();
	assert!(matches!(res, Err(Error::InvalidRequestId)));
}

#[tokio::test]
async fn method_call_with_id_str() {
	let exp = "id as string";
	let server_addr = http_server_with_hardcoded_response(ok_response(exp.into(), Id::Str("0".into())))
		.with_default_timeout()
		.await
		.unwrap();
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().id_format(IdKind::String).build(&uri).unwrap();
	let response: String = client.request("o", rpc_params![]).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(&response, exp);
}

#[tokio::test]
async fn notification_works() {
	let server_addr = http_server_with_hardcoded_response(String::new()).with_default_timeout().await.unwrap();
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&uri).unwrap();
	client
		.notification("i_dont_care_about_the_response_because_the_server_should_not_respond", rpc_params![])
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
}

#[tokio::test]
async fn response_with_wrong_id() {
	let err = run_request_with_response(ok_response("hello".into(), Id::Num(99)))
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap_err();
	assert!(matches!(err, Error::InvalidRequestId));
}

#[tokio::test]
async fn response_method_not_found() {
	let err =
		run_request_with_response(method_not_found(Id::Num(0))).with_default_timeout().await.unwrap().unwrap_err();
	assert_jsonrpc_error_response(err, ErrorObject::from(ErrorCode::MethodNotFound).into_owned());
}

#[tokio::test]
async fn response_parse_error() {
	let err = run_request_with_response(parse_error(Id::Num(0))).with_default_timeout().await.unwrap().unwrap_err();
	assert_jsonrpc_error_response(err, ErrorObject::from(ErrorCode::ParseError).into_owned());
}

#[tokio::test]
async fn invalid_request_works() {
	let err =
		run_request_with_response(invalid_request(Id::Num(0_u64))).with_default_timeout().await.unwrap().unwrap_err();
	assert_jsonrpc_error_response(err, ErrorObject::from(ErrorCode::InvalidRequest).into_owned());
}

#[tokio::test]
async fn invalid_params_works() {
	let err =
		run_request_with_response(invalid_params(Id::Num(0_u64))).with_default_timeout().await.unwrap().unwrap_err();
	assert_jsonrpc_error_response(err, ErrorObject::from(ErrorCode::InvalidParams).into_owned());
}

#[tokio::test]
async fn internal_error_works() {
	let err =
		run_request_with_response(internal_error(Id::Num(0_u64))).with_default_timeout().await.unwrap().unwrap_err();
	assert_jsonrpc_error_response(err, ErrorObject::from(ErrorCode::InternalError).into_owned());
}

#[tokio::test]
async fn subscription_response_to_request() {
	let req = r#"{"jsonrpc":"2.0","method":"subscribe_hello","params":{"subscription":"3px4FrtxSYQ1zBKW154NoVnrDhrq764yQNCXEgZyM6Mu","result":"hello my friend"}}"#.to_string();
	let err = run_request_with_response(req).with_default_timeout().await.unwrap().unwrap_err();
	assert!(matches!(err, Error::ParseError(_)));
}

#[tokio::test]
async fn batch_request_works() {
	let mut builder = BatchParamsBuilder::new();
	builder.insert("say_hello", rpc_params![]).unwrap();
	builder.insert("say_goodbye", rpc_params![0_u64, 1, 2]).unwrap();
	builder.insert("get_swag", rpc_params![]).unwrap();
	let batch_request = builder.build();
	let server_response = r#"[{"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","result":"goodbye","id":1}, {"jsonrpc":"2.0","result":"here's your swag","id":2}]"#.to_string();
	let response =
		run_batch_request_with_response(batch_request, server_response).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, vec!["hello".to_string(), "goodbye".to_string(), "here's your swag".to_string()]);
}

#[tokio::test]
async fn batch_request_out_of_order_response() {
	let mut builder = BatchParamsBuilder::new();
	builder.insert("say_hello", rpc_params![]).unwrap();
	builder.insert("say_goodbye", rpc_params![0_u64, 1, 2]).unwrap();
	builder.insert("get_swag", rpc_params![]).unwrap();
	let batch_request = builder.build();
	let server_response = r#"[{"jsonrpc":"2.0","result":"here's your swag","id":2}, {"jsonrpc":"2.0","result":"hello","id":0}, {"jsonrpc":"2.0","result":"goodbye","id":1}]"#.to_string();
	let response =
		run_batch_request_with_response(batch_request, server_response).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, vec!["hello".to_string(), "goodbye".to_string(), "here's your swag".to_string()]);
}

async fn run_batch_request_with_response(
	batch: Vec<(&str, Option<Box<RawValue>>)>,
	response: String,
) -> Result<Vec<String>, Error> {
	let server_addr = http_server_with_hardcoded_response(response).with_default_timeout().await.unwrap();
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&uri).unwrap();
	client.batch_request(batch).with_default_timeout().await.unwrap()
}

async fn run_request_with_response(response: String) -> Result<String, Error> {
	let server_addr = http_server_with_hardcoded_response(response).with_default_timeout().await.unwrap();
	let uri = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&uri).unwrap();
	client.request("say_hello", rpc_params![]).with_default_timeout().await.unwrap()
}

fn assert_jsonrpc_error_response(err: Error, exp: ErrorObjectOwned) {
	let exp = CallError::Custom(exp);
	match &err {
		Error::Call(err) => {
			assert_eq!(err.to_string(), exp.to_string());
		}
		e => panic!("Expected error: \"{}\", got: {:?}", err, e),
	};
}
