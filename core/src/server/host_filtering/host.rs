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

use std::str::FromStr;

use crate::server::host_filtering::matcher::{Matcher, Pattern};
use crate::Error;
use http::uri::{InvalidUri, Uri};

/// Port pattern
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
enum Port {
	/// No port specified (default port)
	Default,
	/// Port specified as a wildcard pattern (*).
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

/// Represent the http URI scheme that is returned by the HTTP host header
///
/// <http-URI = "http:" "//" authority path-abempty [ "?" query ][ "#" fragment ]>
///
/// Further information can be found: https://www.rfc-editor.org/rfc/rfc7230#section-2.7.1
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Authority {
	hostname: String,
	port: Port,
}

impl FromStr for Authority {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let uri: Uri = s.parse().map_err(|e: InvalidUri| e.to_string())?;
		let authority = uri.authority().ok_or_else(|| "HTTP Host must contain authority".to_owned())?;
		let hostname = authority.host();
		let maybe_port = &authority.as_str()[hostname.len()..];

		// After the host segment, the authority may contain a port such as `fooo:33`, `foo:*` or `foo`
		let port = match maybe_port.split_once(':') {
			Some((_, "*")) => Port::Any,
			Some((_, p)) => {
				let port_u16 = p.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
				Port::Fixed(port_u16)
			}
			None => Port::Default,
		};

		Ok(Self { hostname: hostname.to_string(), port })
	}
}

/// Represents a whitelisted host/authority.
/// which contains a matcher to decide whether to
/// reject or accept a request.
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct AllowHost {
	authority: Authority,
	matcher: Matcher,
}

impl AllowHost {
	fn matches(&self, other: &Authority) -> bool {
		let port_match = match (&self.authority.port, &other.port) {
			(Port::Any, _) => true,
			(Port::Default, Port::Default) => true,
			(Port::Fixed(p1), Port::Fixed(p2)) if p1 == p2 => true,
			_ => false,
		};

		port_match && self.matcher.matches(&other.hostname)
	}
}

impl FromStr for AllowHost {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let authority = Authority::from_str(s)?;
		let matcher = Matcher::new(&authority.hostname);

		Ok(Self { authority, matcher })
	}
}

impl Pattern for AllowHost {
	fn matches<T: AsRef<str>>(&self, other: T) -> bool {
		self.matcher.matches(other)
	}
}

/// Policy for validating the `HTTP host header`.
#[derive(Debug, Clone)]
pub enum AllowHosts {
	/// Allow all hosts (no filter).
	Any,
	/// Allow only specified hosts.
	Only(Vec<AllowHost>),
}

impl AllowHosts {
	/// Verify a host.
	pub fn verify(&self, value: &str) -> Result<(), Error> {
		let authority = Authority::from_str(value)
			.map_err(|_| Error::HttpHeaderRejected("host", format!("Invalid authority: {value}")))?;

		if let AllowHosts::Only(list) = self {
			if !list.iter().any(|o| o.matches(&authority)) {
				return Err(Error::HttpHeaderRejected("host", value.into()));
			}
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::{AllowHost, AllowHosts, Authority, Port};
	use std::str::FromStr;

	fn authority(host: &str, port: Port) -> Authority {
		Authority { hostname: host.to_owned(), port }
	}

	#[test]
	fn should_parse_valid_authority() {
		assert_eq!(Authority::from_str("http://parity.io").unwrap(), authority("parity.io", Port::Default));
		assert_eq!(Authority::from_str("https://parity.io:8443").unwrap(), authority("parity.io", Port::Fixed(8443)));
		assert_eq!(Authority::from_str("chrome-extension://124.0.0.1").unwrap(), authority("124.0.0.1", Port::Default));
		assert_eq!(Authority::from_str("http://*.domain:*/somepath").unwrap(), authority("*.domain", Port::Any));
		assert_eq!(Authority::from_str("parity.io").unwrap(), authority("parity.io", Port::Default));
		assert_eq!(
			Authority::from_str("http://[2001:db8:85a3:8d3:1319:8a2e:370:7348]:9933/").unwrap(),
			authority("[2001:db8:85a3:8d3:1319:8a2e:370:7348]", Port::Fixed(9933))
		);
		assert_eq!(
			Authority::from_str("http://[2001:db8:85a3:8d3:1319:8a2e:370:7348]/").unwrap(),
			authority("[2001:db8:85a3:8d3:1319:8a2e:370:7348]", Port::Default)
		);
		assert_eq!(
			Authority::from_str("https://user:password@example.com/tmp/foo").unwrap(),
			authority("example.com", Port::Default)
		);
	}

	#[test]
	fn should_not_parse_invalid_authority() {
		assert!(Authority::from_str("/foo/bar").is_err());
		assert!(Authority::from_str("user:password").is_err());
		assert!(Authority::from_str("parity.io/somepath").is_err());
		assert!(Authority::from_str("127.0.0.1:8545/somepath").is_err());
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
		assert!(AllowHosts::Only(vec![AllowHost::from_str("parity.io").unwrap()]).verify("parity.io").is_ok());
	}

	#[test]
	fn should_accept_if_on_the_list_with_port() {
		assert!((AllowHosts::Only(vec![AllowHost::from_str("parity.io:443").unwrap()]))
			.verify("parity.io:443")
			.is_ok());
		assert!(AllowHosts::Only(vec![AllowHost::from_str("parity.io").unwrap()]).verify("parity.io:443").is_err());
	}

	#[test]
	fn should_support_wildcards() {
		assert!((AllowHosts::Only(vec![AllowHost::from_str("*.web3.site:*").unwrap()]))
			.verify("parity.web3.site:8180")
			.is_ok());
		assert!((AllowHosts::Only(vec![AllowHost::from_str("*.web3.site:*").unwrap()]))
			.verify("parity.web3.site")
			.is_ok());
	}
}
