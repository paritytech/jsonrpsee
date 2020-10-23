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

mod background;
mod response;

use crate::common;
use crate::http::server_utils::access_control::AccessControl;

use fnv::FnvHashMap;
use futures::{channel::oneshot, prelude::*};
use std::{error, net::SocketAddr, pin::Pin};

pub type RequestId = u64;

/// Event that the [`TransportServer`] can generate.
#[derive(Debug, PartialEq)]
pub enum TransportServerEvent<T> {
	/// A new request has arrived on the wire.
	///
	/// This generates a new "request object" within the state of the [`TransportServer`] that is
	/// identified through the returned `id`. You can then use the other methods of the
	/// [`TransportServer`] trait in order to manipulate that request.
	Request {
		/// Identifier of the request within the state of the [`TransportServer`].
		id: T,
		/// Body of the request.
		request: common::Request,
	},

	/// A request has been cancelled, most likely because the client has closed the connection.
	///
	/// The corresponding request is no longer valid to manipulate.
	Closed(T),
}

/// Implementation of the [`TransportServer`](crate::transport::TransportServer) trait for HTTP.
pub struct HttpTransportServer {
	/// Background thread that processes HTTP requests.
	background_thread: background::BackgroundHttp,

	/// Local address of the server.
	local_addr: SocketAddr,

	/// Next identifier to use when inserting an element in `requests`.
	next_request_id: u64,

	/// The identifier is lineraly increasing and is never leaked on the wire or outside of this
	/// module. Therefore there is no risk of hash collision and using a `FnvHashMap` is safe.
	requests: FnvHashMap<u64, oneshot::Sender<hyper::Response<hyper::Body>>>,
}

impl HttpTransportServer {
	/// Tries to start an HTTP server that listens on the given address.
	///
	/// Returns an error if we fail to start listening, which generally happens if the port is
	/// already occupied.
	//
	// > Note: This function is `async` despite not performing any asynchronous operation. Normally
	// >       starting to listen on a port is an asynchronous operation, but the hyper library
	// >       hides this to us. In order to be future-proof, this function is async, so that we
	// >       might switch out to a different library later without breaking the API.
	pub async fn new(addr: &SocketAddr) -> Result<HttpTransportServer, Box<dyn error::Error + Send + Sync>> {
		let (background_thread, local_addr) = background::BackgroundHttp::bind(addr).await?;

		log::debug!(target: "jsonrpc-http-server", "Starting jsonrpc http server at address={:?}, local_addr={:?}", addr, local_addr);

		Ok(HttpTransportServer { background_thread, local_addr, requests: Default::default(), next_request_id: 0 })
	}

	/// Tries to start an HTTP server that listens on the given address with an access control list.
	pub async fn bind_with_acl(
		addr: &SocketAddr,
		access_control: AccessControl,
	) -> Result<HttpTransportServer, Box<dyn error::Error + Send + Sync>> {
		let (background_thread, local_addr) = background::BackgroundHttp::bind_with_acl(addr, access_control).await?;

		Ok(HttpTransportServer { background_thread, local_addr, requests: Default::default(), next_request_id: 0 })
	}

	/// Returns the address we are actually listening on, which might be different from the one
	/// passed as parameter.
	pub fn local_addr(&self) -> &SocketAddr {
		&self.local_addr
	}
}

// former `TransportServer trait impl`
impl HttpTransportServer {
	/// Returns the next event that the raw server wants to notify us.
	pub fn next_request<'a>(
		&'a mut self,
	) -> Pin<Box<dyn Future<Output = TransportServerEvent<RequestId>> + Send + 'a>> {
		Box::pin(async move {
			let request = match self.background_thread.next().await {
				Ok(r) => r,
				Err(_) => loop {
					log::debug!("http transport server inf loop?!");
					futures::pending!()
				},
			};

			let request_id = {
				let id = self.next_request_id;
				self.next_request_id = match self.next_request_id.checked_add(1) {
					Some(i) => i,
					None => {
						log::error!("Overflow in HttpTransportServer request ID assignment");
						loop {
							futures::pending!()
						}
					}
				};
				id
			};

			self.requests.insert(request_id, request.send_back);

			// Every 128 requests, we call `shrink_to_fit` on the list for a general cleanup.
			if request_id % 128 == 0 {
				self.requests.shrink_to_fit();
			}

			let request = TransportServerEvent::Request { id: request_id, request: request.request };

			log::debug!(target: "jsonrpc-http-transport-server", "received request: {:?}", request);
			request
		})
	}

	/// Sends back a response and destroys the request.
	///
	/// You can pass `None` in order to destroy the request object without sending back anything.
	///
	/// The implementation blindly sends back the response and doesn't check whether there is any
	/// correspondance with the request in terms of logic. For example, `respond` will accept
	/// sending back a batch of six responses even if the original request was a single
	/// notification.
	///
	/// > **Note**: While this method returns a `Future` that must be driven to completion,
	/// >           implementations must be aware that the entire requests processing logic is
	/// >           blocked for as long as this `Future` is pending. As an example, you shouldn't
	/// >           use this `Future` to send back a TCP message, because if the remote is
	/// >           unresponsive and the buffers full, the `Future` would then wait for a long time.
	///
	pub fn finish<'a>(
		&'a mut self,
		request_id: &'a RequestId,
		response: Option<&'a common::Response>,
	) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
		Box::pin(async move {
			let send_back = match self.requests.remove(request_id) {
				Some(rq) => rq,
				None => return Err(()),
			};

			let response = match response.map(|r| serde_json::to_vec(r)) {
				Some(Ok(bytes)) => hyper::Response::builder()
					.status(hyper::StatusCode::OK)
					.header("Content-Type", hyper::header::HeaderValue::from_static("application/json; charset=utf-8"))
					.body(hyper::Body::from(bytes))
					.expect("Unable to parse response body for type conversion"),
				Some(Err(_)) => panic!(), // TODO: no
				None => {
					// TODO: is that a good idea? should the param really be an Option?
					hyper::Response::builder()
						.status(hyper::StatusCode::NO_CONTENT)
						.body(hyper::Body::empty())
						.expect("Unable to parse response body for type conversion")
				}
			};

			if send_back.send(response).is_err() {
				log::error!("Couldn't send back JSON-RPC response, as background task has crashed");
			}

			Ok(())
		})
	}

	/// Returns true if this implementation supports sending back data on this request without
	/// closing it.
	///
	/// Returns an error if the request id is invalid.
	/// > **Note**: Not supported by HTTP
	//
	// TODO: this method is useless remove or create abstraction.
	pub fn supports_resuming(&self, id: &u64) -> Result<bool, ()> {
		if self.requests.contains_key(id) {
			Ok(false)
		} else {
			Err(())
		}
	}

	/// Sends back some data on the request and keeps the request alive.
	///
	/// You can continue sending data on that same request later.
	///
	/// Returns an error if the request identifier is incorrect, or if the implementation doesn't
	/// support that operation (see [`supports_resuming`](TransportServer::supports_resuming)).
	///
	/// > **Note**: Not supported by HTTP.
	//
	// TODO: this method is useless remove or create abstraction.
	pub fn send<'a>(
		&'a mut self,
		_: &'a RequestId,
		_: &'a common::Response,
	) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
		Box::pin(async move { Err(()) })
	}
}

#[cfg(test)]
mod tests {
	use super::HttpTransportServer;

	#[test]
	fn error_if_port_occupied() {
		futures::executor::block_on(async move {
			let addr = "127.0.0.1:0".parse().unwrap();
			let server1 = HttpTransportServer::new(&addr).await.unwrap();
			assert!(HttpTransportServer::new(server1.local_addr()).await.is_err());
		});
	}
}
