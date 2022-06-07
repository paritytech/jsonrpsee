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

//! Middleware for `jsonrpsee` servers.

use jsonrpsee_types::Params;

/// Defines a middleware with callbacks during the RPC request life-cycle. The primary use case for
/// this is to collect timings for a larger metrics collection solution but the only constraints on
/// the associated type is that it be [`Send`] and [`Copy`], giving users some freedom to do what
/// they need to do.
///
/// See the [`WsServerBuilder::set_middleware`](../../jsonrpsee_ws_server/struct.WsServerBuilder.html#method.set_middleware)
/// or the [`HttpServerBuilder::set_middleware`](../../jsonrpsee_http_server/struct.HttpServerBuilder.html#method.set_middleware) method
/// for examples.
pub trait Middleware: Send + Sync + Clone + 'static {
	/// Intended to carry timestamp of a request, for example `std::time::Instant`. How the middleware
	/// measures time, if at all, is entirely up to the implementation.
	type Instant: Send + Copy;

	/// Called when a new client connects (WebSocket only)
	fn on_connect(&self) {}

	/// Called when a new JSON-RPC request comes to the server.
	fn on_request(&self, remote_addr: std::net::SocketAddr, headers: &http::HeaderMap) -> Self::Instant;

	/// Called on each JSON-RPC method call, batch requests will trigger `on_call` multiple times.
	fn on_call(&self, _name: &str, _params: Params) {}

	/// Called on each JSON-RPC method completion, batch requests will trigger `on_result` multiple times.
	fn on_result(&self, _name: &str, _success: bool, _started_at: Self::Instant) {}

	/// Called once the JSON-RPC request is finished and response is sent to the output buffer.
	fn on_response(&self, _result: &str, _started_at: Self::Instant) {}

	/// Called when a client disconnects (WebSocket only)
	fn on_disconnect(&self) {}
}

impl Middleware for () {
	type Instant = ();

	fn on_request(&self, _ip_addr: std::net::SocketAddr, _headers: &http::HeaderMap) -> Self::Instant {}
}

impl<A, B> Middleware for (A, B)
where
	A: Middleware,
	B: Middleware,
{
	type Instant = (A::Instant, B::Instant);

	fn on_request(&self, ip_addr: std::net::SocketAddr, headers: &http::HeaderMap) -> Self::Instant {
		(self.0.on_request(ip_addr, headers), self.1.on_request(ip_addr, headers))
	}

	fn on_call(&self, name: &str, params: Params) {
		self.0.on_call(name, params.clone());
		self.1.on_call(name, params);
	}

	fn on_result(&self, name: &str, success: bool, started_at: Self::Instant) {
		self.0.on_result(name, success, started_at.0);
		self.1.on_result(name, success, started_at.1);
	}

	fn on_response(&self, result: &str, started_at: Self::Instant) {
		self.0.on_response(result, started_at.0);
		self.1.on_response(result, started_at.1);
	}
}
