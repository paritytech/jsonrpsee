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

use crate::types::jsonrpc;

use async_std::net::{TcpStream, ToSocketAddrs};
use async_tls::client::TlsStream;
use futures::prelude::*;
use soketto::connection;
use soketto::handshake::client::{Client as WsRawClient, ServerResponse};
use std::{borrow::Cow, fmt, io, net::SocketAddr, pin::Pin, time::Duration};
use thiserror::Error;

type TlsOrPlain = crate::client::ws::stream::EitherStream<TcpStream, TlsStream<TcpStream>>;

/// Implementation of a raw client for WebSockets requests.
pub struct WsTransportClient {
	/// Sending half of a TCP/IP connection wrapped around a WebSocket encoder.
	sender: connection::Sender<TlsOrPlain>,
	/// Receiving half of a TCP/IP connection wrapped around a WebSocket decoder.
	receiver: connection::Receiver<TlsOrPlain>,
}

/// Builder for a [`WsTransportClient`].
pub struct WsTransportClientBuilder<'a> {
	/// IP address to try to connect to.
	target: SocketAddr,
	/// Host to send during the WS handshake.
	host: Cow<'a, str>,
	/// DNS host name.
	dns_name: Cow<'a, str>,
	/// Stream mode, either plain TCP or TLS.
	mode: Mode,
	/// Url to send during the HTTP handshake.
	url: Cow<'a, str>,
	/// Timeout for the connection.
	timeout: Duration,
	/// `Origin` header to pass during the HTTP handshake. If `None`, no
	/// `Origin` header is passed.
	origin: Option<Cow<'a, str>>,
}

/// Stream mode, either plain TCP or TLS.
#[derive(Clone, Copy, Debug)]
pub enum Mode {
	/// Plain mode (`ws://` URL).
	Plain,
	/// TLS mode (`wss://` URL).
	Tls,
}

/// Error that can happen during the initial handshake.
#[derive(Debug, Error)]
pub enum WsNewError {
	/// Error when opening the TCP socket.
	#[error("Error when opening the TCP socket: {}", 0)]
	Io(io::Error),

	/// Error in the WebSocket handshake.
	#[error("Error in the WebSocket handshake: {}", 0)]
	Handshake(#[source] soketto::handshake::Error),

	/// Invalid DNS name error for TLS
	#[error("Invalid DNS name: {}", 0)]
	InvalidDNSName(#[source] webpki::InvalidDNSNameError),

	/// RawServer rejected our handshake.
	#[error("Server returned an error status code: {}", status_code)]
	Rejected {
		/// HTTP status code that the server returned.
		status_code: u16,
	},

	/// Timeout while trying to connect.
	#[error("Timeout when trying to connect")]
	Timeout,
}

/// Error that can happen during the initial handshake.
#[derive(Debug, Error)]
pub enum WsNewDnsError {
	/// Invalid URL.
	#[error("Invalid url: {}", 0)]
	Url(Cow<'static, str>),

	/// Error when trying to connect.
	///
	/// If multiple IP addresses are attempted, only the last error is returned, similar to how
	/// [`std::net::TcpStream::connect`] behaves.
	#[error("Error when trying to connect: {}", 0)]
	Connect(WsNewError),

	/// Failed to resolve IP addresses for this hostname.
	#[error("Failed to resolve IP addresses for this hostname: {}", 0)]
	ResolutionFailed(io::Error),

	/// Couldn't find any IP address for this hostname.
	#[error("Couldn't find any IP address for this hostname")]
	NoAddressFound,
}

/// Error that can happen during a request.
#[derive(Debug, Error)]
pub enum WsConnectError {
	/// Error while serializing the request.
	// TODO: can that happen?
	#[error("error while serializing the request")]
	Serialization(#[source] serde_json::error::Error),

	/// Error in the WebSocket connection.
	#[error("error in the WebSocket connection")]
	Ws(#[source] soketto::connection::Error),

	/// Failed to parse the JSON returned by the server into a JSON-RPC response.
	#[error("error while parsing the response body")]
	ParseError(#[source] serde_json::error::Error),
}

impl WsTransportClient {
	/// Creates a new [`WsTransportClientBuilder`] containing the given address and hostname.
	pub fn builder<'a>(
		target: SocketAddr,
		host: impl Into<Cow<'a, str>>,
		dns_name: impl Into<Cow<'a, str>>,
		mode: Mode,
	) -> WsTransportClientBuilder<'a> {
		WsTransportClientBuilder {
			target,
			host: host.into(),
			dns_name: dns_name.into(),
			mode,
			url: From::from("/"),
			timeout: Duration::from_secs(10),
			origin: None,
		}
	}

	/// Initializes a new WS client from a URL.
	pub async fn new(target: &str) -> Result<Self, WsNewDnsError> {
		let url = url::Url::parse(target).map_err(|e| WsNewDnsError::Url(format!("Invalid URL: {}", e).into()))?;
		let mode = match url.scheme() {
			"ws" => Mode::Plain,
			"wss" => Mode::Tls,
			_ => return Err(WsNewDnsError::Url("URL scheme not supported, expects 'ws' or 'wss'".into())),
		};
		let host = url.host_str().ok_or(WsNewDnsError::Url("No host in URL".into()))?;
		let target = match url.port_or_known_default() {
			Some(port) => format!("{}:{}", host, port),
			None => host.to_string(),
		};

		let mut error = None;

		for url in target.to_socket_addrs().await.map_err(WsNewDnsError::ResolutionFailed)? {
			match Self::builder(url, &target, host, mode).build().await {
				Ok(ws_raw_client) => return Ok(ws_raw_client),
				Err(err) => error = Some(err),
			}
		}

		if let Some(error) = error {
			Err(WsNewDnsError::Connect(error))
		} else {
			Err(WsNewDnsError::NoAddressFound)
		}
	}
}

// former transport client impl,
impl WsTransportClient {
	/// Sends out out a request. Returns a `Future` that finishes when the request has been
	/// successfully sent.
	pub fn send_request<'a>(
		&'a mut self,
		request: jsonrpc::Request,
	) -> Pin<Box<dyn Future<Output = Result<(), WsConnectError>> + Send + 'a>> {
		Box::pin(async move {
			log::debug!("send: {}", jsonrpc::to_string(&request).expect("valid Request; qed"));
			let request = jsonrpc::to_vec(&request).map_err(WsConnectError::Serialization)?;
			self.sender.send_binary(request).await?;
			self.sender.flush().await?;
			Ok(())
		})
	}

	/// Returns a `Future` resolving when the server sent us something back.
	pub fn next_response<'a>(
		&'a mut self,
	) -> Pin<Box<dyn Future<Output = Result<jsonrpc::Response, WsConnectError>> + Send + 'a>> {
		Box::pin(async move {
			let mut message = Vec::new();
			self.receiver.receive_data(&mut message).await?;
			let response = jsonrpc::from_slice(&message).map_err(WsConnectError::ParseError)?;
			log::debug!("recv: {}", jsonrpc::to_string(&response).expect("valid Request; qed"));
			Ok(response)
		})
	}
}

impl fmt::Debug for WsTransportClient {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_tuple("WsTransportClient").finish()
	}
}

impl<'a> WsTransportClientBuilder<'a> {
	/// Sets the URL to pass during the HTTP handshake.
	///
	/// The default URL is `/`.
	pub fn with_url(mut self, url: impl Into<Cow<'a, str>>) -> Self {
		self.url = url.into();
		self
	}

	/// Sets the `Origin` header to pass during the HTTP handshake.
	///
	/// By default, no `Origin` header is sent.
	pub fn with_origin_header(mut self, origin: impl Into<Cow<'a, str>>) -> Self {
		self.origin = Some(origin.into());
		self
	}

	/// Sets the timeout to use when establishing the TCP connection.
	///
	/// The default timeout is 10 seconds.
	// TODO: design decision: should the timeout not be handled by the user?
	pub fn with_timeout(mut self, timeout: Duration) -> Self {
		self.timeout = timeout;
		self
	}

	/// Try establish the connection.
	pub async fn build(self) -> Result<WsTransportClient, WsNewError> {
		// Try establish the TCP connection.
		let tcp_stream = {
			let socket = TcpStream::connect(self.target);
			let timeout = async_std::task::sleep(self.timeout);
			futures::pin_mut!(socket, timeout);
			match future::select(socket, timeout).await {
				future::Either::Left((socket, _)) => match self.mode {
					Mode::Plain => TlsOrPlain::Plain(socket?),
					Mode::Tls => {
						let connector = async_tls::TlsConnector::default();
						let dns_name = webpki::DNSNameRef::try_from_ascii_str(&self.dns_name)?;
						let tls_stream = connector.connect(&dns_name.to_owned(), socket?).await?;
						TlsOrPlain::Tls(tls_stream)
					}
				},
				future::Either::Right((_, _)) => return Err(WsNewError::Timeout),
			}
		};

		// Configure a WebSockets client on top.
		let mut client = WsRawClient::new(tcp_stream, &self.host, &self.url);
		if let Some(origin) = self.origin.as_ref() {
			client.set_origin(origin);
		}

		// Perform the initial handshake.
		match client.handshake().await? {
			ServerResponse::Accepted { .. } => {}
			ServerResponse::Rejected { status_code } | ServerResponse::Redirect { status_code, .. } => {
				// TODO: HTTP redirects also lead here
				return Err(WsNewError::Rejected { status_code });
			}
		}

		// If the handshake succeeded, return.
		let (sender, receiver) = client.into_builder().finish();
		Ok(WsTransportClient { sender, receiver })
	}
}

impl From<io::Error> for WsNewError {
	fn from(err: io::Error) -> WsNewError {
		WsNewError::Io(err)
	}
}

impl From<webpki::InvalidDNSNameError> for WsNewError {
	fn from(err: webpki::InvalidDNSNameError) -> WsNewError {
		WsNewError::InvalidDNSName(err)
	}
}

impl From<soketto::handshake::Error> for WsNewError {
	fn from(err: soketto::handshake::Error) -> WsNewError {
		WsNewError::Handshake(err)
	}
}

impl From<WsNewError> for WsNewDnsError {
	fn from(err: WsNewError) -> WsNewDnsError {
		WsNewDnsError::Connect(err)
	}
}

impl From<soketto::connection::Error> for WsConnectError {
	fn from(err: soketto::connection::Error) -> Self {
		WsConnectError::Ws(err)
	}
}
