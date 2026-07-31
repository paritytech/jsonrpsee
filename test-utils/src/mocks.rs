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

use std::convert::Infallible;
use std::io;
use std::net::SocketAddr;
use std::time::Duration;

use futures_channel::mpsc;
use futures_channel::oneshot;
use futures_util::future::FutureExt;
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use futures_util::{pin_mut, select};
use hyper_util::rt::{TokioExecutor, TokioIo};
use serde::{Deserialize, Serialize};
use tokio::net::TcpStream;
use yawc::frame::{Frame, OpCode};
use yawc::{Options, WebSocket, WebSocketError};

pub use hyper::{HeaderMap, StatusCode, Uri};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type EmptyBody = http_body_util::Empty<hyper::body::Bytes>;

/// Request Id
#[derive(Debug, PartialEq, Clone, Hash, Eq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[serde(untagged)]
pub enum Id {
	/// No id (notification)
	Null,
	/// Numeric id
	Num(u64),
	/// String id
	Str(String),
}

#[derive(Debug)]
pub struct HttpResponse {
	pub status: StatusCode,
	pub header: HeaderMap,
	pub body: String,
}

/// WebSocket client to construct with arbitrary payload to construct bad payloads.
pub struct WebSocketTestClient {
	tx: futures_util::stream::SplitSink<WebSocket<TcpStream>, Frame>,
	rx: futures_util::stream::SplitStream<WebSocket<TcpStream>>,
}

impl std::fmt::Debug for WebSocketTestClient {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "WebSocketTestClient")
	}
}

#[derive(Debug)]
pub enum WebSocketTestError {
	Redirect,
	RejectedWithStatusCode(u16),
	Yawc(WebSocketError),
}

impl From<io::Error> for WebSocketTestError {
	fn from(err: io::Error) -> Self {
		WebSocketTestError::Yawc(WebSocketError::IoError(err))
	}
}

impl WebSocketTestClient {
	pub async fn new(url: SocketAddr) -> Result<Self, WebSocketTestError> {
		let socket = TcpStream::connect(url).await?;
		let ws_url: url::Url = format!("ws://{url}/").parse().expect("valid url");
		match WebSocket::handshake(ws_url, socket, Options::default()).await {
			Ok(ws) => {
				let (tx, rx) = ws.split();
				Ok(Self { tx, rx })
			}
			Err(WebSocketError::Redirected { .. }) => Err(WebSocketTestError::Redirect),
			Err(WebSocketError::InvalidStatusCode(status_code)) => {
				Err(WebSocketTestError::RejectedWithStatusCode(status_code))
			}
			Err(err) => Err(WebSocketTestError::Yawc(err)),
		}
	}

	pub async fn send_request_text(&mut self, msg: impl AsRef<str>) -> Result<String, Error> {
		self.send(msg).await?;
		self.receive().await
	}

	pub async fn send(&mut self, msg: impl AsRef<str>) -> Result<(), Error> {
		self.tx.send(Frame::text(msg.as_ref().to_string())).await.map_err(Into::into)
	}

	pub async fn send_request_binary(&mut self, msg: &[u8]) -> Result<String, Error> {
		self.tx.send(Frame::binary(msg.to_vec())).await?;
		self.receive().await
	}

	pub async fn receive(&mut self) -> Result<String, Error> {
		loop {
			match self.rx.next().await {
				Some(frame) => match frame.opcode() {
					OpCode::Text | OpCode::Binary => {
						return String::from_utf8(frame.into_payload().to_vec()).map_err(Into::into);
					}
					OpCode::Close => return Err("Connection closed".into()),
					// Skip ping/pong frames
					_ => continue,
				},
				None => return Err("Connection closed".into()),
			}
		}
	}

	pub async fn close(&mut self) -> Result<(), Error> {
		self.tx.close().await.map_err(Into::into)
	}
}

#[derive(Debug, Clone)]
pub enum ServerMode {
	// Send out a hardcoded response on every connection.
	Response(String),
	// Send out a subscription ID on a request and continuously send out data on the subscription.
	Subscription { subscription_id: String, subscription_response: String },
	// Send out a notification after timeout
	Notification(String),
}

/// JSONRPC v2 dummy WebSocket server that sends a hardcoded response.
pub struct WebSocketTestServer {
	local_addr: SocketAddr,
	exit: mpsc::UnboundedSender<()>,
}

impl WebSocketTestServer {
	// Spawns a dummy `JSONRPC v2` WebSocket server that sends out a pre-configured `hardcoded response` for every
	// connection.
	pub async fn with_hardcoded_response(sockaddr: SocketAddr, response: String) -> Self {
		let listener = tokio::net::TcpListener::bind(sockaddr).await.unwrap();
		let local_addr = listener.local_addr().unwrap();
		let (tx, rx) = mpsc::unbounded();
		tokio::spawn(server_backend(listener, rx, ServerMode::Response(response)));

		Self { local_addr, exit: tx }
	}

	// Spawns a dummy `JSONRPC v2` WebSocket server that sends out a pre-configured `hardcoded notification` for every
	// connection.
	pub async fn with_hardcoded_notification(sockaddr: SocketAddr, notification: String) -> Self {
		let (tx, rx) = mpsc::unbounded();
		let (addr_tx, addr_rx) = oneshot::channel();

		tokio::spawn(async move {
			let listener = tokio::net::TcpListener::bind(sockaddr).await.unwrap();
			let local_addr = listener.local_addr().unwrap();

			addr_tx.send(local_addr).unwrap();
			server_backend(listener, rx, ServerMode::Notification(notification)).await
		});

		let local_addr = addr_rx.await.unwrap();

		Self { local_addr, exit: tx }
	}

	// Spawns a dummy `JSONRPC v2` WebSocket server that sends out a pre-configured subscription ID and subscription
	// response.
	//
	// NOTE: ignores the actual subscription and unsubscription method.
	pub async fn with_hardcoded_subscription(
		sockaddr: SocketAddr,
		subscription_id: String,
		subscription_response: String,
	) -> Self {
		let listener = tokio::net::TcpListener::bind(sockaddr).await.unwrap();
		let local_addr = listener.local_addr().unwrap();
		let (tx, rx) = mpsc::unbounded();
		tokio::spawn(server_backend(listener, rx, ServerMode::Subscription { subscription_id, subscription_response }));

		Self { local_addr, exit: tx }
	}

	pub fn local_addr(&self) -> SocketAddr {
		self.local_addr
	}

	pub async fn close(&mut self) {
		self.exit.send(()).await.unwrap();
	}
}

async fn server_backend(listener: tokio::net::TcpListener, mut exit: mpsc::UnboundedReceiver<()>, mode: ServerMode) {
	let mut connections = Vec::new();

	loop {
		let conn_fut = listener.accept().fuse();
		let exit_fut = exit.next();
		pin_mut!(exit_fut, conn_fut);

		select! {
			_ = exit_fut => break,
			conn = conn_fut => {
				if let Ok((stream, _)) = conn {
					let (tx, rx) = mpsc::unbounded();
					let handle = tokio::spawn(connection_task(stream, mode.clone(), rx));
					connections.push((handle, tx));
				}
			}
		}
	}

	// close connections
	for (handle, mut exit) in connections {
		// If the actual connection was never established i.e., returned early
		// It will most likely be caught on the client-side but just to be explicit.
		exit.send(()).await.expect("WebSocket connection was never established");
		handle.await.unwrap();
	}
}

async fn connection_task(socket: tokio::net::TcpStream, mode: ServerMode, mut exit: mpsc::UnboundedReceiver<()>) {
	let io = TokioIo::new(socket);

	// Use a oneshot channel to send the established WebSocket to the main task.
	let (ws_tx, ws_rx) = tokio::sync::oneshot::channel::<hyper::Result<yawc::HttpWebSocket>>();
	let ws_tx = std::sync::Arc::new(std::sync::Mutex::new(Some(ws_tx)));

	let mode_clone = mode.clone();

	let service = hyper::service::service_fn(move |mut req: hyper::Request<hyper::body::Incoming>| {
		let ws_tx = ws_tx.lock().unwrap().take().expect("service_fn called only once for HTTP upgrade");
		async move {
			let (response, upgrade_fut) = yawc::WebSocket::upgrade(&mut req)?;

			tokio::spawn(async move {
				let _ = ws_tx.send(upgrade_fut.await);
			});

			Ok::<_, yawc::WebSocketError>(response)
		}
	});

	// We need to use hyper to handle the HTTP upgrade, then get the WebSocket.
	// But since service_fn requires FnMut and we need to move ws_tx out,
	// let's use a simpler approach: accept the TCP connection with hyper for the upgrade.
	let builder = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());
	let conn = builder.serve_connection_with_upgrades(io, service);

	// Drive the HTTP connection to complete the upgrade
	let _ = conn.await;

	// Now get the WebSocket
	let ws = match ws_rx.await {
		Ok(Ok(ws)) => ws,
		_ => return,
	};

	let (mut sender, receiver) = ws.split();

	let ws_stream = receiver.filter_map(|frame| async move {
		match frame.opcode() {
			OpCode::Text | OpCode::Binary => Some(frame.into_payload().to_vec()),
			OpCode::Close => None,
			_ => None, // skip pings/pongs
		}
	});
	pin_mut!(ws_stream);

	loop {
		let next_ws = ws_stream.next().fuse();
		let next_exit = exit.next().fuse();
		let time_out = tokio::time::sleep(Duration::from_millis(200)).fuse();

		pin_mut!(time_out, next_exit, next_ws);

		select! {
			_ = time_out => {
				 match &mode_clone {
					ServerMode::Subscription { subscription_response, .. } => {
						if let Err(e) = sender.send(Frame::text(subscription_response.clone())).await {
							tracing::warn!("send response to subscription: {:?}", e);
							break;
						}
					},
					ServerMode::Notification(n) => {
						if let Err(e) = sender.send(Frame::text(n.clone())).await {
							tracing::warn!("send notification: {:?}", e);
							break;
						}
					},
					_ => {}
				}
			}
			ws = next_ws => {
				// Got a request on the connection but don't care about the contents.
				// Just send out the pre-configured hardcoded responses.
				if let Some(_) = ws {
					match &mode_clone {
						ServerMode::Response(r) => {
							if let Err(e) = sender.send(Frame::text(r.clone())).await {
								tracing::warn!("send response to request error: {:?}", e);
								break;
							}
						},
						ServerMode::Subscription { subscription_id, .. } => {
							if let Err(e) = sender.send(Frame::text(subscription_id.clone())).await {
								tracing::warn!("send subscription id error: {:?}", e);
								break;
							}
						},
						_ => {}
					}
				} else {
					break;
				}
			}
			_ = next_exit => break,
		}
	}
}

/// Checks whether the incoming request is a WebSocket upgrade request.
fn is_upgrade_request<B>(req: &hyper::Request<B>) -> bool {
	let dominated_upgrade = req
		.headers()
		.get(hyper::header::UPGRADE)
		.and_then(|v| v.to_str().ok())
		.map(|v| v.eq_ignore_ascii_case("websocket"))
		.unwrap_or(false);
	let has_connection_upgrade = req
		.headers()
		.get(hyper::header::CONNECTION)
		.and_then(|v| v.to_str().ok())
		.map(|v| v.to_lowercase().contains("upgrade"))
		.unwrap_or(false);
	dominated_upgrade && has_connection_upgrade
}

// Run a WebSocket server running on localhost that redirects requests for testing.
// Requests to any url except for `/myblock/two` will redirect one or two times (HTTP 301) and eventually end up in `/myblock/two`.
pub async fn ws_server_with_redirect(other_server: String) -> String {
	let listener = tokio::net::TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).await.unwrap();
	let addr = listener.local_addr().unwrap();

	tokio::spawn(async move {
		loop {
			let Ok((stream, _addr)) = listener.accept().await else {
				continue;
			};

			let other_server = other_server.clone();
			tokio::spawn(async {
				let io = TokioIo::new(stream);
				let builder = hyper_util::server::conn::auto::Builder::new(TokioExecutor::new());

				let conn = builder.serve_connection_with_upgrades(
					io,
					hyper::service::service_fn(move |req| {
						let other_server = other_server.clone();
						handler(req, other_server)
					}),
				);

				conn.await.unwrap();
			});
		}
	});
	format!("ws://{addr}")
}

/// Handle incoming HTTP Requests.
async fn handler(
	req: hyper::Request<hyper::body::Incoming>,
	other_server: String,
) -> Result<hyper::Response<EmptyBody>, Infallible> {
	if is_upgrade_request(&req) {
		tracing::debug!("{:?}", req);

		match req.uri().path() {
			"/myblock/two" => {
				let response = hyper::Response::builder()
					.status(301)
					.header("Location", other_server)
					.body(EmptyBody::new())
					.unwrap();
				Ok(response)
			}
			"/myblock/one" => {
				let response =
					hyper::Response::builder().status(301).header("Location", "two").body(EmptyBody::new()).unwrap();
				Ok(response)
			}
			_ => {
				let response = hyper::Response::builder()
					.status(301)
					.header("Location", "/myblock/one")
					.body(EmptyBody::new())
					.unwrap();
				Ok(response)
			}
		}
	} else {
		panic!("expect upgrade to WS");
	}
}
