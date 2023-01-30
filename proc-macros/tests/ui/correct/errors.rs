//! Example of using custom errors.

use std::net::SocketAddr;

use jsonrpsee::core::async_trait;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::ServerBuilder;
use jsonrpsee::ws_client::*;

pub enum CustomError {
	One,
	Two { custom_data: u32 },
}

impl From<CustomError> for jsonrpsee::core::Error {
	fn from(err: CustomError) -> Self {
		let code = match &err {
			CustomError::One => 101,
			CustomError::Two { .. } => 102,
		};
		let data = match &err {
			CustomError::One => None,
			CustomError::Two { custom_data } => Some(serde_json::json!({ "customData": custom_data })),
		};

		let data = data.map(|val| serde_json::value::to_raw_value(&val).unwrap());

		let error_object = jsonrpsee::types::ErrorObjectOwned::owned(code, "custom_error", data);

		Self::Call(jsonrpsee::types::error::CallError::Custom(error_object))
	}
}

#[rpc(client, server, namespace = "foo")]
pub trait Rpc {
	#[method(name = "method1")]
	async fn method1(&self) -> Result<u16, CustomError>;

	#[method(name = "method2")]
	async fn method2(&self) -> Result<u16, CustomError>;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn method1(&self) -> Result<u16, CustomError> {
		Err(CustomError::One)
	}

	async fn method2(&self) -> Result<u16, CustomError> {
		Err(CustomError::Two { custom_data: 123 })
	}
}

pub async fn server() -> SocketAddr {
	let server = ServerBuilder::default().build("127.0.0.1:0").await.unwrap();
	let addr = server.local_addr().unwrap();
	let server_handle = server.start(RpcServerImpl.into_rpc()).unwrap();

	tokio::spawn(server_handle.stopped());

	addr
}

#[tokio::main]
async fn main() {
	let server_addr = server().await;
	let server_url = format!("ws://{}", server_addr);
	let client = WsClientBuilder::default().build(&server_url).await.unwrap();

	let get_error_object = |err| match err {
		jsonrpsee::core::Error::Call(jsonrpsee::types::error::CallError::Custom(object)) => object,
		_ => panic!("wrong error kind: {:?}", err),
	};

	let error = client.method1().await.unwrap_err();
	let error_object = get_error_object(error);
	assert_eq!(error_object.code(), 101);
	assert_eq!(error_object.message(), "custom_error");
	assert!(error_object.data().is_none());

	let error = client.method2().await.unwrap_err();
	let error_object = get_error_object(error);
	assert_eq!(error_object.code(), 102);
	assert_eq!(error_object.message(), "custom_error");
	assert_eq!(error_object.data().unwrap().get(), r#"{"customData":123}"#);
}
