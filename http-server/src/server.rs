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
use crate::AccessControl;
use anyhow::anyhow;
use futures_channel::mpsc;
use futures_util::stream::StreamExt;
use hyper::{
	server::{conn::AddrIncoming, Builder as HyperBuilder},
	service::{make_service_fn, service_fn},
	Error as HyperError,
};
use jsonrpsee_types::error::{Error, GenericTransportError, RpcError};
use jsonrpsee_types::v2::request::{JsonRpcInvalidRequest, JsonRpcRequest};
use jsonrpsee_types::v2::{
	error::{JsonRpcErrorCode, JsonRpcErrorObject},
	params::RpcParams,
};
use jsonrpsee_utils::{hyper_helpers::read_response_to_body, server::send_error};
use serde::Serialize;
use socket2::{Domain, Socket, Type};
use std::{
	net::{SocketAddr, TcpListener},
	sync::Arc,
};

/// Builder to create JSON-RPC HTTP server.
pub struct Builder {
	access_control: AccessControl,
	max_request_body_size: u32,
	keep_alive: bool,
}

impl Builder {
	/// Sets the maximum size of a request body in bytes (default is 10 MiB).
	pub fn max_request_body_size(mut self, size: u32) -> Self {
		self.max_request_body_size = size;
		self
	}

	/// Sets access control settings.
	pub fn set_access_control(mut self, acl: AccessControl) -> Self {
		self.access_control = acl;
		self
	}

	/// Enables or disables HTTP keep-alive.
	///
	/// Default is true.
	pub fn keep_alive(mut self, keep_alive: bool) -> Self {
		self.keep_alive = keep_alive;
		self
	}

	pub fn build(self, addr: SocketAddr) -> anyhow::Result<Server> {
		let domain = Domain::for_address(addr);
		let socket = Socket::new(domain, Type::STREAM, None)?;
		socket.set_nodelay(true)?;
		socket.set_reuse_address(true)?;
		socket.set_nonblocking(true)?;
		socket.set_keepalive(self.keep_alive)?;
		let address = addr.into();
		socket.bind(&address)?;

		socket.listen(128)?;
		let listener: TcpListener = socket.into();
		let local_addr = listener.local_addr().ok();

		let listener = hyper::Server::from_tcp(listener)?;
		Ok(Server {
			listener,
			local_addr,
			root: RpcModule::new(),
			access_control: self.access_control,
			max_request_body_size: self.max_request_body_size,
		})
	}
}

impl Default for Builder {
	fn default() -> Self {
		Self { max_request_body_size: 10 * 1024 * 1024, access_control: AccessControl::default(), keep_alive: true }
	}
}

pub struct Server {
	/// Hyper server.
	listener: HyperBuilder<AddrIncoming>,
	/// Local address
	local_addr: Option<SocketAddr>,
	/// Registered methods.
	root: RpcModule,
	/// Max request body size.
	max_request_body_size: u32,
	/// Access control
	access_control: AccessControl,
}

impl Server {
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

	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> anyhow::Result<SocketAddr> {
		self.local_addr.ok_or_else(|| anyhow!("Local address not found"))
	}

	/// Start the server.
	pub async fn start(self) -> anyhow::Result<()> {
		let methods = Arc::new(self.root.into_methods());
		let max_request_body_size = self.max_request_body_size;
		let access_control = self.access_control;

		let make_service = make_service_fn(move |_| {
			let methods = methods.clone();
			let access_control = access_control.clone();

			async move {
				Ok::<_, HyperError>(service_fn(move |request| {
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
						let body = match read_response_to_body(&parts.headers, body, max_request_body_size).await {
							Ok(body) => body,
							Err(GenericTransportError::TooLarge) => {
								return Ok::<_, HyperError>(response::too_large("The request was too large"))
							}
							Err(GenericTransportError::Inner(e)) => {
								return Ok::<_, HyperError>(response::internal_error(e.to_string()))
							}
						};

						// NOTE(niklasad1): it's a channel because it's needed for batch requests.
						let (tx, mut rx) = mpsc::unbounded();

						match serde_json::from_slice::<JsonRpcRequest>(&body) {
							Ok(req) => {
								log::debug!("recv: {:?}", req);
								let params = RpcParams::new(req.params.map(|params| params.get()));
								if let Some(method) = methods.get(&*req.method) {
									// NOTE(niklasad1): connection ID is unused thus hardcoded to `0`.
									if let Err(err) = (method)(req.id, params, &tx, 0) {
										log::error!("method_call: {} failed: {:?}", req.method, err);
									}
								} else {
									let code = JsonRpcErrorCode::MethodNotFound;
									send_error(
										req.id,
										&tx,
										JsonRpcErrorObject { code, message: code.message(), data: None },
									);
								}
							}
							Err(_e) => {
								let (id, code) = match serde_json::from_slice::<JsonRpcInvalidRequest>(&body) {
									Ok(req) => (req.id, JsonRpcErrorCode::InvalidRequest),
									Err(_) => (None, JsonRpcErrorCode::ParseError),
								};
								send_error(id, &tx, JsonRpcErrorObject { code, message: code.message(), data: None });
							}
						};

						let response = rx.next().await.expect("Sender is still alive managed by us above; qed");
						log::debug!("send: {:?}", response);
						Ok::<_, HyperError>(response::ok_response(response))
					}
				}))
			}
		});

		let server = self.listener.serve(make_service);
		server.await.map_err(Into::into)
	}
}

// Checks to that access control of the received request is the same as configured.
fn access_control_is_valid(
	access_control: &AccessControl,
	request: &hyper::Request<hyper::Body>,
) -> Result<(), hyper::Response<hyper::Body>> {
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

/// Checks that content type of received request is valid for JSON-RPC.
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
