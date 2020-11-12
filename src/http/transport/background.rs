// Copyright 2019 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::http::server_utils::access_control::AccessControl;
use crate::http::transport::response;
use crate::types::{jsonrpc, http::HttpConfig};
use futures::{channel::mpsc, channel::oneshot, prelude::*};
use hyper::service::{make_service_fn, service_fn};
use hyper::Error;
use std::{error, io, net::SocketAddr, thread};

/// Background thread that serves HTTP requests.
pub(super) struct BackgroundHttp {
	/// Receiver for requests coming from the background thread.
	rx: stream::Fuse<mpsc::Receiver<Request>>,
}

/// Request generated from the background thread.
pub(super) struct Request {
	/// Sender for the body of the response to send on the network.
	pub send_back: oneshot::Sender<hyper::Response<hyper::Body>>,
	/// The JSON body that was sent by the client.
	pub request: jsonrpc::Request,
}

impl BackgroundHttp {
	/// Tries to create an HTTP server listening on the given address and start a background
	/// thread.
	///
	/// In addition to `Self`, also returns the local address the server ends up listening on,
	/// which might be different than the one passed as parameter.
	pub async fn bind(addr: &SocketAddr, config: HttpConfig) -> Result<(BackgroundHttp, SocketAddr), Box<dyn error::Error + Send + Sync>> {
		Self::bind_with_acl(addr, AccessControl::default(), config).await
	}

	pub async fn bind_with_acl(
		addr: &SocketAddr,
		access_control: AccessControl,
		config: HttpConfig,
	) -> Result<(BackgroundHttp, SocketAddr), Box<dyn error::Error + Send + Sync>> {
		let (tx, rx) = mpsc::channel(4);

		let make_service = make_service_fn(move |_| {
			let tx = tx.clone();
			let access_control = access_control.clone();
			async move {
				Ok::<_, Error>(service_fn(move |req| {
					let mut tx = tx.clone();
					let access_control = access_control.clone();
					async move { Ok::<_, Error>(process_request(req, &mut tx, &access_control, config).await) }
				}))
			}
		});

		let (addr_tx, addr_rx) = oneshot::channel();
		let addr = *addr;

		// Because hyper can only be polled through tokio, we spawn it in a background thread.
		thread::Builder::new().name("jsonrpsee-hyper-server".to_string()).spawn(move || {
			let mut runtime = match tokio::runtime::Builder::new().basic_scheduler().enable_all().build() {
				Ok(r) => r,
				Err(err) => {
					log::error!("Failed to initialize tokio runtime in HTTP JSON-RPC server: {}", err);
					return;
				}
			};

			runtime.block_on(async move {
				match hyper::Server::try_bind(&addr) {
					Ok(builder) => {
						let server = builder.serve(make_service);
						let _ = addr_tx.send(Ok(server.local_addr()));
						if let Err(err) = server.await {
							log::error!("HTTP JSON-RPC server closed with an error: {}", err);
						}
					}
					Err(err) => {
						log::error!("Failed to bind to address {}: {}", addr, err);
						let _ = addr_tx.send(Err(err));
					}
				};
			});
		})?;

		let local_addr = addr_rx.await??;
		Ok((BackgroundHttp { rx: rx.fuse() }, local_addr))
	}

	/// Returns the next request, or an error if the background thread has unexpectedly closed.
	pub async fn next(&mut self) -> Result<Request, ()> {
		self.rx.next().await.ok_or(())
	}
}

/// Process an HTTP request and sends back a response.
///
/// This function is the main method invoked whenever we receive an HTTP request.
///
/// In order to process JSON-RPC requests, it has access to `fg_process_tx`. Objects sent on this
/// channel will be dispatched to the user.
async fn process_request(
	request: hyper::Request<hyper::Body>,
	fg_process_tx: &mut mpsc::Sender<Request>,
	access_control: &AccessControl,
	config: HttpConfig,
) -> hyper::Response<hyper::Body> {
	// Process access control
	if access_control.deny_host(&request) {
		return response::host_not_allowed();
	}
	if access_control.deny_cors_origin(&request) {
		return response::invalid_allow_origin();
	}
	if access_control.deny_cors_header(&request) {
		return response::invalid_allow_headers();
	}

	// Proceed
	match *request.method() {
		// Validate the ContentType header
		// to prevent Cross-Origin XHRs with text/plain
		hyper::Method::POST if is_json(request.headers().get("content-type")) => {
			let json_body = match body_to_request(request.into_body(), config).await {
				Ok(b) => b,
				Err(e) => match (e.kind(), e.into_inner()) {
					(io::ErrorKind::InvalidData, _) => return response::parse_error(),
					(io::ErrorKind::UnexpectedEof, _) => return response::parse_error(),
					(_, Some(inner)) => return response::internal_error(inner.to_string()),
					(kind, None) => return response::internal_error(format!("{:?}", kind)),
				},
			};

			let (tx, rx) = oneshot::channel();
			log::debug!("recv: {}", jsonrpc::to_string(&json_body).unwrap());
			let user_facing_rq = Request { send_back: tx, request: json_body };
			if fg_process_tx.send(user_facing_rq).await.is_err() {
				return response::internal_error("JSON requests processing channel has shut down");
			}
			match rx.await {
				Ok(response) => {
					log::debug!("send: {:?}", response.body());
					response
				}
				Err(_) => return response::internal_error("JSON request send back channel has shut down"),
			}
		}
		// Disallow other methods.
		_ => response::method_not_allowed(),
	}
}

/// Returns true if the `content_type` header indicates a valid JSON message.
fn is_json(content_type: Option<&hyper::header::HeaderValue>) -> bool {
	match content_type.and_then(|val| val.to_str().ok()) {
		Some(ref content)
			if content.eq_ignore_ascii_case("application/json")
				|| content.eq_ignore_ascii_case("application/json; charset=utf-8")
				|| content.eq_ignore_ascii_case("application/json;charset=utf-8") =>
		{
			true
		}
		_ => false,
	}
}

/// Converts a `hyper` body into a structured JSON object.
///
/// Enforces a size limit on the body.
async fn body_to_request(mut body: hyper::Body, config: HttpConfig) -> Result<jsonrpc::Request, io::Error> {
	let mut json_body = Vec::new();
	while let Some(chunk) = body.next().await {
		let chunk = match chunk {
			Ok(c) => c,
			Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string())), // TODO:
		};
		json_body.extend_from_slice(&chunk);
		if json_body.len() >= 16384 {
			// TODO: some limit
			return Err(io::Error::new(io::ErrorKind::Other, "request too large"));
		}
	}

	Ok(serde_json::from_slice(&json_body)?)
}

#[cfg(test)]
mod tests {
	use super::body_to_request;

	#[test]
	fn body_to_request_works() {
		let s = r#"[{"a":"hello"}]"#;
		let expected: super::jsonrpc::Request = serde_json::from_str(s).unwrap();
		let req = futures::executor::block_on(async move {
			let body = hyper::Body::from(s);
			body_to_request(body).await.unwrap()
		});
		assert_eq!(req, expected);
	}

	#[test]
	fn body_to_request_size_limit_json() {
		let huge_body =
			serde_json::to_vec(&(0..32768).map(|_| serde_json::Value::from("test")).collect::<Vec<_>>()).unwrap();

		futures::executor::block_on(async move {
			let body = hyper::Body::from(huge_body);
			assert!(body_to_request(body).await.is_err());
		});
	}

	#[test]
	fn body_to_request_size_limit_garbage() {
		let huge_body = (0..100_000).map(|_| rand::random::<u8>()).collect::<Vec<_>>();
		futures::executor::block_on(async move {
			let body = hyper::Body::from(huge_body);
			assert!(body_to_request(body).await.is_err());
		});
	}
}
