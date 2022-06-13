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

//! CORS handling utility functions

use std::collections::HashSet;
use std::{fmt, ops};

use crate::server::access_control::host::{Host, Port};
use crate::server::access_control::matcher::{Matcher, Pattern};
use crate::Cow;
use lazy_static::lazy_static;
use unicase::Ascii;

/// Origin Protocol
#[derive(Clone, Hash, Debug, PartialEq, Eq)]
pub enum OriginProtocol {
	/// Http protocol
	Http,
	/// Https protocol
	Https,
	/// Custom protocol
	Custom(String),
}

/// Request Origin
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct Origin {
	protocol: OriginProtocol,
	host: Host,
	host_with_proto: String,
	matcher: Matcher,
}

impl<T: AsRef<str>> From<T> for Origin {
	fn from(origin: T) -> Self {
		Origin::parse(origin.as_ref())
	}
}

impl Origin {
	fn with_host(protocol: OriginProtocol, host: Host) -> Self {
		let host_with_proto = Self::host_with_proto(&protocol, &host);
		let matcher = Matcher::new(&host_with_proto);

		Origin { protocol, host, host_with_proto, matcher }
	}

	/// Creates new origin given protocol, hostname and port parts.
	/// Pre-processes input data if necessary.
	pub fn new<T: Into<Port>>(protocol: OriginProtocol, host: &str, port: T) -> Self {
		Self::with_host(protocol, Host::new(host, port))
	}

	/// Attempts to parse given string as a `Origin`.
	/// NOTE: This method always succeeds and falls back to sensible defaults.
	pub fn parse(origin: &str) -> Self {
		let mut parts = origin.split("://");
		let proto = parts.next().expect("split always returns non-empty iterator.");
		let hostname = parts.next();

		let (proto, hostname) = match hostname {
			None => (None, proto),
			Some(hostname) => (Some(proto), hostname),
		};

		let proto = proto.map(str::to_lowercase);
		let hostname = Host::parse(hostname);

		let protocol = match proto {
			None => OriginProtocol::Http,
			Some(ref p) if p == "http" => OriginProtocol::Http,
			Some(ref p) if p == "https" => OriginProtocol::Https,
			Some(other) => OriginProtocol::Custom(other),
		};

		Origin::with_host(protocol, hostname)
	}

	fn host_with_proto(protocol: &OriginProtocol, host: &Host) -> String {
		format!(
			"{}://{}",
			match *protocol {
				OriginProtocol::Http => "http",
				OriginProtocol::Https => "https",
				OriginProtocol::Custom(ref protocol) => protocol,
			},
			&**host,
		)
	}
}

impl Pattern for Origin {
	fn matches<T: AsRef<str>>(&self, other: T) -> bool {
		self.matcher.matches(other)
	}
}

impl ops::Deref for Origin {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		&self.host_with_proto
	}
}

/// Origins allowed to access
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowOrigin {
	/// Specific origin.
	Origin(Origin),
	/// null-origin (file:///, sandboxed iframe)
	Null,
	/// Any non-null origin
	Any,
}

impl fmt::Display for AllowOrigin {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			match *self {
				Self::Any => "*",
				Self::Null => "null",
				Self::Origin(ref val) => val,
			}
		)
	}
}

impl<T: Into<String>> From<T> for AllowOrigin {
	fn from(s: T) -> Self {
		match s.into().as_str() {
			"all" | "*" | "any" => Self::Any,
			"null" => Self::Null,
			origin => Self::Origin(origin.into()),
		}
	}
}

/// Headers allowed to access
#[derive(Debug, Clone, PartialEq)]
pub enum AllowHeaders {
	/// Specific headers
	Only(Vec<String>),
	/// Any header
	Any,
}

impl AllowHeaders {
	/// Return an appropriate value for the CORS header "Access-Control-Allow-Headers".
	pub fn to_cors_header_value(&self) -> Cow<'_, str> {
		match self {
			AllowHeaders::Any => "*".into(),
			AllowHeaders::Only(headers) => headers.join(", ").into(),
		}
	}
}

/// CORS response headers
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllowCors<T> {
	/// CORS header was not required. Origin is not present in the request.
	NotRequired,
	/// CORS header is not returned, Origin is not allowed to access the resource.
	Invalid,
	/// CORS header to include in the response. Origin is allowed to access the resource.
	Ok(T),
}

impl<T> AllowCors<T> {
	/// Maps `Ok` variant of `AllowCors`.
	pub fn map<F, O>(self, f: F) -> AllowCors<O>
	where
		F: FnOnce(T) -> O,
	{
		use self::AllowCors::*;

		match self {
			NotRequired => NotRequired,
			Invalid => Invalid,
			Ok(val) => Ok(f(val)),
		}
	}
}

impl<T> From<AllowCors<T>> for Option<T> {
	fn from(cors: AllowCors<T>) -> Option<T> {
		use self::AllowCors::*;

		match cors {
			NotRequired | Invalid => None,
			Ok(header) => Some(header),
		}
	}
}

/// Returns correct CORS header (if any) given list of allowed origins and current origin.
pub(crate) fn get_cors_allow_origin(
	origin: Option<&str>,
	allowed: &Option<Vec<AllowOrigin>>,
	host: Option<&str>,
) -> AllowCors<AllowOrigin> {
	match origin {
		None => AllowCors::NotRequired,
		Some(ref origin) => {
			if let Some(host) = host {
				// Request initiated from the same server.
				if origin.ends_with(host) {
					// Additional check
					let origin = Origin::parse(origin);
					if &*origin.host == host {
						return AllowCors::NotRequired;
					}
				}
			}

			match allowed.as_ref() {
				None if *origin == "null" => AllowCors::Ok(AllowOrigin::Null),
				None => AllowCors::Ok(AllowOrigin::Origin(Origin::parse(origin))),
				Some(allowed) if *origin == "null" => allowed
					.iter()
					.find(|cors| **cors == AllowOrigin::Null)
					.cloned()
					.map(AllowCors::Ok)
					.unwrap_or(AllowCors::Invalid),
				Some(allowed) => allowed
					.iter()
					.find(|cors| match **cors {
						AllowOrigin::Any => true,
						AllowOrigin::Origin(ref val) if val.matches(origin) => true,
						_ => false,
					})
					.map(|_| AllowOrigin::Origin(Origin::parse(origin)))
					.map(AllowCors::Ok)
					.unwrap_or(AllowCors::Invalid),
			}
		}
	}
}

/// Validates if the headers in the request are allowed.
///
/// headers: all the headers in the request.
/// cors_request_headers: `values` in the `access-control-request-headers` header.
/// cors_allow_headers: whitelisted headers by the user.
pub(crate) fn get_cors_allow_headers<T: AsRef<str>, O, F: Fn(T) -> O>(
	mut headers: impl Iterator<Item = T>,
	cors_request_headers: impl Iterator<Item = T>,
	cors_allow_headers: &AllowHeaders,
	to_result: F,
) -> AllowCors<Vec<O>> {
	// Check if the header fields which were sent in the request are allowed
	if let AllowHeaders::Only(only) = cors_allow_headers {
		let are_all_allowed = headers.all(|header| {
			let name = &Ascii::new(header.as_ref());
			only.iter().any(|h| Ascii::new(&*h) == name) || ALWAYS_ALLOWED_HEADERS.contains(name)
		});

		if !are_all_allowed {
			return AllowCors::Invalid;
		}
	}

	// Check if `AccessControlRequestHeaders` contains fields which were allowed
	let (filtered, headers) = match cors_allow_headers {
		AllowHeaders::Any => {
			let headers = cors_request_headers.map(to_result).collect();
			(false, headers)
		}
		AllowHeaders::Only(only) => {
			let mut filtered = false;
			let headers: Vec<_> = cors_request_headers
				.filter(|header| {
					let name = &Ascii::new(header.as_ref());
					filtered = true;
					only.iter().any(|h| Ascii::new(&*h) == name) || ALWAYS_ALLOWED_HEADERS.contains(name)
				})
				.map(to_result)
				.collect();

			(filtered, headers)
		}
	};

	if headers.is_empty() {
		if filtered {
			AllowCors::Invalid
		} else {
			AllowCors::NotRequired
		}
	} else {
		AllowCors::Ok(headers)
	}
}

lazy_static! {
	/// Returns headers which are always allowed.
	static ref ALWAYS_ALLOWED_HEADERS: HashSet<Ascii<&'static str>> = {
		let mut hs = HashSet::new();
		hs.insert(Ascii::new("Accept"));
		hs.insert(Ascii::new("Accept-Language"));
		hs.insert(Ascii::new("Access-Control-Request-Headers"));
		hs.insert(Ascii::new("Content-Language"));
		hs.insert(Ascii::new("Content-Type"));
		hs.insert(Ascii::new("Host"));
		hs.insert(Ascii::new("Origin"));
		hs.insert(Ascii::new("Content-Length"));
		hs.insert(Ascii::new("Connection"));
		hs.insert(Ascii::new("User-Agent"));
		hs
	};
}

#[cfg(test)]
mod tests {
	use std::iter;

	use super::*;
	use crate::server::access_control::host::Host;

	#[test]
	fn should_parse_origin() {
		use self::OriginProtocol::*;

		assert_eq!(Origin::parse("http://parity.io"), Origin::new(Http, "parity.io", None));
		assert_eq!(Origin::parse("https://parity.io:8443"), Origin::new(Https, "parity.io", Some(8443)));
		assert_eq!(
			Origin::parse("chrome-extension://124.0.0.1"),
			Origin::new(Custom("chrome-extension".into()), "124.0.0.1", None)
		);
		assert_eq!(Origin::parse("parity.io/somepath"), Origin::new(Http, "parity.io", None));
		assert_eq!(Origin::parse("127.0.0.1:8545/somepath"), Origin::new(Http, "127.0.0.1", Some(8545)));
	}

	#[test]
	fn should_not_allow_partially_matching_origin() {
		// given
		let origin1 = Origin::parse("http://subdomain.somedomain.io");
		let origin2 = Origin::parse("http://somedomain.io:8080");
		let host = Host::parse("http://somedomain.io");

		let origin1 = Some(&*origin1);
		let origin2 = Some(&*origin2);
		let host = Some(&*host);

		// when
		let res1 = get_cors_allow_origin(origin1, &Some(vec![]), host);
		let res2 = get_cors_allow_origin(origin2, &Some(vec![]), host);

		// then
		assert_eq!(res1, AllowCors::Invalid);
		assert_eq!(res2, AllowCors::Invalid);
	}

	#[test]
	fn should_allow_origins_that_matches_hosts() {
		// given
		let origin = Origin::parse("http://127.0.0.1:8080");
		let host = Host::parse("http://127.0.0.1:8080");

		let origin = Some(&*origin);
		let host = Some(&*host);

		// when
		let res = get_cors_allow_origin(origin, &None, host);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_none_when_there_are_no_cors_domains_and_no_origin() {
		// given
		let origin = None;
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, &None, host);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_domain_when_all_are_allowed() {
		// given
		let origin = Some("parity.io");
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, &None, host);

		// then
		assert_eq!(res, AllowCors::Ok("parity.io".into()));
	}

	#[test]
	fn should_return_none_for_empty_origin() {
		// given
		let origin = None;
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, &Some(vec![AllowOrigin::Origin("http://ethereum.org".into())]), host);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_none_for_empty_list() {
		// given
		let origin = None;
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, &Some(Vec::new()), host);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_none_for_not_matching_origin() {
		// given
		let origin = Some("http://parity.io");
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, &Some(vec![AllowOrigin::Origin("http://ethereum.org".into())]), host);

		// then
		assert_eq!(res, AllowCors::Invalid);
	}

	#[test]
	fn should_return_specific_origin_if_we_allow_any() {
		// given
		let origin = Some("http://parity.io");
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, &Some(vec![AllowOrigin::Any]), host);

		// then
		assert_eq!(res, AllowCors::Ok(AllowOrigin::Origin("http://parity.io".into())));
	}

	#[test]
	fn should_return_none_if_origin_is_not_defined() {
		// given
		let origin = None;
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, &Some(vec![AllowOrigin::Null]), host);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_null_if_origin_is_null() {
		// given
		let origin = Some("null");
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, &Some(vec![AllowOrigin::Null]), host);

		// then
		assert_eq!(res, AllowCors::Ok(AllowOrigin::Null));
	}

	#[test]
	fn should_return_specific_origin_if_there_is_a_match() {
		// given
		let origin = Some("http://parity.io");
		let host = None;

		// when
		let res = get_cors_allow_origin(
			origin,
			&Some(vec![
				AllowOrigin::Origin("http://ethereum.org".into()),
				AllowOrigin::Origin("http://parity.io".into()),
			]),
			host,
		);

		// then
		assert_eq!(res, AllowCors::Ok(AllowOrigin::Origin("http://parity.io".into())));
	}

	#[test]
	fn should_support_wildcards() {
		// given
		let origin1 = Some("http://parity.io");
		let origin2 = Some("http://parity.iot");
		let origin3 = Some("chrome-extension://test");
		let host = None;
		let allowed =
			Some(vec![AllowOrigin::Origin("http://*.io".into()), AllowOrigin::Origin("chrome-extension://*".into())]);

		// when
		let res1 = get_cors_allow_origin(origin1, &allowed, host);
		let res2 = get_cors_allow_origin(origin2, &allowed, host);
		let res3 = get_cors_allow_origin(origin3, &allowed, host);

		// then
		assert_eq!(res1, AllowCors::Ok(AllowOrigin::Origin("http://parity.io".into())));
		assert_eq!(res2, AllowCors::Invalid);
		assert_eq!(res3, AllowCors::Ok(AllowOrigin::Origin("chrome-extension://test".into())));
	}

	#[test]
	fn should_return_invalid_if_header_not_allowed() {
		// given
		let cors_allow_headers = AllowHeaders::Only(vec!["x-allowed".to_owned()]);
		let headers = vec!["Access-Control-Request-Headers"];
		let requested = vec!["x-not-allowed"];

		// when
		let res = get_cors_allow_headers(headers.iter(), requested.iter(), &cors_allow_headers, |x| x);

		// then
		assert_eq!(res, AllowCors::Invalid);
	}

	#[test]
	fn should_return_valid_if_header_allowed() {
		// given
		let allowed = vec!["x-allowed".to_owned()];
		let cors_allow_headers = AllowHeaders::Only(allowed);
		let headers = vec!["Access-Control-Request-Headers"];
		let requested = vec!["x-allowed"];

		// when
		let res = get_cors_allow_headers(headers.iter(), requested.iter(), &cors_allow_headers, |x| (*x).to_owned());

		// then
		let allowed = vec!["x-allowed".to_owned()];
		assert_eq!(res, AllowCors::Ok(allowed));
	}

	#[test]
	fn should_return_no_allowed_headers_if_none_in_request() {
		// given
		let allowed = vec!["x-allowed".to_owned()];
		let cors_allow_headers = AllowHeaders::Only(allowed);
		let headers: Vec<String> = vec![];

		// when
		let res = get_cors_allow_headers(headers.iter(), iter::empty(), &cors_allow_headers, |x| x);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_not_required_if_any_header_allowed() {
		// given
		let cors_allow_headers = AllowHeaders::Any;
		let headers: Vec<String> = vec![];

		// when
		let res = get_cors_allow_headers(headers.iter(), iter::empty(), &cors_allow_headers, |x| x);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}
}
