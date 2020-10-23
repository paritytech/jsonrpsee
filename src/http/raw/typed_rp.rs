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

use crate::http::raw::RawServerRequest;
use core::marker::PhantomData;

/// Allows responding to a server request in a more elegant and strongly-typed fashion.
pub struct TypedResponder<'a, T> {
	/// The request to answer.
	rq: RawServerRequest<'a>,
	/// Marker that pins the type of the response.
	response_ty: PhantomData<T>,
}

impl<'a, T> From<RawServerRequest<'a>> for TypedResponder<'a, T> {
	fn from(rq: RawServerRequest<'a>) -> TypedResponder<'a, T> {
		TypedResponder { rq, response_ty: PhantomData }
	}
}

impl<'a, T> TypedResponder<'a, T>
where
	T: serde::Serialize,
{
	/// Returns a successful response.
	pub fn ok(self, response: impl Into<T>) {
		self.respond(Ok(response))
	}

	/// Returns an erroneous response.
	pub fn err(self, err: crate::common::Error) {
		self.respond(Err::<T, _>(err))
	}

	/// Returns a response.
	pub fn respond(self, response: Result<impl Into<T>, crate::common::Error>) {
		let response = match response {
			Ok(v) => crate::common::to_value(v.into()).map_err(|_| crate::common::Error::internal_error()),
			Err(err) => Err(err),
		};

		self.rq.respond(response)
	}
}
