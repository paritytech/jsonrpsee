use crate::client::Subscription;
use crate::error::Error;
use crate::jsonrpc::{DeserializeOwned, Params};
use alloc::string::String;
use async_trait::async_trait;

/// Basic `JSONRPC` client that can make requests, notifications and subscriptions.
#[async_trait]
pub trait Client {
	/// Send a notification request.
	async fn notification<M, P>(&self, method: M, params: P) -> Result<(), Error>
	where
		M: Into<String> + Send,
		P: Into<Params> + Send;

	/// Send a method call request.
	async fn request<T, M, P>(&self, method: M, params: P) -> Result<T, Error>
	where
		T: DeserializeOwned,
		M: Into<String> + Send,
		P: Into<Params> + Send;

	/// Send a subscription request to the server.
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server. The `unsubscribe_method` is used to close the subscription.
	//
	// TODO: ideally this should be a subtrait but let's have it to simplify macro stuff for now.
	async fn subscribe<SM, UM, P, N>(
		&self,
		subscribe_method: SM,
		params: P,
		unsubscribe_method: UM,
	) -> Result<Subscription<N>, Error>
	where
		SM: Into<String> + Send,
		UM: Into<String> + Send,
		P: Into<Params> + Send,
		N: DeserializeOwned;
}
