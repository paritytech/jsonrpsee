use std::time::Duration;

use jsonrpsee_client_transport::web_sys::*;
use jsonrpsee_core::{
	client::{Client, ClientBuilder, ClientT, Subscription, SubscriptionClientT, TransportReceiverT, TransportSenderT},
	rpc_params,
};
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// Run the tests by `$ wasm-pack test --firefox --headless`

fn init_tracing() {
	console_error_panic_hook::set_once();
	tracing_wasm::set_as_global_default();
}

#[wasm_bindgen_test]
async fn wasm_ws_transport_works() {
	init_tracing();
	let (mut tx, mut rx) = connect("ws://localhost:9944", Duration::from_secs(60)).await.unwrap();

	let req = r#"{"jsonrpc": "2.0", "method": "system_name", "id": 1}"#;
	let exp = r#"{"jsonrpc":"2.0","result":"Substrate Node","id":1}"#;

	tx.send(req.to_string()).await.unwrap();
	let rp = rx.receive().await.unwrap();

	assert_eq!(exp, &rp);
}

#[wasm_bindgen_test]
async fn rpc_method_call_works() {
	let (tx, rx) = connect("ws://localhost:9944", Duration::from_secs(60)).await.unwrap();
	let client: Client = ClientBuilder::default().build_with_wasm(tx, rx);

	let rp: String = client.request("system_name", rpc_params![]).await.unwrap();

	assert_eq!("Substrate Node", &rp);
}

#[wasm_bindgen_test]
async fn rpc_subcription_works() {
	let (tx, rx) = connect("ws://localhost:9944", Duration::from_secs(60)).await.unwrap();
	let client: Client = ClientBuilder::default().build_with_wasm(tx, rx);

	let mut sub: Subscription<serde_json::Value> =
		client.subscribe("state_subscribeStorage", rpc_params![], "state_unsubscribeStorage").await.unwrap();

	for _ in 0..3 {
		let val = sub.next().await.unwrap();
		assert!(val.is_ok());
	}
}
