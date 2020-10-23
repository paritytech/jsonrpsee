// Copyright 2019 Parity Technologies (UK) Ltd.
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

use crate::common;

use async_std::net::{TcpListener, TcpStream};
use futures::{channel::mpsc, prelude::*};
use soketto::handshake::{server::Response, Server};
use std::{
	collections::HashMap,
	fmt, io,
	net::SocketAddr,
	pin::Pin,
	sync::{atomic, Arc},
};

/// Event that the [`TransportServer`] can generate.
#[derive(Debug, PartialEq)]
pub enum TransportServerEvent<T> {
	/// A new request has arrived on the wire.
	///
	/// This generates a new "request object" within the state of the [`TransportServer`] that is
	/// identified through the returned `id`. You can then use the other methods of the
	/// [`TransportServer`] trait in order to manipulate that request.
	Request {
		/// Identifier of the request within the state of the [`TransportServer`].
		id: T,
		/// Body of the request.
		request: common::Request,
	},

	/// A request has been cancelled, most likely because the client has closed the connection.
	///
	/// The corresponding request is no longer valid to manipulate.
	Closed(T),
}

/// Implementation of a raw server for WebSockets requests.
//
// # Implementation notes
//
// Every time a connection is received on the TCP listener, we create a new task dedicated to
// processing this specific connection.
//
// These tasks receive a copy of [`WsTransportServer::to_front`], which they use to report
// requests that are received or closed. When a new request is received, it gets assigned an ID
// pooled from [`WsTransportServer::next_request_id`], then a message gets sent to
// [`WsTransportServer::to_front`] and the request gets inserted in
// [`WsTransportServer::to_connections`].
//
// If a task finishes, it must return the list of requests that were assigned to it so that they
// get removed from [`WsTransportServer::to_connections`].
pub struct WsTransportServer {
	/// Local socket address.
	local_addr: SocketAddr,
	/// List of events to for `next_request` to immediately produce.
	pending_events: Vec<TransportServerEvent<WsRequestId>>,
	/// Endpoint for incoming TCP sockets.
	listener: TcpListener,
	/// Next identifier to assign to a request. Shared amongst all the tasks in the server so that
	/// they all assign from the same pool.
	next_request_id: Arc<atomic::AtomicU64>,
	/// Events received from connections.
	from_connections: mpsc::Receiver<BackToFront>,
	/// Sending side of [`WsTransportServer::from_connections`]. Cloned in each member of
	/// [`WsTransportServer::connections_tasks`].
	to_front: mpsc::Sender<BackToFront>,
	/// List of connections, and senders to send them messages.
	to_connections: HashMap<WsRequestId, mpsc::Sender<FrontToBack>>,
	/// List of connections. Must be processed for the system to work. When a task finishes, it
	/// returns the list of pending requests that should now be closed.
	connections_tasks: stream::FuturesUnordered<Pin<Box<dyn Future<Output = Vec<WsRequestId>> + Send>>>,
}

/// Message sent from a per-connection task to the main frontend.
enum BackToFront {
	NewRequest { id: WsRequestId, body: common::Request, sender: mpsc::Sender<FrontToBack> },
}

/// Message sent from the main frontend to a per-connection task.
enum FrontToBack {
	/// Send a payload to the client.
	Send(String),
	/// No more data concerning that request will be sent.
	Finished(WsRequestId),
}

/// Identifier for a request made to a WebSocket server.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WsRequestId(u64);

/// Builder for a [`WsTransportServer`].
pub struct WsTransportServerBuilder {
	/// IP address to try to bind to.
	bind: SocketAddr,
}

impl WsTransportServer {
	/// Creates a new [`WsTransportServerBuilder`] containing the given address and hostname.
	pub fn builder(bind: SocketAddr) -> WsTransportServerBuilder {
		WsTransportServerBuilder { bind }
	}

	/// Local socket address.
	pub fn local_addr(&self) -> &SocketAddr {
		&self.local_addr
	}
}

// former `trait TransportServer` impl.
impl WsTransportServer {
	/// Returns the next event that the raw server wants to notify us.
	pub fn next_request<'a>(
		&'a mut self,
	) -> Pin<Box<dyn Future<Output = TransportServerEvent<WsRequestId>> + Send + 'a>> {
		Box::pin(async move {
			loop {
				if !self.pending_events.is_empty() {
					return self.pending_events.remove(0);
				} else {
					self.pending_events.shrink_to_fit();
				}

				enum Event {
					TaskFinished(Vec<WsRequestId>),
					NewConnection(TcpStream),
					Event(BackToFront),
				}

				let next = {
					let next_connection = {
						let listener = &self.listener;
						async move {
							loop {
								if let Ok((connec, _)) = listener.accept().await {
									break Event::NewConnection(connec);
								}
							}
						}
					};

					let next_event = {
						let from_connections = &mut self.from_connections;
						async move { Event::Event(from_connections.next().await.unwrap()) }
					};

					let next_finished_task = {
						let connections_tasks = &mut self.connections_tasks;
						async move { Event::TaskFinished(connections_tasks.next().await.unwrap()) }
					};

					futures::pin_mut!(next_connection, next_event, next_finished_task);
					match future::select(future::select(next_connection, next_event), next_finished_task).await {
						future::Either::Left((future::Either::Left((ev, _)), _)) => ev,
						future::Either::Left((future::Either::Right((ev, _)), _)) => ev,
						future::Either::Right((ev, _)) => ev,
					}
				};

				match next {
					Event::NewConnection(connec) => {
						log::debug!("new connection with id: {:?}", self.next_request_id);
						self.connections_tasks.push(
							per_connection_task(connec, self.next_request_id.clone(), self.to_front.clone()).boxed(),
						);
					}
					Event::Event(BackToFront::NewRequest { id, body, sender }) => {
						log::debug!("new request with id: {:?}", id);
						let _was_in = self.to_connections.insert(id.clone(), sender);
						debug_assert!(_was_in.is_none());
						return TransportServerEvent::Request { id, request: body };
					}
					Event::TaskFinished(list) => {
						for rq_id in list {
							let was_in = self.to_connections.remove(&rq_id);

							// It is possible that `per_connection_task` returns a list
							// with non-existing connections if `task stream`
							// gets read twice before the actual `event stream` gets read.
							//
							//  -> Poll #1 Ok(read x bytes)
							//  -> Poll #2 Err(receiver closed)
							//
							// The `ws::raw::tests::request_work` is emperic proof of it.
							if was_in.is_some() {
								log::debug!("closed connection with id: {:?}", rq_id);
								self.pending_events.push(TransportServerEvent::Closed(rq_id));
							}
						}
					}
				}
			}
		})
	}

	/// Sends back a response and destroys the request.
	///
	/// You can pass `None` in order to destroy the request object without sending back anything.
	///
	/// The implementation blindly sends back the response and doesn't check whether there is any
	/// correspondance with the request in terms of logic. For example, `respond` will accept
	/// sending back a batch of six responses even if the original request was a single
	/// notification.
	///
	/// > **Note**: While this method returns a `Future` that must be driven to completion,
	/// >           implementations must be aware that the entire requests processing logic is
	/// >           blocked for as long as this `Future` is pending. As an example, you shouldn't
	/// >           use this `Future` to send back a TCP message, because if the remote is
	/// >           unresponsive and the buffers full, the `Future` would then wait for a long time.
	///
	pub fn finish<'a>(
		&'a mut self,
		request_id: &'a WsRequestId,
		response: Option<&'a common::Response>,
	) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
		Box::pin(async move {
			if let Some(mut sender) = self.to_connections.remove(request_id) {
				let serialized = serde_json::to_string(&response).map_err(|_| ())?;
				sender.send(FrontToBack::Send(serialized)).await.map_err(|_| ())?;
				sender.send(FrontToBack::Finished(*request_id)).await.map_err(|_| ())?;
				Ok(())
			} else {
				Err(())
			}
		})
	}

	/// Returns true if this implementation supports sending back data on this request without
	/// closing it.
	///
	/// Returns an error if the request id is invalid.
	pub fn supports_resuming(&self, request_id: &WsRequestId) -> Result<bool, ()> {
		if self.to_connections.contains_key(request_id) {
			Ok(true)
		} else {
			Err(())
		}
	}

	/// Sends back some data on the request and keeps the request alive.
	///
	/// You can continue sending data on that same request later.
	///
	/// Returns an error if the request identifier is incorrect, or if the implementation doesn't
	/// support that operation (see [`supports_resuming`](TransportServer::supports_resuming)).
	pub fn send<'a>(
		&'a mut self,
		request_id: &'a WsRequestId,
		response: &'a common::Response,
	) -> Pin<Box<dyn Future<Output = Result<(), ()>> + Send + 'a>> {
		Box::pin(async move {
			if let Some(sender) = self.to_connections.get_mut(request_id) {
				let serialized = serde_json::to_string(&response).map_err(|_| ())?;
				sender.send(FrontToBack::Send(serialized)).await.map_err(|_| ())?;
			}
			Ok(())
		})
	}
}

impl fmt::Debug for WsTransportServer {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_tuple("WsTransportServer").finish()
	}
}

impl WsTransportServerBuilder {
	/// Try establish the connection.
	pub async fn build(self) -> Result<WsTransportServer, io::Error> {
		let listener = TcpListener::bind(self.bind).await?;
		let local_addr = listener.local_addr()?;

		let connections_tasks = {
			let futures = stream::FuturesUnordered::new();
			// We push a dummy future in order for the `FuturesUnordered` to never produce `None`.
			futures.push(
				async move {
					loop {
						futures::pending!()
					}
				}
				.boxed(),
			);
			futures
		};

		let (to_front, from_connections) = mpsc::channel(256);

		Ok(WsTransportServer {
			local_addr,
			pending_events: Vec::new(),
			listener,
			next_request_id: Arc::new(atomic::AtomicU64::new(1)),
			connections_tasks,
			to_front,
			from_connections,
			to_connections: HashMap::new(),
		})
	}
}

/// Processes a single connection.
//
// TODO: document this function it is quite hard to understand the outcome when it returns `Vec<WsRequestId>`
// both when an error or if the actual connection was terminated.
async fn per_connection_task(
	socket: TcpStream,
	next_request_id: Arc<atomic::AtomicU64>,
	mut to_front: mpsc::Sender<BackToFront>,
) -> Vec<WsRequestId> {
	let mut server = Server::new(socket);

	// Process the handshake from the client.
	let websocket_key = match server.receive_request().await {
		Ok(req) => req.into_key(),
		Err(_) => return Vec::new(),
	};

	// Accept the client unconditionally.
	{
		let res = server.send_response(&{ Response::Accept { key: &websocket_key, protocol: None } }).await;
		if res.is_err() {
			return Vec::new();
		}
	}

	let (mut sender, receiver) = server.into_builder().finish();
	let mut pending_requests = Vec::new();
	let (to_connec, mut from_front) = mpsc::channel(16);

	let socket_packets = stream::unfold(receiver, move |mut receiver| async {
		let mut buf = Vec::new();
		let ret = match receiver.receive_data(&mut buf).await {
			// data is text or binary.
			Ok(_) => Ok(buf),
			Err(err) => Err(err),
		};
		Some((ret, receiver))
	});
	futures::pin_mut!(socket_packets);

	loop {
		let next_from_front = from_front.next();
		let next_socket_packet = socket_packets.next();
		futures::pin_mut!(next_socket_packet, next_from_front);
		match future::select(next_socket_packet, next_from_front).await {
			future::Either::Left((socket_packet, _)) => {
				let socket_packet = match socket_packet {
					Some(Ok(pq)) => {
						log::debug!("received text data from WebSocket: {:?}", pq);
						pq
					}
					Some(Err(err)) => {
						log::error!("failed to receive data from WebSocket: {:?}", err);
						return pending_requests;
					}
					None => {
						log::error!("failed to receive data from Websocket channel closed");
						return pending_requests;
					}
				};

				let body = match serde_json::from_slice(socket_packet.as_ref()) {
					Ok(b) => b,
					Err(err) => {
						log::debug!("Deserialization of incoming request failed: {:?}", err);
						let response = serde_json::to_string(&crate::common::Response::from(
							crate::common::Error::parse_error(),
							crate::common::Version::V2,
						))
						.expect("valid JSON; qed");

						match sender.send_text(&response).await {
							// deserialization failed but the client is still alive
							Ok(_) => continue,
							// deserialization failed and the client is not alive
							Err(e) => {
								log::warn!(
									"Failed to send: {:?} over WebSocket transport with error: {:?}",
									response,
									e
								);
								return pending_requests;
							}
						}
					}
				};

				let request_id = WsRequestId(next_request_id.fetch_add(1, atomic::Ordering::Relaxed));
				debug_assert_ne!(request_id.0, u64::max_value());

				// Important note: since the background task sends messages to the front task via
				// a channel, and the front task sends messages to the background task via a
				// channel as well, and considering that these channels are bounded, a deadlock
				// situation would arise if both the front and background task waited while trying
				// to send something while both channels are full.
				// In order to prevent this from happening, the background -> front sending never
				// blocks. If the back -> front channel is full, we simply kill the task, which
				// has the same effect as a disconnect.
				// The channel is normally large enough for this to never happen unless the server
				// is considerably slowed down or subject to a DoS attack.
				let result = to_front
					.send(BackToFront::NewRequest { id: request_id, body, sender: to_connec.clone() })
					.now_or_never();

				match result {
					// Request was succesfully transmitted to the frontend.
					Some(Ok(_)) => pending_requests.push(request_id),
					// The channel is down or full.
					Some(Err(e)) => {
						log::error!(
							"send request={:?} to frontend failed because of {:?}, terminating the connection",
							request_id,
							e,
						);
						return pending_requests;
					}
					// The future wasn't ready.
					// TODO(niklasad1): verify if this is possible to happen "in practice".
					None => {
						log::error!(
							"send request={:?} to frontend failed future not ready, terminating the connection",
							request_id,
						);
						return pending_requests;
					}
				}
			}

			// Received data to send on the connection.
			future::Either::Right((Some(FrontToBack::Send(to_send)), _)) => {
				log::trace!("transmit: {:?}", to_send);
				if let Err(err) = sender.send_text(&to_send).await {
					log::warn!("failed to send: {:?} over WebSocket transport with error: {:?}", to_send, err);
					return pending_requests;
				}
			}

			// Received data to send on the connection.
			future::Either::Right((Some(FrontToBack::Finished(rq_id)), _)) => {
				log::trace!("finished request_id={:?}", rq_id);
				let pos = pending_requests.iter().position(|r| *r == rq_id).unwrap();
				pending_requests.remove(pos);
			}

			// Channel to main WS server struct has closed. Let's close the task.
			future::Either::Right((None, _)) => return pending_requests,
		}
	}
}
