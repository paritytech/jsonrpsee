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

//! Host header validation.

use crate::server::host_filtering::matcher::{Matcher, Pattern};
use crate::Error;

const SPLIT_PROOF: &str = "split always returns non-empty iterator.";

/// Port pattern
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Port {
	/// No port specified (default port)
	None,
	/// Port specified as a wildcard pattern
	Pattern(String),
	/// Fixed numeric port
	Fixed(u16),
}

impl From<Option<u16>> for Port {
	fn from(opt: Option<u16>) -> Self {
		match opt {
			Some(port) => Port::Fixed(port),
			None => Port::None,
		}
	}
}

impl From<u16> for Port {
	fn from(port: u16) -> Port {
		Port::Fixed(port)
	}
}

/// Host type
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Host {
	hostname: String,
	port: Port,
	host_with_port: String,
	matcher: Matcher,
}

impl<T: AsRef<str>> From<T> for Host {
	fn from(string: T) -> Self {
		Host::parse(string.as_ref())
	}
}

impl Host {
	/// Creates a new `Host` given hostname and port number.
	pub fn new<T: Into<Port>>(hostname: &str, port: T) -> Self {
		let port = port.into();
		let hostname = Self::pre_process(hostname);
		let host_with_port = Self::from_str(&hostname, &port);
		let matcher = Matcher::new(&host_with_port);

		Host { hostname, port, host_with_port, matcher }
	}

	/// Attempts to parse given string as a `Host`.
	/// NOTE: This method always succeeds and falls back to sensible defaults.
	pub fn parse(hostname: &str) -> Self {
		let hostname = Self::pre_process(hostname);
		let mut hostname = hostname.split(':');
		let host = hostname.next().expect(SPLIT_PROOF);
		let port = match hostname.next() {
			None => Port::None,
			Some(port) => match port.parse::<u16>().ok() {
				Some(num) => Port::Fixed(num),
				None => Port::Pattern(port.into()),
			},
		};

		Host::new(host, port)
	}

	fn pre_process(host: &str) -> String {
		// Remove possible protocol definition
		let mut it = host.split("://");
		let protocol = it.next().expect(SPLIT_PROOF);
		let host = match it.next() {
			Some(data) => data,
			None => protocol,
		};

		let mut it = host.split('/');
		it.next().expect(SPLIT_PROOF).to_lowercase()
	}

	fn from_str(hostname: &str, port: &Port) -> String {
		format!(
			"{}{}",
			hostname,
			match *port {
				Port::Fixed(port) => format!(":{}", port),
				Port::Pattern(ref port) => format!(":{}", port),
				Port::None => "".into(),
			},
		)
	}
}

impl Pattern for Host {
	fn matches<T: AsRef<str>>(&self, other: T) -> bool {
		self.matcher.matches(other)
	}
}

impl std::ops::Deref for Host {
	type Target = str;

	fn deref(&self) -> &Self::Target {
		&self.host_with_port
	}
}

/// Policy for validating the `HTTP host header`.
#[derive(Debug, Clone)]
pub enum AllowHosts {
	/// Allow all hosts (no filter).
	Any,
	/// Allow only specified hosts.
	Only(Vec<Host>),
}

impl AllowHosts {
	/// Verify a host.
	pub fn verify(&self, value: &str) -> Result<(), Error> {
		if let AllowHosts::Only(list) = self {
			if !list.iter().any(|o| o.matches(value)) {
				return Err(Error::HttpHeaderRejected("host", value.into()));
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::{AllowHosts, Host, Port};

	#[test]
	fn should_parse_host() {
		assert_eq!(Host::parse("http://parity.io"), Host::new("parity.io", None));
		assert_eq!(Host::parse("https://parity.io:8443"), Host::new("parity.io", Some(8443)));
		assert_eq!(Host::parse("chrome-extension://124.0.0.1"), Host::new("124.0.0.1", None));
		assert_eq!(Host::parse("parity.io/somepath"), Host::new("parity.io", None));
		assert_eq!(Host::parse("127.0.0.1:8545/somepath"), Host::new("127.0.0.1", Some(8545)));

		let host = Host::parse("*.domain:*/somepath");
		assert_eq!(host.port, Port::Pattern("*".into()));
		assert_eq!(host.hostname.as_str(), "*.domain");
	}

	#[test]
	fn should_allow_when_validation_is_disabled() {
		assert!((AllowHosts::Any).verify("any").is_ok());
	}

	#[test]
	fn should_reject_if_header_not_on_the_list() {
		assert!((AllowHosts::Only(vec![])).verify("parity.io").is_err());
	}

	#[test]
	fn should_accept_if_on_the_list() {
		assert!((AllowHosts::Only(vec!["parity.io".into()])).verify("parity.io").is_ok());
	}

	#[test]
	fn should_accept_if_on_the_list_with_port() {
		assert!((AllowHosts::Only(vec!["parity.io:443".into()])).verify("parity.io:443").is_ok());
		assert!((AllowHosts::Only(vec!["parity.io".into()])).verify("parity.io:443").is_err());
	}

	#[test]
	fn should_support_wildcards() {
		assert!((AllowHosts::Only(vec!["*.web3.site:*".into()])).verify("parity.web3.site:8180").is_ok());
	}
}
