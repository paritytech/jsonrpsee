#[derive(Debug)]
/// Wrapper over [`tracing::Span`] to trace individual method calls, notifications and similar.
pub struct RpcTracing(tracing::Span);

impl RpcTracing {
	/// Create a new tracing target.
	///
	/// To enable this you need to call `RpcTracing::new().span().enable()`.
	pub fn new(kind: RpcTracingKind) -> Self {
		let span = match kind {
			RpcTracingKind::MethodCall(method) => tracing::span!(tracing::Level::DEBUG, "method_call", %method),
			RpcTracingKind::Notification(method) => tracing::span!(tracing::Level::DEBUG, "notification", %method),
			RpcTracingKind::Batch => tracing::span!(tracing::Level::DEBUG, "batch"),
		};

		Self(span)
	}

	/// Get the inner span.
	pub fn span(&self) -> &tracing::Span {
		&self.0
	}

	/// Write log
	pub fn write_log_tx<T: std::fmt::Debug>(req: T, len: usize) {
		tracing::debug!(tx_len = len);
		tracing::trace!(tx = ?req);
	}

	/// Write log
	pub fn write_log_rx<T: std::fmt::Debug>(req: T, len: usize) {
		tracing::debug!(rx_len = len);
		tracing::trace!(rx = ?req);
	}
}

#[derive(Debug)]
/// The different kind of tracing targets for JSON-RPC requests.
pub enum RpcTracingKind {
	/// Method call.
	MethodCall(String),
	/// Notification.
	Notification(String),
	/// Batch.
	Batch,
}
