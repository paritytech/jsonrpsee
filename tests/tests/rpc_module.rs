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

use std::collections::HashMap;
use std::time::Duration;

use futures::StreamExt;
use jsonrpsee::core::error::{Error, SubscriptionClosed};
use jsonrpsee::core::server::rpc_module::*;
use jsonrpsee::types::error::{CallError, ErrorCode, ErrorObject};
use jsonrpsee::types::{EmptyParams, Params};
use serde::{Deserialize, Serialize};
use tokio::time::interval;
use tokio_stream::wrappers::IntervalStream;

// Helper macro to assert that a binding is of a specific type.
macro_rules! assert_type {
	( $ty:ty, $expected:expr $(,)?) => {{
		fn assert_type<Expected>(_expected: &Expected) {}
		assert_type::<$ty>($expected)
	}};
}

#[test]
fn rpc_modules_with_different_contexts_can_be_merged() {
	let cx = Vec::<u8>::new();
	let mut mod1 = RpcModule::new(cx);
	mod1.register_method("bla with Vec context", |_: Params, _| Ok(())).unwrap();
	let mut mod2 = RpcModule::new(String::new());
	mod2.register_method("bla with String context", |_: Params, _| Ok(())).unwrap();

	mod1.merge(mod2).unwrap();

	assert!(mod1.method("bla with Vec context").is_some());
	assert!(mod1.method("bla with String context").is_some());
}

#[test]
fn flatten_rpc_modules() {
	let mod1 = RpcModule::new(String::new());
	assert_type!(RpcModule<String>, &mod1);
	let unit_mod = mod1.remove_context();
	assert_type!(RpcModule<()>, &unit_mod);
}

#[test]
fn rpc_context_modules_can_register_subscriptions() {
	let cx = ();
	let mut cxmodule = RpcModule::new(cx);
	cxmodule.register_subscription("hi", "hi", "goodbye", |_, _, _| Ok(())).unwrap();

	assert!(cxmodule.method("hi").is_some());
	assert!(cxmodule.method("goodbye").is_some());
}

#[test]
fn rpc_register_alias() {
	let mut module = RpcModule::new(());

	module.register_method("hello_world", |_: Params, _| Ok(())).unwrap();
	module.register_alias("hello_foobar", "hello_world").unwrap();

	assert!(module.method("hello_world").is_some());
	assert!(module.method("hello_foobar").is_some());
}

#[tokio::test]
async fn calling_method_without_server() {
	// Call sync method with no params
	let mut module = RpcModule::new(());
	module.register_method("boo", |_: Params, _| Ok(String::from("boo!"))).unwrap();

	let res: String = module.call("boo", EmptyParams::new()).await.unwrap();
	assert_eq!(&res, "boo!");

	// Call sync method with params
	module
		.register_method("foo", |params, _| {
			let n: u16 = params.one()?;
			Ok(n * 2)
		})
		.unwrap();
	let res: u64 = module.call("foo", [3_u64]).await.unwrap();
	assert_eq!(res, 6);

	// Call sync method with bad param
	let err = module.call::<_, ()>("foo", (false,)).await.unwrap_err();
	assert!(matches!(
		err,
		Error::Call(CallError::Custom(err)) if err.code() == -32602 && err.message() == "invalid type: boolean `false`, expected u16 at line 1 column 6"
	));

	// Call async method with params and context
	struct MyContext;
	impl MyContext {
		fn roo(&self, things: Vec<u8>) -> u16 {
			things.iter().sum::<u8>().into()
		}
	}
	let mut module = RpcModule::new(MyContext);
	module
		.register_async_method("roo", |params, ctx| {
			let ns: Vec<u8> = params.parse().expect("valid params please");
			async move { Ok(ctx.roo(ns)) }
		})
		.unwrap();
	let res: u64 = module.call("roo", [12, 13]).await.unwrap();
	assert_eq!(res, 25);
}

#[tokio::test]
async fn calling_method_without_server_using_proc_macro() {
	use jsonrpsee::{core::async_trait, proc_macros::rpc};
	// Setup
	#[derive(Debug, Deserialize, Serialize)]
	#[allow(unreachable_pub)]
	pub struct Gun {
		shoots: bool,
	}

	#[derive(Debug, Deserialize, Serialize)]
	#[allow(unreachable_pub)]
	pub struct Beverage {
		ice: bool,
	}

	#[rpc(server)]
	pub trait Cool {
		/// Sync method, no params.
		#[method(name = "rebel_without_cause")]
		fn rebel_without_cause(&self) -> Result<bool, Error>;

		/// Sync method.
		#[method(name = "rebel")]
		fn rebel(&self, gun: Gun, map: HashMap<u8, u8>) -> Result<String, Error>;

		/// Async method.
		#[method(name = "revolution")]
		async fn can_have_any_name(&self, beverage: Beverage, some_bytes: Vec<u8>) -> Result<String, Error>;
	}

	struct CoolServerImpl;

	#[async_trait]
	impl CoolServer for CoolServerImpl {
		fn rebel_without_cause(&self) -> Result<bool, Error> {
			Ok(false)
		}

		fn rebel(&self, gun: Gun, map: HashMap<u8, u8>) -> Result<String, Error> {
			Ok(format!("{} {:?}", map.values().len(), gun))
		}

		async fn can_have_any_name(&self, beverage: Beverage, some_bytes: Vec<u8>) -> Result<String, Error> {
			Ok(format!("drink: {:?}, phases: {:?}", beverage, some_bytes))
		}
	}
	let module = CoolServerImpl.into_rpc();

	// Call sync method with no params
	let res: bool = module.call("rebel_without_cause", EmptyParams::new()).await.unwrap();
	assert!(!res);

	// Call sync method with params
	let res: String = module.call("rebel", (Gun { shoots: true }, HashMap::<u8, u8>::default())).await.unwrap();
	assert_eq!(&res, "0 Gun { shoots: true }");

	// Call sync method with bad params
	let err = module.call::<_, ()>("rebel", (Gun { shoots: true }, false)).await.unwrap_err();
	assert!(matches!(err,
		Error::Call(CallError::Custom(err)) if err.code() == -32602 && err.message() == "invalid type: boolean `false`, expected a map at line 1 column 5"
	));

	// Call async method with params and context
	let result: String = module.call("revolution", (Beverage { ice: true }, vec![1, 2, 3])).await.unwrap();
	assert_eq!(&result, "drink: Beverage { ice: true }, phases: [1, 2, 3]");
}

#[tokio::test]
async fn subscribing_without_server() {
	let mut module = RpcModule::new(());
	module
		.register_subscription("my_sub", "my_sub", "my_unsub", |_, mut sink, _| {
			sink.accept()?;

			let mut stream_data = vec!['0', '1', '2'];
			std::thread::spawn(move || {
				while let Some(letter) = stream_data.pop() {
					tracing::debug!("This is your friendly subscription sending data.");
					let _ = sink.send(&letter);
					std::thread::sleep(std::time::Duration::from_millis(500));
				}
				let close = ErrorObject::borrowed(0, &"closed successfully", None);
				sink.close(close.into_owned());
			});
			Ok(())
		})
		.unwrap();

	let mut my_sub = module.subscribe("my_sub", EmptyParams::new()).await.unwrap();
	for i in (0..=2).rev() {
		let (val, id) = my_sub.next::<char>().await.unwrap().unwrap();
		assert_eq!(val, std::char::from_digit(i, 10).unwrap());
		assert_eq!(&id, my_sub.subscription_id());
	}

	assert!(matches!(my_sub.next::<char>().await, None));
}

#[tokio::test]
async fn close_test_subscribing_without_server() {
	tracing_subscriber::FmtSubscriber::builder()
		.with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
		.try_init()
		.expect("setting default subscriber failed");

	let mut module = RpcModule::new(());
	module
		.register_subscription("my_sub", "my_sub", "my_unsub", |_, mut sink, _| {
			sink.accept()?;

			std::thread::spawn(move || {
				// make sure to only send one item
				sink.send(&"lo").unwrap();
				while !sink.is_closed() {
					tracing::debug!("[test] Sink is open, sleeping");
					std::thread::sleep(std::time::Duration::from_millis(500));
				}
				// Get the close reason.
				if !sink.send(&"lo").expect("str serializable; qed") {
					sink.close(SubscriptionClosed::RemotePeerAborted);
				}
			});
			Ok(())
		})
		.unwrap();

	let mut my_sub = module.subscribe("my_sub", EmptyParams::new()).await.unwrap();
	let (val, id) = my_sub.next::<String>().await.unwrap().unwrap();
	assert_eq!(&val, "lo");
	assert_eq!(&id, my_sub.subscription_id());
	let mut my_sub2 = std::mem::ManuallyDrop::new(module.subscribe("my_sub", EmptyParams::new()).await.unwrap());

	// Close the subscription to ensure it doesn't return any items.
	my_sub.close();
	tracing::info!("[test] closed first sub");

	// The first subscription was not closed using the unsubscribe method and
	// it will be treated as the connection was closed.
	assert!(matches!(my_sub.next::<String>().await, None));

	// The second subscription still works
	let (val, _) = my_sub2.next::<String>().await.unwrap().unwrap();
	assert_eq!(val, "lo".to_string());
	// Simulate a rude client that disconnects suddenly.
	unsafe {
		std::mem::ManuallyDrop::drop(&mut my_sub2);
	}

	assert!(matches!(my_sub.next::<String>().await, None));
}

#[tokio::test]
async fn subscribing_without_server_bad_params() {
	let mut module = RpcModule::new(());
	module
		.register_subscription("my_sub", "my_sub", "my_unsub", |params, mut sink, _| {
			let p = match params.one::<String>() {
				Ok(p) => p,
				Err(e) => {
					let err: Error = e.into();
					let _ = sink.reject(err);
					return Ok(());
				}
			};

			sink.accept()?;
			sink.send(&p).unwrap();
			Ok(())
		})
		.unwrap();

	let sub = module.subscribe("my_sub", EmptyParams::new()).await.unwrap_err();

	assert!(
		matches!(sub, Error::Call(CallError::Custom(e)) if e.message().contains("invalid length 0, expected an array of length 1 at line 1 column 2") && e.code() == ErrorCode::InvalidParams.code())
	);
}

#[tokio::test]
async fn subscribe_unsubscribe_without_server() {
	let mut module = RpcModule::new(());
	module
		.register_subscription("my_sub", "my_sub", "my_unsub", |_, mut sink, _| {
			let interval = interval(Duration::from_millis(200));
			let stream = IntervalStream::new(interval).map(move |_| 1);

			tokio::spawn(async move {
				sink.pipe_from_stream(stream).await;
			});
			Ok(())
		})
		.unwrap();

	async fn subscribe_and_assert(module: &RpcModule<()>) {
		let sub = module.subscribe("my_sub", EmptyParams::new()).await.unwrap();

		let ser_id = serde_json::to_string(sub.subscription_id()).unwrap();

		// Unsubscribe should be valid.
		let unsub_req = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"my_unsub\",\"params\":[{}],\"id\":1}}", ser_id);
		let (response, _) = module.raw_json_request(&unsub_req).await.unwrap();

		assert_eq!(response, r#"{"jsonrpc":"2.0","result":true,"id":1}"#);

		// Unsubscribe already performed; should be error.
		let unsub_req = format!("{{\"jsonrpc\":\"2.0\",\"method\":\"my_unsub\",\"params\":[{}],\"id\":1}}", ser_id);
		let (response, _) = module.raw_json_request(&unsub_req).await.unwrap();

		assert_eq!(response, r#"{"jsonrpc":"2.0","result":false,"id":1}"#);
	}

	let sub1 = subscribe_and_assert(&module);
	let sub2 = subscribe_and_assert(&module);

	futures::future::join(sub1, sub2).await;
}

#[tokio::test]
async fn empty_subscription_without_server() {
	let mut module = RpcModule::new(());
	module
		.register_subscription("my_sub", "my_sub", "my_unsub", |_, mut _sink, _| {
			// Sink was never accepted or rejected. Expected to return `InvalidParams`.
			Ok(())
		})
		.unwrap();

	let sub_err = module.subscribe("my_sub", EmptyParams::new()).await.unwrap_err();

	assert!(
		matches!(sub_err, Error::Call(CallError::Custom(e)) if e.message().contains("Invalid params") && e.code() == ErrorCode::InvalidParams.code())
	);
}

#[tokio::test]
async fn rejected_subscription_without_server() {
	let mut module = RpcModule::new(());
	module
		.register_subscription("my_sub", "my_sub", "my_unsub", |_, mut sink, _| {
			let err = ErrorObject::borrowed(0, &"rejected", None);
			sink.reject(err.into_owned())?;
			Ok(())
		})
		.unwrap();

	let sub_err = module.subscribe("my_sub", EmptyParams::new()).await.unwrap_err();

	assert!(matches!(sub_err, Error::Call(CallError::Custom(e)) if e.message().contains("rejected") && e.code() == 0));
}
