//! JSONRPC WebSocket Transport module.
//!
//! Wraps the underlying WebSocket transport with specific JSONRPC details.

use crate::transport::{self, WsConnectError, WsHandshakeError, WsTransportClientBuilder};
use crate::WsConfig;
use core::convert::TryInto;
use jsonrpsee_types::jsonrpc;

/// Creates a new JSONRPC WebSocket connection, represented as a Sender and Receiver pair.
pub async fn websocket_connection(config: WsConfig<'_>) -> Result<(Sender, Receiver), WsHandshakeError> {
	let builder: WsTransportClientBuilder<'_> = config.try_into()?;
	let (sender, receiver) = builder.build().await?;
	Ok((Sender::new(sender), Receiver::new(receiver)))
}

/// JSONRPC WebSocket sender.
/// It's a wrapper over `WebSocket sender` with additional `JSONRPC request_id`.
pub struct Sender {
	request_id: u64,
	transport: transport::Sender,
}

impl Sender {
	/// Creates a new JSONRPC sender.
	pub fn new(transport: transport::Sender) -> Self {
		Self { transport, request_id: 0 }
	}

	/// Inner implementation for starting either a request or a subscription.
	async fn start_impl(
		&mut self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<u64, WsConnectError> {
		let id = self.request_id;
		self.request_id = id.wrapping_add(1);

		let request = jsonrpc::Request::Single(jsonrpc::Call::MethodCall(jsonrpc::MethodCall {
			jsonrpc: jsonrpc::Version::V2,
			method: method.into(),
			params: params.into(),
			id: jsonrpc::Id::Num(id),
		}));

		// Note that in case of an error, we "lose" the request id.
		// This isn't a problem, however.
		self.transport.send_request(request).await?;

		Ok(id)
	}

	/// Sends a notification to the server. The notification doesn't need any response.
	///
	/// Returns `Ok(())` if the notification was successfully sent otherwise `Err(_)`.
	pub async fn send_notification(
		&mut self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<(), WsConnectError> {
		let request = jsonrpc::Request::Single(jsonrpc::Call::Notification(jsonrpc::Notification {
			jsonrpc: jsonrpc::Version::V2,
			method: method.into(),
			params: params.into(),
		}));

		self.transport.send_request(request).await
	}

	/// Sends a request to the server but it doesn't wait for a response.
	/// Instead, you have keep the request ID and use the [`Receiver`] to get the response.
	///
	/// Returns `Ok(request_id)` if the request was successfully sent otherwise `Err(_)`.
	pub async fn start_request(
		&mut self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<u64, WsConnectError> {
		self.start_impl(method, params).await
	}

	/// Sends a request to the server to start a new subscription but it doesn't wait for a response.
	/// Instead, you have keep the request ID and use the [`Receiver`] to get the response.
	///
	/// Returns `Ok(request_id)` if the request was successfully sent otherwise `Err(_)`.
	pub async fn start_subscription(
		&mut self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<u64, WsConnectError> {
		self.start_impl(method, params).await
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
