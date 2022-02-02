use jsonrpsee_client_transport::web_sys::*;
use jsonrpsee_core::client::{TransportReceiverT, TransportSenderT};
use wasm_bindgen_test::*;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn get() {
	let (mut tx, mut rx) = build_transport("ws://localhost:9999").await.unwrap();
	//let builder = ClientBuilder::default();
	//let client = builder.build(tx, rx);

	tx.send(r#"{"jsonrpc":2.0,"method":"system_name","id":1}"#.to_string()).await.unwrap();
	let rp = rx.receive().await.unwrap();

	assert_eq!("aa", rp);
}
