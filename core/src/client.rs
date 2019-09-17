//! Performing JSON-RPC requests.
// TODO: expand

pub use crate::{client::raw::RawClient, common};
use serde::de::DeserializeOwned;
use std::sync::atomic::{AtomicU64, Ordering};
use std::{error, fmt};

pub mod raw;

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
    R: RawClient,
{
    /// Starts a request.
    pub async fn request<Ret>(
        &mut self,
        method: impl Into<String>,
        params: impl Into<common::Params>,
    ) -> Result<Ret, ClientError<R::Error>>
    where
        Ret: DeserializeOwned,
    {
        let id = {
            let i = self.next_request_id.fetch_add(1, Ordering::Relaxed);
            if i == u64::max_value() {
                log::error!("Overflow in client request ID assignment");
            }
            common::Id::Num(i)
        };

        let request = common::Request::Single(common::Call::MethodCall(common::MethodCall {
            jsonrpc: common::Version::V2,
            method: method.into(),
            params: params.into(),
            id,
        }));

        let result = self
            .inner
            .request(request)
            .await
            .map_err(ClientError::Inner)?;

        let val = match result {
            common::Response::Single(common::Output::Success(s)) => s,
            _ => return Err(ClientError::WrongResponseKind),
        };

        Ok(common::from_value(val.result).map_err(ClientError::Deserialize)?)
    }
}

/// Error that can happen during a request.
#[derive(Debug)]
pub enum ClientError<E> {
    /// Error in the raw client.
    Inner(E),
    /// Error while deserializing the server response.
    Deserialize(serde_json::Error),
    /// Received a batch when we performed a request, or vice-versa.
    WrongResponseKind,
}

impl<E> error::Error for ClientError<E>
where
    E: error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ClientError::Inner(ref err) => Some(err),
            ClientError::Deserialize(ref err) => Some(err),
            ClientError::WrongResponseKind => None,
        }
    }
}

impl<E> fmt::Display for ClientError<E>
where
    E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::Inner(ref err) => write!(f, "Error in the raw client: {}", err),
            ClientError::Deserialize(ref err) => write!(f, "Error when deserializing: {}", err),
            ClientError::WrongResponseKind => write!(
                f,
                "Received a batch when we performed a request, or vice-versa"
            ),
        }
    }
}
