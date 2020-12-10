//! JSONRPC Transport module.
//!
//! Wraps the underlying WebSocket transport with specific JSONRPC details.

use crate::client::ws::transport::WsConnectError;
use crate::types::jsonrpc;

pub async fn websocket_context(target: impl AsRef<str>) -> Result<(Sender, Receiver), WsConnectError> {
	let (sender, receiver) = crate::client::ws::transport::new(target.as_ref()).await.unwrap();
	Ok((Sender::new(sender), Receiver::new(receiver)))
}

pub struct Sender {
	request_id: u64,
	transport: crate::client::ws::transport::Sender,
}

impl Sender {
	pub fn new(transport: crate::client::ws::transport::Sender) -> Self {
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

		// Note that in case of an error, we "lose" the request id (as in, it will never be
		// used). This isn't a problem, however.
		self.transport.send_request(request).await?;

		Ok(id)
	}

	/// Sends a notification to the server. The notification doesn't need any response.
	///
	/// This asynchronous function finishes when the notification has finished being sent.
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

	pub async fn start_request(
		&mut self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<u64, WsConnectError> {
		self.start_impl(method, params).await
	}

	pub async fn start_subscription(
		&mut self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<u64, WsConnectError> {
		self.start_impl(method, params).await
	}
}

pub struct Receiver {
	transport: crate::client::ws::transport::Receiver,
}

impl Receiver {
	pub fn new(transport: crate::client::ws::transport::Receiver) -> Self {
		Self { transport }
	}

	/// Reads the next response, fails if the response ID was not a number.
	pub async fn next_response(&mut self) -> Result<jsonrpc::Response, WsConnectError> {
		self.transport.next_response().await
	}
}
