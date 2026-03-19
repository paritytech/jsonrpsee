#!/usr/bin/env bash
set -euo pipefail

echo "=== Native build with default tls feature ==="
cargo check -p jsonrpsee-http-client

echo ""
echo "=== Native build with tls-rustcrypto (no default features) ==="
cargo check -p jsonrpsee-http-client --no-default-features --features tls-rustcrypto

echo ""
echo "=== Native tests with tls ==="
cargo test -p jsonrpsee-http-client --features tls -- transport::tests

echo ""
echo "=== Native tests with tls-rustcrypto ==="
cargo test -p jsonrpsee-http-client --no-default-features --features tls-rustcrypto -- transport::tests

echo ""
echo "=== wasip2 build with tls-rustcrypto ==="
RUSTFLAGS="--cfg tokio_unstable" cargo +nightly build --target wasm32-wasip2 -p jsonrpsee-http-client --no-default-features --features tls-rustcrypto

echo ""
echo "All checks passed."
