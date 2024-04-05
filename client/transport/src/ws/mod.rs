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
use std::net::SocketAddr;
use std::time::Duration;

use futures_util::io::{BufReader, BufWriter};
use jsonrpsee_core::client::{CertificateStore, MaybeSend, ReceivedMessage, TransportReceiverT, TransportSenderT};
use jsonrpsee_core::TEN_MB_SIZE_BYTES;
use jsonrpsee_core::{async_trait, Cow};
use soketto::connection::Error::Utf8;
use soketto::data::ByteSlice125;
use soketto::handshake::client::{Client as WsHandshakeClient, ServerResponse};
use soketto::{connection, Data, Incoming};
use thiserror::Error;
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

pub use http::{uri::InvalidUri, HeaderMap, HeaderValue, Uri};
pub use soketto::handshake::client::Header;
pub use stream::EitherStream;
pub use tokio::io::{AsyncRead, AsyncWrite};
pub use url::Url;

const LOG_TARGET: &str = "jsonrpsee-client";

/// Sending end of WebSocket transport.
#[derive(Debug)]
pub struct Sender<T> {
	inner: connection::Sender<BufReader<BufWriter<T>>>,
	max_request_size: u32,
}

/// Receiving end of WebSocket transport.
#[derive(Debug)]
pub struct Receiver<T> {
	inner: connection::Receiver<BufReader<BufWriter<T>>>,
}

/// Builder for a WebSocket transport [`Sender`] and [`Receiver`] pair.
#[derive(Debug)]
pub struct WsTransportClientBuilder {
	/// What certificate store to use
	pub certificate_store: CertificateStore,
	/// Timeout for the connection.
	pub connection_timeout: Duration,
	/// Custom headers to pass during the HTTP handshake.
	pub headers: http::HeaderMap,
	/// Max request payload size
	pub max_request_size: u32,
	/// Max response payload size
	pub max_response_size: u32,
	/// Max number of redirections.
	pub max_redirections: usize,
	/// TCP no delay.
	pub tcp_no_delay: bool,
}

impl Default for WsTransportClientBuilder {
	fn default() -> Self {
		Self {
			certificate_store: CertificateStore::Native,
			max_request_size: TEN_MB_SIZE_BYTES,
			max_response_size: TEN_MB_SIZE_BYTES,
			connection_timeout: Duration::from_secs(10),
			headers: http::HeaderMap::new(),
			max_redirections: 5,
			tcp_no_delay: true,
		}
	}
}

impl WsTransportClientBuilder {
	/// Force to use the rustls native certificate store.
	///
	/// Since multiple certificate stores can be optionally enabled, this option will
	/// force the `native certificate store` to be used.
	///
	/// # Optional
	///
	/// This requires the optional `native-tls` feature.
	#[cfg(feature = "native-tls")]
	pub fn use_native_rustls(mut self) -> Self {
		self.certificate_store = CertificateStore::Native;
		self
	}

	/// Force to use the rustls webpki certificate store.
	///
	/// Since multiple certificate stores can be optionally enabled, this option will
	/// force the `webpki certificate store` to be used.
	///
	/// # Optional
	///
	/// This requires the optional `webpki-tls` feature.
	#[cfg(feature = "webpki-tls")]
	pub fn use_webpki_rustls(mut self) -> Self {
		self.certificate_store = CertificateStore::WebPki;
		self
	}

	/// Set the maximum size of a request in bytes. Default is 10 MiB.
	pub fn max_request_size(mut self, size: u32) -> Self {
		self.max_request_size = size;
		self
	}

	/// Set the maximum size of a response in bytes. Default is 10 MiB.
	pub fn max_response_size(mut self, size: u32) -> Self {
		self.max_response_size = size;
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
	#[error("{0}")]
	Transport(#[source] soketto::handshake::Error),

	/// Server rejected the handshake.
	#[error("Connection rejected with status code: {status_code}")]
	Rejected {
		/// HTTP status code that the server returned.
		status_code: u16,
	},

	/// Server redirected to other location.
	#[error("Connection redirected with status code: {status_code} and location: {location}")]
	Redirected {
		/// HTTP status code that the server returned.
		status_code: u16,
		/// The location URL redirected to.
		location: String,
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
	#[error("{0}")]
	Connection(#[source] soketto::connection::Error),
	/// Message was too large.
	#[error("The message was too large")]
	MessageTooLarge,
}

#[async_trait]
impl<T> TransportSenderT for Sender<T>
where
	T: futures_util::io::AsyncRead + futures_util::io::AsyncWrite + Unpin + MaybeSend + 'static,
{
	type Error = WsError;

	/// Sends out a request. Returns a `Future` that finishes when the request has been
	/// successfully sent.
	async fn send(&mut self, body: String) -> Result<(), Self::Error> {
		if body.len() > self.max_request_size as usize {
			return Err(WsError::MessageTooLarge);
		}

		self.inner.send_text(body).await?;
		self.inner.flush().await?;
		Ok(())
	}

	/// Sends out a ping request. Returns a `Future` that finishes when the request has been
	/// successfully sent.
	async fn send_ping(&mut self) -> Result<(), Self::Error> {
		tracing::debug!(target: LOG_TARGET, "Send ping");
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
impl<T> TransportReceiverT for Receiver<T>
where
	T: futures_util::io::AsyncRead + futures_util::io::AsyncWrite + Unpin + MaybeSend + 'static,
{
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
	///
	/// Uses the default connection over TCP.
	pub async fn build(
		self,
		uri: Url,
	) -> Result<(Sender<Compat<EitherStream>>, Receiver<Compat<EitherStream>>), WsHandshakeError> {
		self.try_connect_over_tcp(uri).await
	}

	/// Try to establish the connection over the given data stream.
	pub async fn build_with_stream<T>(
		self,
		uri: Url,
		data_stream: T,
	) -> Result<(Sender<Compat<T>>, Receiver<Compat<T>>), WsHandshakeError>
	where
		T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin,
	{
		let target: Target = uri.try_into()?;
		self.try_connect(&target, data_stream.compat()).await
	}

	// Try to establish the connection over TCP.
	async fn try_connect_over_tcp(
		&self,
		uri: Url,
	) -> Result<(Sender<Compat<EitherStream>>, Receiver<Compat<EitherStream>>), WsHandshakeError> {
		let mut target: Target = uri.try_into()?;
		let mut err = None;

		// Only build TLS connector if `wss` in URL.
		#[cfg(feature = "__tls")]
		let mut connector = match target._mode {
			Mode::Tls => Some(build_tls_config(&self.certificate_store)?),
			Mode::Plain => None,
		};

		for _ in 0..self.max_redirections {
			tracing::debug!(target: LOG_TARGET, "Connecting to target: {:?}", target);

			// The sockaddrs might get reused if the server replies with a relative URI.
			let sockaddrs = std::mem::take(&mut target.sockaddrs);
			for sockaddr in &sockaddrs {
				#[cfg(feature = "__tls")]
				let tcp_stream = match connect(*sockaddr, self.connection_timeout, &target.host, connector.as_ref(), self.tcp_no_delay)
					.await
				{
					Ok(stream) => stream,
					Err(e) => {
						tracing::debug!(target: LOG_TARGET, "Failed to connect to sockaddr: {:?}", sockaddr);
						err = Some(Err(e));
						continue;
					}
				};

				#[cfg(not(feature = "__tls"))]
				let tcp_stream = match connect(*sockaddr, self.connection_timeout).await {
					Ok(stream) => stream,
					Err(e) => {
						tracing::debug!(target: LOG_TARGET, "Failed to connect to sockaddr: {:?}", sockaddr);
						err = Some(Err(e));
						continue;
					}
				};

				match self.try_connect(&target, tcp_stream.compat()).await {
					Ok(result) => return Ok(result),

					Err(WsHandshakeError::Redirected { status_code, location }) => {
						tracing::debug!(target: LOG_TARGET, "Redirection: status_code: {}, location: {}", status_code, location);
						match Url::parse(&location) {
							// redirection with absolute path => need to lookup.
							Ok(uri) => {
								// Absolute URI.
								target = uri.try_into().map_err(|e| {
									tracing::debug!(target: LOG_TARGET, "Redirection failed: {:?}", e);
									e
								})?;

								// Only build TLS connector if `wss` in redirection URL.
								#[cfg(feature = "__tls")]
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

							// Relative URI such as `/foo/bar.html` or `cake.html`
							Err(url::ParseError::RelativeUrlWithoutBase) => {
								// Replace the entire path_and_query if `location` starts with `/` or `//`.
								if location.starts_with('/') {
									target.path_and_query = location;
								} else {
									match target.path_and_query.rfind('/') {
										Some(offset) => target.path_and_query.replace_range(offset + 1.., &location),
										None => {
											let e = format!("path_and_query: {location}; this is a bug it must contain `/` please open issue");
											err = Some(Err(WsHandshakeError::Url(e.into())));
											continue;
										}
									};
								}
								target.sockaddrs = sockaddrs;
								break;
							}

							Err(e) => {
								err = Some(Err(WsHandshakeError::Url(e.to_string().into())));
							}
						};
					}

					Err(e) => {
						err = Some(Err(e));
					}
				};
			}
		}
		err.unwrap_or(Err(WsHandshakeError::NoAddressFound(target.host)))
	}

	/// Try to establish the handshake over the given data stream.
	async fn try_connect<T>(
		&self,
		target: &Target,
		data_stream: T,
	) -> Result<(Sender<T>, Receiver<T>), WsHandshakeError>
	where
		T: futures_util::AsyncRead + futures_util::AsyncWrite + Unpin,
	{
		let mut client = WsHandshakeClient::new(
			BufReader::new(BufWriter::new(data_stream)),
			&target.host_header,
			&target.path_and_query,
		);

		let headers: Vec<_> =
			self.headers.iter().map(|(key, value)| Header { name: key.as_str(), value: value.as_bytes() }).collect();
		client.set_headers(&headers);

		// Perform the initial handshake.
		match client.handshake().await {
			Ok(ServerResponse::Accepted { .. }) => {
				tracing::debug!(target: LOG_TARGET, "Connection established to target: {:?}", target);
				let mut builder = client.into_builder();
				builder.set_max_message_size(self.max_response_size as usize);
				let (sender, receiver) = builder.finish();
				Ok((Sender { inner: sender, max_request_size: self.max_request_size }, Receiver { inner: receiver }))
			}

			Ok(ServerResponse::Rejected { status_code }) => {
				tracing::debug!(target: LOG_TARGET, "Connection rejected: {:?}", status_code);
				Err(WsHandshakeError::Rejected { status_code })
			}

			Ok(ServerResponse::Redirect { status_code, location }) => {
				tracing::debug!(target: LOG_TARGET, "Redirection: status_code: {}, location: {}", status_code, location);
				Err(WsHandshakeError::Redirected { status_code, location })
			}

			Err(e) => Err(e.into()),
		}
	}
}

#[cfg(feature = "__tls")]
async fn connect(
	sockaddr: SocketAddr,
	timeout_dur: Duration,
	host: &str,
	tls_connector: Option<&tokio_rustls::TlsConnector>,
	tcp_no_delay: bool,
) -> Result<EitherStream, WsHandshakeError> {
	let socket = TcpStream::connect(sockaddr);
	let timeout = tokio::time::sleep(timeout_dur);
	tokio::select! {
		socket = socket => {
			let socket = socket?;
			if let Err(err) = socket.set_nodelay(tcp_no_delay) {
				tracing::warn!(target: LOG_TARGET, "set nodelay failed: {:?}", err);
			}
			match tls_connector {
				None => Ok(EitherStream::Plain(socket)),
				Some(connector) => {
					let server_name: rustls_pki_types::ServerName = host.try_into().map_err(|e| WsHandshakeError::Url(format!("Invalid host: {host} {e:?}").into()))?;
					let tls_stream = connector.connect(server_name.to_owned(), socket).await?;
					Ok(EitherStream::Tls(tls_stream))
				}
			}
		}
		_ = timeout => Err(WsHandshakeError::Timeout(timeout_dur))
	}
}

#[cfg(not(feature = "__tls"))]
async fn connect(sockaddr: SocketAddr, timeout_dur: Duration) -> Result<EitherStream, WsHandshakeError> {
	let socket = TcpStream::connect(sockaddr);
	let timeout = tokio::time::sleep(timeout_dur);
	tokio::select! {
		socket = socket => {
			let socket = socket?;
			if let Err(err) = socket.set_nodelay(true) {
				tracing::warn!(target: LOG_TARGET, "set nodelay failed: {:?}", err);
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

impl TryFrom<url::Url> for Target {
	type Error = WsHandshakeError;

	fn try_from(url: Url) -> Result<Self, Self::Error> {
		let _mode = match url.scheme() {
			"ws" => Mode::Plain,
			#[cfg(feature = "__tls")]
			"wss" => Mode::Tls,
			invalid_scheme => {
				#[cfg(feature = "__tls")]
				let err = format!("`{invalid_scheme}` not supported, expects 'ws' or 'wss'");
				#[cfg(not(feature = "__tls"))]
				let err = format!("`{invalid_scheme}` not supported, expects 'ws' ('wss' requires the tls feature)");
				return Err(WsHandshakeError::Url(err.into()));
			}
		};
		let host = url.host_str().map(ToOwned::to_owned).ok_or_else(|| WsHandshakeError::Url("Invalid host".into()))?;

		let mut path_and_query = url.path().to_owned();
		if let Some(query) = url.query() {
			path_and_query.push('?');
			path_and_query.push_str(query);
		}

		let sockaddrs = url.socket_addrs(|| None).map_err(WsHandshakeError::ResolutionFailed)?;
		Ok(Self {
			sockaddrs,
			host,
			host_header: url.authority().to_string(),
			_mode,
			path_and_query: path_and_query.to_string(),
		})
	}
}

// NOTE: this is slow and should be used sparingly.
#[cfg(feature = "__tls")]
fn build_tls_config(cert_store: &CertificateStore) -> Result<tokio_rustls::TlsConnector, WsHandshakeError> {
	use tokio_rustls::rustls;

	let mut roots = rustls::RootCertStore::empty();

	match cert_store {
		#[cfg(feature = "native-tls")]
		CertificateStore::Native => {
			let mut first_error = None;
			let certs = rustls_native_certs::load_native_certs().map_err(WsHandshakeError::CertificateStore)?;
			for cert in certs {
				if let Err(err) = roots.add(cert) {
					first_error = first_error.or_else(|| Some(io::Error::new(io::ErrorKind::InvalidData, err)));
				}
			}
			if roots.is_empty() {
				let err = first_error
					.unwrap_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No valid certificate found"));
				return Err(WsHandshakeError::CertificateStore(err));
			}
		}
		#[cfg(feature = "webpki-tls")]
		CertificateStore::WebPki => {
			roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
		}
		_ => {
			let err = io::Error::new(io::ErrorKind::NotFound, "Invalid certificate store");
			return Err(WsHandshakeError::CertificateStore(err));
		}
	};

	let config = rustls::ClientConfig::builder().with_root_certificates(roots).with_no_client_auth();

	Ok(std::sync::Arc::new(config).into())
}

#[cfg(test)]
mod tests {
	use super::{Mode, Target, Url, WsHandshakeError};

	fn assert_ws_target(target: Target, host: &str, host_header: &str, mode: Mode, path_and_query: &str) {
		assert_eq!(&target.host, host);
		assert_eq!(&target.host_header, host_header);
		assert_eq!(target._mode, mode);
		assert_eq!(&target.path_and_query, path_and_query);
	}

	fn parse_target(uri: &str) -> Result<Target, WsHandshakeError> {
		Url::parse(uri).map_err(|e| WsHandshakeError::Url(e.to_string().into()))?.try_into()
	}

	#[test]
	fn ws_works_with_port() {
		let target = parse_target("ws://127.0.0.1:9933").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1:9933", Mode::Plain, "/");
	}

	#[cfg(feature = "__tls")]
	#[test]
	fn wss_works_with_port() {
		let target = parse_target("wss://kusama-rpc.polkadot.io:9999").unwrap();
		assert_ws_target(target, "kusama-rpc.polkadot.io", "kusama-rpc.polkadot.io:9999", Mode::Tls, "/");
	}

	#[cfg(not(feature = "__tls"))]
	#[test]
	fn wss_fails_with_tls_feature() {
		let err = parse_target("wss://kusama-rpc.polkadot.io").unwrap_err();
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
		let target = parse_target("ws://127.0.0.1/my-special-path").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1", Mode::Plain, "/my-special-path");
	}

	#[test]
	fn url_with_query_works() {
		let target = parse_target("ws://127.0.0.1/my?name1=value1&name2=value2").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1", Mode::Plain, "/my?name1=value1&name2=value2");
	}

	#[test]
	fn url_with_fragment_is_ignored() {
		let target = parse_target("ws://127.0.0.1:/my.htm#ignore").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1", Mode::Plain, "/my.htm");
	}

	#[cfg(feature = "__tls")]
	#[test]
	fn wss_default_port_is_omitted() {
		let target = parse_target("wss://127.0.0.1:443").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1", Mode::Tls, "/");
	}

	#[test]
	fn ws_default_port_is_omitted() {
		let target = parse_target("ws://127.0.0.1:80").unwrap();
		assert_ws_target(target, "127.0.0.1", "127.0.0.1", Mode::Plain, "/");
	}
}
