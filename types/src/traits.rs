use crate::client::Subscription;
use crate::error::Error;
use crate::jsonrpc::{DeserializeOwned, Params};
use alloc::string::String;
use async_trait::async_trait;

/// [JSON-RPC](https://www.jsonrpc.org/specification) client interface that can make requests and notifications.
#[async_trait]
pub trait Client {
	/// Send a [notification request](https://www.jsonrpc.org/specification#notification)
	async fn notification<M, P>(&self, method: M, params: P) -> Result<(), Error>
	where
		M: Into<String> + Send,
		P: Into<Params> + Send;

	/// Send a [method call request](https://www.jsonrpc.org/specification#request_object).
	async fn request<T, M, P>(&self, method: M, params: P) -> Result<T, Error>
	where
		T: DeserializeOwned,
		M: Into<String> + Send,
		P: Into<Params> + Send;
}

/// [JSON-RPC](https://www.jsonrpc.org/specification) client interface that can make requests, notifications and subscriptions.
#[async_trait]
pub trait SubscriptionClient: Client {
	/// Send a subscription request to the server, technically not part of the [JSON-RPC specification](https://www.jsonrpc.org/specification)
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server.
	///
	/// The `unsubscribe_method` is used to close the subscription.
	///
	/// The `Notif` param is a generic type to receive generic subscriptions, see [`Subscription`](crate::client::Subscription) for further documentation.
	async fn subscribe<SM, UM, P, Notif>(
		&self,
		subscribe_method: SM,
		params: P,
		unsubscribe_method: UM,
	) -> Result<Subscription<Notif>, Error>
	where
		SM: Into<String> + Send,
		UM: Into<String> + Send,
		P: Into<Params> + Send,
		Notif: DeserializeOwned;
}
