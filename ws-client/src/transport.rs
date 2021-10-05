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

use crate::{client::Header, stream::EitherStream};
use futures::io::{BufReader, BufWriter};
use http::Uri;
use soketto::connection;
use soketto::handshake::client::{Client as WsHandshakeClient, ServerResponse};
use std::convert::TryInto;
use std::{
	borrow::Cow,
	convert::TryFrom,
	io,
	net::{SocketAddr, ToSocketAddrs},
	sync::Arc,
	time::Duration,
};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_rustls::{
	client::TlsStream,
	webpki::{DNSNameRef, InvalidDNSNameError},
	TlsConnector,
};

type TlsOrPlain = EitherStream<TcpStream, TlsStream<TcpStream>>;

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
	pub origin_header: Option<Header<'a>>,
	/// Max payload size
	pub max_request_body_size: u32,
	/// Max number of redirections.
	pub max_redirections: usize,
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
	#[error("Failed to load system certs: {0}")]
	CertificateStore(io::Error),

	/// Invalid URL.
	#[error("Invalid URL: {0}")]
	Url(Cow<'static, str>),

	/// Error when opening the TCP socket.
	#[error("Error when opening the TCP socket: {0}")]
	Io(io::Error),

	/// Error in the transport layer.
	#[error("Error in the WebSocket handshake: {0}")]
	Transport(#[source] soketto::handshake::Error),

	/// Invalid DNS name error for TLS
	#[error("Invalid DNS name: {0}")]
	InvalidDnsName(#[source] InvalidDNSNameError),

	/// Server rejected the handshake.
	#[error("Connection rejected with status code: {status_code}")]
	Rejected {
		/// HTTP status code that the server returned.
		status_code: u16,
	},

	/// Timeout while trying to connect.
	#[error("Connection timeout exceeded: {0:?}")]
	Timeout(Duration),

	/// Failed to resolve IP addresses for this hostname.
	#[error("Failed to resolve IP addresses for this hostname: {0}")]
	ResolutionFailed(io::Error),

	/// Couldn't find any IP address for this hostname.
	#[error("No IP address found for this hostname: {0}")]
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

	/// Send a close message and close the connection.
	pub async fn close(&mut self) -> Result<(), WsError> {
		self.inner.close().await.map_err(Into::into)
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

		self.try_connect(connector).await
	}

	async fn try_connect(
		self,
		mut tls_connector: Option<TlsConnector>,
	) -> Result<(Sender, Receiver), WsHandshakeError> {
		let mut target = self.target;
		let origin = self.origin_header.map(|o| [o]);
		let mut err = None;

		for _ in 0..self.max_redirections {
			log::debug!("Connecting to target: {:?}", target);

			// The sockaddrs might get reused if the server replies with a URI relative resource.
			let sockaddrs = std::mem::take(&mut target.sockaddrs);
			for sockaddr in &sockaddrs {
				let tcp_stream = match connect(*sockaddr, self.timeout, &target.host, &tls_connector).await {
					Ok(stream) => stream,
					Err(e) => {
						log::debug!("Failed to connect to sockaddr: {:?}", sockaddr);
						err = Some(Err(e));
						continue;
					}
				};
				let mut client = WsHandshakeClient::new(
					BufReader::new(BufWriter::new(tcp_stream)),
					&target.host_header,
					&target.path_and_query,
				);
				if let Some(origin) = &origin {
					client.set_headers(origin);
				}
				// Perform the initial handshake.
				match client.handshake().await {
					Ok(ServerResponse::Accepted { .. }) => {
						log::info!("Connection established to target: {:?}", target);
						let mut builder = client.into_builder();
						builder.set_max_message_size(self.max_request_body_size as usize);
						let (sender, receiver) = builder.finish();
						return Ok((Sender { inner: sender }, Receiver { inner: receiver }));
					}

					Ok(ServerResponse::Rejected { status_code }) => {
						log::debug!("Connection rejected: {:?}", status_code);
						err = Some(Err(WsHandshakeError::Rejected { status_code }));
					}
					Ok(ServerResponse::Redirect { status_code, location }) => {
						log::debug!("Redirection: status_code: {}, location: {}", status_code, location);
						match location.parse::<Uri>() {
							// redirection with absolute path => need to lookup.
							Ok(uri) => {
								// Absolute URI.
								if uri.scheme().is_some() {
									target = uri.try_into()?;
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
									break;
								}
								// Relative URI.
								else {
									// Replace the entire path_and_query if `location` starts with `/` or `//`.
									if location.starts_with('/') {
										target.path_and_query = location;
									} else {
										match target.path_and_query.rfind('/') {
											Some(offset) => {
												target.path_and_query.replace_range(offset + 1.., &location)
											}
											None => {
												err = Some(Err(WsHandshakeError::Url(
													format!(
														"path_and_query: {}; this is a bug it must contain `/` please open issue",
														location
													)
													.into(),
												)));
												continue;
											}
										};
									}
									target.sockaddrs = sockaddrs;
									break;
								}
							}
							Err(e) => {
								err = Some(Err(WsHandshakeError::Url(e.to_string().into())));
							}
						};
					}
					Err(e) => {
						err = Some(Err(e.into()));
					}
				};
			}
		}
		err.unwrap_or(Err(WsHandshakeError::NoAddressFound(target.host)))
	}
}

async fn connect(
	sockaddr: SocketAddr,
	timeout_dur: Duration,
	host: &str,
	tls_connector: &Option<TlsConnector>,
) -> Result<EitherStream<TcpStream, TlsStream<TcpStream>>, WsHandshakeError> {
	let socket = TcpStream::connect(sockaddr);
	let timeout = tokio::time::sleep(timeout_dur);
	tokio::select! {
		socket = socket => {
			let socket = socket?;
			if let Err(err) = socket.set_nodelay(true) {
				log::warn!("set nodelay failed: {:?}", err);
			}
			match tls_connector {
				None => Ok(TlsOrPlain::Plain(socket)),
				Some(connector) => {
					let dns_name = DNSNameRef::try_from_ascii_str(host)?;
					let tls_stream = connector.connect(dns_name, socket).await?;
					Ok(TlsOrPlain::Tls(tls_stream))
				}
			}
		}
		_ = timeout => Err(WsHandshakeError::Timeout(timeout_dur))
	}
}

impl From<io::Error> for WsHandshakeError {
	fn from(err: io::Error) -> WsHandshakeError {
		WsHandshakeError::Io(err)
	}
}

impl From<InvalidDNSNameError> for WsHandshakeError {
	fn from(err: InvalidDNSNameError) -> WsHandshakeError {
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
	/// The path and query parts from an URL.
	path_and_query: String,
}

impl TryFrom<Uri> for Target {
	type Error = WsHandshakeError;

	fn try_from(uri: Uri) -> Result<Self, Self::Error> {
		let mode = match uri.scheme_str() {
			Some("ws") => Mode::Plain,
			Some("wss") => Mode::Tls,
			_ => return Err(WsHandshakeError::Url("URL scheme not supported, expects 'ws' or 'wss'".into())),
		};
		let host = uri.host().map(ToOwned::to_owned).ok_or_else(|| WsHandshakeError::Url("No host in URL".into()))?;
		let port = uri
			.port_u16()
			.ok_or_else(|| WsHandshakeError::Url("No port number in URL (default port is not supported)".into()))?;
		let host_header = format!("{}:{}", host, port);
		let parts = uri.into_parts();
		let path_and_query = parts.path_and_query.ok_or_else(|| WsHandshakeError::Url("No path in URL".into()))?;
		let sockaddrs = host_header.to_socket_addrs().map_err(WsHandshakeError::ResolutionFailed)?;
		Ok(Self { sockaddrs: sockaddrs.collect(), host, host_header, mode, path_and_query: path_and_query.to_string() })
	}
}

#[cfg(test)]
mod tests {
	use super::{Mode, Target, Uri, WsHandshakeError};
	use http::uri::InvalidUri;
	use std::convert::TryInto;

	fn assert_ws_target(target: Target, host: &str, host_header: &str, mode: Mode, path_and_query: &str) {
		assert_eq!(&target.host, host);
		assert_eq!(&target.host_header, host_header);
		assert_eq!(target.mode, mode);
		assert_eq!(&target.path_and_query, path_and_query);
	}

	fn parse_target(uri: &str) -> Result<Target, WsHandshakeError> {
		uri.parse::<Uri>().map_err(|e: InvalidUri| WsHandshakeError::Url(e.to_string().into()))?.try_into()
	}

	#[test]
	fn ws_works() {
		let target = parse_target("ws://127.0.0.1:9933").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1:9933", Mode::Plain, "/");
	}

	#[test]
	fn wss_works() {
		let target = parse_target("wss://kusama-rpc.polkadot.io:443").unwrap();
		assert_ws_target(target, "kusama-rpc.polkadot.io", "kusama-rpc.polkadot.io:443", Mode::Tls, "/");
	}

	#[test]
	fn faulty_url_scheme() {
		let err = parse_target("http://kusama-rpc.polkadot.io:443").unwrap_err();
		assert!(matches!(err, WsHandshakeError::Url(_)));
	}

	#[test]
	fn faulty_port() {
		let err = parse_target("ws://127.0.0.1:-43").unwrap_err();
		assert!(matches!(err, WsHandshakeError::Url(_)));
		let err = parse_target("ws://127.0.0.1:99999").unwrap_err();
		assert!(matches!(err, WsHandshakeError::Url(_)));
	}

	#[test]
	fn url_with_path_works() {
		let target = parse_target("wss://127.0.0.1:443/my-special-path").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1:443", Mode::Tls, "/my-special-path");
	}

	#[test]
	fn url_with_query_works() {
		let target = parse_target("wss://127.0.0.1:443/my?name1=value1&name2=value2").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1:443", Mode::Tls, "/my?name1=value1&name2=value2");
	}

	#[test]
	fn url_with_fragment_is_ignored() {
		let target = parse_target("wss://127.0.0.1:443/my.htm#ignore").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1:443", Mode::Tls, "/my.htm");
	}
}
