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

mod stream;

use std::io;
use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

use futures_util::io::{BufReader, BufWriter};
use jsonrpsee_core::client::{CertificateStore, ReceivedMessage, TransportReceiverT, TransportSenderT};
use jsonrpsee_core::TEN_MB_SIZE_BYTES;
use jsonrpsee_core::{async_trait, Cow};
use soketto::connection::Error::Utf8;
use soketto::data::ByteSlice125;
use soketto::handshake::client::{Client as WsHandshakeClient, ServerResponse};
use soketto::{connection, Data, Incoming};
use stream::EitherStream;
use thiserror::Error;
use tokio::net::TcpStream;

pub use http::{uri::InvalidUri, HeaderMap, HeaderValue, Uri};
pub use soketto::handshake::client::Header;

/// Sending end of WebSocket transport.
#[derive(Debug)]
pub struct Sender {
	inner: connection::Sender<BufReader<BufWriter<EitherStream>>>,
}

/// Receiving end of WebSocket transport.
#[derive(Debug)]
pub struct Receiver {
	inner: connection::Receiver<BufReader<BufWriter<EitherStream>>>,
}

/// Builder for a WebSocket transport [`Sender`] and ['Receiver`] pair.
#[derive(Debug)]
pub struct WsTransportClientBuilder {
	/// What certificate store to use
	pub certificate_store: CertificateStore,
	/// Timeout for the connection.
	pub connection_timeout: Duration,
	/// Custom headers to pass during the HTTP handshake.
	pub headers: http::HeaderMap,
	/// Max payload size
	pub max_request_body_size: u32,
	/// Max number of redirections.
	pub max_redirections: usize,
}

impl Default for WsTransportClientBuilder {
	fn default() -> Self {
		Self {
			certificate_store: CertificateStore::Native,
			max_request_body_size: TEN_MB_SIZE_BYTES,
			connection_timeout: Duration::from_secs(10),
			headers: http::HeaderMap::new(),
			max_redirections: 5,
		}
	}
}

impl WsTransportClientBuilder {
	/// Set whether to use system certificates (default is native).
	pub fn certificate_store(mut self, certificate_store: CertificateStore) -> Self {
		self.certificate_store = certificate_store;
		self
	}

	/// Set max request body size (default is 10 MB).
	pub fn max_request_body_size(mut self, size: u32) -> Self {
		self.max_request_body_size = size;
		self
	}

	/// Set connection timeout for the handshake (default is 10 seconds).
	pub fn connection_timeout(mut self, timeout: Duration) -> Self {
		self.connection_timeout = timeout;
		self
	}

	/// Set a custom header passed to the server during the handshake (default is none).
	///
	/// The caller is responsible for checking that the headers do not conflict or are duplicated.
	pub fn set_headers(mut self, headers: http::HeaderMap) -> Self {
		self.headers = headers;
		self
	}

	/// Set the max number of redirections to perform until a connection is regarded as failed.
	/// (default is 5).
	pub fn max_redirections(mut self, redirect: usize) -> Self {
		self.max_redirections = redirect;
		self
	}
}

/// Stream mode, either plain TCP or TLS.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
	/// Plain mode (`ws://` URL).
	Plain,
	/// TLS mode (`wss://` URL).
	Tls,
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
	#[error("WebSocket connection error: {0}")]
	Connection(#[source] soketto::connection::Error),
}

#[async_trait]
impl TransportSenderT for Sender {
	type Error = WsError;

	/// Sends out a request. Returns a `Future` that finishes when the request has been
	/// successfully sent.
	async fn send(&mut self, body: String) -> Result<(), Self::Error> {
		tracing::trace!("send: {}", body);
		self.inner.send_text(body).await?;
		self.inner.flush().await?;
		Ok(())
	}

	/// Sends out a ping request. Returns a `Future` that finishes when the request has been
	/// successfully sent.
	async fn send_ping(&mut self) -> Result<(), Self::Error> {
		tracing::debug!("Send ping");
		// Submit empty slice as "optional" parameter.
		let slice: &[u8] = &[];
		// Byte slice fails if the provided slice is larger than 125 bytes.
		let byte_slice = ByteSlice125::try_from(slice).expect("Empty slice should fit into ByteSlice125");

		self.inner.send_ping(byte_slice).await?;
		self.inner.flush().await?;
		Ok(())
	}

	/// Send a close message and close the connection.
	async fn close(&mut self) -> Result<(), WsError> {
		self.inner.close().await.map_err(Into::into)
	}
}

#[async_trait]
impl TransportReceiverT for Receiver {
	type Error = WsError;

	/// Returns a `Future` resolving when the server sent us something back.
	async fn receive(&mut self) -> Result<ReceivedMessage, Self::Error> {
		loop {
			let mut message = Vec::new();
			let recv = self.inner.receive(&mut message).await?;

			match recv {
				Incoming::Data(Data::Text(_)) => {
					let s = String::from_utf8(message).map_err(|err| WsError::Connection(Utf8(err.utf8_error())))?;
					break Ok(ReceivedMessage::Text(s));
				}
				Incoming::Data(Data::Binary(_)) => break Ok(ReceivedMessage::Bytes(message)),
				Incoming::Pong(_) => break Ok(ReceivedMessage::Pong),
				_ => continue,
			}
		}
	}
}

impl WsTransportClientBuilder {
	/// Try to establish the connection.
	pub async fn build(self, uri: Uri) -> Result<(Sender, Receiver), WsHandshakeError> {
		let target: Target = uri.try_into()?;
		self.try_connect(target).await
	}

	async fn try_connect(self, mut target: Target) -> Result<(Sender, Receiver), WsHandshakeError> {
		let mut err = None;

		// Only build TLS connector if `wss` in URL.
		#[cfg(feature = "tls")]
		let mut connector = match target._mode {
			Mode::Tls => Some(build_tls_config(&self.certificate_store)?),
			Mode::Plain => None,
		};

		for _ in 0..self.max_redirections {
			tracing::debug!("Connecting to target: {:?}", target);

			// The sockaddrs might get reused if the server replies with a relative URI.
			let sockaddrs = std::mem::take(&mut target.sockaddrs);
			for sockaddr in &sockaddrs {
				#[cfg(feature = "tls")]
				let tcp_stream = match connect(*sockaddr, self.connection_timeout, &target.host, connector.as_ref()).await {
					Ok(stream) => stream,
					Err(e) => {
						tracing::debug!("Failed to connect to sockaddr: {:?}", sockaddr);
						err = Some(Err(e));
						continue;
					}
				};

				#[cfg(not(feature = "tls"))]
				let tcp_stream = match connect(*sockaddr, self.connection_timeout).await {
					Ok(stream) => stream,
					Err(e) => {
						tracing::debug!("Failed to connect to sockaddr: {:?}", sockaddr);
						err = Some(Err(e));
						continue;
					}
				};

				let mut client = WsHandshakeClient::new(
					BufReader::new(BufWriter::new(tcp_stream)),
					&target.host_header,
					&target.path_and_query,
				);

				let headers: Vec<_> = self
					.headers
					.iter()
					.map(|(key, value)| Header { name: key.as_str(), value: value.as_bytes() })
					.collect();
				client.set_headers(&headers);

				// Perform the initial handshake.
				match client.handshake().await {
					Ok(ServerResponse::Accepted { .. }) => {
						tracing::debug!("Connection established to target: {:?}", target);
						let mut builder = client.into_builder();
						builder.set_max_message_size(self.max_request_body_size as usize);
						let (sender, receiver) = builder.finish();
						return Ok((Sender { inner: sender }, Receiver { inner: receiver }));
					}

					Ok(ServerResponse::Rejected { status_code }) => {
						tracing::debug!("Connection rejected: {:?}", status_code);
						err = Some(Err(WsHandshakeError::Rejected { status_code }));
					}
					Ok(ServerResponse::Redirect { status_code, location }) => {
						tracing::debug!("Redirection: status_code: {}, location: {}", status_code, location);
						match location.parse::<Uri>() {
							// redirection with absolute path => need to lookup.
							Ok(uri) => {
								// Absolute URI.
								if uri.scheme().is_some() {
									target = uri.try_into().map_err(|e| {
										tracing::error!("Redirection failed: {:?}", e);
										e
									})?;

									// Only build TLS connector if `wss` in redirection URL.
									#[cfg(feature = "tls")]
									match target._mode {
										Mode::Tls if connector.is_none() => {
											connector = Some(build_tls_config(&self.certificate_store)?);
										}
										Mode::Tls => (),
										// Drop connector if it was configured previously.
										Mode::Plain => {
											connector = None;
										}
									};
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
								}
								break;
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

#[cfg(feature = "tls")]
async fn connect(
	sockaddr: SocketAddr,
	timeout_dur: Duration,
	host: &str,
	tls_connector: Option<&tokio_rustls::TlsConnector>,
) -> Result<EitherStream, WsHandshakeError> {
	let socket = TcpStream::connect(sockaddr);
	let timeout = tokio::time::sleep(timeout_dur);
	tokio::select! {
		socket = socket => {
			let socket = socket?;
			if let Err(err) = socket.set_nodelay(true) {
				tracing::warn!("set nodelay failed: {:?}", err);
			}
			match tls_connector {
				None => Ok(EitherStream::Plain(socket)),
				Some(connector) => {
					let server_name: tokio_rustls::rustls::ServerName = host.try_into().map_err(|e| WsHandshakeError::Url(format!("Invalid host: {} {:?}", host, e).into()))?;
					let tls_stream = connector.connect(server_name, socket).await?;
					Ok(EitherStream::Tls(tls_stream))
				}
			}
		}
		_ = timeout => Err(WsHandshakeError::Timeout(timeout_dur))
	}
}

#[cfg(not(feature = "tls"))]
async fn connect(sockaddr: SocketAddr, timeout_dur: Duration) -> Result<EitherStream, WsHandshakeError> {
	let socket = TcpStream::connect(sockaddr);
	let timeout = tokio::time::sleep(timeout_dur);
	tokio::select! {
		socket = socket => {
			let socket = socket?;
			if let Err(err) = socket.set_nodelay(true) {
				tracing::warn!("set nodelay failed: {:?}", err);
			}
			Ok(EitherStream::Plain(socket))
		}
		_ = timeout => Err(WsHandshakeError::Timeout(timeout_dur))
	}
}

impl From<io::Error> for WsHandshakeError {
	fn from(err: io::Error) -> WsHandshakeError {
		WsHandshakeError::Io(err)
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
	_mode: Mode,
	/// The path and query parts from an URL.
	path_and_query: String,
}

impl TryFrom<Uri> for Target {
	type Error = WsHandshakeError;

	fn try_from(uri: Uri) -> Result<Self, Self::Error> {
		let _mode = match uri.scheme_str() {
			Some("ws") => Mode::Plain,
			#[cfg(feature = "tls")]
			Some("wss") => Mode::Tls,
			invalid_scheme => {
				let scheme = invalid_scheme.unwrap_or("no scheme");
				#[cfg(feature = "tls")]
				let err = format!("`{}` not supported, expects 'ws' or 'wss'", scheme);
				#[cfg(not(feature = "tls"))]
				let err = format!("`{}` not supported, expects 'ws' ('wss' requires the tls feature)", scheme);
				return Err(WsHandshakeError::Url(err.into()));
			}
		};
		let host = uri.host().map(ToOwned::to_owned).ok_or_else(|| WsHandshakeError::Url("No host in URL".into()))?;
		let port = uri
			.port_u16()
			.ok_or_else(|| WsHandshakeError::Url("No port number in URL (default port is not supported)".into()))?;
		let host_header = format!("{}:{}", host, port);
		let parts = uri.into_parts();
		let path_and_query = parts.path_and_query.ok_or_else(|| WsHandshakeError::Url("No path in URL".into()))?;
		let sockaddrs = host_header.to_socket_addrs().map_err(WsHandshakeError::ResolutionFailed)?;
		Ok(Self {
			sockaddrs: sockaddrs.collect(),
			host,
			host_header,
			_mode,
			path_and_query: path_and_query.to_string(),
		})
	}
}

// NOTE: this is slow and should be used sparingly.
#[cfg(feature = "tls")]
fn build_tls_config(cert_store: &CertificateStore) -> Result<tokio_rustls::TlsConnector, WsHandshakeError> {
	use tokio_rustls::rustls;

	let mut roots = rustls::RootCertStore::empty();

	match cert_store {
		CertificateStore::Native => {
			let mut first_error = None;
			let certs = rustls_native_certs::load_native_certs().map_err(WsHandshakeError::CertificateStore)?;
			for cert in certs {
				let cert = rustls::Certificate(cert.0);
				if let Err(err) = roots.add(&cert) {
					first_error = first_error.or_else(|| Some(io::Error::new(io::ErrorKind::InvalidData, err)));
				}
			}
			if roots.is_empty() {
				let err = first_error
					.unwrap_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No valid certificate found"));
				return Err(WsHandshakeError::CertificateStore(err));
			}
		}
		CertificateStore::WebPki => {
			roots.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
				rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(ta.subject, ta.spki, ta.name_constraints)
			}));
		}
		_ => {
			let err = io::Error::new(io::ErrorKind::NotFound, "Invalid certificate store");
			return Err(WsHandshakeError::CertificateStore(err));
		}
	};

	let config =
		rustls::ClientConfig::builder().with_safe_defaults().with_root_certificates(roots).with_no_client_auth();

	Ok(std::sync::Arc::new(config).into())
}

#[cfg(test)]
mod tests {
	use super::{Mode, Target, Uri, WsHandshakeError};
	use http::uri::InvalidUri;

	fn assert_ws_target(target: Target, host: &str, host_header: &str, mode: Mode, path_and_query: &str) {
		assert_eq!(&target.host, host);
		assert_eq!(&target.host_header, host_header);
		assert_eq!(target._mode, mode);
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

	#[cfg(feature = "tls")]
	#[test]
	fn wss_works() {
		let target = parse_target("wss://kusama-rpc.polkadot.io:443").unwrap();
		assert_ws_target(target, "kusama-rpc.polkadot.io", "kusama-rpc.polkadot.io:443", Mode::Tls, "/");
	}

	#[cfg(not(feature = "tls"))]
	#[test]
	fn wss_fails_with_tls_feature() {
		let err = parse_target("wss://kusama-rpc.polkadot.io:443").unwrap_err();
		assert!(matches!(err, WsHandshakeError::Url(_)));
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
		let target = parse_target("ws://127.0.0.1:443/my-special-path").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1:443", Mode::Plain, "/my-special-path");
	}

	#[test]
	fn url_with_query_works() {
		let target = parse_target("ws://127.0.0.1:443/my?name1=value1&name2=value2").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1:443", Mode::Plain, "/my?name1=value1&name2=value2");
	}

	#[test]
	fn url_with_fragment_is_ignored() {
		let target = parse_target("ws://127.0.0.1:443/my.htm#ignore").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1:443", Mode::Plain, "/my.htm");
	}
}
