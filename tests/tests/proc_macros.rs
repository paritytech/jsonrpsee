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

//! Example of using proc macro to generate working client and server.

mod helpers;

use std::net::SocketAddr;

use helpers::init_logger;
use jsonrpsee::core::client::ClientT;
use jsonrpsee::core::{client::SubscriptionClientT, Error};
use jsonrpsee::http_client::HttpClientBuilder;
use jsonrpsee::rpc_params;
use jsonrpsee::server::ServerBuilder;
use jsonrpsee::types::error::{CallError, ErrorCode};

use jsonrpsee::ws_client::*;
use serde_json::json;

mod rpc_impl {
	use jsonrpsee::core::server::{
		IntoSubscriptionCloseResponse, PendingSubscriptionSink, SubscriptionCloseResponse, SubscriptionMessage,
	};
	use jsonrpsee::core::{async_trait, RpcResult, SubscriptionResult};
	use jsonrpsee::proc_macros::rpc;

	pub struct CustomSubscriptionRet;

	impl IntoSubscriptionCloseResponse for CustomSubscriptionRet {
		fn into_response(self) -> SubscriptionCloseResponse {
			SubscriptionCloseResponse::None
		}
	}

	#[rpc(client, server, namespace = "foo")]
	pub trait Rpc {
		#[method(name = "foo")]
		async fn async_method(&self, param_a: u8, param_b: String) -> RpcResult<u16>;

		#[method(name = "bar")]
		fn sync_method(&self) -> RpcResult<u16>;

		#[subscription(name = "sub", unsubscribe = "unsub", item = String)]
		async fn sub(&self) -> SubscriptionResult;
		#[subscription(name = "echo", unsubscribe = "unsubscribe_echo", aliases = ["alias_echo"], item = u32)]
		async fn sub_with_params(&self, val: u32) -> SubscriptionResult;

		#[subscription(name = "not-result", unsubscribe = "unsubscribe-not-result", item = String)]
		async fn sub_not_result(&self);

		#[subscription(name = "custom", unsubscribe = "unsubscribe_custom", item = String)]
		async fn sub_custom_ret(&self, x: usize) -> CustomSubscriptionRet;

		#[subscription(name = "unit_type", unsubscribe = "unsubscribe_unit_type", item = String)]
		async fn sub_unit_type(&self, x: usize);

		#[method(name = "params")]
		fn params(&self, a: u8, b: &str) -> RpcResult<String> {
			Ok(format!("Called with: {}, {}", a, b))
		}

		#[method(name = "optional_params")]
		fn optional_params(&self, a: u32, b: Option<u32>, c: Option<u32>) -> RpcResult<String> {
			Ok(format!("Called with: {}, {:?}, {:?}", a, b, c))
		}

		#[method(name = "lifetimes")]
		fn lifetimes(
			&self,
			a: &str,
			b: &'_ str,
			c: std::borrow::Cow<'_, str>,
			d: Option<beef::Cow<'_, str>>,
		) -> RpcResult<String> {
			Ok(format!("Called with: {}, {}, {}, {:?}", a, b, c, d))
		}

		#[method(name = "zero_copy_cow")]
		fn zero_copy_cow(&self, a: std::borrow::Cow<'_, str>, b: beef::Cow<'_, str>) -> RpcResult<String> {
			Ok(format!("Zero copy params: {}, {}", matches!(a, std::borrow::Cow::Borrowed(_)), b.is_borrowed()))
		}

		#[method(name = "blocking_call", blocking)]
		fn blocking_call(&self) -> RpcResult<u32> {
			std::thread::sleep(std::time::Duration::from_millis(50));
			Ok(42)
		}
	}

	pub struct RpcServerImpl;

	#[async_trait]
	impl RpcServer for RpcServerImpl {
		async fn async_method(&self, _param_a: u8, _param_b: String) -> RpcResult<u16> {
			Ok(42u16)
		}

		fn sync_method(&self) -> RpcResult<u16> {
			Ok(10u16)
		}

		async fn sub(&self, pending: PendingSubscriptionSink) -> SubscriptionResult {
			let sink = pending.accept().await?;
			sink.send("Response_A".into()).await?;
			sink.send("Response_B".into()).await?;

			Ok(())
		}

		async fn sub_with_params(&self, pending: PendingSubscriptionSink, val: u32) -> SubscriptionResult {
			let sink = pending.accept().await?;
			let msg = SubscriptionMessage::from_json(&val)?;
			sink.send(msg.clone()).await?;
			sink.send(msg).await?;

			Ok(())
		}

		async fn sub_not_result(&self, pending: PendingSubscriptionSink) {
			let sink = pending.accept().await.unwrap();
			sink.send("lo".into()).await.unwrap();
		}

		async fn sub_custom_ret(&self, _pending: PendingSubscriptionSink, _x: usize) -> CustomSubscriptionRet {
			CustomSubscriptionRet
		}

		async fn sub_unit_type(&self, _pending: PendingSubscriptionSink, _x: usize) {}
	}
}

// Use generated implementations of server and client.
use jsonrpsee::core::params::{ArrayParams, ObjectParams};
use rpc_impl::{RpcClient, RpcServer, RpcServerImpl};

pub async fn server() -> SocketAddr {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr = server.local_addr().unwrap();
	let handle = server.start(RpcServerImpl.into_rpc()).unwrap();

	tokio::spawn(handle.stopped());

	addr
}

#[tokio::test]
async fn proc_macros_generic_ws_client_api() {
	init_logger();

	let server_addr = server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	assert_eq!(client.async_method(10, "a".into()).await.unwrap(), 42);
	assert_eq!(client.sync_method().await.unwrap(), 10);

	// Sub without params
	let mut sub = client.sub().await.unwrap();
	let first_recv = sub.next().await.unwrap().unwrap();
	assert_eq!(first_recv, "Response_A".to_string());
	let second_recv = sub.next().await.unwrap().unwrap();
	assert_eq!(second_recv, "Response_B".to_string());

	// Sub with params
	let mut sub = client.sub_with_params(42).await.unwrap();
	let first_recv = sub.next().await.unwrap().unwrap();
	assert_eq!(first_recv, 42);
	let second_recv = sub.next().await.unwrap().unwrap();
	assert_eq!(second_recv, 42);
}

#[tokio::test]
async fn macro_param_parsing() {
	let module = RpcServerImpl.into_rpc();

	let res: String = module.call("foo_params", [json!(42_u64), json!("Hello")]).await.unwrap();

	assert_eq!(&res, "Called with: 42, Hello");
}

#[tokio::test]
async fn macro_optional_param_parsing() {
	let module = RpcServerImpl.into_rpc();

	// Optional param omitted at tail
	let res: String = module.call("foo_optional_params", [42_u64, 70]).await.unwrap();
	assert_eq!(&res, "Called with: 42, Some(70), None");

	// Optional param using `null`
	let res: String = module.call("foo_optional_params", [json!(42_u64), json!(null), json!(70_u64)]).await.unwrap();

	assert_eq!(&res, "Called with: 42, None, Some(70)");

	// Named params using a map
	let (resp, _) = module
		.raw_json_request(r#"{"jsonrpc":"2.0","method":"foo_optional_params","params":{"a":22,"c":50},"id":0}"#, 1)
		.await
		.unwrap();
	assert_eq!(resp.result, r#"{"jsonrpc":"2.0","result":"Called with: 22, None, Some(50)","id":0}"#);
}

#[tokio::test]
async fn macro_lifetimes_parsing() {
	let module = RpcServerImpl.into_rpc();

	let res: String = module.call("foo_lifetimes", ["foo", "bar", "baz", "qux"]).await.unwrap();

	assert_eq!(&res, "Called with: foo, bar, baz, Some(\"qux\")");
}

#[tokio::test]
async fn macro_zero_copy_cow() {
	init_logger();

	let module = RpcServerImpl.into_rpc();

	let (resp, _) = module
		.raw_json_request(r#"{"jsonrpc":"2.0","method":"foo_zero_copy_cow","params":["foo", "bar"],"id":0}"#, 1)
		.await
		.unwrap();

	// std::borrow::Cow<str> always deserialized to owned variant here
	assert_eq!(resp.result, r#"{"jsonrpc":"2.0","result":"Zero copy params: false, true","id":0}"#);

	// serde_json will have to allocate a new string to replace `\t` with byte 0x09 (tab)
	let (resp, _) = module
		.raw_json_request(r#"{"jsonrpc":"2.0","method":"foo_zero_copy_cow","params":["\tfoo", "\tbar"],"id":0}"#, 1)
		.await
		.unwrap();
	assert_eq!(resp.result, r#"{"jsonrpc":"2.0","result":"Zero copy params: false, false","id":0}"#);
}

// Disabled on MacOS as GH CI timings on Mac vary wildly (~100ms) making this test fail.
#[cfg(not(target_os = "macos"))]
#[ignore]
#[tokio::test]
async fn multiple_blocking_calls_overlap() {
	use jsonrpsee::core::EmptyServerParams;
	use std::time::{Duration, Instant};

	let module = RpcServerImpl.into_rpc();

	let futures =
		std::iter::repeat_with(|| module.call::<_, u64>("foo_blocking_call", EmptyServerParams::new())).take(4);
	let now = Instant::now();
	let results = futures::future::join_all(futures).await;
	let elapsed = now.elapsed();

	for result in results {
		assert_eq!(result.unwrap(), 42);
	}

	// Each request takes 50ms, added 50ms margin for scheduling
	assert!(elapsed < Duration::from_millis(100), "Expected less than 100ms, got {:?}", elapsed);
}

#[tokio::test]
async fn subscriptions_do_not_work_for_http_servers() {
	let htserver = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr = htserver.local_addr().unwrap();
	let htserver_url = format!("http://{}", addr);
	let _handle = htserver.start(RpcServerImpl.into_rpc()).unwrap();

	let htclient = HttpClientBuilder::default().build(&htserver_url).unwrap();

	assert_eq!(htclient.sync_method().await.unwrap(), 10);
	assert!(htclient.sub().await.is_err());
	assert!(matches!(htclient.sub().await, Err(Error::HttpNotImplemented)));
	assert_eq!(htclient.sync_method().await.unwrap(), 10);
}

#[tokio::test]
async fn calls_with_bad_params() {
	init_logger();

	let server_addr = server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	// Sub with faulty params as array.
	let err: Error = client
		.subscribe::<String, ArrayParams>("foo_echo", rpc_params!["0x0"], "foo_unsubscribe_echo")
		.await
		.unwrap_err();

	assert!(
		matches!(err, Error::Call(CallError::Custom (err)) if err.message().contains("invalid type: string \"0x0\", expected u32") && err.code() == ErrorCode::InvalidParams.code())
	);

	// Call with faulty params as array.
	let err: Error = client.request::<String, ArrayParams>("foo_foo", rpc_params!["faulty", "ok"]).await.unwrap_err();
	assert!(
		matches!(err, Error::Call(CallError::Custom (err)) if err.message().contains("invalid type: string \"faulty\", expected u8") && err.code() == ErrorCode::InvalidParams.code())
	);

	// Sub with faulty params as map.
	let mut params = ObjectParams::new();
	params.insert("val", "0x0").unwrap();

	let err: Error =
		client.subscribe::<String, ObjectParams>("foo_echo", params, "foo_unsubscribe_echo").await.unwrap_err();
	assert!(
		matches!(err, Error::Call(CallError::Custom (err)) if err.message().contains("invalid type: string \"0x0\", expected u32") && err.code() == ErrorCode::InvalidParams.code())
	);

	// Call with faulty params as map.
	let mut params = ObjectParams::new();
	params.insert("param_a", 1).unwrap();
	params.insert("param_b", 2).unwrap();

	let err: Error = client.request::<String, ObjectParams>("foo_foo", params).await.unwrap_err();
	assert!(
		matches!(err, Error::Call(CallError::Custom (err)) if err.message().contains("invalid type: integer `2`, expected a string") && err.code() == ErrorCode::InvalidParams.code())
	);
}

#[tokio::test]
async fn calls_with_object_params_works() {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr = server.local_addr().unwrap();
	let server_url = format!("ws://{}", addr);
	let _handle = server.start(RpcServerImpl.into_rpc()).unwrap();
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	// snake_case params
	let mut params = ObjectParams::new();
	params.insert("param_a", 0).unwrap();
	params.insert("param_b", "0x0").unwrap();

	assert_eq!(client.request::<u64, ObjectParams>("foo_foo", params).await.unwrap(), 42);

	// camelCase params.
	let mut params = ObjectParams::new();
	params.insert("paramA", 0).unwrap();
	params.insert("paramB", "0x0").unwrap();

	assert_eq!(client.request::<u64, ObjectParams>("foo_foo", params).await.unwrap(), 42);
}
