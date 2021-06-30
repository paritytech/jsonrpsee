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

use crate::stream::EitherStream;
use crate::tokio::{TcpStream, TlsStream};
use futures::io::{BufReader, BufWriter};
use futures::prelude::*;
use soketto::connection;
use soketto::handshake::client::{Client as WsRawClient, ServerResponse};
use std::{borrow::Cow, io, net::SocketAddr, sync::Arc, time::Duration};
use thiserror::Error;

type TlsOrPlain = crate::stream::EitherStream<TcpStream, TlsStream<TcpStream>>;

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
	/// What certificate store to use
	pub certificate_store: CertificateStore,
	/// Remote WebSocket target.
	pub target: Target,
	/// Timeout for the connection.
	pub timeout: Duration,
	/// `Origin` header to pass during the HTTP handshake. If `None`, no
	/// `Origin` header is passed.
	pub origin_header: Option<Cow<'a, str>>,
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

/// What certificate store to use
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum CertificateStore {
	/// Use the native system certificate store
	Native,
	/// Use webPki's certificate store
	WebPki,
}

/// Error that can happen during the WebSocket handshake.
///
/// If multiple IP addresses are attempted, only the last error is returned, similar to how
/// [`std::net::TcpStream::connect`] behaves.
#[derive(Debug, Error)]
pub enum WsHandshakeError {
	/// Failed to load system certs
	#[error("Failed to load system certs: {}", 0)]
	CertificateStore(io::Error),

	/// Invalid URL.
	#[error("Invalid url: {}", 0)]
	Url(Cow<'static, str>),

	/// Error when opening the TCP socket.
	#[error("Error when opening the TCP socket: {}", 0)]
	Io(io::Error),

	/// Error in the transport layer.
	#[error("Error in the WebSocket handshake: {}", 0)]
	Transport(#[source] soketto::handshake::Error),

	/// Invalid DNS name error for TLS
	#[error("Invalid DNS name: {}", 0)]
	InvalidDnsName(#[source] crate::tokio::InvalidDNSNameError),

	/// RawServer rejected our handshake.
	#[error("Connection rejected with status code: {}", status_code)]
	Rejected {
		/// HTTP status code that the server returned.
		status_code: u16,
	},

	/// Timeout while trying to connect.
	#[error("Connection timeout exceeded: {}", 0)]
	Timeout(Duration),

	/// Failed to resolve IP addresses for this hostname.
	#[error("Failed to resolve IP addresses for this hostname: {}", 0)]
	ResolutionFailed(io::Error),

	/// Couldn't find any IP address for this hostname.
	#[error("No IP address found for this hostname: {}", 0)]
	NoAddressFound(String),
}

/// Error that can occur when reading or sending messages on an established connection.
#[derive(Debug, Error)]
pub enum WsError {
	/// Error in the WebSocket connection.
	#[error("WebSocket connection error: {}", 0)]
	Connection(#[source] soketto::connection::Error),

	/// Failed to parse the message in JSON.
	#[error("Failed to parse message in JSON: {}", 0)]
	ParseError(#[source] serde_json::error::Error),
}

impl Sender {
	/// Sends out a request. Returns a `Future` that finishes when the request has been
	/// successfully sent.
	pub async fn send(&mut self, body: String) -> Result<(), WsError> {
		log::debug!("send: {}", body);
		self.inner.send_text(body).await?;
		self.inner.flush().await?;
		Ok(())
	}
}

impl Receiver {
	/// Returns a `Future` resolving when the server sent us something back.
	pub async fn next_response(&mut self) -> Result<Vec<u8>, WsError> {
		let mut message = Vec::new();
		self.inner.receive_data(&mut message).await?;
		Ok(message)
	}
}

impl<'a> WsTransportClientBuilder<'a> {
	/// Try to establish the connection.
	pub async fn build(self) -> Result<(Sender, Receiver), WsHandshakeError> {
		let connector = match self.target.mode {
			Mode::Tls => {
				let mut client_config = rustls::ClientConfig::default();
				if let CertificateStore::Native = self.certificate_store {
					client_config.root_store = rustls_native_certs::load_native_certs()
						.map_err(|(_, e)| WsHandshakeError::CertificateStore(e))?;
				}
				Some(Arc::new(client_config).into())
			}
			Mode::Plain => None,
		};

		let mut err = None;
		for sockaddr in &self.target.sockaddrs {
			match self.try_connect(*sockaddr, &connector).await {
				Ok(res) => return Ok(res),
				Err(e) => {
					log::debug!("Failed to connect to sockaddr: {:?} with err: {:?}", sockaddr, e);
					err = Some(Err(e));
				}
			}
		}
		// NOTE(niklasad1): this is most likely unreachable because [`Url::socket_addrs`] doesn't
		// return an empty `Vec` if no socket address was found for the host name.
		err.unwrap_or(Err(WsHandshakeError::NoAddressFound(self.target.host)))
	}

	async fn try_connect(
		&self,
		mut sockaddr: SocketAddr,
		tls_connector: &Option<crate::tokio::TlsConnector>,
	) -> Result<(Sender, Receiver), WsHandshakeError> {
		let mut path = self.target.path.clone();
		let mut host = self.target.host.clone();
		let mut host_header = self.target.host_header.clone();
		let mut tls_connector = tls_connector.clone();

		let client = loop {
			let tcp_stream = connect(sockaddr, self.timeout, &host, tls_connector).await?;
			let mut client = WsRawClient::new(BufReader::new(BufWriter::new(tcp_stream)), &host_header, &path);
			if let Some(origin) = self.origin_header.as_ref() {
				client.set_origin(origin);
			}
			// Perform the initial handshake.
			match client.handshake().await? {
				ServerResponse::Accepted { .. } => break Ok(client),
				ServerResponse::Rejected { status_code } => {
					break Err(WsHandshakeError::Rejected { status_code });
				}
				ServerResponse::Redirect { status_code, location } => {
					log::trace!("recv redirection: status_code: {}, location: {}", status_code, location);
					log::debug!("trying to reconnect to redirection: {}", location);
					match url::Url::parse(&location) {
						// absolute URL => need to lookup sockaddr.
						// TODO: this is hacky, we need to actually test all sockaddrs
						Ok(url) => {
							let target = Target::parse(url)?;
							// TODO.
							sockaddr = target.sockaddrs[0];
							path = target.path;
							host = target.host;
							host_header = target.host_header;
							tls_connector = match target.mode {
								Mode::Tls => {
									let mut client_config = rustls::ClientConfig::default();
									if let CertificateStore::Native = self.certificate_store {
										client_config.root_store = rustls_native_certs::load_native_certs()
											.map_err(|(_, e)| WsHandshakeError::CertificateStore(e))?;
									}
									Some(Arc::new(client_config).into())
								}
								Mode::Plain => None,
							};
						}
						// URL is relative, just set it as location.
						Err(
							url::ParseError::RelativeUrlWithCannotBeABaseBase | url::ParseError::RelativeUrlWithoutBase,
						) => {
							path = location;
						}
						Err(e) => return Err(WsHandshakeError::Url(format!("Invalid URL: {}", e).into())),
					};
				}
			};
		}?;

		// If the handshake succeeded, return.
		let mut builder = client.into_builder();
		builder.set_max_message_size(self.max_request_body_size as usize);
		let (sender, receiver) = builder.finish();
		Ok((Sender { inner: sender }, Receiver { inner: receiver }))
	}
}

async fn connect(
	sockaddr: SocketAddr,
	timeout_dur: Duration,
	host: &str,
	tls_connector: &Option<crate::tokio::TlsConnector>,
) -> Result<EitherStream<TcpStream, TlsStream<TcpStream>>, WsHandshakeError> {
	let socket = TcpStream::connect(sockaddr);
	let timeout = crate::tokio::sleep(timeout_dur);
	futures::pin_mut!(socket, timeout);
	Ok(match future::select(socket, timeout).await {
		future::Either::Left((socket, _)) => {
			let socket = socket?;
			if let Err(err) = socket.set_nodelay(true) {
				log::warn!("set nodelay failed: {:?}", err);
			}
			match tls_connector {
				None => TlsOrPlain::Plain(socket),
				Some(connector) => {
					let dns_name = crate::tokio::DNSNameRef::try_from_ascii_str(host)?;
					let tls_stream = connector.connect(dns_name, socket).await?;
					TlsOrPlain::Tls(tls_stream)
				}
			}
		}
		future::Either::Right((_, _)) => return Err(WsHandshakeError::Timeout(timeout_dur)),
	})
}

impl From<io::Error> for WsHandshakeError {
	fn from(err: io::Error) -> WsHandshakeError {
		WsHandshakeError::Io(err)
	}
}

impl From<crate::tokio::InvalidDNSNameError> for WsHandshakeError {
	fn from(err: crate::tokio::InvalidDNSNameError) -> WsHandshakeError {
		WsHandshakeError::InvalidDnsName(err)
	}
}

impl From<soketto::handshake::Error> for WsHandshakeError {
	fn from(err: soketto::handshake::Error) -> WsHandshakeError {
		WsHandshakeError::Transport(err)
	}
}

impl From<soketto::connection::Error> for WsError {
	fn from(err: soketto::connection::Error) -> Self {
		WsError::Connection(err)
	}
}

/// Represents a verified remote WebSocket address.
#[derive(Debug, Clone)]
pub struct Target {
	/// Socket addresses resolved the host name.
	sockaddrs: Vec<SocketAddr>,
	/// The host name (domain or IP address).
	host: String,
	/// The Host request header specifies the host and port number of the server to which the request is being sent.
	host_header: String,
	/// WebSocket stream mode, see [`Mode`] for further documentation.
	mode: Mode,
	/// The HTTP host resource path.
	path: String,
}

impl Target {
	/// Parse an URL String to a WebSocket address.
	pub fn parse(url: impl AsRef<str>) -> Result<Self, WsHandshakeError> {
		let url =
			url::Url::parse(url.as_ref()).map_err(|e| WsHandshakeError::Url(format!("Invalid URL: {}", e).into()))?;
		let mode = match url.scheme() {
			"ws" => Mode::Plain,
			"wss" => Mode::Tls,
			_ => return Err(WsHandshakeError::Url("URL scheme not supported, expects 'ws' or 'wss'".into())),
		};
		let host =
			url.host_str().map(ToOwned::to_owned).ok_or_else(|| WsHandshakeError::Url("No host in URL".into()))?;
		let port = url.port_or_known_default().ok_or_else(|| WsHandshakeError::Url("No port number in URL".into()))?;
		let host_header = format!("{}:{}", host, port);
		// NOTE: `Url::socket_addrs` is using the default port if it's missing (ws:// - 80, wss:// - 443)
		let sockaddrs = url.socket_addrs(|| None).map_err(WsHandshakeError::ResolutionFailed)?;
		Ok(Self { sockaddrs, host, host_header, mode, path: url.path().to_owned() })
	}
}

#[cfg(test)]
mod tests {
	use super::{Mode, Target, WsHandshakeError};

	fn assert_ws_target(target: Target, host: &str, host_header: &str, mode: Mode, path: &str) {
		assert_eq!(&target.host, host);
		assert_eq!(&target.host_header, host_header);
		assert_eq!(target.mode, mode);
		assert_eq!(&target.path, path);
	}

	#[test]
	fn ws_works() {
		let target = Target::parse("ws://127.0.0.1:9933").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1:9933", Mode::Plain, "/");
	}

	#[test]
	fn wss_works() {
		let target = Target::parse("wss://kusama-rpc.polkadot.io:443").unwrap();
		assert_ws_target(target, "kusama-rpc.polkadot.io", "kusama-rpc.polkadot.io:443", Mode::Tls, "/");
	}

	#[test]
	fn faulty_url_scheme() {
		let err = Target::parse("http://kusama-rpc.polkadot.io:443").unwrap_err();
		assert!(matches!(err, WsHandshakeError::Url(_)));
	}

	#[test]
	fn faulty_port() {
		let err = Target::parse("ws://127.0.0.1:-43").unwrap_err();
		assert!(matches!(err, WsHandshakeError::Url(_)));
		let err = Target::parse("ws://127.0.0.1:99999").unwrap_err();
		assert!(matches!(err, WsHandshakeError::Url(_)));
	}

	#[test]
	fn default_port_works() {
		let target = Target::parse("ws://127.0.0.1").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1:80", Mode::Plain, "/");
	}

	#[test]
	fn url_with_path_works() {
		let target = Target::parse("wss://127.0.0.1/my-special-path").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1:443", Mode::Tls, "/my-special-path");
	}
}
