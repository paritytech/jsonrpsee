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

//! Access control based on HTTP headers

pub(crate) mod cors;
pub(crate) mod hosts;
mod matcher;

use crate::types::Error;

use cors::{AccessControlAllowHeaders, AccessControlAllowOrigin};
use hosts::{AllowHosts, Host};
use hyper::header;
use jsonrpsee_utils::http_helpers;

/// Define access on control on HTTP layer.
#[derive(Clone, Debug)]
pub struct AccessControl {
	allowed_hosts: AllowHosts,
	allowed_origins: Option<Vec<AccessControlAllowOrigin>>,
	allowed_headers: AccessControlAllowHeaders,
	continue_on_invalid_cors: bool,
}

impl AccessControl {
	/// Validate incoming request by http HOST
	pub fn deny_host(&self, request: &hyper::Request<hyper::Body>) -> bool {
		!hosts::is_host_valid(http_helpers::read_header_value(request.headers(), "host"), &self.allowed_hosts)
	}

	/// Validate incoming request by CORS origin
	pub fn deny_cors_origin(&self, request: &hyper::Request<hyper::Body>) -> bool {
		let header = cors::get_cors_allow_origin(
			http_helpers::read_header_value(request.headers(), "origin"),
			http_helpers::read_header_value(request.headers(), "host"),
			&self.allowed_origins,
		)
		.map(|origin| {
			use self::cors::AccessControlAllowOrigin::*;
			match origin {
				Value(ref val) => {
					header::HeaderValue::from_str(val).unwrap_or_else(|_| header::HeaderValue::from_static("null"))
				}
				Null => header::HeaderValue::from_static("null"),
				Any => header::HeaderValue::from_static("*"),
			}
		});
		header == cors::AllowCors::Invalid && !self.continue_on_invalid_cors
	}

	/// Validate incoming request by CORS header
	pub fn deny_cors_header(&self, request: &hyper::Request<hyper::Body>) -> bool {
		let headers = request.headers().keys().map(|name| name.as_str());
		let requested_headers = http_helpers::read_header_values(request.headers(), "access-control-request-headers")
			.filter_map(|val| val.to_str().ok())
			.flat_map(|val| val.split(", "))
			.flat_map(|val| val.split(','));

		let header = cors::get_cors_allow_headers(headers, requested_headers, &self.allowed_headers, |name| {
			header::HeaderValue::from_str(name).unwrap_or_else(|_| header::HeaderValue::from_static("unknown"))
		});
		header == cors::AllowCors::Invalid && !self.continue_on_invalid_cors
	}
}

impl Default for AccessControl {
	fn default() -> Self {
		Self {
			allowed_hosts: AllowHosts::Any,
			allowed_origins: None,
			allowed_headers: AccessControlAllowHeaders::Any,
			continue_on_invalid_cors: false,
		}
	}
}

/// Convenience builder pattern
#[derive(Debug)]
pub struct AccessControlBuilder {
	allowed_hosts: AllowHosts,
	allowed_origins: Option<Vec<AccessControlAllowOrigin>>,
	allowed_headers: AccessControlAllowHeaders,
	continue_on_invalid_cors: bool,
}

impl Default for AccessControlBuilder {
	fn default() -> Self {
		Self {
			allowed_hosts: AllowHosts::Any,
			allowed_origins: None,
			allowed_headers: AccessControlAllowHeaders::Any,
			continue_on_invalid_cors: false,
		}
	}
}

impl AccessControlBuilder {
	/// Create a new builder for `AccessControl`.
	pub fn new() -> Self {
		Self::default()
	}

	/// Allow all hosts.
	pub fn allow_all_hosts(mut self) -> Self {
		self.allowed_hosts = AllowHosts::Any;
		self
	}

	/// Allow all origins.
	pub fn allow_all_origins(mut self) -> Self {
		self.allowed_headers = AccessControlAllowHeaders::Any;
		self
	}

	/// Allow all headers.
	pub fn allow_all_headers(mut self) -> Self {
		self.allowed_origins = None;
		self
	}

	/// Configure allowed hosts.
	///
	/// Default - allow all.
	pub fn set_allowed_hosts<List, H>(mut self, list: List) -> Result<Self, Error>
	where
		List: IntoIterator<Item = H>,
		H: Into<Host>,
	{
		let allowed_hosts: Vec<Host> = list.into_iter().map(Into::into).collect();
		if allowed_hosts.is_empty() {
			return Err(Error::EmptyAllowList("Host"));
		}
		self.allowed_hosts = AllowHosts::Only(allowed_hosts);
		Ok(self)
	}

	/// Configure allowed origins.
	///
	/// Default - allow all.
	pub fn set_allowed_origins<Origin, List>(mut self, list: List) -> Result<Self, Error>
	where
		List: IntoIterator<Item = Origin>,
		Origin: Into<AccessControlAllowOrigin>,
	{
		let allowed_origins: Vec<AccessControlAllowOrigin> = list.into_iter().map(Into::into).collect();
		if allowed_origins.is_empty() {
			return Err(Error::EmptyAllowList("Origin"));
		}
		self.allowed_origins = Some(allowed_origins);
		Ok(self)
	}

	/// Configure allowed CORS headers.
	///
	/// Default - allow all.
	pub fn set_allowed_headers<Header, List>(mut self, list: List) -> Result<Self, Error>
	where
		List: IntoIterator<Item = Header>,
		Header: Into<String>,
	{
		let allowed_headers: Vec<String> = list.into_iter().map(Into::into).collect();
		if allowed_headers.is_empty() {
			return Err(Error::EmptyAllowList("Header"));
		}
		self.allowed_headers = AccessControlAllowHeaders::Only(allowed_headers);
		Ok(self)
	}

	/// Enable or disable to continue with invalid CORS.
	///
	/// Default: false.
	pub fn continue_on_invalid_cors(mut self, continue_on_invalid_cors: bool) -> Self {
		self.continue_on_invalid_cors = continue_on_invalid_cors;
		self
	}

	/// Build.
	pub fn build(self) -> AccessControl {
		AccessControl {
			allowed_hosts: self.allowed_hosts,
			allowed_origins: self.allowed_origins,
			allowed_headers: self.allowed_headers,
			continue_on_invalid_cors: self.continue_on_invalid_cors,
		}
	}
}
