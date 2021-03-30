// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

use crate::module::RpcModule;
use crate::response;
use crate::HttpConfig;
use hyper::server::{conn::AddrIncoming, Builder as HyperBuilder};
use hyper::service::{make_service_fn, service_fn};
use hyper::Error as HyperError;
use jsonrpsee_types::error::{Error, GenericTransportError};
use jsonrpsee_types::jsonrpc_v2::{helpers::send_error, JsonRpcInvalidRequest, JsonRpcRequest, RpcError, RpcParams};
use jsonrpsee_utils::http::{access_control::AccessControl, hyper_helpers::read_response_to_body};
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct Server {
	/// Hyper server.
	listener: HyperBuilder<AddrIncoming>,
	/// Registered methods.
	root: RpcModule,
	/// Http settings.
	config: HttpConfig,
	/// Access control
	access_control: AccessControl,
}

impl Server {
	/// ...
	pub async fn new(addr: &SocketAddr, config: HttpConfig, access_control: AccessControl) -> anyhow::Result<Self> {
		// TODO: use create the TCP socket manually to more fine-grained settings.
		let listener = hyper::Server::try_bind(&addr)?.tcp_nodelay(true);
		Ok(Self { listener, root: RpcModule::new(), config, access_control })
	}

	/// Register a new RPC method, which responds with a given callback.
	pub fn register_method<F, R>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize,
		F: Fn(RpcParams) -> Result<R, RpcError> + Send + Sync + 'static,
	{
		self.root.register_method(method_name, callback)
	}

	/// Register all methods from a module on this server.
	pub fn register_module(&mut self, module: RpcModule) -> Result<(), Error> {
		self.root.merge(module)
	}

	/// Start responding to connections requests. This will block current thread until the server is stopped.
	pub async fn start(self) -> anyhow::Result<SocketAddr> {
		let methods = Arc::new(self.root.into_methods());
		let config = self.config;
		let access_control = self.access_control;

		let make_service = make_service_fn(move |_| {
			let methods = methods.clone();
			let access_control = access_control.clone();

			async move {
				Ok::<_, HyperError>(service_fn(move |request| {
					log::info!("{:?}", request);
					let methods = methods.clone();
					let access_control = access_control.clone();
					async move {
						if let Err(e) = access_control_is_valid(&access_control, &request) {
							return Ok::<_, HyperError>(e);
						}

						if let Err(e) = content_type_is_valid(&request) {
							return Ok::<_, HyperError>(e);
						}

						let (parts, body) = request.into_parts();
						let body = match read_response_to_body(&parts.headers, body, config).await {
							Ok(body) => body,
							Err(GenericTransportError::TooLarge) => {
								return Ok::<_, HyperError>(response::too_large("The request was too large"))
							}
							Err(GenericTransportError::Inner(e)) => {
								return Ok::<_, HyperError>(response::internal_error(e.to_string()))
							}
						};

						// TODO: oneshot would sufficient too.
						let (tx, mut rx) = mpsc::unbounded_channel();

						match serde_json::from_slice::<JsonRpcRequest>(&body) {
							Ok(req) => {
								log::info!("recv: {:?}", req);
								let params = RpcParams::new(req.params.map(|params| params.get()));
								if let Some(method) = methods.get(&*req.method) {
									(method)(req.id, params, &tx, 0).unwrap();
								} else {
									send_error(req.id, &tx, -32601, "Method not found");
								}
							}
							Err(_e) => {
								let (id, code, msg) = match serde_json::from_slice::<JsonRpcInvalidRequest>(&body) {
									Ok(req) => (req.id, -32600, "Invalid request"),
									Err(_) => (None, -32700, "Parse error"),
								};
								send_error(id, &tx, code, msg);
							}
						};

						let response = rx.recv().await.unwrap();
						log::info!("send: {:?}", response);
						Ok::<_, HyperError>(response::ok_response(response))
					}
				}))
			}
		});

		let server = self.listener.serve(make_service);
		let addr = server.local_addr();
		// Run server forever.
		tokio::spawn(async move { server.await.unwrap() });
		Ok(addr)
	}
}

fn access_control_is_valid(
	access_control: &AccessControl,
	request: &hyper::Request<hyper::Body>,
) -> Result<(), hyper::Response<hyper::Body>> {
	// Process access control
	if access_control.deny_host(request) {
		return Err(response::host_not_allowed());
	}
	if access_control.deny_cors_origin(request) {
		return Err(response::invalid_allow_origin());
	}
	if access_control.deny_cors_header(request) {
		return Err(response::invalid_allow_headers());
	}
	Ok(())
}

fn content_type_is_valid(request: &hyper::Request<hyper::Body>) -> Result<(), hyper::Response<hyper::Body>> {
	match *request.method() {
		hyper::Method::POST if is_json(request.headers().get("content-type")) => Ok(()),
		_ => Err(response::method_not_allowed()),
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
