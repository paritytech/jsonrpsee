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

use std::fmt::Debug;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use std::time::Duration;

use crate::error::{Error, SubscriptionAcceptRejectError};
use crate::server::helpers::{MethodSink};
use crate::{SubscriptionCallbackError, SubscriptionResult};
use futures_util::future::Either;
use futures_util::{FutureExt};
use jsonrpsee_types::error::{ErrorCode, ErrorObject, ErrorObjectOwned};
use jsonrpsee_types::{
	Id, Params, SubscriptionId as RpcSubscriptionId,
};
use serde::{Serialize};
use tokio::sync::{mpsc, oneshot};

use super::helpers::{MethodResponse, SubscriptionPermit};

pub use methods::*;
pub use types::*;

mod methods;
mod types;

/// Sets of JSON-RPC methods can be organized into "module"s that are in turn registered on the server or,
/// alternatively, merged with other modules to construct a cohesive API. [`RpcModule`] wraps an additional context
/// argument that can be used to access data during call execution.
#[derive(Debug, Clone)]
pub struct RpcModule<Context> {
	ctx: Arc<Context>,
	methods: Methods,
}

impl<Context> RpcModule<Context> {
	/// Create a new module with a given shared `Context`.
	pub fn new(ctx: Context) -> Self {
		Self { ctx: Arc::new(ctx), methods: Default::default() }
	}

	/// Transform a module into an `RpcModule<()>` (unit context).
	pub fn remove_context(self) -> RpcModule<()> {
		let mut module = RpcModule::new(());
		module.methods = self.methods;
		module
	}
}

impl<Context> From<RpcModule<Context>> for Methods {
	fn from(module: RpcModule<Context>) -> Methods {
		module.methods
	}
}

impl<Context: Send + Sync + 'static> RpcModule<Context> {
	/// Register a new synchronous RPC method, which computes the response with the given callback.
	pub fn register_method<R, F>(
		&mut self,
		method_name: &'static str,
		callback: F,
	) -> Result<&mut MethodCallback, Error>
	where
		Context: Send + Sync + 'static,
		R: Serialize,
		F: Fn(Params, &Context) -> Result<R, Error> + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		self.methods.verify_and_insert(
			method_name,
			MethodCallback::new_sync(Arc::new(move |id, params, max_response_size| match callback(params, &*ctx) {
				Ok(res) => MethodResponse::response(id, res, max_response_size),
				Err(err) => MethodResponse::error(id, err),
			})),
		)
	}

	/// Register a new asynchronous RPC method, which computes the response with the given callback.
	pub fn register_async_method<R, E, Fun, Fut>(
		&mut self,
		method_name: &'static str,
		callback: Fun,
	) -> Result<&mut MethodCallback, Error>
	where
		R: Serialize + Send + Sync + 'static,
		E: Into<Error>,
		Fut: Future<Output = Result<R, E>> + Send,
		Fun: (Fn(Params<'static>, Arc<Context>) -> Fut) + Clone + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		self.methods.verify_and_insert(
			method_name,
			MethodCallback::new_async(Arc::new(move |id, params, _, max_response_size| {
				let ctx = ctx.clone();
				let callback = callback.clone();

				let future = async move {
					match callback(params, ctx).await {
						Ok(res) => MethodResponse::response(id, res, max_response_size),
						Err(err) => MethodResponse::error(id, err.into()),
					}
				};
				future.boxed()
			})),
		)
	}

	/// Register a new **blocking** synchronous RPC method, which computes the response with the given callback.
	/// Unlike the regular [`register_method`](RpcModule::register_method), this method can block its thread and perform expensive computations.
	pub fn register_blocking_method<R, E, F>(
		&mut self,
		method_name: &'static str,
		callback: F,
	) -> Result<&mut MethodCallback, Error>
	where
		Context: Send + Sync + 'static,
		R: Serialize,
		E: Into<Error>,
		F: Fn(Params, Arc<Context>) -> Result<R, E> + Clone + Send + Sync + 'static,
	{
		let ctx = self.ctx.clone();
		let callback = self.methods.verify_and_insert(
			method_name,
			MethodCallback::new_async(Arc::new(move |id, params, _, max_response_size| {
				let ctx = ctx.clone();
				let callback = callback.clone();

				tokio::task::spawn_blocking(move || match callback(params, ctx) {
					Ok(result) => MethodResponse::response(id, result, max_response_size),
					Err(err) => MethodResponse::error(id, err.into()),
				})
				.map(|result| match result {
					Ok(r) => r,
					Err(err) => {
						tracing::error!("Join error for blocking RPC method: {:?}", err);
						MethodResponse::error(Id::Null, ErrorObject::from(ErrorCode::InternalError))
					}
				})
				.boxed()
			})),
		)?;

		Ok(callback)
	}

	/// Register a new publish/subscribe interface using JSON-RPC notifications.
	///
	/// It implements the [ethereum pubsub specification](https://geth.ethereum.org/docs/rpc/pubsub)
	/// with an option to choose custom subscription ID generation.
	///
	/// Furthermore, it generates the `unsubscribe implementation` where a `bool` is used as
	/// the result to indicate whether the subscription was successfully unsubscribed to or not.
	/// For instance an `unsubscribe call` may fail if a non-existent subscription ID is used in the call.
	///
	/// This method ensures that the `subscription_method_name` and `unsubscription_method_name` are unique.
	/// The `notif_method_name` argument sets the content of the `method` field in the JSON document that
	/// the server sends back to the client. The uniqueness of this value is not machine checked and it's up to
	/// the user to ensure it is not used in any other [`RpcModule`] used in the server.
	///
	/// # Arguments
	///
	/// * `subscription_method_name` - name of the method to call to initiate a subscription
	/// * `notif_method_name` - name of method to be used in the subscription payload (technically a JSON-RPC notification)
	/// * `unsubscription_method` - name of the method to call to terminate a subscription
	/// * `callback` - A callback to invoke on each subscription; it takes three parameters:
	///     - [`Params`]: JSON-RPC parameters in the subscription call.
	///     - [`PendingSubscriptionSink`]: A pending subscription waiting to be accepted, in order to send out messages on the subscription
	///     - Context: Any type that can be embedded into the [`RpcModule`].
	///
	/// # Returns
	///
	/// An async block which returns `Result<(), SubscriptionCallbackError>` the error is simply
	/// for a more ergonomic API and is not used (except logged for user-related caused errors).
	/// By default jsonrpsee doesn't send any special close notification,
	/// it can be a footgun if one wants to send out a "special notification" to indicate that an error occurred.
	///
	/// If you want to a special error notification use `SubscriptionSink::close` or
	/// `SubscriptionSink::send` before returning from the async block.
	///
	/// # Examples
	///
	/// ```no_run
	///
	/// use jsonrpsee_core::server::rpc_module::{RpcModule, SubscriptionSink, SubscriptionMessage};
	/// use jsonrpsee_core::Error;
	///
	/// let mut ctx = RpcModule::new(99_usize);
	/// ctx.register_subscription("sub", "notif_name", "unsub", |params, pending, ctx| async move {
	///     let x = params.one::<usize>()?;
	///
	///     // mark the subscription is accepted after the params has been parsed successful.
	///     let sink = pending.accept().await?;
	///
	///     let sum = x + (*ctx);
	///
	///     // NOTE: the error handling here is for easy of use
	///     // and are thrown away
	///     let msg = SubscriptionMessage::from_json(&sum)?;
	///     sink.send(msg).await?;
	///
	///     Ok(())
	/// });
	/// ```
	pub fn register_subscription<F, Fut>(
		&mut self,
		subscribe_method_name: &'static str,
		notif_method_name: &'static str,
		unsubscribe_method_name: &'static str,
		callback: F,
	) -> Result<&mut MethodCallback, Error>
	where
		Context: Send + Sync + 'static,
		F: (Fn(Params<'static>, PendingSubscriptionSink, Arc<Context>) -> Fut) + Send + Sync + Clone + 'static,
		Fut: Future<Output = SubscriptionResult> + Send + 'static,
	{
		if subscribe_method_name == unsubscribe_method_name {
			return Err(Error::SubscriptionNameConflict(subscribe_method_name.into()));
		}

		self.methods.verify_method_name(subscribe_method_name)?;
		self.methods.verify_method_name(unsubscribe_method_name)?;

		let ctx = self.ctx.clone();
		let subscribers = Subscribers::default();

		// Unsubscribe
		{
			let subscribers = subscribers.clone();
			self.methods.mut_callbacks().insert(
				unsubscribe_method_name,
				MethodCallback::new_unsubscription(Arc::new(move |id, params, conn_id, max_response_size| {
					let sub_id = match params.one::<RpcSubscriptionId>() {
						Ok(sub_id) => sub_id,
						Err(_) => {
							tracing::warn!(
								"Unsubscribe call `{}` failed: couldn't parse subscription id={:?} request id={:?}",
								unsubscribe_method_name,
								params,
								id
							);

							return MethodResponse::response(id, false, max_response_size);
						}
					};

					let key = SubscriptionKey { conn_id, sub_id: sub_id.into_owned() };
					let result = subscribers.lock().remove(&key).is_some();

					if !result {
						tracing::debug!(
							"Unsubscribe call `{}` subscription key={:?} not an active subscription",
							unsubscribe_method_name,
							key,
						);
					}

					MethodResponse::response(id, result, max_response_size)
				})),
			);
		}

		// Subscribe
		let callback = {
			self.methods.verify_and_insert(
				subscribe_method_name,
				MethodCallback::new_subscription(Arc::new(move |id, params, method_sink, conn| {
					let uniq_sub = SubscriptionKey { conn_id: conn.conn_id, sub_id: conn.id_provider.next_id() };

					// response to the subscription call.
					let (tx, rx) = oneshot::channel();

					let sink = PendingSubscriptionSink {
						inner: method_sink,
						method: notif_method_name,
						subscribers: subscribers.clone(),
						uniq_sub,
						id: id.clone().into_owned(),
						subscribe: tx,
						permit: conn.subscription_permit,
					};

					// The subscription callback is a future from the subscription
					// definition and not the as same when the subscription call has been completed.
					//
					// This runs until the subscription callback has completed.
					let sub_fut = callback(params.into_owned(), sink, ctx.clone());

					tokio::spawn(async move {
						if let Err(SubscriptionCallbackError::Some(msg)) = sub_fut.await {
							tracing::warn!("Subscribe call `{subscribe_method_name}` failed: {msg}");
						}
					});

					let id = id.clone().into_owned();

					let result = async move {
						match rx.await {
							Ok(r) => SubscriptionAnswered::Yes(r),
							Err(_) => {
								let response = MethodResponse::error(id, ErrorObject::from(ErrorCode::InternalError));
								SubscriptionAnswered::No(response)
							}
						}
					};

					Box::pin(result)
				})),
			)?
		};

		Ok(callback)
	}

	/// Register an alias for an existing_method. Alias uniqueness is enforced.
	pub fn register_alias(&mut self, alias: &'static str, existing_method: &'static str) -> Result<(), Error> {
		self.methods.verify_method_name(alias)?;

		let callback = match self.methods.callbacks.get(existing_method) {
			Some(callback) => callback.clone(),
			None => return Err(Error::MethodNotFound(existing_method.into())),
		};

		self.methods.mut_callbacks().insert(alias, callback);

		Ok(())
	}
}

impl<Context> Deref for RpcModule<Context> {
	type Target = Methods;

	fn deref(&self) -> &Methods {
		&self.methods
	}
}

impl<Context> DerefMut for RpcModule<Context> {
	fn deref_mut(&mut self) -> &mut Methods {
		&mut self.methods
	}
}

/// Represents a subscription until it is unsubscribed.
///
// NOTE: The reason why we use `mpsc` here is because it allows `IsUnsubscribed::unsubscribed`
// to be &self instead of &mut self.
#[derive(Debug, Clone)]
struct IsUnsubscribed(mpsc::Sender<()>);

impl IsUnsubscribed {
	/// Returns true if the unsubscribe method has been invoked or the subscription has been canceled.
	///
	/// This can be called multiple times as the element in the channel is never
	/// removed.
	fn is_unsubscribed(&self) -> bool {
		self.0.is_closed()
	}

	/// Wrapper over [`tokio::sync::mpsc::Sender::closed`]
	///
	/// # Cancel safety
	///
	/// This method is cancel safe. Once the channel is closed,
	/// it stays closed forever and all future calls to closed will return immediately.
	async fn unsubscribed(&self) {
		self.0.closed().await;
	}
}

/// Represents a single subscription that is waiting to be accepted or rejected.
///
/// If this is dropped without calling `PendingSubscription::reject` or `PendingSubscriptionSink::accept`
/// a default error is sent out as response to the subscription call.
///
/// Thus, if you want a customized error message then `PendingSubscription::reject` must be called.
#[derive(Debug)]
#[must_use = "PendningSubscriptionSink does nothing unless `accept` or `reject` is called"]
pub struct PendingSubscriptionSink {
	/// Sink.
	inner: MethodSink,
	/// MethodCallback.
	method: &'static str,
	/// Shared Mutex of subscriptions for this method.
	subscribers: Subscribers,
	/// Unique subscription.
	uniq_sub: SubscriptionKey,
	/// ID of the `subscription call` (i.e. not the same as subscription id) which is used
	/// to reply to subscription method call and must only be used once.
	id: Id<'static>,
	/// Sender to answer the subscribe call.
	subscribe: oneshot::Sender<MethodResponse>,
	/// Subscription permit.
	permit: SubscriptionPermit,
}

impl PendingSubscriptionSink {
	/// Reject the subscription call with the error from [`ErrorObject`].
	pub async fn reject(self, err: impl Into<ErrorObjectOwned>) -> Result<(), SubscriptionAcceptRejectError> {
		let err = MethodResponse::error(self.id, err.into());
		self.inner.send(err.result.clone()).await.map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;
		self.subscribe.send(err).map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;

		Ok(())
	}

	/// Attempt to accept the subscription and respond the subscription method call.
	///
	/// Fails if the connection was closed or the message was too large.
	pub async fn accept(self) -> Result<SubscriptionSink, SubscriptionAcceptRejectError> {
		let response =
			MethodResponse::response(self.id, &self.uniq_sub.sub_id, self.inner.max_response_size() as usize);
		let success = response.success;
		self.inner.send(response.result.clone()).await.map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;
		self.subscribe.send(response).map_err(|_| SubscriptionAcceptRejectError::RemotePeerAborted)?;

		if success {
			let (tx, rx) = mpsc::channel(1);
			self.subscribers.lock().insert(self.uniq_sub.clone(), (self.inner.clone(), rx));
			Ok(SubscriptionSink {
				inner: self.inner,
				method: self.method,
				subscribers: self.subscribers,
				uniq_sub: self.uniq_sub,
				unsubscribe: IsUnsubscribed(tx),
				_permit: Arc::new(self.permit),
			})
		} else {
			Err(SubscriptionAcceptRejectError::MessageTooLarge)
		}
	}
}

/// Represents a single subscription that hasn't been processed yet.
#[derive(Debug, Clone)]
pub struct SubscriptionSink {
	/// Sink.
	inner: MethodSink,
	/// MethodCallback.
	method: &'static str,
	/// Shared Mutex of subscriptions for this method.
	subscribers: Subscribers,
	/// Unique subscription.
	uniq_sub: SubscriptionKey,
	/// A future to that fires once the unsubscribe method has been called.
	unsubscribe: IsUnsubscribed,
	/// Subscription permit
	_permit: Arc<SubscriptionPermit>,
}

impl SubscriptionSink {
	/// Get the subscription ID.
	pub fn subscription_id(&self) -> RpcSubscriptionId<'static> {
		self.uniq_sub.sub_id.clone()
	}

	/// Get the method name.
	pub fn method_name(&self) -> &str {
		self.method
	}

	/// Send out a response on the subscription and wait until there is capacity.
	///
	///
	/// Returns
	/// - `Ok(())` if the message could be sent.
	/// - `Err(err)` if the connection or subscription was closed.
	///
	/// # Cancel safety
	///
	/// This method is cancel-safe and dropping a future loses its spot in the waiting queue.
	pub async fn send(&self, msg: SubscriptionMessage) -> Result<(), DisconnectError> {
		// Only possible to trigger when the connection is dropped.
		if self.is_closed() {
			return Err(DisconnectError(msg));
		}

		let json = self.sub_message_to_json(msg, SubNotifResultOrError::Result);
		self.inner.send(json).await.map_err(Into::into)
	}

	/// Similar to to `SubscriptionSink::send` but only waits for a limited time.
	pub async fn send_timeout(&self, msg: SubscriptionMessage, timeout: Duration) -> Result<(), SendTimeoutError> {
		// Only possible to trigger when the connection is dropped.
		if self.is_closed() {
			return Err(SendTimeoutError::Closed(msg));
		}

		let json = self.sub_message_to_json(msg, SubNotifResultOrError::Result);
		self.inner.send_timeout(json, timeout).await.map_err(Into::into)
	}

	/// Attempts to immediately send out the message as JSON string to the subscribers but fails if the
	/// channel is full or the connection/subscription is closed
	///
	///
	/// This differs from [`SubscriptionSink::send`] where it will until there is capacity
	/// in the channel.
	pub fn try_send(&mut self, msg: SubscriptionMessage) -> Result<(), TrySendError> {
		// Only possible to trigger when the connection is dropped.
		if self.is_closed() {
			return Err(TrySendError::Closed(msg));
		}

		let json = self.sub_message_to_json(msg, SubNotifResultOrError::Result);
		self.inner.try_send(json).map_err(Into::into)
	}

	/// Returns whether the subscription is closed.
	pub fn is_closed(&self) -> bool {
		self.inner.is_closed() || !self.is_active_subscription()
	}

	/// Completes when the subscription has been closed.
	pub async fn closed(&self) {
		// Both are cancel-safe thus ok to use select here.
		tokio::select! {
			_ = self.inner.closed() => (),
			_ = self.unsubscribe.unsubscribed() => (),
		}
	}

	fn sub_message_to_json(&self, msg: SubscriptionMessage, result_or_err: SubNotifResultOrError) -> String {
		let result_or_err = result_or_err.as_str();

		match msg.0 {
			SubscriptionMessageInner::Complete(msg) => msg,
			SubscriptionMessageInner::NeedsData(result) => {
				let sub_id = serde_json::to_string(&self.uniq_sub.sub_id).expect("valid JSON; qed");
				let method = self.method;
				format!(
					r#"{{"jsonrpc":"2.0","method":"{method}","params":{{"subscription":{sub_id},"{result_or_err}":{result}}}}}"#,
				)
			}
		}
	}

	/// Close the subscription, sending a notification with a special `error` field containing the provided close reason.
	///
	/// This can be used to signal that an subscription was closed because of some particular state
	/// and doesn't imply that subscription was closed because of an error occurred. Just
	/// a custom way to indicate to the client that the subscription was closed.
	///
	/// If you'd like to to close the subscription without sending an extra notification,
	/// just drop it and don't call this method.
	///
	///
	/// ```json
	/// {
	///  "jsonrpc": "2.0",
	///  "method": "<method>",
	///  "params": {
	///    "subscription": "<subscriptionID>",
	///    "error": <your msg>
	///    }
	///  }
	/// }
	/// ```
	///
	pub fn close_with_error(self, msg: SubscriptionMessage) -> impl Future<Output = ()> {
		self.inner_close(msg, SubNotifResultOrError::Error)
	}

	fn inner_close(self, msg: SubscriptionMessage, result_or_err: SubNotifResultOrError) -> impl Future<Output = ()> {
		if self.is_active_subscription() {
			if let Some((sink, _)) = self.subscribers.lock().remove(&self.uniq_sub) {
				tracing::debug!("Closing subscription: {:?}", self.uniq_sub.sub_id);
				let msg = self.sub_message_to_json(msg, result_or_err);

				return Either::Right(async move {
					// This only fails if the connection was closed
					// Fine to ignore
					let _ = sink.send(msg).await;
				});
			}
		}
		Either::Left(futures_util::future::ready(()))
	}

	fn is_active_subscription(&self) -> bool {
		!self.unsubscribe.is_unsubscribed()
	}
}

impl Drop for SubscriptionSink {
	fn drop(&mut self) {
		if self.is_active_subscription() {
			self.subscribers.lock().remove(&self.uniq_sub);
		}
	}
}
