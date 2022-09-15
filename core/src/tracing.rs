use serde::Serialize;
use tracing::Level;

#[derive(Debug)]
/// Wrapper over [`tracing::Span`] to trace individual method calls, notifications and similar.
pub struct RpcTracing(tracing::Span);

impl RpcTracing {
	/// Create a `method_call` tracing target.
	///
	/// To enable this you need to call `RpcTracing::method_call("some_method").span().enable()`.
	pub fn method_call(method: &str) -> Self {
		Self(tracing::span!(tracing::Level::TRACE, "method_call", %method))
	}

	/// Create a `notification` tracing target.
	///
	/// To enable this you need to call `RpcTracing::notification("some_method").span().enable()`.
	pub fn notification(method: &str) -> Self {
		Self(tracing::span!(tracing::Level::TRACE, "notification", %method))
	}

	/// Create a `batch` tracing target.
	///
	/// To enable this you need to call `RpcTracing::batch().span().enable()`.
	pub fn batch() -> Self {
		Self(tracing::span!(tracing::Level::TRACE, "batch"))
	}

	/// Get the inner span.
	pub fn into_span(self) -> tracing::Span {
		self.0
	}
}

/// Helper for writing trace logs from str.
pub fn tx_log_from_str(s: impl AsRef<str>, max: u32) {
	if tracing::enabled!(Level::TRACE) {
		let msg = truncate_at_char_boundary(s.as_ref(), max as usize);
		tracing::trace!(send = msg);
	}
}

/// Helper for writing trace logs from JSON.
pub fn tx_log_from_json(s: &impl Serialize, max: u32) {
	if tracing::enabled!(Level::TRACE) {
		let json = serde_json::to_string(s).unwrap_or_default();
		let msg = truncate_at_char_boundary(&json, max as usize);
		tracing::trace!(send = msg);
	}
}

/// Helper for writing trace logs from str.
pub fn rx_log_from_str(s: impl AsRef<str>, max: u32) {
	if tracing::enabled!(Level::TRACE) {
		let msg = truncate_at_char_boundary(s.as_ref(), max as usize);
		tracing::trace!(recv = msg);
	}
}

/// Helper for writing trace logs from JSON.
pub fn rx_log_from_json(s: &impl Serialize, max: u32) {
	if tracing::enabled!(Level::TRACE) {
		let res = serde_json::to_string(s).unwrap_or_default();
		let msg = truncate_at_char_boundary(res.as_str(), max as usize);
		tracing::trace!(recv = msg);
	}
}

/// Helper for writing trace logs from bytes.
pub fn rx_log_from_bytes(bytes: &[u8], max: u32) {
	if tracing::enabled!(Level::TRACE) {
		let res = serde_json::from_slice::<serde_json::Value>(bytes).unwrap_or_default();
		rx_log_from_json(&res, max);
	}
}

/// Find the next char boundary to truncate at.
fn truncate_at_char_boundary(s: &str, max: usize) -> &str {
	if s.len() < max {
		return s;
	}

	match s.char_indices().nth(max as usize) {
		None => s,
		Some((idx, _)) => &s[..idx],
	}
}

#[cfg(test)]
mod tests {
	use super::truncate_at_char_boundary;

	#[test]
	fn truncate_at_char_boundary_works() {
		assert_eq!(truncate_at_char_boundary("ボルテックス", 0), "");
		assert_eq!(truncate_at_char_boundary("ボルテックス", 4), "ボルテッ");
		assert_eq!(truncate_at_char_boundary("ボルテックス", 100), "ボルテックス");
		assert_eq!(truncate_at_char_boundary("hola-hola", 4), "hola");
	}
}
