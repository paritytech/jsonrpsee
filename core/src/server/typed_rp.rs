
// TODO: too much pub
pub struct TypedResponder<'a, R, I, T> {
    pub rq: crate::server::ServerRequest<'a, R, I>,
    pub response_ty: std::marker::PhantomData<T>,
}

impl<'a, R, I, T> TypedResponder<'a, R, I, T>
where R: crate::server::raw::RawServer<RequestId = I>,
        I: Clone + PartialEq + Eq + Send + Sync,
        T: serde::Serialize,
{
    pub async fn ok(self, response: impl Into<T>) {
        self.respond(Ok(response)).await
    }

    pub async fn err(self, err: crate::common::Error) {
        self.respond(Err::<T, _>(err)).await
    }

    pub async fn respond(self, response: Result<impl Into<T>, crate::common::Error>) {
        let response = match response {
            Ok(v) => crate::common::to_value(v.into())
                .map_err(|_| crate::common::Error::internal_error()),
            Err(err) => Err(err),
        };

        self.rq.respond(response).await
    }
}
