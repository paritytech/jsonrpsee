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

//! # jsonrpsee-wasm-client

#![warn(missing_docs, missing_debug_implementations, missing_copy_implementations, unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(target_arch = "wasm32")]

pub use jsonrpsee_core::client::Client;
pub use jsonrpsee_types as types;

use std::time::Duration;

use jsonrpsee_client_transport::web;
use jsonrpsee_core::client::{ClientBuilder, IdKind};
use jsonrpsee_core::Error;

/// Builder for [`Client`].
///
/// # Examples
///
/// ```no_run
///
/// use jsonrpsee_wasm_client::WasmClientBuilder;
///
/// #[tokio::main]
/// async fn main() {
///     // build client
///     let client = WasmClientBuilder::default()
///          .build("wss://localhost:443")
///          .await
///          .unwrap();
///
///     // use client....
/// }
///
/// ```
#[derive(Copy, Clone, Debug)]
pub struct WasmClientBuilder {
	id_kind: IdKind,
	max_concurrent_requests: usize,
	max_buffer_capacity_per_subscription: usize,
	max_log_length: u32,
	request_timeout: Duration,
}

impl Default for WasmClientBuilder {
	fn default() -> Self {
		Self {
			id_kind: IdKind::Number,
			max_log_length: 4096,
			max_concurrent_requests: 256,
			max_buffer_capacity_per_subscription: 1024,
			request_timeout: Duration::from_secs(60),
		}
	}
}

impl WasmClientBuilder {
	/// See documentation [`ClientBuilder::request_timeout`] (default is 60 seconds).
	pub fn request_timeout(mut self, timeout: Duration) -> Self {
		self.request_timeout = timeout;
		self
	}

	/// See documentation [`ClientBuilder::max_concurrent_requests`] (default is 256).
	pub fn max_concurrent_requests(mut self, max: usize) -> Self {
		self.max_concurrent_requests = max;
		self
	}

	/// See documentation [`ClientBuilder::max_buffer_capacity_per_subscription`] (default is 1024).
	pub fn max_buffer_capacity_per_subscription(mut self, max: usize) -> Self {
		self.max_buffer_capacity_per_subscription = max;
		self
	}

	/// See documentation for [`ClientBuilder::id_format`] (default is Number).
	pub fn id_format(mut self, kind: IdKind) -> Self {
		self.id_kind = kind;
		self
	}

	/// Set maximum length for logging calls and responses.
	///
	/// Logs bigger than this limit will be truncated.
	pub fn set_max_logging_length(mut self, max: u32) -> Self {
		self.max_log_length = max;
		self
	}

	/// Build the client with specified URL to connect to.
	pub async fn build(self, url: impl AsRef<str>) -> Result<Client, Error> {
		let Self {
			max_log_length,
			id_kind,
			request_timeout,
			max_concurrent_requests,
			max_buffer_capacity_per_subscription,
		} = self;
		let (sender, receiver) = web::connect(url).await.map_err(|e| Error::Transport(e.into()))?;

		let builder = ClientBuilder::default()
			.set_max_logging_length(max_log_length)
			.request_timeout(request_timeout)
			.id_format(id_kind)
			.max_buffer_capacity_per_subscription(max_buffer_capacity_per_subscription)
			.max_concurrent_requests(max_concurrent_requests);

		Ok(builder.build_with_wasm(sender, receiver))
	}
}
