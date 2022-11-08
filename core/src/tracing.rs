use serde::Serialize;
use tracing::Level;

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

	match s.char_indices().nth(max) {
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
