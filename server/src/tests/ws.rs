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

use crate::tests::helpers::{deser_call, init_logger, server_with_context};
use crate::types::SubscriptionId;
use crate::{RpcModule, ServerBuilder};
use jsonrpsee_core::{traits::IdProvider, Error};
use jsonrpsee_test_utils::helpers::*;
use jsonrpsee_test_utils::mocks::{Id, WebSocketTestClient, WebSocketTestError};
use jsonrpsee_test_utils::TimeoutFutureExt;
use serde_json::Value as JsonValue;

use super::helpers::server;

#[tokio::test]
async fn can_set_the_max_request_body_size() {
	init_logger();

	let addr = "127.0.0.1:0";
	// Rejects all requests larger than 100 bytes
	let server = ServerBuilder::default().max_request_body_size(100).build(addr).await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("anything", |_p, _cx| Ok("a".repeat(100))).unwrap();
	let addr = server.local_addr().unwrap();
	let handle = server.start(module).unwrap();

	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	// Invalid: too long
	let req = format!(r#"{{"jsonrpc":"2.0","method":"{}","id":1}}"#, "a".repeat(100));
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, oversized_request(100));

	// Max request body size should not override the max response body size
	let req = r#"{"jsonrpc":"2.0","method":"anything","id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::String("a".repeat(100)), Id::Num(1)));

	handle.stop().unwrap();
	handle.stopped().await;
}

#[tokio::test]
async fn can_set_the_max_response_body_size() {
	init_logger();

	let addr = "127.0.0.1:0";
	// Set the max response body size to 100 bytes
	let server = ServerBuilder::default().max_response_body_size(100).build(addr).await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("anything", |_p, _cx| Ok("a".repeat(101))).unwrap();
	let addr = server.local_addr().unwrap();
	let server_handle = server.start(module).unwrap();

	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	// Oversized response.
	let req = r#"{"jsonrpc":"2.0", "method":"anything", "id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, oversized_response(Id::Num(1), 100));

	server_handle.stop().unwrap();
	server_handle.stopped().await;
}

#[tokio::test]
async fn can_set_the_max_response_size_to_batch() {
	init_logger();

	let addr = "127.0.0.1:0";
	// Set the max response body size to 100 bytes
	let server = ServerBuilder::default().max_response_body_size(100).build(addr).await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("anything", |_p, _cx| Ok("a".repeat(51))).unwrap();
	let addr = server.local_addr().unwrap();
	let server_handle = server.start(module).unwrap();

	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	// Two response will end up in a response bigger than 100 bytes.
	let req = r#"[{"jsonrpc":"2.0", "method":"anything", "id":1},{"jsonrpc":"2.0", "method":"anything", "id":2}]"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, invalid_request(Id::Null));

	server_handle.stop().unwrap();
	server_handle.stopped().await;
}

#[tokio::test]
async fn can_set_max_connections() {
	init_logger();

	let addr = "127.0.0.1:0";
	// Server that accepts max 2 connections
	let server = ServerBuilder::default().max_connections(2).build(addr).await.unwrap();
	let mut module = RpcModule::new(());
	module.register_method("anything", |_p, _cx| Ok(())).unwrap();
	let addr = server.local_addr().unwrap();

	let server_handle = server.start(module).unwrap();

	let conn1 = WebSocketTestClient::new(addr).await;
	let conn2 = WebSocketTestClient::new(addr).await;
	let conn3 = WebSocketTestClient::new(addr).await;
	assert!(conn1.is_ok());
	assert!(conn2.is_ok());
	// Third connection is rejected
	assert!(conn3.is_err());
	if !matches!(conn3, Err(WebSocketTestError::RejectedWithStatusCode(429))) {
		panic!("Expected RejectedWithStatusCode(429), got: {:#?}", conn3);
	}

	// Decrement connection count
	drop(conn2);

	tokio::time::sleep(std::time::Duration::from_millis(100)).await;

	// Can connect again
	let conn4 = WebSocketTestClient::new(addr).await;
	assert!(conn4.is_ok());

	server_handle.stop().unwrap();
	server_handle.stopped().await;
}

#[tokio::test]
async fn single_method_calls_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	for i in 0..10 {
		let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i);
		let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();

		assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(i)));
	}
}

#[tokio::test]
async fn async_method_calls_works() {
	init_logger();
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	for i in 0..10 {
		let req = format!(r#"{{"jsonrpc":"2.0","method":"say_hello_async","id":{}}}"#, i);
		let response = client.send_request_text(req).await.unwrap();

		assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(i)));
	}
}

#[tokio::test]
async fn slow_method_calls_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"sleep_for","params":[1000],"id":123}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();

	assert_eq!(response, ok_response(JsonValue::String("Yawn!".to_owned()), Id::Num(123)));
}

#[tokio::test]
async fn batch_method_call_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let mut batch = vec![r#"{"jsonrpc":"2.0","method":"sleep_for","params":[1000],"id":123}"#.to_string()];
	for i in 1..4 {
		batch.push(format!(r#"{{"jsonrpc":"2.0","method":"say_hello","id":{}}}"#, i));
	}
	let batch = format!("[{}]", batch.join(","));
	let response = client.send_request_text(batch).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(
		response,
		r#"[{"jsonrpc":"2.0","result":"Yawn!","id":123},{"jsonrpc":"2.0","result":"hello","id":1},{"jsonrpc":"2.0","result":"hello","id":2},{"jsonrpc":"2.0","result":"hello","id":3}]"#
	);
}

#[tokio::test]
async fn batch_method_call_where_some_calls_fail() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let batch = vec![
		r#"{"jsonrpc":"2.0","method":"say_hello","id":1}"#,
		r#"{"jsonrpc":"2.0","method":"call_fail","id":2}"#,
		r#"{"jsonrpc":"2.0","method":"add","params":[34, 45],"id":3}"#,
	];
	let batch = format!("[{}]", batch.join(","));

	let response = client.send_request_text(batch).with_default_timeout().await.unwrap().unwrap();

	assert_eq!(
		response,
		r#"[{"jsonrpc":"2.0","result":"hello","id":1},{"jsonrpc":"2.0","error":{"code":-32000,"message":"MyAppError"},"id":2},{"jsonrpc":"2.0","result":79,"id":3}]"#
	);
}

#[tokio::test]
async fn garbage_request_fails() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"dsdfs fsdsfds"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"{ "#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"{}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"{sds}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"["#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"[dsds]"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));

	let req = r#"[]"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, invalid_request(Id::Null));

	let req = r#"[{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, parse_error(Id::Null));
}

#[tokio::test]
async fn whitespace_is_not_significant() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"         {"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::Number(3u32.into()), Id::Num(1)));

	let req = r#" [{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}]"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, r#"[{"jsonrpc":"2.0","result":3,"id":1}]"#);
}

#[tokio::test]
async fn single_method_call_with_params_works() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_faulty_params_returns_err() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();
	let expected = r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"invalid type: string \"should be a number\", expected u64 at line 1 column 21"},"id":1}"#;

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":["should be a number"],"id":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, expected);
}

#[tokio::test]
async fn single_method_call_with_faulty_context() {
	let addr = server_with_context().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"should_err", "params":[],"id":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, call_execution_failed("RPC context failed", Id::Num(1)));
}

#[tokio::test]
async fn single_method_call_with_ok_context() {
	let addr = server_with_context().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"should_ok", "params":[],"id":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response("ok".into(), Id::Num(1)));
}

#[tokio::test]
async fn async_method_call_with_ok_context() {
	let addr = server_with_context().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"should_ok_async", "params":[],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, ok_response("ok!".into(), Id::Num(1)));
}

#[tokio::test]
async fn async_method_call_with_params() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add_async", "params":[1, 2],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn async_method_call_that_fails() {
	let addr = server_with_context().await;
	let mut client = WebSocketTestClient::new(addr).await.unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"err_async", "params":[],"id":1}"#;
	let response = client.send_request_text(req).await.unwrap();
	assert_eq!(response, call_execution_failed("nah", Id::Num(1)));
}

#[tokio::test]
async fn single_method_send_binary() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"add", "params":[1, 2],"id":1}"#;
	let response = client.send_request_binary(req.as_bytes()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response(JsonValue::Number(3.into()), Id::Num(1)));
}

#[tokio::test]
async fn should_return_method_not_found() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"bar","id":"foo"}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, method_not_found(Id::Str("foo".into())));
}

#[tokio::test]
async fn invalid_json_id_missing_value() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"say_hello","id"}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request), it MUST be
	// Null.
	assert_eq!(response, parse_error(Id::Null));
}

#[tokio::test]
async fn invalid_request_object() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"method":"bar","id":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, invalid_request(Id::Num(1)));
}

#[tokio::test]
async fn unknown_field_is_ok() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"jsonrpc":"2.0","method":"say_hello","id":1,"is_not_request_object":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(1)));
}

#[tokio::test]
async fn register_methods_works() {
	let mut module = RpcModule::new(());
	assert!(module.register_method("say_hello", |_, _| Ok("lo")).is_ok());
	assert!(module.register_method("say_hello", |_, _| Ok("lo")).is_err());
	assert!(module
		.register_subscription("subscribe_hello", "subscribe_hello", "unsubscribe_hello", |_, _, _| { Ok(()) })
		.is_ok());
	assert!(module
		.register_subscription("subscribe_hello_again", "subscribe_hello_again", "unsubscribe_hello", |_, _, _| {
			Ok(())
		})
		.is_err());
	assert!(
		module.register_method("subscribe_hello_again", |_, _| Ok("lo")).is_ok(),
		"Failed register_subscription should not have side-effects"
	);
}

#[tokio::test]
async fn register_same_subscribe_unsubscribe_is_err() {
	let mut module = RpcModule::new(());
	assert!(matches!(
		module.register_subscription("subscribe_hello", "subscribe_hello", "subscribe_hello", |_, _, _| { Ok(()) }),
		Err(Error::SubscriptionNameConflict(_))
	));
}

#[tokio::test]
async fn parse_error_request_should_not_close_connection() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let invalid_request = r#"{"jsonrpc":"2.0","method":"bar","params":[1,"id":99}"#;
	let response1 = client.send_request_text(invalid_request).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response1, parse_error(Id::Null));
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":33}"#;
	let response2 = client.send_request_text(request).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response2, ok_response(JsonValue::String("hello".to_owned()), Id::Num(33)));
}

#[tokio::test]
async fn invalid_request_should_not_close_connection() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let req = r#"{"method":"bar","id":1}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, invalid_request(Id::Num(1)));
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":33}"#;
	let response = client.send_request_text(request).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(33)));
}

#[tokio::test]
async fn valid_request_that_fails_to_execute_should_not_close_connection() {
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	// Good request, executes fine
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":33}"#;
	let response = client.send_request_text(request).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(33)));

	// Good request, but causes error.
	let req = r#"{"jsonrpc":"2.0","method":"call_fail","params":[],"id":123}"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, r#"{"jsonrpc":"2.0","error":{"code":-32000,"message":"MyAppError"},"id":123}"#);

	// Connection is still good.
	let request = r#"{"jsonrpc":"2.0","method":"say_hello","id":333}"#;
	let response = client.send_request_text(request).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, ok_response(JsonValue::String("hello".to_owned()), Id::Num(333)));
}

#[tokio::test]
async fn can_register_modules() {
	let cx = String::new();
	let mut mod1 = RpcModule::new(cx);

	let cx2 = Vec::<u8>::new();
	let mut mod2 = RpcModule::new(cx2);

	assert_eq!(mod1.method_names().count(), 0);
	assert_eq!(mod2.method_names().count(), 0);
	mod1.register_method("bla", |_, cx| Ok(format!("Gave me {}", cx))).unwrap();
	mod1.register_method("bla2", |_, cx| Ok(format!("Gave me {}", cx))).unwrap();
	mod2.register_method("yada", |_, cx| Ok(format!("Gave me {:?}", cx))).unwrap();

	// Won't register, name clashes
	mod2.register_method("bla", |_, cx| Ok(format!("Gave me {:?}", cx))).unwrap();

	assert_eq!(mod1.method_names().count(), 2);
	let err = mod1.merge(mod2).unwrap_err();
	assert!(matches!(err, Error::MethodAlreadyRegistered(err) if err == "bla"));
	assert_eq!(mod1.method_names().count(), 2);
}

#[tokio::test]
async fn unsubscribe_twice_should_indicate_error() {
	init_logger();
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let sub_call = call("subscribe_hello", Vec::<()>::new(), Id::Num(0));
	let sub_id: u64 = deser_call(client.send_request_text(sub_call).await.unwrap());

	let unsub_call = call("unsubscribe_hello", vec![sub_id], Id::Num(1));
	let unsub_1: bool = deser_call(client.send_request_text(unsub_call).await.unwrap());
	assert!(unsub_1);

	let unsub_call = call("unsubscribe_hello", vec![sub_id], Id::Num(2));
	let unsub_2: bool = deser_call(client.send_request_text(unsub_call).await.unwrap());

	assert!(!unsub_2);
}

#[tokio::test]
async fn unsubscribe_wrong_sub_id_type() {
	init_logger();
	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let unsub: bool =
		deser_call(client.send_request_text(call("unsubscribe_hello", vec![13.99_f64], Id::Num(0))).await.unwrap());
	assert!(!unsub);
}

#[tokio::test]
async fn custom_subscription_id_works() {
	#[derive(Debug, Clone)]
	struct HardcodedSubscriptionId;

	impl IdProvider for HardcodedSubscriptionId {
		fn next_id(&self) -> SubscriptionId<'static> {
			"0xdeadbeef".to_string().into()
		}
	}

	init_logger();
	let server = ServerBuilder::default()
		.set_id_provider(HardcodedSubscriptionId)
		.build("127.0.0.1:0")
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();
	let addr = server.local_addr().unwrap();
	let mut module = RpcModule::new(());
	module
		.register_subscription("subscribe_hello", "subscribe_hello", "unsubscribe_hello", |_, mut sink, _| {
			// There is no subscription ID prior to calling accept.
			let sub_id = sink.subscription_id();
			assert!(sub_id.is_none());

			sink.accept()?;

			let sub_id = sink.subscription_id();
			assert!(matches!(sub_id, Some(SubscriptionId::Str(id)) if id == "0xdeadbeef"));

			tokio::spawn(async move {
				loop {
					let _ = &sink;
					tokio::time::sleep(std::time::Duration::from_secs(30)).await;
				}
			});
			Ok(())
		})
		.unwrap();
	let _handle = server.start(module).unwrap();

	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	let sub = client.send_request_text(call("subscribe_hello", Vec::<()>::new(), Id::Num(0))).await.unwrap();
	assert_eq!(&sub, r#"{"jsonrpc":"2.0","result":"0xdeadbeef","id":0}"#);
	let unsub = client.send_request_text(call("unsubscribe_hello", vec!["0xdeadbeef"], Id::Num(1))).await.unwrap();
	assert_eq!(&unsub, r#"{"jsonrpc":"2.0","result":true,"id":1}"#);
}

#[tokio::test]
async fn disabled_batches() {
	// Disable batches support.
	let server = ServerBuilder::default()
		.batch_requests_supported(false)
		.build("127.0.0.1:0")
		.with_default_timeout()
		.await
		.unwrap()
		.unwrap();

	let mut module = RpcModule::new(());
	module.register_method("should_ok", |_, _ctx| Ok("ok")).unwrap();
	let addr = server.local_addr().unwrap();

	let server_handle = server.start(module).unwrap();

	// Send a valid batch.
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();
	let req = r#"[
		{"jsonrpc":"2.0","method":"should_ok", "params":[],"id":1},
		{"jsonrpc":"2.0","method":"should_ok", "params":[],"id":2}
	]"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, batches_not_supported());

	server_handle.stop().unwrap();
	server_handle.stopped().await;
}

#[tokio::test]
async fn invalid_batch_calls() {
	init_logger();

	let addr = server().await;
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();

	// batch with no requests
	let req = r#"[]"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, invalid_request(Id::Null));

	// batch with invalid request
	let req = r#"[123]"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, invalid_batch(vec![Id::Null]));

	// batch with invalid request
	let req = r#"[1, 2, 3]"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, invalid_batch(vec![Id::Null, Id::Null, Id::Null]));

	// invalid JSON in batch
	let req = r#"[
		{"jsonrpc": "2.0", "method": "sum", "params": [1,2,4], "id": "1"},
		{"jsonrpc": "2.0", "method"
	  ]"#;
	let response = client.send_request_text(req).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, parse_error(Id::Null));
}

#[tokio::test]
async fn batch_with_mixed_calls() {
	init_logger();

	let addr = server().with_default_timeout().await.unwrap();
	let mut client = WebSocketTestClient::new(addr).with_default_timeout().await.unwrap().unwrap();
	// mixed notifications, method calls and valid json should be valid.
	let req = r#"[
			{"jsonrpc": "2.0", "method": "add", "params": [1,2,4], "id": "1"},
			{"jsonrpc": "2.0", "method": "add", "params": [7]},
			{"foo": "boo"},
			{"jsonrpc": "2.0", "method": "foo.get", "params": {"name": "myself"}, "id": "5"}
		]"#;
	let res = r#"[{"jsonrpc":"2.0","result":7,"id":"1"},{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid request"},"id":null},{"jsonrpc":"2.0","error":{"code":-32601,"message":"Method not found"},"id":"5"}]"#;
	let response = client.send_request_text(req.to_string()).with_default_timeout().await.unwrap().unwrap();
	assert_eq!(response, res);
}
