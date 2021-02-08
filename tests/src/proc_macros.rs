use crate::helpers;
use futures::channel::oneshot;
use jsonrpsee_http_client::*;
use jsonrpsee_ws_client::*;
use std::net::SocketAddr;

jsonrpsee_proc_macros::rpc_client_api! {
	Test<T> {
		fn say_hello() -> T;
	}
}

jsonrpsee_proc_macros::rpc_client_api! {
	Test2<B, T> {
		#[rpc(method = "say_hello")]
		fn foo(b: B) -> T;
	}
}

jsonrpsee_proc_macros::rpc_client_api! {
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
jsonrpsee_proc_macros::rpc_client_api! {
	Registrar {
		#[rpc(method = "say_hello")]
		fn register_para(foo: i32, bar: String);
	}
}

#[tokio::test]
async fn proc_macros_generic_ws_client_api() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	helpers::websocket_server(server_started_tx);
	let server_addr = server_started_rx.await.unwrap();
	let server_url = format!("ws://{}", server_addr);
	let client = WsClient::new(WsConfig::with_url(&server_url)).await.unwrap();

	assert_eq!(Test::<String>::say_hello(&client).await.unwrap(), "hello".to_string());
	assert_eq!(Test2::<u16, String>::foo(&client, 99_u16).await.unwrap(), "hello".to_string());
	assert!(Registrar::register_para(&client, 99, "para").await.is_ok());
}

#[tokio::test]
async fn proc_macros_generic_http_client_api() {
	let (server_started_tx, server_started_rx) = oneshot::channel::<SocketAddr>();
	helpers::http_server(server_started_tx);
	let server_addr = server_started_rx.await.unwrap();
	let server_url = format!("http://{}", server_addr);
	let client = HttpClient::new(&server_url, HttpConfig::default()).unwrap();

	assert_eq!(Test::<String>::say_hello(&client).await.unwrap(), "hello".to_string());
	assert_eq!(Test2::<u16, String>::foo(&client, 99_u16).await.unwrap(), "hello".to_string());
	// TODO: check why this fails.
	//assert!(Registrar::register_para(&client, 99, "para").await.is_ok());
}
