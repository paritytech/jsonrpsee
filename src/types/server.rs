/// Server Error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
	/// The method, notification or subscription is already registered.
	#[error("{0} is already registered")]
	AlreadyRegistered(String),
	/// Frontend/backend channel error.
	#[error("Frontend/backend channel error: {0}")]
	InternalChannel(#[from] futures::channel::mpsc::SendError),
}
