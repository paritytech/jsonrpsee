use hyper::{client::HttpConnector, Client};
use jsonrpsee::{
	proc_macros::rpc,
	server::{
		middleware::request_middleware::{RequestMiddleware, RequestMiddlewareAction, RequestMiddlewareLayer},
		ServerBuilder,
	},
	types::ErrorObject,
};
use std::net::SocketAddr;

#[rpc(server)]
pub trait MyRpc {
	#[method(name = "sayHi")]
	fn say_hi(&self) -> Result<String, ErrorObject<'static>>;
}

pub struct MyRpc;

impl MyRpcServer for MyRpc {
	fn say_hi(&self) -> Result<String, ErrorObject<'static>> {
		Ok("hi".to_string())
	}
}

struct MyMiddleware;

impl RequestMiddleware for MyMiddleware {
	fn on_request(&self, req: hyper::Request<hyper::Body>) -> RequestMiddlewareAction {
		if req.uri() == "/foo" {
			RequestMiddlewareAction::Respond(Box::pin(async { Ok(hyper::Response::new(hyper::Body::from("bar"))) }))
		} else {
			// same as RequestMiddlewareAction::Proceed(req)
			req.into()
		}
	}
}

async fn request_foo_path(client: &Client<HttpConnector>, base_uri: &str) -> anyhow::Result<String> {
	let res =
		client.request(hyper::Request::builder().uri(format!("{base_uri}/foo")).body(hyper::Body::empty())?).await?;
	let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
	// Convert the body bytes to utf-8
	let body = String::from_utf8(body_bytes.to_vec()).unwrap();

	Ok(body)
}

async fn request_say_hi_method(client: &Client<HttpConnector>, base_uri: &str) -> anyhow::Result<String> {
	let res = client
		.request(
			hyper::Request::builder()
				.method("POST")
				.uri(base_uri)
				.header("Content-Type", "application/json")
				.body(hyper::Body::from(r#"{"jsonrpc":"2.0","id":1,"method":"sayHi"}"#))?,
		)
		.await?;

	let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
	// Convert the body bytes to utf-8
	let body = String::from_utf8(body_bytes.to_vec()).unwrap();

	Ok(body)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let service_builder = tower::ServiceBuilder::new().layer(RequestMiddlewareLayer::new(MyMiddleware));

	let server = ServerBuilder::new()
		.set_middleware(service_builder)
		.build("127.0.0.1:0".parse::<SocketAddr>().unwrap())
		.await
		.unwrap();

	let base_uri = format!("http://{}", server.local_addr().unwrap());

	let _server = server.start(MyRpc.into_rpc()).unwrap();

	let client = hyper::Client::new();

	assert_eq!("bar", &request_foo_path(&client, &base_uri).await?);
	assert_eq!(r#"{"jsonrpc":"2.0","result":"hi","id":1}"#, &request_say_hi_method(&client, &base_uri).await?);

	Ok(())
}
