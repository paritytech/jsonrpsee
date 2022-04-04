#[derive(Debug)]
/// Wrapper over [`tracing::Span`] to trace individual method calls, notifications and similar.
pub struct RpcTracing(tracing::Span);

impl RpcTracing {
	/// Create a `method_call` tracing target.
	///
	/// To enable this you need to call `RpcTracing::method_call("some_method").span().enable()`.
	pub fn method_call(method: &str) -> Self {
		Self(tracing::span!(tracing::Level::DEBUG, "method_call", %method))
	}

	/// Create a `notification` tracing target.
	///
	/// To enable this you need to call `RpcTracing::notification("some_method").span().enable()`.
	pub fn notification(method: &str) -> Self {
		Self(tracing::span!(tracing::Level::DEBUG, "notification", %method))
	}

	/// Create a `batch` tracing target.
	///
	/// To enable this you need to call `RpcTracing::batch().span().enable()`.
	pub fn batch() -> Self {
		Self(tracing::span!(tracing::Level::DEBUG, "batch"))
	}

	/// Get the inner span.
	pub fn span(&self) -> &tracing::Span {
		&self.0
	}
}
