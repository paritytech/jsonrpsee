pub use crate::{raw_client::RawClientRef, types};
use derive_more::*;
use err_derive::*;
use serde::de::DeserializeOwned;
use std::sync::atomic::{AtomicU64, Ordering};

/// Wraps around a "raw client" and analyzes everything correctly.
pub struct Client<R> {
    inner: R,
    /// Id to assign to the next request.
    next_request_id: AtomicU64,
}

impl<R> Client<R> {
    /// Initializes a new `Client` using the given raw client as backend.
    pub fn new(inner: R) -> Self {
        Client {
            inner,
            next_request_id: AtomicU64::new(0),
        }
    }
}

impl<R> Client<R>
where
    for<'r> &'r R: RawClientRef<'r>,
{
    /// Starts a request.
    pub async fn request<'a, Ret>(&'a self, method: impl Into<String>)
        -> Result<Ret, ClientError<<&'a R as RawClientRef<'a>>::Error>>
    where Ret: DeserializeOwned,
    {
        let id = {
            let i = self.next_request_id.fetch_add(1, Ordering::Relaxed);
            if i == u64::max_value() {
                log::error!("Overflow in client request ID assignment");
            }
            types::Id::Num(i)
        };

        let request = types::Request::Single(types::Call::MethodCall(types::MethodCall {
            jsonrpc: types::Version::V2,
            method: method.into(),
            params: types::Params::None/*::Map(
                Default::default()      // TODO:
            )*/,
            id,
        }));

        let result = self.inner.request("http://localhost:9933", request).await
            .map_err(ClientError::Inner)?;

        let val = match result {
            types::Response::Single(types::Output::Success(s)) => s,
            _ => panic!("error in request")       // TODO: no
        };

        Ok(types::from_value(val.result).map_err(ClientError::Deserialize)?)
    }
}

/// Error that can happen during a request.
#[derive(Debug)]        // TODO: derive Error
pub enum ClientError<E> {
    //#[error(display = "error in the raw client")]
    Inner(/*#[error(cause)]*/ E),

    //#[error(display = "error while deserializing the server response")]
    Deserialize(serde_json::Error),
}
