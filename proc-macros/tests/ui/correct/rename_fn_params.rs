//! Example showing intermediate renaming of parameters in json produced by client and consumed by server.

use futures_util::future::BoxFuture;
use std::net::SocketAddr;

use jsonrpsee::core::{async_trait, RpcResult};
use jsonrpsee::http_client::*;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::server::{
	middleware::rpc::{RpcServiceBuilder, RpcServiceT},
	MethodResponse, ServerBuilder,
};
use jsonrpsee::types::Request;

#[rpc(client, server)]
pub trait Rpc {
	#[method(name = "renamed_params_array", param_kind = array)]
	async fn renamed_params_array(
		&self,
		#[argument(rename = "type")] r#type: u16,
		#[argument(rename = "camelCase")] camel_case: bool,
	) -> RpcResult<()>;

	#[method(name = "keys_like_values", param_kind = map)]
	async fn keys_like_values(
		&self,
		#[argument(rename = "type")] r#type: String,
		#[argument(rename = "camelCase")] camel_case: String,
		#[argument(rename = "const")] r#const: String,
		#[argument(rename = "as")] r#as: String,
	) -> RpcResult<()>;
}

pub struct RpcServerImpl;

#[async_trait]
impl RpcServer for RpcServerImpl {
	async fn renamed_params_array(&self, _type: u16, _camel_case: bool) -> RpcResult<()> {
		Ok(())
	}

	async fn keys_like_values(&self, _type: String, _camel_case: String, _const: String, _as: String) -> RpcResult<()> {
		Ok(())
	}
}

pub async fn server() -> SocketAddr {
	/// middleware that asserts correct renaming for Rpc trait method params
	#[derive(Clone)]
	struct RequestParamRenameCheck<S> {
		service: S,
	}

	impl<'a, S> RpcServiceT<'a> for RequestParamRenameCheck<S>
	where
		S: RpcServiceT<'a> + Send + Sync + Clone + 'static,
	{
		type Future = BoxFuture<'a, MethodResponse>;

		fn call(&self, req: Request<'a>) -> Self::Future {
			let service = self.service.clone();

			Box::pin(async move {
				// inspect request and panic if unexpected param names occur
				match req.method.as_ref() {
					"renamed_params_array" => {
						// normal behavior for arrays params
						let json: serde_json::Value = serde_json::to_value(req.params.as_ref().unwrap()).unwrap();
						let array = json.as_array().unwrap();
						let mut i = array.iter();

						assert!(i.next().unwrap().is_number());
						assert!(i.next().unwrap().is_boolean());
						assert!(i.next().is_none());
					}
					"keys_like_values" => {
						// renamed json keys
						// assert that all keys and values are equal strings

						let json: serde_json::Value = serde_json::to_value(req.params.as_ref().unwrap()).unwrap();
						let obj = json.as_object().unwrap();

						for (k, v) in obj {
							assert_eq!(k, v);
						}
					}
					_ => {}
				}

				service.call(req).await
			})
		}
	}

	let server = ServerBuilder::default()
		.set_rpc_middleware(RpcServiceBuilder::new().layer_fn(move |service| RequestParamRenameCheck { service }))
		.build("127.0.0.1:0")
		.await
		.unwrap();
	let addr = server.local_addr().unwrap();
	let server_handle = server.start(RpcServerImpl.into_rpc());

	tokio::spawn(server_handle.stopped());

	addr
}

#[tokio::main]
async fn main() {
	let server_addr = server().await;
	let server_url = format!("http://{}", server_addr);
	let client = HttpClientBuilder::default().build(&server_url).unwrap();

	client.renamed_params_array(u16::MAX, true).await.unwrap();
	client
		.keys_like_values(String::from("type"), String::from("camelCase"), String::from("const"), String::from("as"))
		.await
		.unwrap();
}
