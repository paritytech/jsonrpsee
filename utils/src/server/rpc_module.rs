use crate::server::helpers::{send_error, send_response};
use futures_channel::mpsc;
use jsonrpsee_types::error::{CallError, Error};
use jsonrpsee_types::traits::RpcMethod;
use jsonrpsee_types::v2::error::{JsonRpcErrorCode, JsonRpcErrorObject, CALL_EXECUTION_FAILED_CODE};
use jsonrpsee_types::v2::params::{Id, JsonRpcNotificationParams, RpcParams, TwoPointZero};
use jsonrpsee_types::v2::response::JsonRpcSubscriptionResponse;

use parking_lot::Mutex;
use rustc_hash::FxHashMap;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::value::{to_raw_value, RawValue};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

/// A `Method` is an RPC endpoint, callable with a standard JSON-RPC request,
/// implemented as a function pointer to a `Fn` function taking four arguments:
/// the `id`, `params`, a channel the function uses to communicate the result (or error)
/// back to `jsonrpsee`, and the connection ID (useful for the websocket transport).
pub type Method = Box<dyn Send + Sync + Fn(Id, RpcParams, &MethodSink, ConnectionId) -> anyhow::Result<()>>;
/// A collection of registered [`Method`]s.
pub type Methods = FxHashMap<&'static str, Method>;
/// Connection ID, used for stateful protocol such as WebSockets.
/// For stateless protocols such as http it's unused, so feel free to set it some hardcoded value.
pub type ConnectionId = usize;
/// Subscription ID.
pub type SubscriptionId = u64;
/// Sink that is used to send back the result to the server for a specific method.
pub type MethodSink = mpsc::UnboundedSender<String>;

/// Map of subscribers keyed by the connection and subscription ids to an [`InnerSink`] that contains the parameters
/// they used to subscribe and the tx side of a channel used to convey results and errors back.
type Subscribers<P> = Arc<Mutex<FxHashMap<(ConnectionId, SubscriptionId), InnerSink<P>>>>;

/// Sets of JSON-RPC methods can be organized into a "module"s that are in turn registered on the server or,
/// alternatively, merged with other modules to construct a cohesive API.
#[derive(Default)]
pub struct RpcModule {
	methods: Methods,
}

impl RpcModule {
	/// Instantiate a new `RpcModule`.
	pub fn new() -> Self {
		RpcModule { methods: Methods::default() }
	}

	/// Add context for this module, turning it into an `RpcContextModule`.
	pub fn with_context<Context>(self, ctx: Context) -> RpcContextModule<Context> {
		RpcContextModule { ctx: Arc::new(ctx), module: self }
	}

	fn verify_method_name(&mut self, name: &str) -> Result<(), Error> {
		if self.methods.get(name).is_some() {
			return Err(Error::MethodAlreadyRegistered(name.into()));
		}

		Ok(())
	}

	/// Register a new RPC method, which responds with a given callback.
	pub fn register_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		R: Serialize,
		F: RpcMethod<R, CallError>,
	{
		self.verify_method_name(method_name)?;

		self.methods.insert(
			method_name,
			Box::new(move |id, params, tx, _| {
				match callback(params) {
					Ok(res) => send_response(id, tx, res),
					Err(CallError::InvalidParams) => send_error(id, tx, JsonRpcErrorCode::InvalidParams.into()),
					Err(CallError::Failed(err)) => {
						log::error!("Call failed with: {}", err);
						let err = JsonRpcErrorObject {
							code: JsonRpcErrorCode::ServerError(CALL_EXECUTION_FAILED_CODE),
							message: &err.to_string(),
							data: None,
						};
						send_error(id, tx, err)
					}
				};

				Ok(())
			}),
		);

		Ok(())
	}

	/// Register a new RPC subscription, with subscribe and unsubscribe methods. Returns a [`SubscriptionSink`]. If a
	/// method with the same name is already registered, an [`Error::MethodAlreadyRegistered`] is returned.
	/// If the subscription does not take any parameters, set `Params` to `()`.
	pub fn register_subscription<Params: DeserializeOwned + Send + Sync + 'static>(
		&mut self,
		subscribe_method_name: &'static str,
		unsubscribe_method_name: &'static str,
	) -> Result<SubscriptionSink<Params>, Error> {
		if subscribe_method_name == unsubscribe_method_name {
			return Err(Error::SubscriptionNameConflict(subscribe_method_name.into()));
		}

		self.verify_method_name(subscribe_method_name)?;
		self.verify_method_name(unsubscribe_method_name)?;

		let subscribers = Arc::new(Mutex::new(FxHashMap::default()));

		{
			let subscribers = subscribers.clone();
			self.methods.insert(
				subscribe_method_name,
				Box::new(move |id, params, tx, conn| {
					let params = match params.parse().or_else(|_| params.one()) {
						Ok(p) => p,
						Err(err) => {
							log::error!("Params={:?}, in subscription couldn't be parsed: {:?}", params, err);
							return Err(err.into());
						}
					};
					let sub_id = {
						const JS_NUM_MASK: SubscriptionId = !0 >> 11;

						let sub_id = rand::random::<SubscriptionId>() & JS_NUM_MASK;

						let inner = InnerSink { sink: tx.clone(), params, method: subscribe_method_name, sub_id };
						subscribers.lock().insert((conn, sub_id), inner);

						sub_id
					};

					send_response(id, tx, sub_id);

					Ok(())
				}),
			);
		}

		{
			let subscribers = subscribers.clone();
			self.methods.insert(
				unsubscribe_method_name,
				Box::new(move |id, params, tx, conn| {
					let sub_id = params.one().map_err(|e| anyhow::anyhow!("{:?}", e))?;

					subscribers.lock().remove(&(conn, sub_id));

					send_response(id, tx, "Unsubscribed");

					Ok(())
				}),
			);
		}

		Ok(SubscriptionSink { method: subscribe_method_name, subscribers })
	}

	/// Convert a module into methods.
	pub fn into_methods(self) -> Methods {
		self.methods
	}

	/// Merge two [`RpcModule`]'s by adding all [`Method`]s from `other` into `self`.
	/// Fails if any of the methods in `other` is present already.
	pub fn merge(&mut self, other: RpcModule) -> Result<(), Error> {
		for name in other.methods.keys() {
			self.verify_method_name(name)?;
		}

		for (name, callback) in other.methods {
			self.methods.insert(name, callback);
		}

		Ok(())
	}
}

/// Similar to [`RpcModule`] but wraps an additional context argument that can be used
/// to access data during call execution.
pub struct RpcContextModule<Context> {
	ctx: Arc<Context>,
	module: RpcModule,
}

impl<Context> RpcContextModule<Context> {
	/// Create a new module with a given shared `Context`.
	pub fn new(ctx: Context) -> Self {
		RpcContextModule { ctx: Arc::new(ctx), module: RpcModule::new() }
	}

	/// Register a new RPC method, which responds with a given callback.
	pub fn register_method<R, F>(&mut self, method_name: &'static str, callback: F) -> Result<(), Error>
	where
		Context: Send + Sync + 'static,
		R: Serialize,
		F: Fn(RpcParams, &Context) -> Result<R, CallError> + Send + Sync + 'static,
	{
		self.module.verify_method_name(method_name)?;

		let ctx = self.ctx.clone();

		self.module.methods.insert(
			method_name,
			Box::new(move |id, params, tx, _| {
				match callback(params, &*ctx) {
					Ok(res) => send_response(id, tx, res),
					Err(CallError::InvalidParams) => send_error(id, tx, JsonRpcErrorCode::InvalidParams.into()),
					Err(CallError::Failed(err)) => {
						let err = JsonRpcErrorObject {
							code: JsonRpcErrorCode::ServerError(CALL_EXECUTION_FAILED_CODE),
							message: &err.to_string(),
							data: None,
						};
						send_error(id, tx, err)
					}
				};

				Ok(())
			}),
		);
		Ok(())
	}

	/// Convert this `RpcContextModule` into a regular `RpcModule` that can be registered on the `Server`.
	pub fn into_module(self) -> RpcModule {
		self.module
	}

	/// Convert a module into methods. Consumes self.
	pub fn into_methods(self) -> Methods {
		self.into_module().into_methods()
	}
}

impl<Cx> Deref for RpcContextModule<Cx> {
	type Target = RpcModule;
	fn deref(&self) -> &Self::Target {
		&self.module
	}
}

impl<Cx> DerefMut for RpcContextModule<Cx> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.module
	}
}
/// Used by the server to send data back to subscribers.
#[derive(Clone)]
pub struct SubscriptionSink<Params = ()> {
	method: &'static str,
	subscribers: Subscribers<Params>,
}

impl<Params> SubscriptionSink<Params> {
	/// Send a message to all subscribers.
	///
	/// If you have subscriptions with params/input you should most likely
	/// call `send_each` to the process the input/params and send out
	/// the result on each subscription individually instead.
	pub fn broadcast<T>(&self, result: &T) -> anyhow::Result<()>
	where
		T: Serialize,
	{
		let result = to_raw_value(result)?;

		let mut errored = Vec::new();
		let mut subs = self.subscribers.lock();

		for ((conn_id, sub_id), sink) in subs.iter() {
			// Mark broken connections, to be removed.
			if sink.send_raw_value(&result).is_err() {
				errored.push((*conn_id, *sub_id));
			}
		}

		// Remove broken connections
		for entry in errored {
			subs.remove(&entry);
		}

		Ok(())
	}

	/// Send a message to all subscribers one by one, parsing the params they sent with the provided closure. If the
	/// closure `F` fails to parse the params the message is not sent.
	///
	/// F: is a closure that you need to provide to apply on the input P.
	pub fn send_each<T, F>(&self, f: F) -> anyhow::Result<()>
	where
		F: Fn(&Params) -> anyhow::Result<Option<T>>,
		T: Serialize,
	{
		let mut subs = self.subscribers.lock();
		let mut errored = Vec::new();

		for ((conn_id, sub_id), sink) in subs.iter() {
			match f(&sink.params) {
				Ok(Some(res)) => {
					let result = match to_raw_value(&res) {
						Ok(res) => res,
						Err(err) => {
							log::error!("Subscription: {} failed to serialize message: {:?}; ignoring", sub_id, err);
							continue;
						}
					};

					if sink.send_raw_value(&result).is_err() {
						errored.push((*conn_id, *sub_id));
					}
				}
				// NOTE(niklasad1): This might be used to fetch data in closure.
				Ok(None) => (),
				Err(e) => {
					if sink.inner_send(format!("Error: {:?}", e)).is_err() {
						errored.push((*conn_id, *sub_id));
					}
				}
			}
		}

		// Remove broken connections
		for entry in errored {
			subs.remove(&entry);
		}

		Ok(())
	}

	/// Consumes the current subscriptions at the given time to get access to the inner Sinks.
	/// The SubscriptionSink will accept new subscriptions after this occurs.
	// TODO: we should get rid of this if possible.
	pub fn into_sinks(&self) -> impl IntoIterator<Item = InnerSink<Params>> {
		let mut subs = self.subscribers.lock();
		let sinks = std::mem::take(&mut *subs);
		sinks.into_iter().map(|(_, v)| v)
	}
}

/// Represents a single subscription.
pub struct InnerSink<Params> {
	/// Sink.
	sink: mpsc::UnboundedSender<String>,
	/// Params.
	params: Params,
	/// Method.
	method: &'static str,
	/// Subscription ID.
	sub_id: SubscriptionId,
}

impl<Params> InnerSink<Params> {
	/// Send message on this subscription.
	pub fn send<T: Serialize>(&self, result: &T) -> anyhow::Result<()> {
		let result = to_raw_value(result)?;
		self.send_raw_value(&result)
	}

	fn send_raw_value(&self, result: &RawValue) -> anyhow::Result<()> {
		let msg = serde_json::to_string(&JsonRpcSubscriptionResponse {
			jsonrpc: TwoPointZero,
			method: self.method,
			params: JsonRpcNotificationParams { subscription: self.sub_id, result: &*result },
		})?;

		self.inner_send(msg).map_err(Into::into)
	}

	fn inner_send(&self, msg: String) -> anyhow::Result<()> {
		self.sink.unbounded_send(msg).map_err(Into::into)
	}

	/// Get params of the subscription.
	pub fn params(&self) -> &Params {
		&self.params
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn rpc_context_modules_can_merge_with_rpc_module() {
		// Prove that we can merge an RpcContextModule with a RpcModule.
		let cx = Vec::<u8>::new();
		let mut cxmodule = RpcContextModule::new(cx);
		cxmodule.register_method("bla with context", |_: RpcParams, _| Ok(())).unwrap();
		let mut module = RpcModule::new();
		module.register_method("bla", |_: RpcParams| Ok(())).unwrap();

		// `merge` is a method on `RpcModule` => deref works
		cxmodule.merge(module).unwrap();
		let mut cx_methods = cxmodule.into_methods().keys().cloned().collect::<Vec<&str>>();
		cx_methods.sort();
		assert_eq!(cx_methods, vec!["bla", "bla with context"]);
	}

	#[test]
	fn rpc_context_modules_can_register_subscriptions() {
		let cx = ();
		let mut cxmodule = RpcContextModule::new(cx);
		let _subscription = cxmodule.register_subscription::<()>("hi", "goodbye");

		let methods = cxmodule.into_methods().keys().cloned().collect::<Vec<&str>>();
		assert!(methods.contains(&"hi"));
		assert!(methods.contains(&"goodbye"));
	}
}
