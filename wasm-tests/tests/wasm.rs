use jsonrpsee_client_transport::web_sys::*;
use jsonrpsee_core::{
	client::{Client, ClientT, Subscription, SubscriptionClientT, TransportReceiverT, TransportSenderT},
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
	let (mut tx, mut rx) = build_transport("wss://kusama-rpc.polkadot.io").await.unwrap();

	let req = r#"{"jsonrpc": "2.0", "method": "system_name", "id": 1}"#;
	let exp = r#"{"jsonrpc":"2.0","result":"Parity Polkadot","id":1}"#;

	tx.send(req.to_string()).await.unwrap();
	let rp = rx.receive().await.unwrap();

	assert_eq!(exp, &rp);
}

#[wasm_bindgen_test]
async fn rpc_method_call_works() {
	let client: Client = build_transport("wss://kusama-rpc.polkadot.io").await.unwrap().into();

	let rp: String = client.request("system_name", rpc_params![]).await.unwrap();

	assert_eq!("Parity Polkadot", &rp);
}

#[wasm_bindgen_test]
async fn rpc_subcription_works() {
	let client: Client = build_transport("wss://kusama-rpc.polkadot.io").await.unwrap().into();

	let mut sub: Subscription<serde_json::Value> =
		client.subscribe("state_subscribeStorage", rpc_params![], "state_unsubscribeStorage").await.unwrap();

	for _ in 0..3 {
		let val = sub.next().await.unwrap();
		assert!(val.is_ok());
	}
}
