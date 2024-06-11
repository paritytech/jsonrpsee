#[cold]
pub fn log_fail_parse(arg_pat: &str, ty: &str, e: &dyn std::fmt::Debug, optional: bool) {
	let optional = if optional { "optional " } else { "" };
	tracing::debug!("Error parsing {optional}\"{arg_pat}\" as \"{ty}\": {e:?}");
}

#[cold]
pub fn log_fail_parse_as_object(e: &dyn std::fmt::Display) {
	tracing::debug!("Failed to parse JSON-RPC params as object: {e}");
}
