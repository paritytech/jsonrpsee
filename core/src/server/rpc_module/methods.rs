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

use std::collections::hash_map::Entry;
use std::fmt::{Debug};
use std::sync::Arc;

use crate::error::{Error};
use crate::id_providers::RandomIntegerIdProvider;
use crate::server::helpers::{BoundedSubscriptions, MethodSink};
use crate::traits::{ToRpcParams};
use jsonrpsee_types::error::{CallError, ErrorCode, ErrorObject};
use jsonrpsee_types::{
	ErrorResponse, Id, Params, Request, Response, SubscriptionId as RpcSubscriptionId,
};
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned};
use tokio::sync::{mpsc};

use crate::server::helpers::{MethodResponse};

use super::types::*;

/// Reference-counted, clone-on-write collection of synchronous and asynchronous methods.
#[derive(Default, Debug, Clone)]
pub struct Methods {
	pub(crate) callbacks: Arc<FxHashMap<&'static str, MethodCallback>>,
}

impl Methods {
	/// Creates a new empty [`Methods`].
	pub fn new() -> Self {
		Self::default()
	}

	/// Verify a method name is available, or return an error if it's already taken.
	pub fn verify_method_name(&mut self, name: &'static str) -> Result<(), Error> {
		if self.callbacks.contains_key(name) {
			return Err(Error::MethodAlreadyRegistered(name.into()));
		}

		Ok(())
	}

	/// Inserts the method callback for a given name, or returns an error if the name was already taken.
	/// On success it returns a mut reference to the [`MethodCallback`] just inserted.
	pub fn verify_and_insert(
		&mut self,
		name: &'static str,
		callback: MethodCallback,
	) -> Result<&mut MethodCallback, Error> {
		match self.mut_callbacks().entry(name) {
			Entry::Occupied(_) => Err(Error::MethodAlreadyRegistered(name.into())),
			Entry::Vacant(vacant) => Ok(vacant.insert(callback)),
		}
	}

	/// Helper for obtaining a mut ref to the callbacks HashMap.
	pub(crate) fn mut_callbacks(&mut self) -> &mut FxHashMap<&'static str, MethodCallback> {
		Arc::make_mut(&mut self.callbacks)
	}

	/// Merge two [`Methods`]'s by adding all [`MethodCallback`]s from `other` into `self`.
	/// Fails if any of the methods in `other` is present already.
	pub fn merge(&mut self, other: impl Into<Methods>) -> Result<(), Error> {
		let mut other = other.into();

		for name in other.callbacks.keys() {
			self.verify_method_name(name)?;
		}

		let callbacks = self.mut_callbacks();

		for (name, callback) in other.mut_callbacks().drain() {
			callbacks.insert(name, callback);
		}

		Ok(())
	}

	/// Returns the method callback.
	pub fn method(&self, method_name: &str) -> Option<&MethodCallback> {
		self.callbacks.get(method_name)
	}

	/// Returns the method callback along with its name. The returned name is same as the
	/// `method_name`, but its lifetime bound is `'static`.
	pub fn method_with_name(&self, method_name: &str) -> Option<(&'static str, &MethodCallback)> {
		self.callbacks.get_key_value(method_name).map(|(k, v)| (*k, v))
	}

	/// Helper to call a method on the `RPC module` without having to spin up a server.
	///
	/// The params must be serializable as JSON array, see [`ToRpcParams`] for further documentation.
	///
	/// Returns the decoded value of the `result field` in JSON-RPC response if successful.
	///
	/// # Examples
	///
	/// ```
	/// #[tokio::main]
	/// async fn main() {
	///     use jsonrpsee::RpcModule;
	///
	///     let mut module = RpcModule::new(());
	///     module.register_method("echo_call", |params, _| {
	///         params.one::<u64>().map_err(Into::into)
	///     }).unwrap();
	///
	///     let echo: u64 = module.call("echo_call", [1_u64]).await.unwrap();
	///     assert_eq!(echo, 1);
	/// }
	/// ```
	pub async fn call<Params: ToRpcParams, T: DeserializeOwned>(
		&self,
		method: &str,
		params: Params,
	) -> Result<T, Error> {
		let params = params.to_rpc_params()?;
		let req = Request::new(method.into(), params.as_ref().map(|p| p.as_ref()), Id::Number(0));
		tracing::trace!("[Methods::call] Method: {:?}, params: {:?}", method, params);
		let (resp, _, _) = self.inner_call(req, 1).await;

		if resp.success {
			serde_json::from_str::<Response<T>>(&resp.result).map(|r| r.result).map_err(Into::into)
		} else {
			match serde_json::from_str::<ErrorResponse>(&resp.result) {
				Ok(err) => Err(Error::Call(CallError::Custom(err.error_object().clone().into_owned()))),
				Err(e) => Err(e.into()),
			}
		}
	}

	/// Make a request (JSON-RPC method call or subscription) by using raw JSON.
	///
	/// Returns the raw JSON response to the call and a stream to receive notifications if the call was a subscription.
	///
	/// # Examples
	///
	/// ```
	/// #[tokio::main]
	/// async fn main() {
	///     use jsonrpsee::{RpcModule, SubscriptionMessage};
	///     use jsonrpsee::types::Response;
	///     use futures_util::StreamExt;
	///
	///     let mut module = RpcModule::new(());
	///     module.register_subscription("hi", "hi", "goodbye", |_, pending, _| async {
	///         let sink = pending.accept().await?;
	///         let msg = SubscriptionMessage::from_json(&"one answer")?;
	///         sink.send(msg).await?;
	///         Ok(())
	///     }).unwrap();
	///     let (resp, mut stream) = module.raw_json_request(r#"{"jsonrpc":"2.0","method":"hi","id":0}"#, 1).await.unwrap();
	///     let resp = serde_json::from_str::<Response<u64>>(&resp.result).unwrap();
	///     let sub_resp = stream.recv().await.unwrap();
	///     assert_eq!(
	///         format!(r#"{{"jsonrpc":"2.0","method":"hi","params":{{"subscription":{},"result":"one answer"}}}}"#, resp.result),
	///         sub_resp
	///     );
	/// }
	/// ```
	pub async fn raw_json_request(
		&self,
		request: &str,
		buf_size: usize,
	) -> Result<(MethodResponse, mpsc::Receiver<String>), Error> {
		tracing::trace!("[Methods::raw_json_request] Request: {:?}", request);
		let req: Request = serde_json::from_str(request)?;
		let (resp, rx, _) = self.inner_call(req, buf_size).await;

		Ok((resp, rx))
	}

	/// Execute a callback.
	async fn inner_call(&self, req: Request<'_>, buf_size: usize) -> RawRpcResponse {
		let (tx, mut rx) = mpsc::channel(buf_size);
		let id = req.id.clone();
		let params = Params::new(req.params.map(|params| params.get()));
		let bounded_subs = BoundedSubscriptions::new(u32::MAX);
		let p1 = bounded_subs.acquire().expect("u32::MAX permits is sufficient; qed");
		let p2 = bounded_subs.acquire().expect("u32::MAX permits is sufficient; qed");

		let response = match self.method(&req.method).map(|c| &c.callback) {
			None => MethodResponse::error(req.id, ErrorObject::from(ErrorCode::MethodNotFound)),
			Some(MethodKind::Sync(cb)) => (cb)(id, params, usize::MAX),
			Some(MethodKind::Async(cb)) => (cb)(id.into_owned(), params.into_owned(), 0, usize::MAX).await,
			Some(MethodKind::Subscription(cb)) => {
				let conn_state =
					ConnState { conn_id: 0, id_provider: &RandomIntegerIdProvider, subscription_permit: p1 };
				let res = (cb)(id, params, MethodSink::new(tx.clone()), conn_state).await;

				// This message is not used because it's used for metrics so we discard in other to
				// not read once this is used for subscriptions.
				//
				// The same information is part of `res` above.
				let _ = rx.recv().await.expect("Every call must at least produce one response; qed");

				match res {
					SubscriptionAnswered::Yes(r) => r,
					SubscriptionAnswered::No(r) => r,
				}
			}
			Some(MethodKind::Unsubscription(cb)) => (cb)(id, params, 0, usize::MAX),
		};

		tracing::trace!("[Methods::inner_call] Method: {}, response: {:?}", req.method, response);

		(response, rx, p2)
	}

	/// Helper to create a subscription on the `RPC module` without having to spin up a server.
	///
	/// The params must be serializable as JSON array, see [`ToRpcParams`] for further documentation.
	///
	/// Returns [`Subscription`] on success which can used to get results from the subscription.
	///
	/// # Examples
	///
	/// ```
	/// #[tokio::main]
	/// async fn main() {
	///     use jsonrpsee::{RpcModule, core::EmptyServerParams, SubscriptionMessage};
	///
	///     let mut module = RpcModule::new(());
	///     module.register_subscription("hi", "hi", "goodbye", |_, pending, _| async move {
	///         let sink = pending.accept().await?;
	///
	///         let msg = SubscriptionMessage::from_json(&"one answer")?;
	///         sink.send(msg).await?;
	///         Ok(())
	///
	///     }).unwrap();
	///
	///     let mut sub = module.subscribe_unbounded("hi", EmptyServerParams::new()).await.unwrap();
	///     // In this case we ignore the subscription ID,
	///     let (sub_resp, _sub_id) = sub.next::<String>().await.unwrap().unwrap();
	///     assert_eq!(&sub_resp, "one answer");
	/// }
	/// ```
	pub async fn subscribe_unbounded(&self, sub_method: &str, params: impl ToRpcParams) -> Result<Subscription, Error> {
		self.subscribe(sub_method, params, u32::MAX as usize).await
	}

	/// Similar to [`Methods::subscribe_unbounded`] but it's using a bounded channel and the buffer capacity must be provided.
	pub async fn subscribe(
		&self,
		sub_method: &str,
		params: impl ToRpcParams,
		buf_size: usize,
	) -> Result<Subscription, Error> {
		let params = params.to_rpc_params()?;
		let req = Request::new(sub_method.into(), params.as_ref().map(|p| p.as_ref()), Id::Number(0));

		tracing::trace!("[Methods::subscribe] Method: {}, params: {:?}", sub_method, params);

		let (resp, rx, permit) = self.inner_call(req, buf_size).await;

		let subscription_response = match serde_json::from_str::<Response<RpcSubscriptionId>>(&resp.result) {
			Ok(r) => r,
			Err(_) => match serde_json::from_str::<ErrorResponse>(&resp.result) {
				Ok(err) => return Err(Error::Call(CallError::Custom(err.error_object().clone().into_owned()))),
				Err(err) => return Err(err.into()),
			},
		};

		let sub_id = subscription_response.result.into_owned();

		Ok(Subscription { sub_id, rx, _permit: permit })
	}

	/// Returns an `Iterator` with all the method names registered on this server.
	pub fn method_names(&self) -> impl Iterator<Item = &'static str> + '_ {
		self.callbacks.keys().copied()
	}
}
