use crate::jsonrpc::{Error, Request, Response};
use async_trait::async_trait;

/// Low level transport sender.
#[async_trait]
pub trait TransportSender {
	/// Send.
	async fn send(&mut self, request: Request) -> Result<(), Error>;
}

/// Low level transport receiver.
#[async_trait]
pub trait TransportReceiver {
	/// Receive.
	async fn receive(&mut self) -> Result<Response, Error>;
}
