// Copyright 2019-2021 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any
// person obtaining a copy of this software and associated
// documentation files (the "Software"), to deal in the
// Software without restriction, including without
// limitation the rights to use, copy, modify, merge,
// publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software
// is furnished to do so, subject to the following
// conditions:
//
// The above copyright notice and this permission notice
// shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
// ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
// TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
// PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
// SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
// CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
// IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use crate::v2::params::ParamsSer;
use crate::{Error, Subscription};
use async_trait::async_trait;
use serde::de::DeserializeOwned;

/// [JSON-RPC](https://www.jsonrpc.org/specification) client interface that can make requests and notifications.
#[async_trait]
pub trait Client {
	/// Send a [notification request](https://www.jsonrpc.org/specification#notification)
	async fn notification<'a>(&self, method: &'a str, params: ParamsSer<'a>) -> Result<(), Error>;

	/// Send a [method call request](https://www.jsonrpc.org/specification#request_object).
	async fn request<'a, R>(&self, method: &'a str, params: ParamsSer<'a>) -> Result<R, Error>
	where
		R: DeserializeOwned;

	/// Send a [batch request](https://www.jsonrpc.org/specification#batch).
	///
	/// The response to batch are returned in the same order as it was inserted in the batch.
	///
	/// Returns `Ok` if all requests in the batch were answered successfully.
	/// Returns `Error` if any of the requests in batch fails.
	async fn batch_request<'a, R>(&self, batch: Vec<(&'a str, ParamsSer<'a>)>) -> Result<Vec<R>, Error>
	where
		R: DeserializeOwned + Default + Clone;
}

/// [JSON-RPC](https://www.jsonrpc.org/specification) client interface that can make requests, notifications and subscriptions.
#[async_trait]
pub trait SubscriptionClient: Client {
	/// Initiate a subscription by performing a JSON-RPC method call where the server responds with
	/// a `Subscription ID` that is used to fetch messages on that subscription,
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server.
	///
	/// The params may be used as input for the subscription for the server to process.
	///
	/// The `unsubscribe_method` is used to close the subscription
	///
	/// The `Notif` param is a generic type to receive generic subscriptions, see [`Subscription`] for further documentation.
	async fn subscribe<'a, Notif>(
		&self,
		subscribe_method: &'a str,
		params: ParamsSer<'a>,
		unsubscribe_method: &'a str,
	) -> Result<Subscription<Notif>, Error>
	where
		Notif: DeserializeOwned;

	/// Register a method subscription, this is used to filter only server notifications that a user is interested in.
	///
	/// The `Notif` param is a generic type to receive generic subscriptions, see [`Subscription`] for further documentation.
	async fn subscribe_to_method<'a, Notif>(&self, method: &'a str) -> Result<Subscription<Notif>, Error>
	where
		Notif: DeserializeOwned;
}
