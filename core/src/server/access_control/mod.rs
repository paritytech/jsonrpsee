//! Access control based on HTTP headers

pub mod origin;
pub mod host;
mod matcher;

pub use origin::{AllowOrigins, OriginType};
pub use host::{Host, AllowHosts};

use crate::Error;

/// Define access on control on HTTP layer.
#[derive(Clone, Debug)]
pub struct AccessControl {
	allowed_hosts: AllowHosts,
	allowed_origins: AllowOrigins,
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
		self.allowed_origins.verify(origin, host)
	}
}

impl Default for AccessControl {
	fn default() -> Self {
		Self { allowed_hosts: AllowHosts::Any, allowed_origins: AllowOrigins::Any }
	}
}

/// Convenience builder pattern
#[derive(Debug)]
pub struct AccessControlBuilder {
	allowed_hosts: AllowHosts,
	allowed_origins: AllowOrigins,
}

impl Default for AccessControlBuilder {
	fn default() -> Self {
		Self { allowed_hosts: AllowHosts::Any, allowed_origins: AllowOrigins::Any }
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
		self.allowed_origins = AllowOrigins::Any;
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
		let allowed_origins: Vec<OriginType> = list.into_iter().map(Into::into).map(Into::into).collect();
		if allowed_origins.is_empty() {
			return Err(Error::EmptyAllowList("Origin"));
		}
		self.allowed_origins = AllowOrigins::Only(allowed_origins);
		Ok(self)
	}

	/// Finalize the `AccessControl` settings.
	pub fn build(self) -> AccessControl {
		AccessControl {
			allowed_hosts: self.allowed_hosts,
			allowed_origins: self.allowed_origins,
		}
	}
}
