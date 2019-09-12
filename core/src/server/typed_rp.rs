use crate::server::{ServerRequest, raw::RawServer};
use std::marker::PhantomData;

/// Allows responding to a server request in a more elegant and strongly-typed fashion.
pub struct TypedResponder<'a, R, I, T> {
    /// The request to answer.
    rq: ServerRequest<'a, R, I>,
    /// Marker that pins the type of the response.
    response_ty: PhantomData<T>,
}

impl<'a, R, I, T> From<ServerRequest<'a, R, I>> for TypedResponder<'a, R, I, T> {
    fn from(rq: ServerRequest<'a, R, I>) -> TypedResponder<'a, R, I, T> {
        TypedResponder {
            rq,
            response_ty: PhantomData,
        }
    }
}

impl<'a, R, I, T> TypedResponder<'a, R, I, T>
where
    R: RawServer<RequestId = I>,
    I: Clone + PartialEq + Eq + Send + Sync,
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
