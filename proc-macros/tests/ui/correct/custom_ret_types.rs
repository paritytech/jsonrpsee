//! Example of using custom response type.

use std::net::SocketAddr;

use jsonrpsee::core::{async_trait, RpcResult, Serialize};
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::{IntoResponse, ServerBuilder};
use jsonrpsee::types::PartialResponse;
use jsonrpsee::ws_client::*;

// This serialize impl is not used as the responses are sent out as error's.
#[derive(Serialize)]
pub enum CustomError {
	One,
	Two { custom_data: u32 },
}

impl IntoResponse for CustomError {
	type Output = Self;

	fn into_response(self) -> PartialResponse<Self::Output> {
		let code = match &self {
			CustomError::One => 101,
			CustomError::Two { .. } => 102,
		};
		let data = match &self {
			CustomError::One => None,
			CustomError::Two { custom_data } => Some(serde_json::json!({ "customData": custom_data })),
		};

		let data = data.map(|val| serde_json::value::to_raw_value(&val).unwrap());

		let error_object = jsonrpsee::types::ErrorObjectOwned::owned(code, "custom_error", data);
		PartialResponse::Error(error_object)
	}
}

#[rpc(server, namespace = "foo")]
pub trait Rpc {
	#[method(name = "method1")]
	async fn method1(&self) -> CustomError;

	#[method(name = "method2")]
	async fn method2(&self) -> CustomError;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn method1(&self) -> CustomError {
		CustomError::One
	}

	async fn method2(&self) -> CustomError {
		CustomError::Two { custom_data: 123 }
	}
}

// TODO: https://github.com/paritytech/jsonrpsee/issues/1067
//
// The client accepts only return types that are `Result<T, E>`.
#[rpc(client, namespace = "foo")]
pub trait RpcClient {
	#[method(name = "method1")]
	async fn client_method1(&self) -> RpcResult<serde_json::Value>;

	#[method(name = "method2")]
	async fn client_method2(&self) -> Result<serde_json::Value, ()>;
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

	let error = client.client_method1().await.unwrap_err();
	let error_object = get_error_object(error);
	assert_eq!(error_object.code(), 101);
	assert_eq!(error_object.message(), "custom_error");
	assert!(error_object.data().is_none());

	let error = client.client_method2().await.unwrap_err();
	let error_object = get_error_object(error);
	assert_eq!(error_object.code(), 102);
	assert_eq!(error_object.message(), "custom_error");
	assert_eq!(error_object.data().unwrap().get(), r#"{"customData":123}"#);
}
