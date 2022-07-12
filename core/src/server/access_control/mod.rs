//! Access control based on HTTP headers

pub mod cors;
pub mod host;
mod matcher;

pub use cors::{AllowHeaders, AllowOrigin, Origin};
pub use host::{AllowHosts, Host};

use crate::Error;

use self::cors::get_cors_allow_origin;

/// Define access on control on HTTP layer.
#[derive(Clone, Debug)]
pub struct AccessControl {
	allowed_hosts: AllowHosts,
	allowed_origins: Option<Vec<AllowOrigin>>,
	allowed_headers: AllowHeaders,
}

impl AccessControl {
	/// Validate incoming request by host.
	///
	/// `host` is the return value from the `host header`
	pub fn verify_host(&self, host: &str) -> Result<(), Error> {
		self.allowed_hosts.verify(host)
	}

	/// Validate incoming request by origin.
	///
	/// `host` is the return value from the `host header`
	/// `origin` is the value from the `origin header`.
	pub fn verify_origin(&self, origin: Option<&str>, host: &str) -> Result<(), Error> {
		if let cors::AllowCors::Invalid = get_cors_allow_origin(origin, &self.allowed_origins, Some(host)) {
			Err(Error::HttpHeaderRejected("origin", origin.unwrap_or("<missing>").into()))
		} else {
			Ok(())
		}
	}

	/// Validate incoming request by CORS(`access-control-request-headers`).
	///
	/// header_name: all keys of the header in the request
	/// cors_request_headers: values of `access-control-request-headers` headers.
	pub fn verify_headers<T, I, II>(&self, header_names: I, cors_request_headers: II) -> Result<AllowHeaders, Error>
	where
		T: AsRef<str>,
		I: Iterator<Item = T>,
		II: Iterator<Item = T>,
	{
		let header =
			cors::get_cors_allow_headers(header_names, cors_request_headers, &self.allowed_headers, |name| name);

		match header {
			cors::AllowCors::Invalid => Err(Error::HttpHeaderRejected(
				"access-control-request-headers",
				"<too inefficient to displayed; use wireshark or something similar to find the header values>".into(),
			)),
			cors::AllowCors::NotRequired => Ok(AllowHeaders::Any),
			cors::AllowCors::Ok(headers) => Ok(AllowHeaders::Only(headers.into_iter().map(|h| h.as_ref().to_owned()).collect())),
		}
	}

	/// Return the allowed headers we've set
	pub fn allowed_headers(&self) -> &AllowHeaders {
		&self.allowed_headers
	}
}

impl Default for AccessControl {
	fn default() -> Self {
		Self { allowed_hosts: AllowHosts::Any, allowed_origins: None, allowed_headers: AllowHeaders::Any }
	}
}

/// Convenience builder pattern
#[derive(Debug)]
pub struct AccessControlBuilder {
	allowed_hosts: AllowHosts,
	allowed_origins: Option<Vec<AllowOrigin>>,
	allowed_headers: AllowHeaders,
}

impl Default for AccessControlBuilder {
	fn default() -> Self {
		Self { allowed_hosts: AllowHosts::Any, allowed_origins: None, allowed_headers: AllowHeaders::Any }
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
		self.allowed_origins = None;
		self
	}

	/// Allow all headers.
	pub fn allow_all_headers(mut self) -> Self {
		self.allowed_headers = AllowHeaders::Any;
		self
	}

	/// Configure allowed hosts.
	///
	/// Default - allow all.
	pub fn set_allowed_hosts<List, H>(mut self, list: List) -> Result<Self, Error>
	where
		List: IntoIterator<Item = H>,
		H: Into<String>,
	{
		let allowed_hosts: Vec<_> = list.into_iter().map(|s| Host::parse(&s.into())).map(Into::into).collect();
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
		Origin: Into<String>,
	{
		let allowed_origins: Vec<AllowOrigin> = list.into_iter().map(Into::into).map(Into::into).collect();
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
		self.allowed_headers = AllowHeaders::Only(allowed_headers);
		Ok(self)
	}

	/// Finalize the `AccessControl` settings.
	pub fn build(self) -> AccessControl {
		AccessControl {
			allowed_hosts: self.allowed_hosts,
			allowed_origins: self.allowed_origins,
			allowed_headers: self.allowed_headers,
		}
	}
}
