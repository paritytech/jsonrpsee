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

use crate::raw::server::RawServerRequest;
use crate::transport::TransportServer;

use core::{hash::Hash, marker::PhantomData};

/// Allows responding to a server request in a more elegant and strongly-typed fashion.
pub struct TypedResponder<'a, R, I, T> {
    /// The request to answer.
    rq: RawServerRequest<'a, R, I>,
    /// Marker that pins the type of the response.
    response_ty: PhantomData<T>,
}

impl<'a, R, I, T> From<RawServerRequest<'a, R, I>> for TypedResponder<'a, R, I, T> {
    fn from(rq: RawServerRequest<'a, R, I>) -> TypedResponder<'a, R, I, T> {
        TypedResponder {
            rq,
            response_ty: PhantomData,
        }
    }
}

impl<'a, R, I, T> TypedResponder<'a, R, I, T>
where
    R: TransportServer<RequestId = I>,
    I: Clone + PartialEq + Eq + Hash + Send + Sync,
    T: serde::Serialize,
{
    /// Returns a successful response.
    pub async fn ok(self, response: impl Into<T>) {
        self.respond(Ok(response)).await
    }

    /// Returns an erroneous response.
    pub async fn err(self, err: crate::common::Error) {
        self.respond(Err::<T, _>(err)).await
    }

    /// Returns a response.
    pub async fn respond(self, response: Result<impl Into<T>, crate::common::Error>) {
        let response = match response {
            Ok(v) => crate::common::to_value(v.into())
                .map_err(|_| crate::common::Error::internal_error()),
            Err(err) => Err(err),
        };

        self.rq.respond(response).await
    }
}
