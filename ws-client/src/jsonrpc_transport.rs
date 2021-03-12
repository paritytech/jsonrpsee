//! JSONRPC WebSocket Transport module.
//!
//! Wraps the underlying WebSocket transport with specific JSONRPC details.

use crate::WsConfig;
use crate::{
	manager::RequestManager,
	transport::{self, WsConnectError, WsHandshakeError, WsTransportClientBuilder},
};
use core::convert::TryInto;
use jsonrpsee_types::client::{NotificationMessage, RequestMessage, SubscriptionMessage};
use jsonrpsee_types::error::Error;
use jsonrpsee_types::jsonrpc;

/// Creates a new JSONRPC WebSocket connection, represented as a Sender and Receiver pair.
pub async fn websocket_connection(config: WsConfig<'_>) -> Result<(Sender, Receiver), WsHandshakeError> {
	let builder: WsTransportClientBuilder<'_> = config.try_into()?;
	let (sender, receiver) = builder.build().await?;
	Ok((Sender::new(sender), Receiver::new(receiver)))
}

/// JSONRPC WebSocket sender.
#[derive(Debug)]
pub struct Sender {
	transport: transport::Sender,
}

impl Sender {
	/// Creates a new JSONRPC sender.
	pub fn new(transport: transport::Sender) -> Self {
		Self { transport }
	}

	/// Sends a request to the server but it doesn’t wait for a response.
	/// Instead, you have keep the request ID and use the Receiver to get the response.
	///
	/// Returns Ok() if the request was successfully sent otherwise Err(_).
	pub async fn start_request(
		&mut self,
		request: RequestMessage,
		request_manager: &mut RequestManager,
	) -> Result<(), Error> {
		let id = match request_manager.next_request_id() {
			Ok(id) => id,
			Err(err) => {
				let str_err = err.to_string();
				request.send_back.map(|tx| tx.send(Err(err)));
				return Err(Error::Custom(str_err));
			}
		};
		let req = jsonrpc::Request::Single(jsonrpc::Call::MethodCall(jsonrpc::MethodCall {
			jsonrpc: jsonrpc::Version::V2,
			method: request.method,
			params: request.params,
			id: jsonrpc::Id::Num(id as u64),
		}));
		match self.transport.send_request(req).await {
			Ok(_) => {
				request_manager.insert_pending_call(id, request.send_back).expect("ID unused checked above; qed");
				Ok(())
			}
			Err(e) => {
				let str_err = e.to_string();
				let _ = request.send_back.map(|tx| tx.send(Err(Error::TransportError(Box::new(e)))));
				Err(Error::Custom(str_err))
			}
		}
	}

	/// Sends a notification to the server. The notification doesn't need any response.
	///
	/// Returns `Ok(())` if the notification was successfully sent otherwise `Err(_)`.
	pub async fn send_notification(&mut self, notif: NotificationMessage) -> Result<(), Error> {
		let request = jsonrpc::Request::Single(jsonrpc::Call::Notification(jsonrpc::Notification {
			jsonrpc: jsonrpc::Version::V2,
			method: notif.method,
			params: notif.params,
		}));

		self.transport.send_request(request).await.map_err(|e| Error::TransportError(Box::new(e)))
	}

	/// Sends a request to the server to start a new subscription but it doesn't wait for a response.
	/// Instead, you have keep the request ID and use the [`Receiver`] to get the response.
	///
	/// Returns `Ok()` if the request was successfully sent otherwise `Err(_)`.
	pub async fn start_subscription(
		&mut self,
		subscription: SubscriptionMessage,
		request_manager: &mut RequestManager,
	) -> Result<(), Error> {
		let id = match request_manager.next_request_id() {
			Ok(id) => id,
			Err(err) => {
				let str_err = err.to_string();
				let _ = subscription.send_back.send(Err(err));
				return Err(Error::Custom(str_err));
			}
		};

		let req = jsonrpc::Request::Single(jsonrpc::Call::MethodCall(jsonrpc::MethodCall {
			jsonrpc: jsonrpc::Version::V2,
			method: subscription.subscribe_method,
			params: subscription.params,
			id: jsonrpc::Id::Num(id as u64),
		}));
		if let Err(e) = self.transport.send_request(req).await {
			let str_err = e.to_string();
			let _ = subscription.send_back.send(Err(Error::TransportError(Box::new(e))));
			return Err(Error::Custom(str_err));
		}
		request_manager
			.insert_pending_subscription(id, subscription.send_back, subscription.unsubscribe_method)
			.expect("Request ID unused checked above; qed");
		Ok(())
	}
}

/// JSONRPC WebSocket receiver.
#[derive(Debug)]
pub struct Receiver {
	transport: transport::Receiver,
}

impl Receiver {
	/// Create a new JSONRPC WebSocket receiver.
	pub fn new(transport: transport::Receiver) -> Self {
		Self { transport }
	}

	/// Reads the next response, fails if the response ID was not a number.
	pub async fn next_response(&mut self) -> Result<jsonrpc::Response, WsConnectError> {
		self.transport.next_response().await
	}
}
