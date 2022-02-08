use jsonrpsee_client_transport::web_sys::*;
use jsonrpsee_core::client::{TransportReceiverT, TransportSenderT};
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn wasm_ws_transport_works() {
	let (mut tx, mut rx) = build_transport("wss://kusama-rpc.polkadot.io").await.unwrap();

	let req = r#"{"jsonrpc": "2.0", "method": "system_name", "id": 1}"#;
	let exp = r#"{"jsonrpc":"2.0","result":"Parity Polkadot","id":1}"#;

	tx.send(req.to_string()).await.unwrap();
	let rp = rx.receive().await.unwrap();

	assert_eq!(exp, &rp);
}
