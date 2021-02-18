use jsonrpsee_types::jsonrpc::{self, Error};
use jsonrpsee_types::traits::TransportSender;

/// JSONRPC Sender.
/// It's a wrapper over [`TransportSender`] with additional `JSONRPC request_id`.
pub struct Sender<S> {
	request_id: u64,
	transport: S,
}

impl<S: TransportSender> From<S> for Sender<S> {
	fn from(sender: S) -> Self {
		Self::new(sender)
	}
}

impl<S: TransportSender> Sender<S> {
	/// Creates a new JSONRPC sender.
	pub fn new(transport: S) -> Self {
		Self { transport, request_id: 0 }
	}

	/// Inner implementation for starting either a request or a subscription.
	async fn start_impl(
		&mut self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<u64, Error> {
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
		self.transport.send(request).await.unwrap();

		Ok(id)
	}

	/// Sends a notification to the server. The notification doesn't need any response.
	///
	/// Returns `Ok(())` if the notification was successfully sent otherwise `Err(_)`.
	pub async fn send_notification(
		&mut self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<(), Error> {
		let request = jsonrpc::Request::Single(jsonrpc::Call::Notification(jsonrpc::Notification {
			jsonrpc: jsonrpc::Version::V2,
			method: method.into(),
			params: params.into(),
		}));

		self.transport.send(request).await
	}

	/// Sends a request to the server but it doesn't wait for a response.
	/// Instead, you have keep the request ID and use the [`Receiver`] to get the response.
	///
	/// Returns `Ok(request_id)` if the request was successfully sent otherwise `Err(_)`.
	pub async fn start_request(
		&mut self,
		method: impl Into<String>,
		params: impl Into<jsonrpc::Params>,
	) -> Result<u64, Error> {
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
	) -> Result<u64, Error> {
		let r = self.start_impl(method, params).await.unwrap();
		Ok(r)
	}
}
