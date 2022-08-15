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

//! Origin filtering functions

use std::{fmt, ops};

use crate::server::access_control::host::{Host, Port};
use crate::server::access_control::matcher::{Matcher, Pattern};
use crate::Error;

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
pub struct InnerOrigin {
	protocol: OriginProtocol,
	host: Host,
	host_with_proto: String,
	matcher: Matcher,
}

impl<T: AsRef<str>> From<T> for InnerOrigin {
	fn from(origin: T) -> Self {
		InnerOrigin::parse(origin.as_ref())
	}
}

impl InnerOrigin {
	fn with_host(protocol: OriginProtocol, host: Host) -> Self {
		let host_with_proto = Self::host_with_proto(&protocol, &host);
		let matcher = Matcher::new(&host_with_proto);

		InnerOrigin { protocol, host, host_with_proto, matcher }
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

		InnerOrigin::with_host(protocol, hostname)
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

impl Pattern for InnerOrigin {
	fn matches<T: AsRef<str>>(&self, other: T) -> bool {
		self.matcher.matches(other)
	}
}

impl ops::Deref for InnerOrigin {
	type Target = str;
	fn deref(&self) -> &Self::Target {
		&self.host_with_proto
	}
}

/// Origin type allowed to access.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Origin {
	/// Specific origin.
	Origin(InnerOrigin),
	/// Null origin (file:///, sandboxed iframe).
	Null,
	/// Allow all origins i.e, the literal value "*" which is regarded as a wildcard.
	Wildcard,
}

impl Pattern for Origin {
	fn matches<T: AsRef<str>>(&self, other: T) -> bool {
		if other.as_ref() == "null"  {
			return *self == Origin::Null;
		}

		match self {
			Origin::Wildcard => true,
			Origin::Null => false,
			Origin::Origin(ref origin) => origin.matches(other),
		}
	}
}


impl fmt::Display for Origin {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			match *self {
				Self::Wildcard => "*",
				Self::Null => "null",
				Self::Origin(ref val) => val,
			}
		)
	}
}

impl<T: Into<String>> From<T> for Origin {
	fn from(s: T) -> Self {
		match s.into().as_str() {
			"all" | "*" | "any" => Self::Wildcard,
			"null" => Self::Null,
			origin => Self::Origin(origin.into()),
		}
	}
}

/// Policy for validating the `HTTP origin header`.
#[derive(Clone, Debug)]
pub enum AllowOrigins {
	/// Allow all origins (no filter).
	Any,
	/// Allow only specified origins.
	Only(Vec<Origin>),
}

impl AllowOrigins {
	/// Verify a origin.
	pub fn verify(&self, origin: Option<&str>, host: &str) -> Result<(), Error> {
		// Nothing to be checked if origin is not part of the request's headers.
		let origin = match origin {
			Some(ref origin) => origin,
			None => return Ok(()),
		};

		// Requests initiated from the same server are allowed.
		if origin.ends_with(host) {
			// Additional check
			let origin = InnerOrigin::parse(origin);
			if &*origin.host == host {
				return Ok(());
			}
		}

		match self {
			AllowOrigins::Any => return Ok(()),
			AllowOrigins::Only(list) => {
				if !list.iter().any(|allowed_origin| allowed_origin.matches(*origin)) {
					return Err(Error::HttpHeaderRejected("origin", origin.to_string()));
				}
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::server::access_control::host::Host;

	#[test]
	fn should_parse_origin() {
		use self::OriginProtocol::*;

		assert_eq!(InnerOrigin::parse("http://parity.io"), InnerOrigin::new(Http, "parity.io", None));
		assert_eq!(InnerOrigin::parse("https://parity.io:8443"), InnerOrigin::new(Https, "parity.io", Some(8443)));
		assert_eq!(
			InnerOrigin::parse("chrome-extension://124.0.0.1"),
			InnerOrigin::new(Custom("chrome-extension".into()), "124.0.0.1", None)
		);
		assert_eq!(InnerOrigin::parse("parity.io/somepath"), InnerOrigin::new(Http, "parity.io", None));
		assert_eq!(InnerOrigin::parse("127.0.0.1:8545/somepath"), InnerOrigin::new(Http, "127.0.0.1", Some(8545)));
	}

	#[test]
	fn should_not_allow_partially_matching_origin() {
		let origin1 = InnerOrigin::parse("http://subdomain.somedomain.io");
		let origin2 = InnerOrigin::parse("http://somedomain.io:8080");
		let host = Host::parse("http://somedomain.io");

		let origin1 = Some(&*origin1);
		let origin2 = Some(&*origin2);

		let allow_origins = AllowOrigins::Only(vec![]);

		assert!(allow_origins.verify(origin1, &*host).is_err());
		assert!(allow_origins.verify(origin2, &*host).is_err());
	}

	#[test]
	fn should_allow_origins_that_matches_hosts() {
		let origin = InnerOrigin::parse("http://127.0.0.1:8080");
		let host = Host::parse("http://127.0.0.1:8080");

		let origin = Some(&*origin);
		let allow_origins = AllowOrigins::Any;

		assert!(allow_origins.verify(origin, &*host).is_ok());
	}

	#[test]
	fn should_allow_when_there_are_no_domains_and_no_origin() {
		let origin = None;
		let host = "";
		let allow_origins = AllowOrigins::Any;

		assert!(allow_origins.verify(origin, host).is_ok());
	}

	#[test]
	fn should_allow_domain_when_all_are_allowed() {
		let origin = Some("parity.io");
		let host = "";
		let allow_origins = AllowOrigins::Any;

		assert!(allow_origins.verify(origin, host).is_ok());
	}

	#[test]
	fn should_allow_for_empty_origin() {
		let origin = None;
		let host = "";
		let allow_origins = AllowOrigins::Only(vec![Origin::Origin("http://ethereum.org".into())]);

		assert!(allow_origins.verify(origin, host).is_ok());
	}

	#[test]
	fn should_allow_specific_empty_list() {
		let origin = None;
		let host = "";
		let allow_origins = AllowOrigins::Only(vec![]);

		assert!(allow_origins.verify(origin, host).is_ok());
	}

	#[test]
	fn should_deny_for_different_origin() {
		let origin = Some("http://parity.io");
		let host = "";
		let allow_origins = AllowOrigins::Only(vec![Origin::Origin("http://ethereum.org".into())]);

		assert!(allow_origins.verify(origin, host).is_err());
	}

	#[test]
	fn should_allow_for_any() {
		let origin = Some("http://parity.io");
		let host = "";
		let allow_origins = AllowOrigins::Only(vec![Origin::Wildcard]);

		assert!(allow_origins.verify(origin, host).is_ok());
	}

	#[test]
	fn should_allow_if_origin_is_not_defined() {
		let origin = None;
		let host = "";
		let allow_origins = AllowOrigins::Only(vec![Origin::Null]);

		assert!(allow_origins.verify(origin, host).is_ok());
	}

	#[test]
	fn should_allow_if_origin_is_null() {
		let origin = Some("null");
		let host = "";
		let allow_origins = AllowOrigins::Only(vec![Origin::Null]);

		assert!(allow_origins.verify(origin, host).is_ok());
	}

	#[test]
	fn should_allow_if_there_is_a_match() {
		let origin = Some("http://parity.io");
		let host = "";

		let allow_origins = AllowOrigins::Only(vec![
			Origin::Origin("http://ethereum.org".into()),
			Origin::Origin("http://parity.io".into()),
		]);

		assert!(allow_origins.verify(origin, host).is_ok());
	}

	#[test]
	fn should_support_wildcards() {
		let origin1 = Some("http://parity.io");
		let origin2 = Some("http://parity.iot");
		let origin3 = Some("chrome-extension://test");
		let host = "";
		let allow_origins =
			AllowOrigins::Only(vec![Origin::Origin("http://*.io".into()), Origin::Origin("chrome-extension://*".into())]);

		assert!(allow_origins.verify(origin1, host).is_ok());
		assert!(allow_origins.verify(origin2, host).is_err());
		assert!(allow_origins.verify(origin3, host).is_ok());
	}
}
