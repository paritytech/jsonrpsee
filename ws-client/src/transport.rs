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
use async_std::net::TcpStream;
use async_tls::client::TlsStream;
use futures::io::{BufReader, BufWriter};
use futures::prelude::*;
use jsonrpsee_types::jsonrpc;
use soketto::connection;
use soketto::handshake::client::{Client as WsRawClient, ServerResponse};
use std::{
	borrow::Cow,
	convert::{TryFrom, TryInto},
	io,
	net::SocketAddr,
	time::Duration,
};
use thiserror::Error;

type TlsOrPlain = crate::stream::EitherStream<TcpStream, TlsStream<TcpStream>>;

/// String representation of the host (domain or IP address) of an URL.
#[derive(Clone, Debug)]
pub struct Host(String);

impl Host {
	pub fn as_str(&self) -> &str {
		self.0.as_str()
	}
}

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
	/// Socket addresses to try to connect to.
	sockaddrs: Vec<SocketAddr>,
	/// Host.
	host: Host,
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
	let builder: WsTransportClientBuilder<'_> = config.try_into()?;
	builder.build().await.map_err(Into::into)
}

impl Sender {
	/// Sends out out a request. Returns a `Future` that finishes when the request has been
	/// successfully sent.
	pub async fn send_request(&mut self, request: jsonrpc::Request) -> Result<(), WsConnectError> {
		log::debug!("send: {}", request);
		let request = jsonrpc::to_vec(&request).map_err(WsConnectError::Serialization)?;
		self.inner.send_binary(request).await?;
		self.inner.flush().await?;
		Ok(())
	}
}

impl Receiver {
	/// Returns a `Future` resolving when the server sent us something back.
	pub async fn next_response(&mut self) -> Result<jsonrpc::Response, WsConnectError> {
		let mut message = Vec::new();
		self.inner.receive_data(&mut message).await?;
		let response = jsonrpc::from_slice(&message).map_err(WsConnectError::ParseError)?;
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
	pub async fn build(self) -> Result<(Sender, Receiver), WsHandshakeError> {
		for sockaddr in &self.sockaddrs {
			match self.try_connect(*sockaddr).await {
				Ok(res) => return Ok(res),
				Err(e) => {
					log::debug!("Failed to connect to sockaddr: {:?} with err: {:?}", sockaddr, e);
				}
			}
		}
		Err(WsHandshakeError::NoAddressFound)
	}

	async fn try_connect(&self, sockaddr: SocketAddr) -> Result<(Sender, Receiver), WsNewError> {
		// Try establish the TCP connection.
		let tcp_stream = {
			let socket = TcpStream::connect(sockaddr);
			let timeout = async_std::task::sleep(self.timeout);
			futures::pin_mut!(socket, timeout);
			match future::select(socket, timeout).await {
				future::Either::Left((socket, _)) => match self.mode {
					Mode::Plain => TlsOrPlain::Plain(socket?),
					Mode::Tls => {
						let connector = async_tls::TlsConnector::default();
						let dns_name = webpki::DNSNameRef::try_from_ascii_str(self.host.as_str())?;
						let tls_stream = connector.connect(&dns_name.to_owned(), socket?).await?;
						TlsOrPlain::Tls(tls_stream)
					}
				},
				future::Either::Right((_, _)) => return Err(WsNewError::Timeout),
			}
		};

		let mut client =
			WsRawClient::new(BufReader::new(BufWriter::new(tcp_stream)), self.host.as_str(), &self.handshake_url);
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

impl<'a> TryFrom<WsConfig<'a>> for WsTransportClientBuilder<'a> {
	type Error = WsHandshakeError;

	fn try_from(config: WsConfig<'a>) -> Result<Self, Self::Error> {
		let url =
			url::Url::parse(&config.url).map_err(|e| WsHandshakeError::Url(format!("Invalid URL: {}", e).into()))?;
		let mode = match url.scheme() {
			"ws" => Mode::Plain,
			"wss" => Mode::Tls,
			_ => return Err(WsHandshakeError::Url("URL scheme not supported, expects 'ws' or 'wss'".into())),
		};
		let host = url.host_str().ok_or_else(|| WsHandshakeError::Url("No host in URL".into()))?.into();
		let sockaddrs: Vec<SocketAddr> = url.socket_addrs(|| None).map_err(WsHandshakeError::ResolutionFailed)?;
		Ok(Self {
			sockaddrs,
			host: Host(host),
			mode,
			handshake_url: config.handshake_url.clone(),
			timeout: config.connection_timeout,
			origin: None,
			max_request_body_size: config.max_request_body_size,
		})
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

#[cfg(test)]
mod tests {
	use super::{Mode, WsConfig, WsHandshakeError, WsTransportClientBuilder};
	use std::convert::TryInto;

	#[test]
	fn ws_works() {
		let ws_config = WsConfig::with_url("ws://127.0.0.1:9933");
		let builder: WsTransportClientBuilder = ws_config.try_into().unwrap();
		assert_eq!(builder.host.as_str(), "127.0.0.1");
		assert_eq!(builder.mode, Mode::Plain);
	}

	#[test]
	fn wss_works() {
		let builder: WsTransportClientBuilder =
			WsConfig::with_url("wss://kusama-rpc.polkadot.io:443").try_into().unwrap();
		assert_eq!(builder.host.as_str(), "kusama-rpc.polkadot.io");
		assert_eq!(builder.mode, Mode::Tls);
	}

	#[test]
	fn faulty_url_scheme() {
		let err: Result<WsTransportClientBuilder, _> =
			WsConfig::with_url("http://kusama-rpc.polkadot.io:443").try_into();
		assert!(matches!(err, Err(WsHandshakeError::Url(_))));
	}

	#[test]
	fn faulty_port() {
		let builder: Result<WsTransportClientBuilder, _> = WsConfig::with_url("ws://127.0.0.1:-43").try_into();
		assert!(matches!(builder, Err(super::WsHandshakeError::Url(_))));
		let builder: Result<WsTransportClientBuilder, _> = WsConfig::with_url("ws://127.0.0.1:99999").try_into();
		assert!(matches!(builder, Err(super::WsHandshakeError::Url(_))));
	}

	#[test]
	fn default_port_works() {
		let builder: WsTransportClientBuilder = WsConfig::with_url("ws://127.0.0.1").try_into().unwrap();
		assert_eq!(builder.host.as_str(), "127.0.0.1");
		assert_eq!(builder.mode, Mode::Plain);
	}
}
