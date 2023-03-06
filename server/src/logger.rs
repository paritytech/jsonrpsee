// Copyright 2019-2021 Parity Technologies (UK) Ltd.
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

//! Logger for `jsonrpsee` servers.

use std::net::SocketAddr;

/// HTTP request.
pub type HttpRequest = hyper::Request<Body>;
pub use hyper::{Body, HeaderMap as Headers};
pub use jsonrpsee_types::Params;

/// The type JSON-RPC v2 call, it can be a subscription, method call or unknown.
#[derive(Debug, Copy, Clone)]
pub enum MethodKind {
	/// Subscription Call.
	Subscription,
	/// Unsubscription Call.
	Unsubscription,
	/// Method call.
	MethodCall,
	/// Unknown method.
	Unknown,
}

impl std::fmt::Display for MethodKind {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::Subscription => "subscription",
			Self::MethodCall => "method call",
			Self::Unknown => "unknown",
			Self::Unsubscription => "unsubscription",
		};

		write!(f, "{s}")
	}
}

/// The transport protocol used to send or receive a call or request.
#[derive(Debug, Copy, Clone)]
pub enum TransportProtocol {
	/// HTTP transport.
	Http,
	/// WebSocket transport.
	WebSocket,
}

impl std::fmt::Display for TransportProtocol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let s = match self {
			Self::Http => "http",
			Self::WebSocket => "websocket",
		};

		write!(f, "{s}")
	}
}

/// Defines a logger specifically for WebSocket connections with callbacks during the RPC request life-cycle.
/// The primary use case for this is to collect timings for a larger metrics collection solution.
///
/// See the [`ServerBuilder::set_logger`](../../jsonrpsee_server/struct.ServerBuilder.html#method.set_logger)
/// for examples.
pub trait Logger: Send + Sync + Clone + 'static {
	/// Intended to carry timestamp of a request, for example `std::time::Instant`. How the trait
	/// measures time, if at all, is entirely up to the implementation.
	type Instant: std::fmt::Debug + Send + Sync + Copy;

	/// Called when a new client connects
	fn on_connect(&self, _remote_addr: SocketAddr, _request: &HttpRequest, _t: TransportProtocol);

	/// Called when a new JSON-RPC request comes to the server.
	fn on_request(&self, transport: TransportProtocol) -> Self::Instant;

	/// Called on each JSON-RPC method call, batch requests will trigger `on_call` multiple times.
	fn on_call(&self, method_name: &str, params: Params, kind: MethodKind, transport: TransportProtocol);

	/// Called on each JSON-RPC method completion, batch requests will trigger `on_result` multiple times.
	fn on_result(&self, method_name: &str, success: bool, started_at: Self::Instant, transport: TransportProtocol);

	/// Called once the JSON-RPC request is finished and response is sent to the output buffer.
	fn on_response(&self, result: &str, started_at: Self::Instant, transport: TransportProtocol);

	/// Called when a client disconnects
	fn on_disconnect(&self, _remote_addr: SocketAddr, transport: TransportProtocol);
}

impl Logger for () {
	type Instant = ();

	fn on_connect(&self, _: SocketAddr, _: &HttpRequest, _p: TransportProtocol) -> Self::Instant {}

	fn on_request(&self, _p: TransportProtocol) -> Self::Instant {}

	fn on_call(&self, _: &str, _: Params, _: MethodKind, _p: TransportProtocol) {}

	fn on_result(&self, _: &str, _: bool, _: Self::Instant, _p: TransportProtocol) {}

	fn on_response(&self, _: &str, _: Self::Instant, _p: TransportProtocol) {}

	fn on_disconnect(&self, _: SocketAddr, _p: TransportProtocol) {}
}

impl<A, B> Logger for (A, B)
where
	A: Logger,
	B: Logger,
{
	type Instant = (A::Instant, B::Instant);

	fn on_connect(&self, remote_addr: std::net::SocketAddr, request: &HttpRequest, transport: TransportProtocol) {
		self.0.on_connect(remote_addr, request, transport);
		self.1.on_connect(remote_addr, request, transport);
	}

	fn on_request(&self, transport: TransportProtocol) -> Self::Instant {
		(self.0.on_request(transport), self.1.on_request(transport))
	}

	fn on_call(&self, method_name: &str, params: Params, kind: MethodKind, transport: TransportProtocol) {
		self.0.on_call(method_name, params.clone(), kind, transport);
		self.1.on_call(method_name, params, kind, transport);
	}

	fn on_result(&self, method_name: &str, success: bool, started_at: Self::Instant, transport: TransportProtocol) {
		self.0.on_result(method_name, success, started_at.0, transport);
		self.1.on_result(method_name, success, started_at.1, transport);
	}

	fn on_response(&self, result: &str, started_at: Self::Instant, transport: TransportProtocol) {
		self.0.on_response(result, started_at.0, transport);
		self.1.on_response(result, started_at.1, transport);
	}

	fn on_disconnect(&self, remote_addr: SocketAddr, transport: TransportProtocol) {
		self.0.on_disconnect(remote_addr, transport);
		self.1.on_disconnect(remote_addr, transport);
	}
}
