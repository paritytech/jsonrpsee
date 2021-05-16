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

use async_std::net::TcpStream;
use async_tls::client::TlsStream;
use futures::io::{BufReader, BufWriter};
use futures::prelude::*;
use soketto::connection;
use soketto::handshake::client::{Client as WsRawClient, ServerResponse};
use std::{borrow::Cow, io, net::SocketAddr, time::Duration};
use thiserror::Error;

type TlsOrPlain = crate::stream::EitherStream<TcpStream, TlsStream<TcpStream>>;

/// String representation of the host (domain or IP address) of an URL.
#[derive(Clone, Debug)]
pub struct Host(String);

impl Host {
	/// Extracts a string slice from the inner String.
	pub fn as_str(&self) -> &str {
		self.0.as_str()
	}
}

/// Sending end of WebSocket transport.
#[derive(Debug)]
pub struct Sender {
	inner: connection::Sender<BufReader<BufWriter<TlsOrPlain>>>,
}

/// Receiving end of WebSocket transport.
#[derive(Debug)]
pub struct Receiver {
	inner: connection::Receiver<BufReader<BufWriter<TlsOrPlain>>>,
}

/// Builder for a WebSocket transport [`Sender`] and ['Receiver`] pair.
#[derive(Debug)]
pub struct WsTransportClientBuilder<'a> {
	/// Use the systems certificates
	pub use_system_certificates: bool,
	/// Socket addresses to try to connect to.
	pub sockaddrs: Vec<SocketAddr>,
	/// Host.
	pub host: Host,
	/// Stream mode, either plain TCP or TLS.
	pub mode: Mode,
	/// Url to send during the HTTP handshake.
	pub handshake_url: Cow<'a, str>,
	/// Timeout for the connection.
	pub timeout: Duration,
	/// `Origin` header to pass during the HTTP handshake. If `None`, no
	/// `Origin` header is passed.
	pub origin: Option<Cow<'a, str>>,
	/// Max payload size
	pub max_request_body_size: u32,
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
	InvalidDnsName(#[source] webpki::InvalidDnsNameError),

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
	/// Failed to load system certs
	#[error("Failed to load system certs: {}", 0)]
	NativeCert(io::Error),

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

impl Sender {
	/// Sends out a request. Returns a `Future` that finishes when the request has been
	/// successfully sent.
	pub async fn send(&mut self, body: String) -> Result<(), WsConnectError> {
		log::debug!("send: {}", body);
		self.inner.send_text(body).await?;
		self.inner.flush().await?;
		Ok(())
	}
}

impl Receiver {
	/// Returns a `Future` resolving when the server sent us something back.
	pub async fn next_response(&mut self) -> Result<Vec<u8>, WsConnectError> {
		let mut message = Vec::new();
		self.inner.receive_data(&mut message).await?;
		Ok(message)
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
		let client_config = match self.mode {
			Mode::Tls => {
				let mut client_config = rustls::ClientConfig::default();
				if self.use_system_certificates {
					client_config.root_store =
						rustls_native_certs::load_native_certs().map_err(|(_, e)| WsHandshakeError::NativeCert(e))?;
				}
				Some(client_config.into())
			}
			Mode::Plain => None,
		};

		for sockaddr in &self.sockaddrs {
			match self.try_connect(*sockaddr, &client_config).await {
				Ok(res) => return Ok(res),
				Err(e) => {
					log::debug!("Failed to connect to sockaddr: {:?} with err: {:?}", sockaddr, e);
				}
			}
		}
		Err(WsHandshakeError::NoAddressFound)
	}

	async fn try_connect(
		&self,
		sockaddr: SocketAddr,
		tls_connector: &Option<async_tls::TlsConnector>,
	) -> Result<(Sender, Receiver), WsNewError> {
		// Try establish the TCP connection.
		let tcp_stream = {
			let socket = TcpStream::connect(sockaddr);
			let timeout = async_std::task::sleep(self.timeout);
			futures::pin_mut!(socket, timeout);
			match future::select(socket, timeout).await {
				future::Either::Left((socket, _)) => {
					let socket = socket?;
					if let Err(err) = socket.set_nodelay(true) {
						log::warn!("set nodelay failed: {:?}", err);
					}
					match tls_connector {
						None => TlsOrPlain::Plain(socket),
						Some(connector) => {
							let dns_name: &str = webpki::DnsNameRef::try_from_ascii_str(self.host.as_str())?.into();
							let tls_stream = connector.connect(dns_name, socket).await?;
							TlsOrPlain::Tls(tls_stream)
						}
					}
				}
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
		builder.set_max_message_size(self.max_request_body_size as usize);
		let (sender, receiver) = builder.finish();
		Ok((Sender { inner: sender }, Receiver { inner: receiver }))
	}
}

impl From<io::Error> for WsNewError {
	fn from(err: io::Error) -> WsNewError {
		WsNewError::Io(err)
	}
}

impl From<webpki::InvalidDnsNameError> for WsNewError {
	fn from(err: webpki::InvalidDnsNameError) -> WsNewError {
		WsNewError::InvalidDnsName(err)
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

/// Helper to parse an URL to a WebSocket address.
pub fn parse_url(url: impl AsRef<str>) -> Result<(Vec<SocketAddr>, Host, Mode), WsHandshakeError> {
	let url = url::Url::parse(url.as_ref()).map_err(|e| WsHandshakeError::Url(format!("Invalid URL: {}", e).into()))?;
	let mode = match url.scheme() {
		"ws" => Mode::Plain,
		"wss" => Mode::Tls,
		_ => return Err(WsHandshakeError::Url("URL scheme not supported, expects 'ws' or 'wss'".into())),
	};
	let host = Host(url.host_str().ok_or_else(|| WsHandshakeError::Url("No host in URL".into()))?.into());
	// NOTE: `Url::socket_addrs` is using the default port if it's missing (ws:// - 80, wss:// - 443)
	let sockaddrs = url.socket_addrs(|| None).map_err(WsHandshakeError::ResolutionFailed)?;
	Ok((sockaddrs, host, mode))
}

#[cfg(test)]
mod tests {
	use super::{parse_url, Mode, WsHandshakeError};

	#[test]
	fn ws_works() {
		let (_sockaddrs, host, mode) = parse_url("ws://127.0.0.1:9933").unwrap();
		assert_eq!(host.as_str(), "127.0.0.1");
		assert_eq!(mode, Mode::Plain);
	}

	#[test]
	fn wss_works() {
		let (_sockaddrs, host, mode) = parse_url("wss://kusama-rpc.polkadot.io:443").unwrap();
		assert_eq!(host.as_str(), "kusama-rpc.polkadot.io");
		assert_eq!(mode, Mode::Tls);
	}

	#[test]
	fn faulty_url_scheme() {
		let err = parse_url("http://kusama-rpc.polkadot.io:443").unwrap_err();
		assert!(matches!(err, WsHandshakeError::Url(_)));
	}

	#[test]
	fn faulty_port() {
		let err = parse_url("ws://127.0.0.1:-43").unwrap_err();
		assert!(matches!(err, WsHandshakeError::Url(_)));
		let err = parse_url("ws://127.0.0.1:99999").unwrap_err();
		assert!(matches!(err, WsHandshakeError::Url(_)));
	}

	#[test]
	fn default_port_works() {
		let (_sockaddr, host, mode) = parse_url("ws://127.0.0.1").unwrap();
		assert_eq!(host.as_str(), "127.0.0.1");
		assert_eq!(mode, Mode::Plain);
	}
}
