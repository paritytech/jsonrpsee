use crate::jsonrpc::{DeserializeOwned, Params};
use alloc::string::String;
use core::{fmt, pin::Pin};
use futures::prelude::*;

/// Basic `JSONRPC` client that can make requests and notifications.
pub trait Client {
	/// Error.
	type Error: fmt::Display;
	/// Subscription.
	type Subscription;

	/// Send a method call request.
	fn request<'a, T: DeserializeOwned>(
		&'a self,
		method: impl Into<String>,
		params: impl Into<Params>,
	) -> Pin<Box<dyn Future<Output = Result<T, Self::Error>> + Send + 'a>>;

	/// Send a notification request.
	fn notification<'a>(
		&'a self,
		method: impl Into<String>,
		params: impl Into<Params>,
	) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>;

	/// Send a subscription request to the server.
	///
	/// The `subscribe_method` and `params` are used to ask for the subscription towards the
	/// server. The `unsubscribe_method` is used to close the subscription.
	//
	// TODO: ideally this should be a subtrait but let's have it to simplify macro stuff for now.
	fn subscribe<'a>(
		&'a self,
		subscribe_method: impl Into<String>,
		params: impl Into<Params>,
		unsubscribe_method: impl Into<String>,
	) -> Pin<Box<dyn Future<Output = Result<Self::Subscription, Self::Error>> + Send + 'a>>;
}
