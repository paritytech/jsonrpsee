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

use crate::WsConfig;
use async_std::net::{TcpStream, ToSocketAddrs};
use async_tls::client::TlsStream;
use async_trait::async_trait;
use futures::io::{BufReader, BufWriter};
use futures::prelude::*;
use jsonrpsee_types::jsonrpc::{self, Error};
use jsonrpsee_types::traits::{TransportReceiver, TransportSender};
use soketto::connection;
use soketto::handshake::client::{Client as WsRawClient, ServerResponse};
use std::{borrow::Cow, io, net::SocketAddr, time::Duration};
use thiserror::Error;

type TlsOrPlain = crate::stream::EitherStream<TcpStream, TlsStream<TcpStream>>;

/// Sending end of WebSocket transport.
pub struct Sender {
	inner: connection::Sender<BufReader<BufWriter<TlsOrPlain>>>,
}

/// Receiving end of WebSocket transport.
pub struct Receiver {
	inner: connection::Receiver<BufReader<BufWriter<TlsOrPlain>>>,
}

/// Builder for a WebSocket transport [`Sender`] and ['Receiver`] pair.
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
	handshake_url: Cow<'a, str>,
	/// Timeout for the connection.
	timeout: Duration,
	/// `Origin` header to pass during the HTTP handshake. If `None`, no
	/// `Origin` header is passed.
	origin: Option<Cow<'a, str>>,
	/// Max payload size
	max_request_body_size: usize,
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
pub enum WsHandshakeError {
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

/// Creates a new WebSocket connection based on [`WsConfig`](crate::WsConfig) represented as a Sender and Receiver pair.
pub async fn websocket_connection(config: WsConfig<'_>) -> Result<(Sender, Receiver), WsHandshakeError> {
	let url = url::Url::parse(&config.url).map_err(|e| WsHandshakeError::Url(format!("Invalid URL: {}", e).into()))?;
	let mode = match url.scheme() {
		"ws" => Mode::Plain,
		"wss" => Mode::Tls,
		_ => return Err(WsHandshakeError::Url("URL scheme not supported, expects 'ws' or 'wss'".into())),
	};
	let host = url.host_str().ok_or_else(|| WsHandshakeError::Url("No host in URL".into()))?;
	let target = match url.port_or_known_default() {
		Some(port) => format!("{}:{}", host, port),
		None => host.to_string(),
	};

	let mut error = None;

	for sockaddr in target.to_socket_addrs().await.map_err(WsHandshakeError::ResolutionFailed)? {
		let builder = WsTransportClientBuilder {
			target: sockaddr,
			host: host.into(),
			dns_name: target.as_str().into(),
			mode,
			handshake_url: config.handshake_url.clone(),
			timeout: config.connection_timeout,
			origin: None,
			max_request_body_size: config.max_request_body_size,
		};

		match builder.build().await {
			Ok(ws) => return Ok(ws),
			Err(err) => error = Some(err),
		};
	}

	if let Some(error) = error {
		Err(WsHandshakeError::Connect(error))
	} else {
		Err(WsHandshakeError::NoAddressFound)
	}
}

#[async_trait]
impl TransportSender for Sender {
	/// Sends out out a request. Returns a `Future` that finishes when the request has been
	/// successfully sent.
	async fn send(&mut self, request: jsonrpc::Request) -> Result<(), Error> {
		log::debug!("send: {}", request);
		let request = jsonrpc::to_vec(&request).map_err(WsConnectError::Serialization).unwrap();
		self.inner.send_binary(request).await.unwrap();
		self.inner.flush().await.unwrap();
		Ok(())
	}
}

#[async_trait]
impl TransportReceiver for Receiver {
	/// Returns a `Future` resolving when the server sent us something back.
	async fn receive(&mut self) -> Result<jsonrpc::Response, Error> {
		let mut message = Vec::new();
		self.inner.receive_data(&mut message).await.unwrap();
		let response = jsonrpc::from_slice(&message).map_err(WsConnectError::ParseError).unwrap();
		log::debug!("recv: {}", response);
		Ok(response)
	}
}

impl<'a> WsTransportClientBuilder<'a> {
	/// Sets the URL to pass during the HTTP handshake.
	///
	/// The default URL is `/`.
	pub fn with_handshake_url(mut self, url: impl Into<Cow<'a, str>>) -> Self {
		self.handshake_url = url.into();
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
	pub fn with_timeout(mut self, timeout: Duration) -> Self {
		self.timeout = timeout;
		self
	}

	/// Try establish the connection.
	pub async fn build(self) -> Result<(Sender, Receiver), WsNewError> {
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
		let mut client = WsRawClient::new(BufReader::new(BufWriter::new(tcp_stream)), &self.host, &self.handshake_url);
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
		let mut builder = client.into_builder();
		builder.set_max_message_size(self.max_request_body_size);
		let (sender, receiver) = builder.finish();
		Ok((Sender { inner: sender }, Receiver { inner: receiver }))
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

impl From<WsNewError> for WsHandshakeError {
	fn from(err: WsNewError) -> WsHandshakeError {
		WsHandshakeError::Connect(err)
	}
}

impl From<soketto::connection::Error> for WsConnectError {
	fn from(err: soketto::connection::Error) -> Self {
		WsConnectError::Ws(err)
	}
}
