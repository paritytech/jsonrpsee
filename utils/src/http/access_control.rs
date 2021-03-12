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

//! Access control based on http headers

use crate::http::cors::{AccessControlAllowHeaders, AccessControlAllowOrigin};
use crate::http::hosts::{AllowHosts, Host};
use crate::http::{cors, hosts, hyper_helpers};
use hyper::header;

/// Define access on control on http layer
#[derive(Clone)]
pub struct AccessControl {
	allow_hosts: AllowHosts,
	cors_allow_origin: Option<Vec<AccessControlAllowOrigin>>,
	cors_max_age: Option<u32>,
	cors_allow_headers: AccessControlAllowHeaders,
	continue_on_invalid_cors: bool,
}

impl AccessControl {
	/// Validate incoming request by http HOST
	pub fn deny_host(&self, request: &hyper::Request<hyper::Body>) -> bool {
		!hosts::is_host_valid(hyper_helpers::read_header_value(request.headers(), "host"), &self.allow_hosts)
	}

	/// Validate incoming request by CORS origin
	pub fn deny_cors_origin(&self, request: &hyper::Request<hyper::Body>) -> bool {
		let header = cors::get_cors_allow_origin(
			hyper_helpers::read_header_value(request.headers(), "origin"),
			hyper_helpers::read_header_value(request.headers(), "host"),
			&self.cors_allow_origin,
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
		let requested_headers = hyper_helpers::read_header_values(request.headers(), "access-control-request-headers")
			.filter_map(|val| val.to_str().ok())
			.flat_map(|val| val.split(", "))
			.flat_map(|val| val.split(','));

		let header = cors::get_cors_allow_headers(headers, requested_headers, &self.cors_allow_headers, |name| {
			header::HeaderValue::from_str(name).unwrap_or_else(|_| header::HeaderValue::from_static("unknown"))
		});
		header == cors::AllowCors::Invalid && !self.continue_on_invalid_cors
	}
}

impl Default for AccessControl {
	fn default() -> Self {
		Self {
			allow_hosts: AllowHosts::Any,
			cors_allow_origin: None,
			cors_max_age: None,
			cors_allow_headers: AccessControlAllowHeaders::Any,
			continue_on_invalid_cors: false,
		}
	}
}

/// Convenience builder pattern
pub struct AccessControlBuilder {
	allow_hosts: AllowHosts,
	cors_allow_origin: Option<Vec<AccessControlAllowOrigin>>,
	cors_max_age: Option<u32>,
	cors_allow_headers: AccessControlAllowHeaders,
	continue_on_invalid_cors: bool,
}

impl Default for AccessControlBuilder {
	fn default() -> Self {
		Self {
			allow_hosts: AllowHosts::Any,
			cors_allow_origin: None,
			cors_max_age: None,
			cors_allow_headers: AccessControlAllowHeaders::Any,
			continue_on_invalid_cors: false,
		}
	}
}

impl AccessControlBuilder {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn allow_host(mut self, host: Host) -> Self {
		let allow_hosts = match self.allow_hosts {
			AllowHosts::Any => vec![host],
			AllowHosts::Only(mut allow_hosts) => {
				allow_hosts.push(host);
				allow_hosts
			}
		};
		self.allow_hosts = AllowHosts::Only(allow_hosts);
		self
	}

	pub fn cors_allow_origin(mut self, allow_origin: AccessControlAllowOrigin) -> Self {
		let cors_allow_origin = match self.cors_allow_origin {
			Some(mut cors_allow_origin) => {
				cors_allow_origin.push(allow_origin);
				cors_allow_origin
			}
			None => vec![allow_origin],
		};
		self.cors_allow_origin = Some(cors_allow_origin);
		self
	}

	pub fn cors_max_age(mut self, max_age: u32) -> Self {
		self.cors_max_age = Some(max_age);
		self
	}

	pub fn cors_allow_header(mut self, header: String) -> Self {
		let allow_headers = match self.cors_allow_headers {
			AccessControlAllowHeaders::Any => vec![header],
			AccessControlAllowHeaders::Only(mut allow_headers) => {
				allow_headers.push(header);
				allow_headers
			}
		};
		self.cors_allow_headers = AccessControlAllowHeaders::Only(allow_headers);
		self
	}

	pub fn continue_on_invalid_cors(mut self, continue_on_invalid_cors: bool) -> Self {
		self.continue_on_invalid_cors = continue_on_invalid_cors;
		self
	}

	pub fn build(self) -> AccessControl {
		AccessControl {
			allow_hosts: self.allow_hosts,
			cors_allow_origin: self.cors_allow_origin,
			cors_max_age: self.cors_max_age,
			cors_allow_headers: self.cors_allow_headers,
			continue_on_invalid_cors: self.continue_on_invalid_cors,
		}
	}
}
