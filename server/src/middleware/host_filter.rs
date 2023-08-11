// Copyright 2019-2023 Parity Technologies (UK) Ltd.
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

//! HTTP host validation middleware.

use crate::transport::http as http_helpers;
use futures_util::{Future, FutureExt, TryFutureExt};
use http::uri::{InvalidUri, Uri};
use hyper::{Body, Request, Response};
use jsonrpsee_core::Error;
use route_recognizer::Router;
use std::error::Error as StdError;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tower::{Layer, Service};

/// Middleware to enable host filtering.
#[derive(Debug)]
pub struct HostFilterLayer(Arc<WhitelistedHosts>);

impl HostFilterLayer {
	/// Enables host filtering and allow only the specified hosts.
	pub fn new<T: IntoIterator<Item = U>, U: TryInto<Authority>>(allow_only: T) -> Result<Self, AuthorityError>
	where
		T: IntoIterator<Item = U>,
		U: TryInto<Authority, Error = AuthorityError>,
	{
		let allow_only: Result<Vec<_>, _> = allow_only.into_iter().map(|a| a.try_into()).collect();
		Ok(Self(Arc::new(WhitelistedHosts::from(allow_only?))))
	}
}

impl<S> Layer<S> for HostFilterLayer {
	type Service = HostFilter<S>;

	fn layer(&self, inner: S) -> Self::Service {
		HostFilter { inner, filter: self.0.clone() }
	}
}

/// Middleware to enable host filtering.
#[derive(Debug)]
pub struct HostFilter<S> {
	inner: S,
	filter: Arc<WhitelistedHosts>,
}

impl<S> Service<Request<Body>> for HostFilter<S>
where
	S: Service<Request<Body>, Response = Response<Body>>,
	S::Response: 'static,
	S::Error: Into<Box<dyn StdError + Send + Sync>> + 'static,
	S::Future: Send + 'static,
{
	type Response = S::Response;
	type Error = Box<dyn StdError + Send + Sync + 'static>;
	type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.inner.poll_ready(cx).map_err(Into::into)
	}

	fn call(&mut self, request: Request<Body>) -> Self::Future {
		let Some(authority) = http_helpers::authority(&request) else {
			return async { Ok(http_helpers::response::malformed()) }.boxed();
		};

		if self.filter.recognize(&authority) {
			Box::pin(self.inner.call(request).map_err(Into::into))
		} else {
			tracing::debug!("Denied request: {:?}", request);
			async { Ok(http_helpers::response::host_not_allowed()) }.boxed()
		}
	}
}

/// Port pattern
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Port {
	/// No port specified (default port)
	Default,
	/// Port specified as a wildcard pattern (*).
	Any,
	/// Fixed numeric port
	Fixed(u16),
}

impl From<u16> for Port {
	fn from(port: u16) -> Port {
		Port::Fixed(port)
	}
}

/// Represent the http URI scheme that is returned by the HTTP host header
///
/// Further information can be found: <https://www.rfc-editor.org/rfc/rfc7230#section-2.7.1>
#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub struct Authority {
	hostname: String,
	port: Port,
}

impl Authority {
	fn inner_from_str(value: &str) -> Result<Self, AuthorityError> {
		let uri: Uri = value.parse().map_err(AuthorityError::InvalidUri)?;
		let authority = uri.authority().ok_or(AuthorityError::MissingHost)?;
		let hostname = authority.host();
		let maybe_port = &authority.as_str()[hostname.len()..];

		// After the host segment, the authority may contain a port such as `fooo:33`, `foo:*` or `foo`
		let port = match maybe_port.split_once(':') {
			Some((_, "*")) => Port::Any,
			Some((_, p)) => {
				let port_u16: u16 =
					p.parse().map_err(|e: std::num::ParseIntError| AuthorityError::InvalidPort(e.to_string()))?;

				// Omit default port to allow both requests with and without the default port.
				match default_port(uri.scheme_str()) {
					Some(p) if p == port_u16 => Port::Default,
					_ => port_u16.into(),
				}
			}
			None => Port::Default,
		};

		Ok(Self { hostname: hostname.to_string(), port })
	}
}

/// Error that can happen when parsing an URI authority fails.
#[derive(Debug, thiserror::Error)]
pub enum AuthorityError {
	/// Invalid URI.
	#[error("{0}")]
	InvalidUri(InvalidUri),
	/// Invalid port.
	#[error("{0}")]
	InvalidPort(String),
	/// The host was not found.
	#[error("The host was not found")]
	MissingHost,
}

impl<'a> TryFrom<&'a str> for Authority {
	type Error = AuthorityError;

	fn try_from(value: &'a str) -> Result<Self, Self::Error> {
		Self::inner_from_str(value)
	}
}

impl TryFrom<String> for Authority {
	type Error = AuthorityError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		Self::inner_from_str(&value)
	}
}

impl TryFrom<std::net::SocketAddr> for Authority {
	type Error = AuthorityError;

	fn try_from(sockaddr: SocketAddr) -> Result<Self, Self::Error> {
		Self::inner_from_str(&sockaddr.to_string())
	}
}

/// Represent the URL patterns that is whitelisted.
#[derive(Default, Debug, Clone)]
pub struct WhitelistedHosts(Router<Port>);

impl<T> From<T> for WhitelistedHosts
where
	T: IntoIterator<Item = Authority>,
{
	fn from(value: T) -> Self {
		let mut router = Router::new();

		for auth in value.into_iter() {
			router.add(&auth.hostname, auth.port);
		}

		Self(router)
	}
}

impl WhitelistedHosts {
	fn recognize(&self, other: &Authority) -> bool {
		if let Ok(p) = self.0.recognize(&other.hostname) {
			let p = p.handler();

			match (p, &other.port) {
				(Port::Any, _) => true,
				(Port::Default, Port::Default) => true,
				(Port::Fixed(p1), Port::Fixed(p2)) if p1 == p2 => true,
				_ => false,
			}
		} else {
			false
		}
	}
}

/// Policy for validating the `HTTP host header`.
#[derive(Debug, Clone)]
pub enum AllowHosts {
	/// Allow all hosts (no filter).
	Any,
	/// Allow only specified hosts.
	Only(WhitelistedHosts),
}

impl AllowHosts {
	/// Verify a host.
	pub fn verify(&self, value: &str) -> Result<(), Error> {
		let auth = Authority::try_from(value)
			.map_err(|_| Error::HttpHeaderRejected("host", format!("Invalid authority: {value}")))?;

		if let AllowHosts::Only(url_pat) = self {
			if !url_pat.recognize(&auth) {
				return Err(Error::HttpHeaderRejected("host", value.into()));
			}
		}

		Ok(())
	}
}

fn default_port(scheme: Option<&str>) -> Option<u16> {
	match scheme {
		Some("http") | Some("ws") => Some(80),
		Some("https") | Some("wss") => Some(443),
		Some("ftp") => Some(21),
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use super::{AllowHosts, Authority, Port};

	fn authority(host: &str, port: Port) -> Authority {
		Authority { hostname: host.to_owned(), port }
	}

	#[test]
	fn should_parse_valid_authority() {
		assert_eq!(Authority::try_from("http://parity.io").unwrap(), authority("parity.io", Port::Default));
		assert_eq!(Authority::try_from("https://parity.io:8443").unwrap(), authority("parity.io", Port::Fixed(8443)));
		assert_eq!(Authority::try_from("chrome-extension://124.0.0.1").unwrap(), authority("124.0.0.1", Port::Default));
		assert_eq!(Authority::try_from("http://*.domain:*/somepath").unwrap(), authority("*.domain", Port::Any));
		assert_eq!(Authority::try_from("parity.io").unwrap(), authority("parity.io", Port::Default));
		assert_eq!(Authority::try_from("127.0.0.1:8845").unwrap(), authority("127.0.0.1", Port::Fixed(8845)));
		assert_eq!(
			Authority::try_from("http://[2001:db8:85a3:8d3:1319:8a2e:370:7348]:9933/").unwrap(),
			authority("[2001:db8:85a3:8d3:1319:8a2e:370:7348]", Port::Fixed(9933))
		);
		assert_eq!(
			Authority::try_from("http://[2001:db8:85a3:8d3:1319:8a2e:370:7348]/").unwrap(),
			authority("[2001:db8:85a3:8d3:1319:8a2e:370:7348]", Port::Default)
		);
		assert_eq!(
			Authority::try_from("https://user:password@example.com/tmp/foo").unwrap(),
			authority("example.com", Port::Default)
		);
	}

	#[test]
	fn should_not_parse_invalid_authority() {
		assert!(Authority::try_from("/foo/bar").is_err());
		assert!(Authority::try_from("user:password").is_err());
		assert!(Authority::try_from("parity.io/somepath").is_err());
		assert!(Authority::try_from("127.0.0.1:8545/somepath").is_err());
		assert!(Authority::try_from("127.0.0.1:-1337").is_err());
	}

	#[test]
	fn should_allow_when_validation_is_disabled() {
		assert!((AllowHosts::Any).verify("any").is_ok());
	}

	#[test]
	fn should_reject_if_header_not_on_the_list() {
		assert!((AllowHosts::Only(vec![].into())).verify("parity.io").is_err());
	}

	#[test]
	fn should_accept_if_on_the_list() {
		assert!(AllowHosts::Only(vec![Authority::try_from("parity.io").unwrap()].into()).verify("parity.io").is_ok());
	}

	#[test]
	fn should_accept_if_on_the_list_with_port() {
		assert!((AllowHosts::Only(vec![Authority::try_from("parity.io:443").unwrap()].into()))
			.verify("parity.io:443")
			.is_ok());
		assert!(AllowHosts::Only(vec![Authority::try_from("parity.io").unwrap()].into())
			.verify("parity.io:443")
			.is_err());
	}

	#[test]
	fn should_support_wildcards() {
		assert!((AllowHosts::Only(vec![Authority::try_from("*.web3.site:*").unwrap()].into()))
			.verify("parity.web3.site:8180")
			.is_ok());
		assert!((AllowHosts::Only(vec![Authority::try_from("*.web3.site:*").unwrap()].into()))
			.verify("parity.web3.site")
			.is_ok());
	}

	#[test]
	fn should_accept_with_and_without_default_port() {
		assert!(AllowHosts::Only(vec![Authority::try_from("https://parity.io:443").unwrap()].into())
			.verify("https://parity.io")
			.is_ok());

		assert!(AllowHosts::Only(vec![Authority::try_from("https://parity.io").unwrap()].into())
			.verify("https://parity.io:443")
			.is_ok());
	}
}
