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
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Port {
	/// No port specified (default port)
	Default,
	/// Wildcard i.e, `*` matches any port.
	Any,
	/// Fixed numeric port
	Fixed(u16),
}

impl From<Option<u16>> for Port {
	fn from(opt: Option<u16>) -> Self {
		match opt {
			Some(port) => Port::Fixed(port),
			None => Port::Default,
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
	matcher: Matcher,
	port: Port,
}

impl<T: AsRef<str>> From<T> for Host {
	fn from(string: T) -> Self {
		Host::parse(string.as_ref())
	}
}

impl Host {
	/// Attempts to parse given string as a `Host`.
	/// NOTE: This method always succeeds and falls back to sensible defaults.
	pub fn parse(hostname: &str) -> Self {
		let (hostname, port) = parse_host(hostname);
		Self::new(hostname, port)
	}

	pub fn new(host: String, port: Port) -> Self {
		Self { matcher: Matcher::new(&host), hostname: host.to_string(), port }
	}

	fn matches(&self, other_host: &str, other_port: Port) -> bool {
		let port_matches = match (&self.port, &other_port) {
			(Port::Any, _) => true,
			(Port::Default, Port::Default) => true,
			(Port::Fixed(p), Port::Fixed(o)) if p == o => true,
			_ => false,
		};

		port_matches && self.matcher.matches(other_host)
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
		let (host, port) = parse_host(value);

		if let AllowHosts::Only(list) = self {
			if !list.iter().any(|o| o.matches(&host, port)) {
				return Err(Error::HttpHeaderRejected("host", value.into()));
			}
		}

		Ok(())
	}
}

fn parse_host(input: &str) -> (String, Port) {
	// Remove possible protocol definition such as `wss://<hostname>:<port>/<route>`.
	let with_route = input.split("://").last().expect(SPLIT_PROOF);

	// Remove everything beyond `/` which is part of the route such as `<hostname>:<port>/<route>`.
	let mut it = with_route.split('/');
	let host_and_port = it.next().expect(SPLIT_PROOF).to_lowercase();

	// The rest is `host:<optional port>`.
	let mut it = host_and_port.split(":");

	let host = it.next().expect(SPLIT_PROOF).to_string();

	let port = match it.next() {
		None => Port::Default,
		Some("*") => Port::Any,
		Some(p) => Port::Fixed(p.parse().expect("Port must be u16; qed")),
	};

	(host, port)
}

#[cfg(test)]
mod tests {
	use super::{AllowHosts, Host, Port};

	#[test]
	fn should_parse_host() {
		assert_eq!(Host::parse("http://parity.io"), Host::new("parity.io".into(), Port::Default));
		assert_eq!(Host::parse("https://parity.io:8443"), Host::new("parity.io".into(), Port::Fixed(8443)));
		assert_eq!(Host::parse("chrome-extension://124.0.0.1"), Host::new("124.0.0.1".into(), Port::Default));
		assert_eq!(Host::parse("parity.io/somepath"), Host::new("parity.io".into(), Port::Default));
		assert_eq!(Host::parse("127.0.0.1:8545/somepath"), Host::new("127.0.0.1".into(), Port::Fixed(8545)));

		let host = Host::parse("*.domain:*/somepath");
		assert_eq!(host.port, Port::Any);
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
		assert!((AllowHosts::Only(vec!["*.web3.site:*".into()])).verify("parity.web3.site").is_ok());
	}
}
