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
pub struct Sender {
	transport: transport::Sender,
}

impl Sender {
	/// Creates a new JSONRPC sender.
	pub fn new(transport: transport::Sender) -> Self {
		Self { transport }
	}

	/// Start sending a request.
	pub async fn start_request(
		&mut self,
		request: RequestMessage,
		request_manager: &mut RequestManager,
	) -> Result<(), ()> {
		let id = request_manager.next_request_id().ok_or(())?;
		let req = jsonrpc::Request::Single(jsonrpc::Call::MethodCall(jsonrpc::MethodCall {
			jsonrpc: jsonrpc::Version::V2,
			method: request.method,
			params: request.params,
			id: jsonrpc::Id::Num(id),
		}));
		if let Err(e) = self.transport.send_request(req).await {
			let _ = request.send_back.map(|tx| tx.send(Err(Error::TransportError(Box::new(e)))));
			return Err(());
		}
		request_manager.insert_pending_call(id, request.send_back).expect("ID unused checked above; qed");
		Ok(())
	}

	/// Sends a notification to the server. The notification doesn't need any response.
	///
	/// Returns `Ok(())` if the notification was successfully sent otherwise `Err(_)`.
	pub async fn send_notification(&mut self, notif: NotificationMessage) -> Result<(), WsConnectError> {
		let request = jsonrpc::Request::Single(jsonrpc::Call::Notification(jsonrpc::Notification {
			jsonrpc: jsonrpc::Version::V2,
			method: notif.method,
			params: notif.params,
		}));

		self.transport.send_request(request).await
	}

	/// Sends a request to the server to start a new subscription but it doesn't wait for a response.
	/// Instead, you have keep the request ID and use the [`Receiver`] to get the response.
	///
	/// Returns `Ok(request_id)` if the request was successfully sent otherwise `Err(_)`.
	pub async fn start_subscription(
		&mut self,
		subscription: SubscriptionMessage,
		request_manager: &mut RequestManager,
	) -> Result<(), ()> {
		let id = request_manager.next_request_id().unwrap();
		let req = jsonrpc::Request::Single(jsonrpc::Call::MethodCall(jsonrpc::MethodCall {
			jsonrpc: jsonrpc::Version::V2,
			method: subscription.subscribe_method,
			params: subscription.params,
			id: jsonrpc::Id::Num(id),
		}));
		if let Err(e) = self.transport.send_request(req).await {
			let _ = subscription.send_back.send(Err(Error::TransportError(Box::new(e))));
			return Err(());
		}
		request_manager
			.insert_pending_subscription(id, subscription.send_back, subscription.unsubscribe_method)
			.expect("Request ID unused checked above; qed");
		Ok(())
	}
}

/// JSONRPC WebSocket receiver.
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
