use std::collections::HashMap;
use std::{error, io};

use crate::client::http::raw::*;
use crate::client::http::transport::HttpTransportClient;
use crate::common::{self, JsonValue};

use futures::{channel::mpsc, channel::oneshot, future::Either, pin_mut, prelude::*};

/// Client that wraps a `RawClient` where the `RawClient` is spawned in a background worker tasks.
///
/// The communication is performed via a `mpsc` channel where the `Client` acts as simple frontend
/// and just passes requests along to the backend (worker thread)
pub struct Client {
	backend: mpsc::Sender<FrontToBack>,
}

/// Message that the [`Client`] can send to the background task.
enum FrontToBack {
	/// Send a one-shot notification to the server. The server doesn't give back any feedback.
	Notification {
		/// Method for the notification.
		method: String,
		/// Parameters to send to the server.
		params: common::Params,
	},

	/// Send a request to the server.
	StartRequest {
		/// Method for the request.
		method: String,
		/// Parameters of the request.
		params: common::Params,
		/// One-shot channel where to send back the outcome of that request.
		send_back: oneshot::Sender<Result<JsonValue, RequestError>>,
	},
}

/// Error produced by [`Client::request`] and [`Client::subscribe`].
#[derive(Debug, thiserror::Error)]
pub enum RequestError {
	/// Networking error or error on the low-level protocol layer (e.g. missing field,
	/// invalid ID, etc.).
	#[error("Networking or low-level protocol error: {0}")]
	TransportError(#[source] Box<dyn error::Error + Send + Sync>),
	/// RawServer responded to our request with an error.
	#[error("Server responded to our request with an error: {0:?}")]
	Request(#[source] common::Error),
	/// Failed to parse the data that the server sent back to us.
	#[error("Parse error: {0}")]
	ParseError(#[source] common::ParseError),
}

impl Client {
	/// Create a client to connect to the server at address `endpoint`
	pub fn new(endpoint: &str) -> Self {
		let client = RawClient::new(HttpTransportClient::new(endpoint));

		let (to_back, from_front) = mpsc::channel(16);
		async_std::task::spawn(async move {
			background_task(client, from_front).await;
		});

		Self { backend: to_back }
	}

	/// Send a notification to the server.
	pub async fn notification(&self, method: impl Into<String>, params: impl Into<crate::common::Params>) {
		let method = method.into();
		let params = params.into();
		log::debug!(target: "jsonrpsee-http-client", "transmitting notification: method={:?}, params={:?}", method, params);

		// TODO: do we care if the channel is just temporarly full or closed in this context?
		let _ = self.backend.clone().send(FrontToBack::Notification { method, params }).await;
	}

	/// Perform a request towards the server.
	pub async fn request<Ret>(
		&self,
		method: impl Into<String>,
		params: impl Into<crate::common::Params>,
	) -> Result<Ret, RequestError>
	where
		Ret: common::DeserializeOwned,
	{
		let method = method.into();
		let params = params.into();
		log::debug!(target: "jsonrpsee-http-client", "transmitting request: method={:?}, params={:?}", method, params);
		let (send_back_tx, send_back_rx) = oneshot::channel();

		// TODO: do we care if the channel is just temporarly full or closed in this context?
		if let Err(e) =
			self.backend.clone().send(FrontToBack::StartRequest { method, params, send_back: send_back_tx }).await
		{
			log::debug!(target: "jsonrpsee-http-client", "failed to send request to background task={:?}", e);
		}

		let json_value = match send_back_rx.await {
			Ok(Ok(v)) => {
				log::debug!(target: "jsonrpsee-http-client", "response={:?}", v);
				v
			}
			Ok(Err(err)) => return Err(err),
			Err(_) => {
				let err = io::Error::new(io::ErrorKind::Other, "background task closed");
				return Err(RequestError::TransportError(Box::new(err)));
			}
		};

		common::from_value(json_value).map_err(RequestError::ParseError)
	}
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task(mut client: RawClient, mut from_front: mpsc::Receiver<FrontToBack>) {
	log::debug!(target: "jsonrpsee-http-client", "background thread started");

	// List of requests that the server must answer.
	let mut ongoing_requests: HashMap<RawClientRequestId, oneshot::Sender<Result<_, _>>> = HashMap::new();

	loop {
		// We need to do a little transformation in order to destroy the borrow to `client`
		// and `from_front`.
		let outcome = {
			let next_message = from_front.next();
			let next_event = client.next_event();
			pin_mut!(next_message);
			pin_mut!(next_event);
			match future::select(next_message, next_event).await {
				Either::Left((v, _)) => Either::Left(v),
				Either::Right((v, _)) => Either::Right(v),
			}
		};

		match outcome {
			// If the channel is closed, then the `Client` has been destroyed and we
			// stop this task.
			Either::Left(None) => {
				log::debug!(target: "jsonrpsee-http-client", "background thread terminated");
				if !ongoing_requests.is_empty() {
					log::warn!(target: "jsonrpsee-http-client", "client was dropped with {} pending requests", ongoing_requests.len());
				}
				return;
			}

			// User called `notification` on the front-end.
			Either::Left(Some(FrontToBack::Notification { method, params })) => {
				let _ = client.send_notification(method, params).await;
			}

			// User called `request` on the front-end.
			Either::Left(Some(FrontToBack::StartRequest { method, params, send_back })) => {
				match client.start_request(method, params).await {
					Ok(id) => {
						log::debug!(target: "jsonrpsee-http-client", "background thread; inserting ingoing request={:?}", id);
						ongoing_requests.insert(id, send_back);
					}
					Err(err) => {
						let _ = send_back.send(Err(RequestError::TransportError(Box::new(err))));
					}
				}
			}

			// Received a response to a request from the server.
			Either::Right(Ok(RawClientEvent::Response { request_id, result })) => {
				let _ = ongoing_requests.remove(&request_id).unwrap().send(result.map_err(RequestError::Request));
			}

			Either::Right(Err(e)) => {
				// TODO: https://github.com/paritytech/jsonrpsee/issues/67
				log::error!("Client Error: {:?}", e);
			}
		}
	}
}
