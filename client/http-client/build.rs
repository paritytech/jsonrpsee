//! Build script for jsonrpsee-http-client: sets up cfg aliases.

fn main() {
    cfg_aliases::cfg_aliases! {
        wasip2: { all(target_os = "wasi", target_env = "p2") },
    }
}
