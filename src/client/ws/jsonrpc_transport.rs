//! Wrapper module on-top transport.
//!

use crate::client::ws::transport::{self, WsConnectError};
use crate::types::jsonrpc;

// Type of request that has been sent out and that is waiting for a response.
#[derive(Debug, PartialEq, Eq)]
enum Request {
	/// A single request expecting a response.
	Request,
	/// A potential subscription. As a response, we expect a single subscription id.
	PendingSubscription,
	/// The request is stale and was originally used to open a subscription. The subscription ID
	/// decided by the server is contained as parameter.
	ActiveSubscription {
		sub_id: String,
		/// We sent a subscription closing message to the server.
		closing: bool,
	},
	/// Unsubscribing from an active subscription. The request corresponding to the active
	/// subscription to unsubscribe from is contained as parameter.
	Unsubscribe(u64),
}

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
		loop {
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

			break Ok(id);
		}
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

	pub async fn next_response(&mut self) -> Result<Vec<(u64, jsonrpc::Output)>, ()> {
		let response = self.transport.next_response().await.map_err(|_e| ())?;

		match response {
			jsonrpc::Response::Single(rp) => {
				let response = process_response(rp).map_err(|_e| ())?;
				Ok(vec![response])
			}
			jsonrpc::Response::Batch(rps) => {
				let mut responses = Vec::new();
				for rp in rps {
					// TODO: batch requests are not supported https://github.com/paritytech/jsonrpsee/issues/103
					let response = process_response(rp).map_err(|_e| ())?;
					responses.push(response);
				}
				Ok(responses)
			}
			jsonrpc::Response::Notif(notif) => {
				let sub_id = notif.params.subscription.into_string();
				todo!("");
			}
		}
	}
}

/// Processes the response obtained from the server. Updates the internal state of `self` to
/// account for it.
fn process_response(response: jsonrpc::Output) -> Result<(u64, jsonrpc::Output), ()> {
	match response.id() {
		jsonrpc::Id::Num(n) => Ok((*n, response)),
		jsonrpc::Id::Str(s) => {
			log::warn!("Server responded with an invalid request id: {:?}", s);
			return Err(());
		}
		jsonrpc::Id::Null => {
			log::warn!("Server responded with a null request id");
			return Err(());
		}
	}
}
