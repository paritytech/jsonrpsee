//! Shared test helpers for JSONRPC v2.

#![recursion_limit = "256"]

use std::{future::Future, time::Duration};
use tokio::time::{timeout, Timeout};

pub mod helpers;
pub mod types;

/// Helper extension trait which allows to limit execution time for the futures.
/// It is helpful in tests to ensure that no future will ever get stuck forever.
pub trait TimeoutFutureExt<T>: Future<Output = T> + Sized {
	/// Returns a reasonable value that can be used as a future timeout with a certain
	/// degree of confidence that timeout won't be triggered by the test specifics.
	fn default_timeout() -> Duration {
		// If some future wasn't done in 60 seconds, it's either a poorly written test
		// or (most likely) a bug related to some future never actually being completed.
		const TIMEOUT_SECONDS: u64 = 60;
		Duration::from_secs(TIMEOUT_SECONDS)
	}

	/// Adds a fixed timeout to the future.
	fn with_default_timeout(self) -> Timeout<Self> {
		self.with_timeout(Self::default_timeout())
	}

	/// Adds a custom timeout to the future.
	fn with_timeout(self, timeout_value: Duration) -> Timeout<Self> {
		timeout(timeout_value, self)
	}
}

impl<T, U> TimeoutFutureExt<T> for U where U: Future<Output = T> + Sized {}
