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

use crate::{response, AccessControl, TEN_MB_SIZE_BYTES};
use futures_channel::mpsc;
use futures_util::stream::StreamExt;
use hyper::{
	server::{conn::AddrIncoming, Builder as HyperBuilder},
	service::{make_service_fn, service_fn},
	Error as HyperError,
};
use jsonrpsee_types::error::{Error, GenericTransportError};
use jsonrpsee_types::v2::error::JsonRpcErrorCode;
use jsonrpsee_types::v2::params::{Id, RpcParams};
use jsonrpsee_types::v2::request::{JsonRpcInvalidRequest, JsonRpcNotification, JsonRpcRequest};
use jsonrpsee_utils::hyper_helpers::read_response_to_body;
use jsonrpsee_utils::server::helpers::{collect_batch_response, send_error};
use jsonrpsee_utils::server::rpc_module::{MethodSink, Methods, RpcModule};

use socket2::{Domain, Socket, Type};
use std::{
	cmp,
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

	pub fn build(self, addr: SocketAddr) -> Result<Server, Error> {
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
			methods: Methods::default(),
			access_control: self.access_control,
			max_request_body_size: self.max_request_body_size,
		})
	}
}

impl Default for Builder {
	fn default() -> Self {
		Self { max_request_body_size: TEN_MB_SIZE_BYTES, access_control: AccessControl::default(), keep_alive: true }
	}
}

pub struct Server {
	/// Hyper server.
	listener: HyperBuilder<AddrIncoming>,
	/// Local address
	local_addr: Option<SocketAddr>,
	/// Registered methods.
	methods: Methods,
	/// Max request body size.
	max_request_body_size: u32,
	/// Access control
	access_control: AccessControl,
}

impl Server {
	/// Register all [`Methods`] from an [`RpcModule`] on this server. In case a method already is registered with the
	/// same name, no method is added and a [`Error::MethodAlreadyRegistered`] is returned. Note that the [`RpcModule`]
	/// is consumed after this call.
	pub fn register_module<Context>(&mut self, module: RpcModule<Context>) -> Result<(), Error> {
		let methods = module.into_methods();
		for (name, _) in &methods {
			if self.methods.contains_key(name) {
				return Err(Error::MethodAlreadyRegistered(name.to_string()));
			}
		}
		self.methods.extend(methods);
		Ok(())
	}

	/// Returns a `Vec` with all the method names registered on this server.
	pub fn method_names(&self) -> Vec<String> {
		self.methods.keys().map(|name| name.to_string() ).collect()
	}

	/// Returns socket address to which the server is bound.
	pub fn local_addr(&self) -> Result<SocketAddr, Error> {
		self.local_addr.ok_or_else(|| Error::Custom("Local address not found".into()))
	}

	/// Start the server.
	pub async fn start(self) -> Result<(), Error> {
		let methods = Arc::new(self.methods);
		let max_request_body_size = self.max_request_body_size;
		let access_control = self.access_control;

		let make_service = make_service_fn(move |_| {
			let methods = methods.clone();
			let access_control = access_control.clone();

			async move {
				Ok::<_, HyperError>(service_fn(move |request| {
					let methods = methods.clone();
					let access_control = access_control.clone();

					// Look up the "method" (i.e. function pointer) from the registered methods and run it passing in
					// the params from the request. The result of the computation is sent back over the `tx` channel and
					// the result(s) are collected into a `String` and sent back over the wire.
					let execute = move |tx: &MethodSink, req: JsonRpcRequest| {
						if let Some(method) = methods.get(&*req.method) {
							let params = RpcParams::new(req.params.map(|params| params.get()));
							// NOTE(niklasad1): connection ID is unused thus hardcoded to `0`.
							if let Err(err) = (method)(req.id.clone(), params, &tx, 0) {
								log::error!(
									"execution of method call '{}' failed: {:?}, request id={:?}",
									req.method,
									err,
									req.id
								);
								send_error(req.id, &tx, JsonRpcErrorCode::ServerError(-1).into());
							}
						} else {
							send_error(req.id, &tx, JsonRpcErrorCode::MethodNotFound.into());
						}
					};

					// Run some validation on the http request, then read the body and try to deserialize it into one of
					// two cases: a single RPC request or a batch of RPC requests.
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
						let (tx, mut rx) = mpsc::unbounded::<String>();
						// Is this a single request or a batch (or error)?
						let mut single = true;

						// For reasons outlined [here](https://github.com/serde-rs/json/issues/497), `RawValue` can't be
						// used with untagged enums at the moment. This means we can't use an `SingleOrBatch` untagged
						// enum here and have to try each case individually: first the single request case, then the
						// batch case and lastly the error. For the worst case – unparseable input – we make three calls
						// to [`serde_json::from_slice`] which is pretty annoying.
						// Our [issue](https://github.com/paritytech/jsonrpsee/issues/296).
						if let Ok(req) = serde_json::from_slice::<JsonRpcRequest>(&body) {
							execute(&tx, req);
						} else if let Ok(_req) = serde_json::from_slice::<JsonRpcNotification>(&body) {
							return Ok::<_, HyperError>(response::ok_response("".into()));
						} else if let Ok(batch) = serde_json::from_slice::<Vec<JsonRpcRequest>>(&body) {
							if !batch.is_empty() {
								single = false;
								for req in batch {
									execute(&tx, req);
								}
							} else {
								send_error(Id::Null, &tx, JsonRpcErrorCode::InvalidRequest.into());
							}
						} else if let Ok(_batch) = serde_json::from_slice::<Vec<JsonRpcNotification>>(&body) {
							return Ok::<_, HyperError>(response::ok_response("".into()));
						} else {
							log::error!(
								"[service_fn], Cannot parse request body={:?}",
								String::from_utf8_lossy(&body[..cmp::min(body.len(), 1024)])
							);
							let (id, code) = match serde_json::from_slice::<JsonRpcInvalidRequest>(&body) {
								Ok(req) => (req.id, JsonRpcErrorCode::InvalidRequest),
								Err(_) => (Id::Null, JsonRpcErrorCode::ParseError),
							};
							send_error(id, &tx, code.into());
						}
						// Closes the receiving half of a channel without dropping it. This prevents any further
						// messages from being sent on the channel.
						rx.close();
						let response = if single {
							rx.next().await.expect("Sender is still alive managed by us above; qed")
						} else {
							collect_batch_response(rx).await
						};
						log::debug!("[service_fn] sending back: {:?}", &response[..cmp::min(response.len(), 1024)]);
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
