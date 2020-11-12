// Copyright 2019-2020 Parity Technologies (UK) Ltd.
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

use crate::http::raw::{RawServer, RawServerEvent, RawServerRequestId};
use crate::http::transport::HttpTransportServer;
use crate::types::http::HttpConfig;
use crate::types::jsonrpc::{self, JsonValue};
use crate::types::server::Error;

use futures::{channel::mpsc, future::Either, pin_mut, prelude::*};
use parking_lot::Mutex;
use std::{
	collections::{HashMap, HashSet},
	error,
	net::SocketAddr,
	sync::{atomic, Arc},
};

/// Server that can be cloned.
///
/// > **Note**: This struct is designed to be easy to use, but it works by maintaining a background
/// >           task running in parallel. If this is not desirable, you are encouraged to use the
/// >           [`RawServer`] struct instead.
#[derive(Clone)]
pub struct Server {
	/// Local socket address of the transport server.
	local_addr: SocketAddr,
	/// Channel to send requests to the background task.
	to_back: mpsc::UnboundedSender<FrontToBack>,
	/// List of methods (for RPC queries, subscriptions, and unsubscriptions) that have been
	/// registered. Serves no purpose except to check for duplicates.
	registered_methods: Arc<Mutex<HashSet<String>>>,
	/// Next unique ID used when registering a subscription.
	next_subscription_unique_id: Arc<atomic::AtomicUsize>,
}

/// Notification method that's been registered.
pub struct RegisteredNotification {
	/// Receives notifications that the client sent to us.
	queries_rx: mpsc::Receiver<jsonrpc::Params>,
}

/// Method that's been registered.
pub struct RegisteredMethod {
	/// Clone of [`Server::to_back`].
	to_back: mpsc::UnboundedSender<FrontToBack>,
	/// Receives requests that the client sent to us.
	queries_rx: mpsc::Receiver<(RawServerRequestId, jsonrpc::Params)>,
}

/// Active request that needs to be answered.
pub struct IncomingRequest {
	/// Clone of [`Server::to_back`].
	to_back: mpsc::UnboundedSender<FrontToBack>,
	/// Identifier of the request towards the server.
	request_id: RawServerRequestId,
	/// Parameters of the request.
	params: jsonrpc::Params,
}

/// Message that the [`Server`] can send to the background task.
enum FrontToBack {
	/// Registers a notifications endpoint.
	RegisterNotifications {
		/// Name of the method.
		name: String,
		/// Where to send incoming notifications.
		handler: mpsc::Sender<jsonrpc::Params>,
		/// See the documentation of [`Server::register_notifications`].
		allow_losses: bool,
	},

	/// Registers a method. The server will then handle requests using this method.
	RegisterMethod {
		/// Name of the method.
		name: String,
		/// Where to send requests.
		handler: mpsc::Sender<(RawServerRequestId, jsonrpc::Params)>,
	},

	/// Send a response to a request that a client made.
	AnswerRequest {
		/// Request to answer.
		request_id: RawServerRequestId,
		/// Response to send back.
		answer: Result<JsonValue, jsonrpc::Error>,
	},
}

impl Server {
	/// Initializes a new server based upon this raw server.
	pub async fn new(url: &str, config: HttpConfig) -> Result<Self, Box<dyn error::Error + Send + Sync>> {
		let sockaddr = url.parse()?;
		let transport_server = HttpTransportServer::new(&sockaddr, config).await?;
		let local_addr = *transport_server.local_addr();

		// We use an unbounded channel because the only exchanged messages concern registering
		// methods. The volume of messages is therefore very low and it doesn't make sense to have
		// a backpressure mechanism.
		// TODO: that's not true anymore ^
		let (to_back, from_front) = mpsc::unbounded();

		async_std::task::spawn(async move {
			background_task(transport_server.into(), from_front).await;
		});

		Ok(Server {
			local_addr,
			to_back,
			registered_methods: Arc::new(Mutex::new(Default::default())),
			next_subscription_unique_id: Arc::new(atomic::AtomicUsize::new(0)),
		})
	}

	/// Local socket address of the transport server.
	pub fn local_addr(&self) -> &SocketAddr {
		&self.local_addr
	}

	/// Registers a notification method name towards the server.
	///
	/// Clients will then be able to call this method.
	/// The returned object allows you to process incoming notifications.
	///
	/// If `allow_losses` is true, then the server is allowed to drop notifications if the
	/// notifications handler (i.e. the code that uses [`RegisteredNotifications`]) is too slow
	/// to process notifications.
	///
	/// Returns an error if the method name was already registered.
	pub fn register_notification(
		&self,
		method_name: String,
		allow_losses: bool,
	) -> Result<RegisteredNotification, Error> {
		if !self.registered_methods.lock().insert(method_name.clone()) {
			return Err(Error::AlreadyRegistered(method_name));
		}

		log::trace!("[frontend]: register_notification={}", method_name);
		let (tx, rx) = mpsc::channel(32);

		self.to_back
			.unbounded_send(FrontToBack::RegisterNotifications { name: method_name, handler: tx, allow_losses })
			.map_err(|e| Error::InternalChannel(e.into_send_error()))?;

		Ok(RegisteredNotification { queries_rx: rx })
	}

	/// Registers a method towards the server.
	///
	/// Clients will then be able to call this method.
	/// The returned object allows you to handle incoming requests.
	///
	/// Contrary to [`register_notifications`](Server::register_notifications), there is no
	/// `allow_losses` parameter here. If the handler is too slow to process requests, then the
	/// server automatically returns an "internal error" to the client.
	///
	/// Returns an error if the method name was already registered.
	pub fn register_method(&self, method_name: String) -> Result<RegisteredMethod, Error> {
		if !self.registered_methods.lock().insert(method_name.clone()) {
			return Err(Error::AlreadyRegistered(method_name));
		}

		log::trace!("[frontend]: register_method={}", method_name);
		let (tx, rx) = mpsc::channel(32);

		self.to_back
			.unbounded_send(FrontToBack::RegisterMethod { name: method_name, handler: tx })
			.map_err(|e| Error::InternalChannel(e.into_send_error()))?;

		Ok(RegisteredMethod { to_back: self.to_back.clone(), queries_rx: rx })
	}
}

impl RegisteredNotification {
	/// Returns the next notification.
	pub async fn next(&mut self) -> jsonrpc::Params {
		loop {
			match self.queries_rx.next().await {
				Some(v) => break v,
				None => futures::pending!(),
			}
		}
	}
}

impl RegisteredMethod {
	/// Returns the next request.
	pub async fn next(&mut self) -> IncomingRequest {
		let (request_id, params) = loop {
			match self.queries_rx.next().await {
				Some(v) => break v,
				None => futures::pending!(),
			}
		};
		IncomingRequest { to_back: self.to_back.clone(), request_id, params }
	}
}

impl IncomingRequest {
	/// Returns the parameters of the request.
	pub fn params(&self) -> &jsonrpc::Params {
		&self.params
	}

	/// Respond to the request.
	pub async fn respond(mut self, response: impl Into<Result<JsonValue, jsonrpc::Error>>) -> Result<(), Error> {
		self.to_back
			.send(FrontToBack::AnswerRequest { request_id: self.request_id, answer: response.into() })
			.await
			.map_err(Error::InternalChannel)
	}
}

/// Function being run in the background that processes messages from the frontend.
async fn background_task(mut server: RawServer, mut from_front: mpsc::UnboundedReceiver<FrontToBack>) {
	// List of notifications methods that the user has registered, and the channels to dispatch
	// incoming notifications.
	let mut registered_notifications: HashMap<String, (mpsc::Sender<_>, bool)> = HashMap::new();
	// List of methods that the user has registered, and the channels to dispatch incoming
	// requests.
	let mut registered_methods: HashMap<String, mpsc::Sender<_>> = HashMap::new();

	loop {
		// We need to do a little transformation in order to destroy the borrow to `client`
		// and `from_front`.
		let outcome = {
			let next_message = from_front.next();
			let next_event = server.next_event();
			pin_mut!(next_message);
			pin_mut!(next_event);
			match future::select(next_message, next_event).await {
				Either::Left((v, _)) => Either::Left(v),
				Either::Right((v, _)) => Either::Right(v),
			}
		};

		match outcome {
			Either::Left(None) => {
				log::trace!("[backend]: background_task terminated");
				return;
			}
			Either::Left(Some(FrontToBack::AnswerRequest { request_id, answer })) => {
				log::trace!("[backend]: answer_request: {:?} id: {:?}", answer, request_id);
				server.request_by_id(&request_id).unwrap().respond(answer);
			}
			Either::Left(Some(FrontToBack::RegisterNotifications { name, handler, allow_losses })) => {
				log::trace!("[backend]: register_notification: {:?}", name);
				registered_notifications.insert(name, (handler, allow_losses));
			}
			Either::Left(Some(FrontToBack::RegisterMethod { name, handler })) => {
				log::trace!("[backend]: register_method: {:?}", name);
				registered_methods.insert(name, handler);
			}
			Either::Right(RawServerEvent::Notification(notification)) => {
				log::trace!("[backend]: received notification: {:?}", notification);
				if let Some((handler, allow_losses)) = registered_notifications.get_mut(notification.method()) {
					let params: &jsonrpc::Params = notification.params().into();
					// Note: we just ignore errors. It doesn't make sense logically speaking to
					// unregister the notification here.
					if *allow_losses {
						let _ = handler.send(params.clone()).now_or_never();
					} else {
						let _ = handler.send(params.clone()).await;
					}
				}
			}
			Either::Right(RawServerEvent::Request(request)) => {
				log::trace!("[backend]: received request: {:?}", request);
				if let Some(handler) = registered_methods.get_mut(request.method()) {
					let params: &jsonrpc::Params = request.params().into();
					match handler.send((request.id(), params.clone())).now_or_never() {
						Some(Ok(())) => {}
						Some(Err(_)) | None => {
							request.respond(Err(From::from(jsonrpc::ErrorCode::ServerError(0))));
						}
					}
				} else {
					// TODO: we assert that the request is valid because the parsing succeeded but
					// not registered.
					request.respond(Err(From::from(jsonrpc::ErrorCode::MethodNotFound)));
				}
			}
		}
	}
}
