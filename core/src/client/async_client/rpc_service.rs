use std::time::Duration;

use crate::{
	client::{
		BatchMessage, Error as ClientError, FrontToBack, MethodResponse, RequestMessage, SubscriptionMessage,
		async_client::helpers::call_with_timeout,
	},
	middleware::{Batch, BatchEntry, IsSubscription, Notification, Request, ResponseBoxFuture, RpcServiceT},
};

use futures_timer::Delay;
use futures_util::{
	FutureExt,
	future::{self, Either},
};
use http::Extensions;
use tokio::sync::{mpsc, oneshot};

/// RpcService error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// Client error.
	#[error(transparent)]
	Client(#[from] ClientError),
	#[error("Fetch from backend")]
	/// Internal error state when the underlying channel is closed
	/// and the error reason needs to be fetched from the backend.
	FetchFromBackend,
}

impl From<mpsc::error::SendError<FrontToBack>> for Error {
	fn from(_: mpsc::error::SendError<FrontToBack>) -> Self {
		Error::FetchFromBackend
	}
}

impl From<oneshot::error::RecvError> for Error {
	fn from(_: oneshot::error::RecvError) -> Self {
		Error::FetchFromBackend
	}
}

/// RpcService implementation for the async client.
#[derive(Debug, Clone)]
pub struct RpcService {
	tx: mpsc::Sender<FrontToBack>,
	request_timeout: Duration,
}

impl RpcService {
	/// Create a new RpcService instance.
	pub fn new(tx: mpsc::Sender<FrontToBack>, request_timeout: Duration) -> Self {
		Self { tx, request_timeout }
	}
}

impl<'a> RpcServiceT<'a> for RpcService {
	type Future = ResponseBoxFuture<'a, Self::Response, Self::Error>;
	type Response = MethodResponse;
	type Error = Error;

	fn call(&self, request: Request<'a>) -> Self::Future {
		let tx = self.tx.clone();
		let request_timeout = self.request_timeout;

		async move {
			let raw = serde_json::to_string(&request).map_err(client_err)?;

			match request.extensions.get::<IsSubscription>() {
				Some(sub) => {
					let (send_back_tx, send_back_rx) = tokio::sync::oneshot::channel();

					tx.clone()
						.send(FrontToBack::Subscribe(SubscriptionMessage {
							raw,
							subscribe_id: sub.sub_id.clone(),
							unsubscribe_id: sub.unsub_id.clone(),
							unsubscribe_method: sub.unsub_method.clone(),
							send_back: send_back_tx,
						}))
						.await?;

					let (subscribe_rx, sub_id) = call_with_timeout(request_timeout, send_back_rx).await??;

					Ok(MethodResponse::subscription(sub_id, subscribe_rx, request.extensions))
				}
				None => {
					let (send_back_tx, send_back_rx) = oneshot::channel();

					tx.send(FrontToBack::Request(RequestMessage {
						raw,
						send_back: Some(send_back_tx),
						id: request.id.clone().into_owned(),
					}))
					.await?;
					let rp = call_with_timeout(request_timeout, send_back_rx).await??;

					Ok(MethodResponse::method_call(rp, request.extensions, request.id.clone().into_owned()))
				}
			}
		}
		.boxed()
	}

	fn batch(&self, batch: Batch<'a>) -> Self::Future {
		let tx = self.tx.clone();
		let request_timeout = self.request_timeout;

		async move {
			let (send_back_tx, send_back_rx) = oneshot::channel();

			let raw = serde_json::to_string(&batch).map_err(client_err)?;
			let first = {
				match batch.first() {
					Some(&BatchEntry::Call(ref r)) => r.id.clone().into_owned().try_parse_inner_as_number().unwrap(),
					_ => unreachable!("Only method calls are allowed in batch requests"),
				}
			};
			let last = {
				match batch.last() {
					Some(&BatchEntry::Call(ref r)) => r.id.clone().into_owned().try_parse_inner_as_number().unwrap(),
					_ => unreachable!("Only method calls are allowed in batch requests"),
				}
			};

			tx.send(FrontToBack::Batch(BatchMessage { raw, ids: first..last + 1, send_back: send_back_tx })).await?;
			let json = call_with_timeout(request_timeout, send_back_rx).await??;

			let mut extensions = Extensions::new();

			for entry in batch.into_iter() {
				extensions.extend(entry.into_extensions());
			}

			Ok(MethodResponse::batch(json, extensions))
		}
		.boxed()
	}

	fn notification(&self, n: Notification<'a>) -> Self::Future {
		let tx = self.tx.clone();
		let request_timeout = self.request_timeout;

		async move {
			let raw = serde_json::to_string(&n).map_err(client_err)?;
			let fut = tx.send(FrontToBack::Notification(raw));

			tokio::pin!(fut);

			match future::select(fut, Delay::new(request_timeout)).await {
				Either::Left((Ok(()), _)) => Ok(MethodResponse::notification(n.extensions)),
				Either::Left((Err(e), _)) => Err(e.into()),
				Either::Right((_, _)) => Err(ClientError::RequestTimeout.into()),
			}
		}
		.boxed()
	}
}

fn client_err(err: impl Into<ClientError>) -> Error {
	Error::Client(err.into())
}
