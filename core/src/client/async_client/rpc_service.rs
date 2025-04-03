use std::time::Duration;

use crate::{
	client::{
		BatchMessage, Error as ClientError, FrontToBack, MethodResponse, RequestMessage, SubscriptionMessage,
		SubscriptionResponse, async_client::helpers::call_with_timeout,
	},
	middleware::{Batch, IsSubscription, Notification, Request, RpcServiceT},
};

use futures_timer::Delay;
use futures_util::{
	FutureExt,
	future::{self, Either},
};
use http::Extensions;
use jsonrpsee_types::{InvalidRequestId, Response, ResponsePayload};
use tokio::sync::{mpsc, oneshot};

use super::helpers::call_with_timeout_sub;

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
	// This is a private interface but we need to expose it for the async client
	// to be able to create the service.
	#[allow(private_interfaces)]
	pub(crate) fn new(tx: mpsc::Sender<FrontToBack>, request_timeout: Duration) -> Self {
		Self { tx, request_timeout }
	}
}

impl RpcServiceT for RpcService {
	type Response = MethodResponse;
	type Error = Error;

	fn call<'a>(&self, request: Request<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
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
							subscribe_id: sub.sub_req_id(),
							unsubscribe_id: sub.unsub_req_id(),
							unsubscribe_method: sub.unsubscribe_method().to_owned(),
							send_back: send_back_tx,
						}))
						.await?;

					let (subscribe_rx, sub_id) = call_with_timeout_sub(request_timeout, send_back_rx).await??;

					let s = serde_json::value::to_raw_value(&sub_id).map_err(client_err)?;

					Ok(MethodResponse::subscription(
						SubscriptionResponse {
							rp: Response::new(ResponsePayload::success(s), request.id.clone().into_owned()).into(),
							sub_id,
							stream: subscribe_rx,
						},
						request.extensions,
					))
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

					Ok(MethodResponse::method_call(rp, request.extensions))
				}
			}
		}
		.boxed()
	}

	fn batch<'a>(&self, batch: Batch<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let tx = self.tx.clone();
		let request_timeout = self.request_timeout;

		async move {
			let (send_back_tx, send_back_rx) = oneshot::channel();

			let raw = serde_json::to_string(&batch).map_err(client_err)?;
			let id_range = batch.id_range().ok_or(ClientError::InvalidRequestId(InvalidRequestId::Invalid(
				"Batch request id range missing".to_owned(),
			)))?;

			tx.send(FrontToBack::Batch(BatchMessage { raw, ids: id_range, send_back: send_back_tx })).await?;
			let json = call_with_timeout(request_timeout, send_back_rx).await??;

			let mut extensions = Extensions::new();

			for entry in batch.into_batch_entries() {
				let Ok(entry) = entry else {
					continue;
				};
				extensions.extend(entry.into_extensions());
			}

			Ok(MethodResponse::batch(json, extensions))
		}
		.boxed()
	}

	fn notification<'a>(
		&self,
		n: Notification<'a>,
	) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
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
