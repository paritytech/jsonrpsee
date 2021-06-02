use crate::v2::params::JsonRpcParams;
use crate::{Error, NotificationHandler, Subscription};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// [JSON-RPC](https://www.jsonrpc.org/specification) client interface that can make requests and notifications.
#[async_trait]
pub trait Client {
	/// Send a [notification request](https://www.jsonrpc.org/specification#notification)
	async fn notification<'a>(&self, method: &'a str, params: JsonRpcParams<'a>) -> Result<(), Error>;

	/// Send a [method call request](https://www.jsonrpc.org/specification#request_object).
	async fn request<'a, R>(&self, method: &'a str, params: JsonRpcParams<'a>) -> Result<R, Error>
	where
		R: DeserializeOwned;

	/// Send a [batch request](https://www.jsonrpc.org/specification#batch).
	///
	/// The response to batch are returned in the same order as it was inserted in the batch.
	///
	/// Returns `Ok` if all requests in the batch were answered successfully.
	/// Returns `Error` if any of the requests in batch fails.
	async fn batch_request<'a, R>(&self, batch: Vec<(&'a str, JsonRpcParams<'a>)>) -> Result<Vec<R>, Error>
	where
		R: DeserializeOwned + Default + Clone;
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
	async fn subscribe<'a, Notif>(
		&self,
		subscribe_method: &'a str,
		params: JsonRpcParams<'a>,
		unsubscribe_method: &'a str,
	) -> Result<Subscription<Notif>, Error>
	where
		Notif: DeserializeOwned + std::fmt::Debug;

	/// Register a NotificationHandler<Notif> that will listen for incoming JSON-RPC notifications
	async fn register_notification<'a, Notif>(&self, method: &'a str) -> Result<NotificationHandler<Notif>, Error>
	where
		Notif: DeserializeOwned + std::fmt::Debug;
}
