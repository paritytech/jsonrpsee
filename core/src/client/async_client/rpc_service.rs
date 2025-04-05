use crate::{
	client::{
		BatchMessage, Error as ClientError, FrontToBack, MethodResponse, RequestMessage, SubscriptionMessage,
		SubscriptionResponse,
	},
	middleware::{Batch, IsBatch, IsSubscription, Notification, Request, RpcServiceT},
};

use jsonrpsee_types::{Response, ResponsePayload};
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
pub struct RpcService(mpsc::Sender<FrontToBack>);

impl RpcService {
	// This is a private interface but we need to expose it for the async client
	// to be able to create the service.
	#[allow(private_interfaces)]
	pub(crate) fn new(tx: mpsc::Sender<FrontToBack>) -> Self {
		Self(tx)
	}
}

impl RpcServiceT for RpcService {
	type Response = MethodResponse;
	type Error = Error;

	fn call<'a>(&self, request: Request<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let tx = self.0.clone();

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

					let (subscribe_rx, sub_id) = send_back_rx.await??;

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
					let rp = send_back_rx.await?.map_err(client_err)?;

					Ok(MethodResponse::method_call(rp, request.extensions))
				}
			}
		}
	}

	fn batch<'a>(&self, mut batch: Batch<'a>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let tx = self.0.clone();

		async move {
			let (send_back_tx, send_back_rx) = oneshot::channel();

			let raw = serde_json::to_string(&batch).map_err(client_err)?;
			let id_range = batch
				.extensions()
				.get::<IsBatch>()
				.map(|b| b.id_range.clone())
				.expect("Batch ID range must be set in extensions");

			tx.send(FrontToBack::Batch(BatchMessage { raw, ids: id_range, send_back: send_back_tx })).await?;
			let json = send_back_rx.await?.map_err(client_err)?;

			Ok(MethodResponse::batch(json, batch.into_extensions()))
		}
	}

	fn notification<'a>(
		&self,
		n: Notification<'a>,
	) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + 'a {
		let tx = self.0.clone();

		async move {
			let raw = serde_json::to_string(&n).map_err(client_err)?;
			tx.send(FrontToBack::Notification(raw)).await?;
			Ok(MethodResponse::notification(n.extensions))
		}
	}
}

fn client_err(err: impl Into<ClientError>) -> Error {
	Error::Client(err.into())
}
