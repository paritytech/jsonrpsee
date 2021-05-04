use crate::types::{Body, HttpResponse, Id, Uri};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Request, Response, Server};
use serde_json::Value;
use std::convert::Infallible;
use std::net::SocketAddr;

pub const PARSE_ERROR: &str = "Parse error";
pub const INTERNAL_ERROR: &str = "Internal error";
pub const INVALID_PARAMS: &str = "Invalid params";
pub const INVALID_REQUEST: &str = "Invalid request";
pub const METHOD_NOT_FOUND: &str = "Method not found";

/// Converts a sockaddress to a WebSocket URI.
pub fn to_ws_uri_string(addr: SocketAddr) -> String {
	let mut s = String::new();
	s.push_str("ws://");
	s.push_str(&addr.to_string());
	s
}

/// Converts a sockaddress to a HTTP URI.
pub fn to_http_uri(sockaddr: SocketAddr) -> Uri {
	let s = sockaddr.to_string();
	Uri::builder().scheme("http").authority(s.as_str()).path_and_query("/").build().unwrap()
}

pub fn ok_response(result: Value, id: Id) -> String {
	format!(r#"{{"jsonrpc":"2.0","result":{},"id":{}}}"#, result, serde_json::to_string(&id).unwrap())
}

pub fn method_not_found(id: Id) -> String {
	format!(
		r#"{{"jsonrpc":"2.0","error":{{"code":-32601,"message":"Method not found"}},"id":{}}}"#,
		serde_json::to_string(&id).unwrap()
	)
}

pub fn parse_error(id: Id) -> String {
	format!(
		r#"{{"jsonrpc":"2.0","error":{{"code":-32700,"message":"Parse error"}},"id":{}}}"#,
		serde_json::to_string(&id).unwrap()
	)
}

pub fn invalid_request(id: Id) -> String {
	format!(
		r#"{{"jsonrpc":"2.0","error":{{"code":-32600,"message":"Invalid request"}},"id":{}}}"#,
		serde_json::to_string(&id).unwrap()
	)
}

pub fn invalid_params(id: Id) -> String {
	format!(
		r#"{{"jsonrpc":"2.0","error":{{"code":-32602,"message":"Invalid params"}},"id":{}}}"#,
		serde_json::to_string(&id).unwrap()
	)
}

pub fn invalid_context(msg: &str, id: Id) -> String {
	format!(
		r#"{{"jsonrpc":"2.0","error":{{"code":-32000,"message":"{}"}},"id":{}}}"#,
		msg,
		serde_json::to_string(&id).unwrap()
	)
}

pub fn internal_error(id: Id) -> String {
	format!(
		r#"{{"jsonrpc":"2.0","error":{{"code":-32603,"message":"Internal error"}},"id":{}}}"#,
		serde_json::to_string(&id).unwrap()
	)
}

// TODO: remove?
pub fn server_error(id: Id) -> String {
	format!(
		r#"{{"jsonrpc":"2.0","error":{{"code":-1,"message":"Server error"}},"id":{}}}"#,
		serde_json::to_string(&id).unwrap()
	)
}

/// Hardcoded server response when a client initiates a new subscription.
///
/// NOTE: works only for one subscription because the subscription ID is hardcoded.
pub fn server_subscription_id_response(id: Id) -> String {
	format!(
		r#"{{"jsonrpc":"2.0","result":"D3wwzU6vvoUUYehv4qoFzq42DZnLoAETeFzeyk8swH4o","id":{}}}"#,
		serde_json::to_string(&id).unwrap()
	)
}

/// Server response to a hardcoded pending subscription
pub fn server_subscription_response(result: Value) -> String {
	format!(
		r#"{{"jsonrpc":"2.0","method":"bar","params":{{"subscription":"D3wwzU6vvoUUYehv4qoFzq42DZnLoAETeFzeyk8swH4o","result":{}}}}}"#,
		serde_json::to_string(&result).unwrap()
	)
}

pub async fn http_request(body: Body, uri: Uri) -> Result<HttpResponse, String> {
	let client = hyper::Client::new();
	let r = hyper::Request::post(uri)
		.header(hyper::header::CONTENT_TYPE, hyper::header::HeaderValue::from_static("application/json"))
		.body(body)
		.expect("uri and request headers are valid; qed");
	let res = client.request(r).await.map_err(|e| format!("{:?}", e))?;

	let (parts, body) = res.into_parts();
	let bytes = hyper::body::to_bytes(body).await.unwrap();

	Ok(HttpResponse { status: parts.status, header: parts.headers, body: String::from_utf8(bytes.to_vec()).unwrap() })
}

/// Spawn HTTP server that responds with a hardcoded response.
//
// NOTE: This must be spawned on tokio because hyper only works with tokio.
pub async fn http_server_with_hardcoded_response(response: String) -> SocketAddr {
	async fn process_request(_req: Request<Body>, response: String) -> Result<Response<Body>, Infallible> {
		Ok(Response::new(hyper::Body::from(response)))
	}

	let make_service = make_service_fn(move |_| {
		let response = response.clone();
		async move {
			Ok::<_, Infallible>(service_fn(move |req| {
				let response = response.clone();
				async move { Ok::<_, Infallible>(process_request(req, response).await.unwrap()) }
			}))
		}
	});

	let (tx, rx) = futures_channel::oneshot::channel::<SocketAddr>();

	tokio::spawn(async {
		let addr = SocketAddr::from(([127, 0, 0, 1], 0));
		let server = Server::bind(&addr).serve(make_service);
		tx.send(server.local_addr()).unwrap();
		server.await.unwrap()
	});

	rx.await.unwrap()
}
