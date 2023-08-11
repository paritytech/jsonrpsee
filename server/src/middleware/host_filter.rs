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

use crate::transport::http;
use crate::uri::{Authority, AuthorityError, Port};
use futures_util::{Future, FutureExt, TryFutureExt};
use hyper::{Body, Request, Response};
use route_recognizer::Router;
use std::error::Error as StdError;
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
		let Some(authority) = Authority::from_http_request(&request) else {
			return async { Ok(http::response::malformed()) }.boxed();
		};

		if self.filter.recognize(&authority) {
			Box::pin(self.inner.call(request).map_err(Into::into))
		} else {
			tracing::debug!("Denied request: {:?}", request);
			async { Ok(http::response::host_not_allowed()) }.boxed()
		}
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
			router.add(&auth.host, auth.port);
		}

		Self(router)
	}
}

impl WhitelistedHosts {
	fn recognize(&self, other: &Authority) -> bool {
		if let Ok(p) = self.0.recognize(&other.host) {
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

#[cfg(test)]
mod tests {
	use super::{Authority, WhitelistedHosts};

	fn unwrap_auth(a: &str) -> Authority {
		a.try_into().unwrap()
	}

	fn unwrap_filter(list: &[&str]) -> WhitelistedHosts {
		let l: Vec<_> = list.into_iter().map(|&a| a.try_into().unwrap()).collect();
		WhitelistedHosts::from(l)
	}

	#[test]
	fn should_reject_if_header_not_on_the_list() {
		let filter = unwrap_filter(&[]);
		assert!(!filter.recognize(&unwrap_auth("parity.io")));
	}

	#[test]
	fn should_accept_if_on_the_list() {
		let filter = unwrap_filter(&["parity.io"]);
		assert!(filter.recognize(&unwrap_auth("parity.io")));
	}

	#[test]
	fn should_accept_if_on_the_list_with_port() {
		let filter = unwrap_filter(&["parity.io:443"]);
		assert!(filter.recognize(&unwrap_auth("parity.io:443")));
		assert!(!filter.recognize(&unwrap_auth("parity.io")));
	}

	#[test]
	fn should_support_wildcards() {
		let filter = unwrap_filter(&["*.web3.site:*"]);
		assert!(filter.recognize(&unwrap_auth("parity.web3.site:8180")));
		assert!(filter.recognize(&unwrap_auth("parity.web3.site")));
	}

	#[test]
	fn should_accept_with_and_without_default_port() {
		let filter = unwrap_filter(&["https://parity.io:443"]);
		assert!(filter.recognize(&unwrap_auth("https://parity.io")));
		assert!(filter.recognize(&unwrap_auth("https://parity.io:443")));
	}
}
