use crate::error::Error;
use crate::{
	client::Subscription,
	v2::dummy::{JsonRpcMethod, JsonRpcParams},
};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// [JSON-RPC](https://www.jsonrpc.org/specification) client interface that can make requests and notifications.
#[async_trait]
pub trait Client {
	/// Send a [notification request](https://www.jsonrpc.org/specification#notification)
	async fn notification<'a>(&self, method: JsonRpcMethod<'a>, params: JsonRpcParams<'a>) -> Result<(), Error>;

	/// Send a [method call request](https://www.jsonrpc.org/specification#request_object).
	async fn request<'a, T>(&self, method: JsonRpcMethod<'a>, params: JsonRpcParams<'a>) -> Result<T, Error>
	where
		T: DeserializeOwned;

	/// Send a [batch request](https://www.jsonrpc.org/specification#batch).
	///
	/// The response to batch are returned in the same order as it was inserted in the batch.
	///
	/// Returns `Ok` if all requests in the batch were answered successfully.
	/// Returns `Error` if any of the requests in batch fails.
	async fn batch_request<'a, T>(&self, batch: Vec<(JsonRpcMethod<'a>, JsonRpcParams<'a>)>) -> Result<Vec<T>, Error>
	where
		T: DeserializeOwned + Default + Clone;
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
		subscribe_method: JsonRpcMethod<'a>,
		params: JsonRpcParams<'a>,
		unsubscribe_method: JsonRpcMethod<'a>,
	) -> Result<Subscription<Notif>, Error>
	where
		Notif: DeserializeOwned;
}
