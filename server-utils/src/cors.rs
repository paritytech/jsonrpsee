//! CORS handling utility functions
use unicase;

pub use self::unicase::Ascii;
use crate::hosts::{Host, Port};
use crate::matcher::{Matcher, Pattern};
use std::collections::HashSet;
use std::{fmt, ops};

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
	as_string: String,
	matcher: Matcher,
}

impl<T: AsRef<str>> From<T> for Origin {
	fn from(string: T) -> Self {
		Origin::parse(string.as_ref())
	}
}

impl Origin {
	fn with_host(protocol: OriginProtocol, host: Host) -> Self {
		let string = Self::to_string(&protocol, &host);
		let matcher = Matcher::new(&string);

		Origin {
			protocol,
			host,
			as_string: string,
			matcher,
		}
	}

	/// Creates new origin given protocol, hostname and port parts.
	/// Pre-processes input data if necessary.
	pub fn new<T: Into<Port>>(protocol: OriginProtocol, host: &str, port: T) -> Self {
		Self::with_host(protocol, Host::new(host, port))
	}

	/// Attempts to parse given string as a `Origin`.
	/// NOTE: This method always succeeds and falls back to sensible defaults.
	pub fn parse(data: &str) -> Self {
		let mut it = data.split("://");
		let proto = it.next().expect("split always returns non-empty iterator.");
		let hostname = it.next();

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

	fn to_string(protocol: &OriginProtocol, host: &Host) -> String {
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
		&self.as_string
	}
}

/// Origins allowed to access
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessControlAllowOrigin {
	/// Specific hostname
	Value(Origin),
	/// null-origin (file:///, sandboxed iframe)
	Null,
	/// Any non-null origin
	Any,
}

impl fmt::Display for AccessControlAllowOrigin {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}",
			match *self {
				AccessControlAllowOrigin::Any => "*",
				AccessControlAllowOrigin::Null => "null",
				AccessControlAllowOrigin::Value(ref val) => val,
			}
		)
	}
}

impl<T: Into<String>> From<T> for AccessControlAllowOrigin {
	fn from(s: T) -> AccessControlAllowOrigin {
		match s.into().as_str() {
			"all" | "*" | "any" => AccessControlAllowOrigin::Any,
			"null" => AccessControlAllowOrigin::Null,
			origin => AccessControlAllowOrigin::Value(origin.into()),
		}
	}
}

/// Headers allowed to access
#[derive(Debug, Clone, PartialEq)]
pub enum AccessControlAllowHeaders {
	/// Specific headers
	Only(Vec<String>),
	/// Any header
	Any,
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

impl<T> Into<Option<T>> for AllowCors<T> {
	fn into(self) -> Option<T> {
		use self::AllowCors::*;

		match self {
			NotRequired | Invalid => None,
			Ok(header) => Some(header),
		}
	}
}

/// Returns correct CORS header (if any) given list of allowed origins and current origin.
pub fn get_cors_allow_origin(
	origin: Option<&str>,
	host: Option<&str>,
	allowed: &Option<Vec<AccessControlAllowOrigin>>,
) -> AllowCors<AccessControlAllowOrigin> {
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
				None if *origin == "null" => AllowCors::Ok(AccessControlAllowOrigin::Null),
				None => AllowCors::Ok(AccessControlAllowOrigin::Value(Origin::parse(origin))),
				Some(ref allowed) if *origin == "null" => allowed
					.iter()
					.find(|cors| **cors == AccessControlAllowOrigin::Null)
					.cloned()
					.map(AllowCors::Ok)
					.unwrap_or(AllowCors::Invalid),
				Some(ref allowed) => allowed
					.iter()
					.find(|cors| match **cors {
						AccessControlAllowOrigin::Any => true,
						AccessControlAllowOrigin::Value(ref val) if val.matches(origin) => true,
						_ => false,
					})
					.map(|_| AccessControlAllowOrigin::Value(Origin::parse(origin)))
					.map(AllowCors::Ok)
					.unwrap_or(AllowCors::Invalid),
			}
		}
	}
}

/// Validates if the `AccessControlAllowedHeaders` in the request are allowed.
pub fn get_cors_allow_headers<T: AsRef<str>, O, F: Fn(T) -> O>(
	mut headers: impl Iterator<Item = T>,
	requested_headers: impl Iterator<Item = T>,
	cors_allow_headers: &AccessControlAllowHeaders,
	to_result: F,
) -> AllowCors<Vec<O>> {
	// Check if the header fields which were sent in the request are allowed
	if let AccessControlAllowHeaders::Only(only) = cors_allow_headers {
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
		AccessControlAllowHeaders::Any => {
			let headers = requested_headers.map(to_result).collect();
			(false, headers)
		}
		AccessControlAllowHeaders::Only(only) => {
			let mut filtered = false;
			let headers: Vec<_> = requested_headers
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
		hs.insert(Ascii::new("Access-Control-Allow-Origin"));
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
	use crate::hosts::Host;

	#[test]
	fn should_parse_origin() {
		use self::OriginProtocol::*;

		assert_eq!(Origin::parse("http://parity.io"), Origin::new(Http, "parity.io", None));
		assert_eq!(
			Origin::parse("https://parity.io:8443"),
			Origin::new(Https, "parity.io", Some(8443))
		);
		assert_eq!(
			Origin::parse("chrome-extension://124.0.0.1"),
			Origin::new(Custom("chrome-extension".into()), "124.0.0.1", None)
		);
		assert_eq!(
			Origin::parse("parity.io/somepath"),
			Origin::new(Http, "parity.io", None)
		);
		assert_eq!(
			Origin::parse("127.0.0.1:8545/somepath"),
			Origin::new(Http, "127.0.0.1", Some(8545))
		);
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
		let res1 = get_cors_allow_origin(origin1, host, &Some(vec![]));
		let res2 = get_cors_allow_origin(origin2, host, &Some(vec![]));

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
		let res = get_cors_allow_origin(origin, host, &None);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_none_when_there_are_no_cors_domains_and_no_origin() {
		// given
		let origin = None;
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, host, &None);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_domain_when_all_are_allowed() {
		// given
		let origin = Some("parity.io");
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, host, &None);

		// then
		assert_eq!(res, AllowCors::Ok("parity.io".into()));
	}

	#[test]
	fn should_return_none_for_empty_origin() {
		// given
		let origin = None;
		let host = None;

		// when
		let res = get_cors_allow_origin(
			origin,
			host,
			&Some(vec![AccessControlAllowOrigin::Value("http://ethereum.org".into())]),
		);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_none_for_empty_list() {
		// given
		let origin = None;
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, host, &Some(Vec::new()));

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_none_for_not_matching_origin() {
		// given
		let origin = Some("http://parity.io".into());
		let host = None;

		// when
		let res = get_cors_allow_origin(
			origin,
			host,
			&Some(vec![AccessControlAllowOrigin::Value("http://ethereum.org".into())]),
		);

		// then
		assert_eq!(res, AllowCors::Invalid);
	}

	#[test]
	fn should_return_specific_origin_if_we_allow_any() {
		// given
		let origin = Some("http://parity.io".into());
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, host, &Some(vec![AccessControlAllowOrigin::Any]));

		// then
		assert_eq!(
			res,
			AllowCors::Ok(AccessControlAllowOrigin::Value("http://parity.io".into()))
		);
	}

	#[test]
	fn should_return_none_if_origin_is_not_defined() {
		// given
		let origin = None;
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, host, &Some(vec![AccessControlAllowOrigin::Null]));

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_null_if_origin_is_null() {
		// given
		let origin = Some("null".into());
		let host = None;

		// when
		let res = get_cors_allow_origin(origin, host, &Some(vec![AccessControlAllowOrigin::Null]));

		// then
		assert_eq!(res, AllowCors::Ok(AccessControlAllowOrigin::Null));
	}

	#[test]
	fn should_return_specific_origin_if_there_is_a_match() {
		// given
		let origin = Some("http://parity.io".into());
		let host = None;

		// when
		let res = get_cors_allow_origin(
			origin,
			host,
			&Some(vec![
				AccessControlAllowOrigin::Value("http://ethereum.org".into()),
				AccessControlAllowOrigin::Value("http://parity.io".into()),
			]),
		);

		// then
		assert_eq!(
			res,
			AllowCors::Ok(AccessControlAllowOrigin::Value("http://parity.io".into()))
		);
	}

	#[test]
	fn should_support_wildcards() {
		// given
		let origin1 = Some("http://parity.io".into());
		let origin2 = Some("http://parity.iot".into());
		let origin3 = Some("chrome-extension://test".into());
		let host = None;
		let allowed = Some(vec![
			AccessControlAllowOrigin::Value("http://*.io".into()),
			AccessControlAllowOrigin::Value("chrome-extension://*".into()),
		]);

		// when
		let res1 = get_cors_allow_origin(origin1, host, &allowed);
		let res2 = get_cors_allow_origin(origin2, host, &allowed);
		let res3 = get_cors_allow_origin(origin3, host, &allowed);

		// then
		assert_eq!(
			res1,
			AllowCors::Ok(AccessControlAllowOrigin::Value("http://parity.io".into()))
		);
		assert_eq!(res2, AllowCors::Invalid);
		assert_eq!(
			res3,
			AllowCors::Ok(AccessControlAllowOrigin::Value("chrome-extension://test".into()))
		);
	}

	#[test]
	fn should_return_invalid_if_header_not_allowed() {
		// given
		let cors_allow_headers = AccessControlAllowHeaders::Only(vec!["x-allowed".to_owned()]);
		let headers = vec!["Access-Control-Request-Headers"];
		let requested = vec!["x-not-allowed"];

		// when
		let res = get_cors_allow_headers(headers.iter(), requested.iter(), &cors_allow_headers.into(), |x| x);

		// then
		assert_eq!(res, AllowCors::Invalid);
	}

	#[test]
	fn should_return_valid_if_header_allowed() {
		// given
		let allowed = vec!["x-allowed".to_owned()];
		let cors_allow_headers = AccessControlAllowHeaders::Only(allowed.clone());
		let headers = vec!["Access-Control-Request-Headers"];
		let requested = vec!["x-allowed"];

		// when
		let res = get_cors_allow_headers(headers.iter(), requested.iter(), &cors_allow_headers.into(), |x| {
			(*x).to_owned()
		});

		// then
		let allowed = vec!["x-allowed".to_owned()];
		assert_eq!(res, AllowCors::Ok(allowed));
	}

	#[test]
	fn should_return_no_allowed_headers_if_none_in_request() {
		// given
		let allowed = vec!["x-allowed".to_owned()];
		let cors_allow_headers = AccessControlAllowHeaders::Only(allowed.clone());
		let headers: Vec<String> = vec![];

		// when
		let res = get_cors_allow_headers(headers.iter(), iter::empty(), &cors_allow_headers, |x| x);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

	#[test]
	fn should_return_not_required_if_any_header_allowed() {
		// given
		let cors_allow_headers = AccessControlAllowHeaders::Any;
		let headers: Vec<String> = vec![];

		// when
		let res = get_cors_allow_headers(headers.iter(), iter::empty(), &cors_allow_headers.into(), |x| x);

		// then
		assert_eq!(res, AllowCors::NotRequired);
	}

}
