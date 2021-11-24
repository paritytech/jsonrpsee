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

//! TODO

/// TODO
pub trait Middleware: Default + Send + Sync + Clone + 'static {
	/// Intended to carry timestamp of a request, for example `std::time::Instant`. How the middleware
	/// measures time, if at all, is entirely up to the implementation.
	type Instant: Send + Copy;

	/// Called when a new JSON-RPC comes to the server.
	fn on_request(&self) -> Self::Instant;

	/// Called on each JSON-RPC method call, batch requests will trigger `on_call` multiple times.
	fn on_call(&self, name: &str);

	/// Called on each JSON-RPC method completion, batch requests will trigger `on_result` multiple times.
	fn on_result(&self, name: &str, succeess: bool, started_at: Self::Instant);

	/// Called once the JSON-RPC request is finished and response is sent to the output buffer.
	fn on_response(&self, started_at: Self::Instant);
}

impl Middleware for () {
	type Instant = ();

	fn on_request(&self) -> Self::Instant {}

	fn on_call(&self, _name: &str) {}

	fn on_result(&self, _name: &str, _succeess: bool, _started_at: Self::Instant) {}

	fn on_response(&self, _started_at: Self::Instant) {}
}

impl<A, B> Middleware for (A, B)
where
	A: Middleware,
	B: Middleware,
{
	type Instant = (A::Instant, B::Instant);

	fn on_request(&self) -> Self::Instant {
		(self.0.on_request(), self.1.on_request())
	}

	fn on_call(&self, name: &str) {
		self.0.on_call(name);
		self.1.on_call(name);
	}

	fn on_result(&self, name: &str, success: bool, started_at: Self::Instant) {
		self.0.on_result(name, success, started_at.0);
		self.1.on_result(name, success, started_at.1);
	}

	fn on_response(&self, started_at: Self::Instant) {
		self.0.on_response(started_at.0);
		self.1.on_response(started_at.1);
	}
}
