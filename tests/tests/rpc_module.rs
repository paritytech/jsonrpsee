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

use jsonrpsee::core::server::rpc_module::*;
use jsonrpsee::core::Error;
use jsonrpsee::types::{EmptyParams, Params};
use serde::{Deserialize, Serialize};

fn assert_type<T: 'static, Expected: 'static>(_expected: &Expected, explain: Option<&'static str>) {
	use core::any::TypeId;
	match explain {
		None => assert_eq!(TypeId::of::<T>(), TypeId::of::<Expected>()),
		Some(explain) => assert_eq!(TypeId::of::<T>(), TypeId::of::<Expected>(), "{}", explain),
	}
}
// Helper macro to assert that a binding is of a specific type.
macro_rules! assert_type {
    ( $ty:ty, $expected:expr $(,)?) => {{
        assert_type::<$ty, _>($expected, None)
    }};
    ( $ty:ty, $expected:expr, $($arg:tt)+ ) => {{
        assert_type::<$ty, _>($expected, Some($($arg)+))
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
	assert_type!(RpcModule<String>, &mod1, "Expected an RpcModule with String context.");
	let unit_mod = mod1.remove_context();
	assert_type!(RpcModule<()>, &unit_mod, "Expected an RpcModule with unit context.");
}

#[test]
fn rpc_context_modules_can_register_subscriptions() {
	let cx = ();
	let mut cxmodule = RpcModule::new(cx);
	let _subscription = cxmodule.register_subscription("hi", "hi", "goodbye", |_, _, _| Ok(()));

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
	assert!(
		matches!(err, Error::Request(err) if err == r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"invalid type: boolean `false`, expected u16 at line 1 column 6"},"id":0}"#)
	);

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
	assert!(matches!(
		err,
		Error::Request(err) if err == r#"{"jsonrpc":"2.0","error":{"code":-32602,"message":"invalid type: boolean `false`, expected a map at line 1 column 5"},"id":0}"#
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
			let mut stream_data = vec!['0', '1', '2'];
			std::thread::spawn(move || {
				while let Some(letter) = stream_data.pop() {
					tracing::debug!("This is your friendly subscription sending data.");
					if let Err(Error::SubscriptionClosed(_)) = sink.send(&letter) {
						return;
					}
					std::thread::sleep(std::time::Duration::from_millis(500));
				}
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

	let sub_err = my_sub.next::<char>().await.unwrap().unwrap_err();

	// The subscription is now closed by the server.
	assert!(matches!(sub_err, Error::SubscriptionClosed(_)));
}

#[tokio::test]
async fn close_test_subscribing_without_server() {
	let mut module = RpcModule::new(());
	module
		.register_subscription("my_sub", "my_sub", "my_unsub", |_, mut sink, _| {
			std::thread::spawn(move || loop {
				if let Err(Error::SubscriptionClosed(_)) = sink.send(&"lo") {
					return;
				}
				std::thread::sleep(std::time::Duration::from_millis(500));
			});
			Ok(())
		})
		.unwrap();

	let mut my_sub = module.subscribe("my_sub", EmptyParams::new()).await.unwrap();
	let (val, id) = my_sub.next::<String>().await.unwrap().unwrap();
	assert_eq!(&val, "lo");
	assert_eq!(&id, my_sub.subscription_id());

	// close the subscription to ensure it doesn't return any items.
	my_sub.close();
	assert!(matches!(my_sub.next::<String>().await, None));
}
