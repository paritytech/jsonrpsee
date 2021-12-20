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

#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]

//! # jsonrpsee-ws-client
//!
//! `jsonrpsee-ws-client` is a [JSON RPC](https://www.jsonrpc.org/specification) WebSocket client library that's is built for `async/await`.
//!
//! ## Runtime support
//!
//! This library uses `tokio` as the runtime and does not support other kinds of runtimes.

#[cfg(test)]
mod tests;

pub use jsonrpsee_types as types;

use jsonrpsee_client_transport::ws::{Header, InvalidUri, Uri, WsTransportClientBuilder};
use jsonrpsee_core_client::{Client, ClientBuilder};
use jsonrpsee_core::{client::CertificateStore, Error};
use std::time::Duration;
use types::TEN_MB_SIZE_BYTES;

/// Builder for [`Client`].
///
/// # Examples
///
/// ```no_run
///
/// use jsonrpsee_ws_client::WsClientBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     // build client
///     let client = WsClientBuilder::default()
///          .add_header("Any-Header-You-Like", "42")
///          .build("wss://localhost:443")
///          .await
///          .unwrap();
///
///     // use client....
/// }
///
/// ```
#[derive(Clone, Debug)]
pub struct WsClientBuilder<'a> {
	certificate_store: CertificateStore,
	max_request_body_size: u32,
	request_timeout: Duration,
	connection_timeout: Duration,
	headers: Vec<Header<'a>>,
	max_concurrent_requests: usize,
	max_notifs_per_subscription: usize,
	max_redirections: usize,
}

impl<'a> Default for WsClientBuilder<'a> {
	fn default() -> Self {
		Self {
			certificate_store: CertificateStore::Native,
			max_request_body_size: TEN_MB_SIZE_BYTES,
			request_timeout: Duration::from_secs(60),
			connection_timeout: Duration::from_secs(10),
			headers: Vec::new(),
			max_concurrent_requests: 256,
			max_notifs_per_subscription: 1024,
			max_redirections: 5,
		}
	}
}

impl<'a> WsClientBuilder<'a> {
	/// Set whether to use system certificates
	pub fn certificate_store(mut self, certificate_store: CertificateStore) -> Self {
		self.certificate_store = certificate_store;
		self
	}

	/// Set max request body size.
	pub fn max_request_body_size(mut self, size: u32) -> Self {
		self.max_request_body_size = size;
		self
	}

	/// Set request timeout (default is 60 seconds).
	pub fn request_timeout(mut self, timeout: Duration) -> Self {
		self.request_timeout = timeout;
		self
	}

	/// Set connection timeout for the handshake.
	pub fn connection_timeout(mut self, timeout: Duration) -> Self {
		self.connection_timeout = timeout;
		self
	}

	/// Set a custom header passed to the server during the handshake.
	///
	/// The caller is responsible for checking that the headers do not conflict or are duplicated.
	pub fn add_header(mut self, name: &'a str, value: &'a str) -> Self {
		self.headers.push(Header { name, value: value.as_bytes() });
		self
	}

	/// Set max concurrent requests.
	pub fn max_concurrent_requests(mut self, max: usize) -> Self {
		self.max_concurrent_requests = max;
		self
	}

	/// Set max concurrent notification capacity for each subscription; when the capacity is exceeded the subscription
	/// will be dropped.
	///
	/// You can also prevent the subscription being dropped by calling
	/// [`Subscription::next()`](crate::types::Subscription) frequently enough such that the buffer capacity doesn't
	/// exceeds.
	///
	/// **Note**: The actual capacity is `num_senders + max_subscription_capacity`
	/// because it is passed to [`futures::channel::mpsc::channel`].
	pub fn max_notifs_per_subscription(mut self, max: usize) -> Self {
		self.max_notifs_per_subscription = max;
		self
	}

	/// Set the max number of redirections to perform until a connection is regarded as failed.
	pub fn max_redirections(mut self, redirect: usize) -> Self {
		self.max_redirections = redirect;
		self
	}

	/// Build the client with specified URL to connect to.
	/// You must provide the port number in the URL.
	///
	/// ## Panics
	///
	/// Panics if being called outside of `tokio` runtime context.
	pub async fn build(self, url: impl AsRef<str>) -> Result<Client, Error> {
		let transport_builder = WsTransportClientBuilder {
			certificate_store: self.certificate_store,
			connection_timeout: self.connection_timeout,
			headers: self.headers,
			max_request_body_size: self.max_request_body_size,
			max_redirections: self.max_redirections,
		};

		let uri: Uri = url.as_ref().parse().map_err(|e: InvalidUri| Error::Transport(e.into()))?;
		let (sender, receiver) = transport_builder.build(uri).await.map_err(|e| Error::Transport(e.into()))?;

		Ok(ClientBuilder::default()
			.max_notifs_per_subscription(self.max_notifs_per_subscription)
			.request_timeout(self.request_timeout)
			.max_concurrent_requests(self.max_concurrent_requests)
			.build(sender, receiver))
	}
}
