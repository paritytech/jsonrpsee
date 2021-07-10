// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

// Not all proc macros are used let's suppress it for now.
#![allow(dead_code)]

mod helpers;

use jsonrpsee::{http_client::*, proc_macros, ws_client::*};

proc_macros::rpc_client_api! {
	Test<T: Send + Sync> {
		fn say_hello() -> T;
	}
}

proc_macros::rpc_client_api! {
	pub(crate) Test2<B: Send + Sync, T: Send + Sync> {
		#[rpc(method = "say_hello")]
		fn foo(b: B) -> T;
	}
}

proc_macros::rpc_client_api! {
	Author {
		#[rpc(method = "author_submitExtrinsic", positional_params)]
		fn submit_extrinsic(extrinsic: String) -> u128;
	}

	Chain {
		#[rpc(method = "chain_getFinalizedHead")]
		fn current_block_hash() -> u128;

		#[rpc(method = "chain_getHeader", positional_params)]
		fn header(hash: u128) -> Option<u128>;

		#[rpc(method = "chain_getBlockHash", positional_params)]
		fn block_hash(hash: Option<u64>) -> Option<u128>;
	}

	State {
		#[rpc(method = "state_getRuntimeVersion")]
		fn runtime_version() -> u128;
	}
}

// https://github.com/paritytech/jsonrpsee/issues/104
proc_macros::rpc_client_api! {
	Registrar {
		#[rpc(method = "say_hello")]
		fn register_para(foo: i32, bar: String);
	}
}

proc_macros::rpc_client_api! {
	ManyReturnTypes<A: Send + Sync, B: Send + Sync, C: Send + Sync, D: Send + Sync, E: Send + Sync> {
		#[rpc(method = "say_hello")]
		fn a() -> A;
		fn b() -> B;
		fn c() -> C;
		fn d() -> D;
		fn e() -> E;
	}
}

#[tokio::test]
async fn proc_macros_generic_ws_client_api() {
	let server_addr = helpers::websocket_server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	assert_eq!(Test::<String>::say_hello(&client).await.unwrap(), "hello".to_string());
	assert_eq!(Test2::<u16, String>::foo(&client, 99_u16).await.unwrap(), "hello".to_string());
	assert!(Registrar::register_para(&client, 99, "para").await.is_ok());
}

#[tokio::test]
async fn proc_macros_generic_http_client_api() {
	let server_addr = helpers::http_server().await;
	let server_url = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&server_url).unwrap();

	assert_eq!(Test::<String>::say_hello(&client).await.unwrap(), "hello".to_string());
	assert_eq!(Test2::<u16, String>::foo(&client, 99_u16).await.unwrap(), "hello".to_string());
	assert!(Registrar::register_para(&client, 99, "para").await.is_ok());
}
