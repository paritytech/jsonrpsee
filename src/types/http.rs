//! Shared HTTP types

/// Default maximum request body size (10 MB).
const DEFAULT_MAX_BODY_SIZE_TEN_MB: u32 = 10 * 1024 * 1024;

/// HTTP configuration.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HttpConfig {
	/// Maximum request body size in bytes.
	pub max_request_body_size: u32,
}

impl Default for HttpConfig {
	fn default() -> Self {
		Self { max_request_body_size: DEFAULT_MAX_BODY_SIZE_TEN_MB }
	}
}
